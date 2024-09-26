use core::fmt;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::{self, TcpStream},
    sync::{
        self,
        mpsc::{Receiver, Sender},
        Mutex,
    },
};

pub struct Message {
    sender: u128,
    message: String,
}

pub struct BiChan<T> {
    sender: Sender<T>,
    // s_mutex: Mutex<T>,
    receiver: Receiver<T>,
    // r_mutex: Mutex<T>
}

impl<T> BiChan<T> {
    fn new(size: usize) -> Self {
        let (sender, receiver): (Sender<T>, Receiver<T>) = sync::mpsc::channel(size);
        BiChan { sender, receiver }
    }
}

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    CommandFailed(String),
    ConfigurationError(String),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::CommandFailed(cmd) => write!(f, "Command failed: {}", cmd),
            AppError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

async fn accept_conn(ln: Arc<net::TcpListener>, conns_chan: Sender<TcpStream>) {
    loop {
        match ln.accept().await {
            Ok((connection, _)) => {
                println!("connecting");
                conns_chan.send(connection).await.unwrap();
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

async fn handle_client(
    clients: Arc<Mutex<HashMap<u128, TcpStream>>>,
    client_id: u128,
    message_chan: Sender<Message>,
) { 
    loop {
        let mut message = String::new();

        {

            let mut clients_guard = clients.lock().await;
            if let Some(client) = clients_guard.get_mut(&client_id) {
                let mut reader = tokio::io::BufReader::new(client);
                // Read the message from the client
                match reader.read_line(&mut message).await {
                    Ok(0) => {
                        // EOF reached, client disconnected
                        eprintln!("Client disconnected");

                        clients_guard.remove(&client_id);
                        break;
                    }
                    Ok(_) => {
                        println!("Message: {}", message);
                        if message.len() > 1 {
                            let msg_obj = Message {
                                sender: client_id,
                                message: message.clone(),
                            };
                            // Send the message to the message channel
                            if let Err(e) = message_chan.send(msg_obj).await {
                                eprintln!("Error sending message: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from client: {}", e);
                        break;
                    }
                }
            } else {
                // If the client was not found (perhaps already removed)
                break;
            }
        }
    }
}

async fn handle_message(msg: Message, clients: &mut HashMap<u128, TcpStream>) {
    println!("Sending message {}", msg.message);
    let recipients: Vec<u128> = clients.keys().cloned().collect();
    println!("{:?}", recipients);
    for id in recipients {
        println!("For id: {} and message sender {}", id, msg.sender);
        if id != msg.sender {
            let conn = clients.get_mut(&id);
            if conn.is_some() {
                if let Err(e) = conn.unwrap().write_all(msg.message.as_bytes()).await {
                    eprintln!("Failed to send message to client {}: {}", id, e);
                }
            }
        }
    }
}
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let port = "127.0.0.1:8030";

    let ln = match net::TcpListener::bind(port).await {
        Ok(ln) => Arc::new(ln),
        Err(e) => {
            return Err(AppError::CommandFailed(format!(
                "Error creating listener on port {}: {}",
                port, e
            )));
        }
    };

    let mut connection_chan: BiChan<TcpStream> = BiChan::new(400);
    let mut message_chan: BiChan<Message> = BiChan::new(3200);
    
    
    
    let connection_chan_sender = connection_chan.sender.clone(); 
    tokio::task::spawn(async move {
        accept_conn(ln, connection_chan_sender).await;
    });
    
    let clients = Arc::new(Mutex::new(HashMap::<u128, TcpStream>::new()));
    loop {
        tokio::select! {
            Some(conn) = connection_chan.receiver.recv() => {
                // Handle new connection
                let client_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(); 
                let clients_clone = Arc::clone(&clients);  
                let message_chan_clone = message_chan.sender.clone(); 
                let mut clients_guard = clients_clone.lock().await;
                clients_guard.insert(client_id, conn);
                let recipients: Vec<u128> = clients_guard.keys().cloned().collect();
                println!("Number of clients connected: {}, Client ID: {}, item: {:?}, keys: {:?}", clients_guard.len(), client_id, clients_guard.get_key_value(&client_id).unwrap(), recipients);
                drop(clients_guard);
                
                // Spawn task to handle client
                tokio::spawn(async move {

                    handle_client(clients_clone, client_id, message_chan_clone).await;
                });

            },
            Some(msg) = message_chan.receiver.recv() => {
                println!("{}", msg.message);
                let clients_clone = Arc::clone(&clients);

                tokio::spawn(async move {
                    let mut clients_guard = clients_clone.lock().await;
                    handle_message(msg, &mut clients_guard).await;
                    drop(clients_guard);
                });
            }
        }
    }
}

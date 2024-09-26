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
    sender: i32,
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
    clients: &mut HashMap<i32, TcpStream>,
    client_id: i32,
    message_chan: Sender<Message>,
) {
    let client = clients.remove(&client_id).unwrap();
    let mut reader = tokio::io::BufReader::new(client);

    loop {
        let mut message = String::new();

        // Read the message from the client
        match reader.read_line(&mut message).await {
            Ok(0) => {
                eprintln!("Client disconnected");

                let mut client = reader.into_inner();

                if let Err(e) = client.shutdown().await {
                    eprintln!("Failed to shutdown client: {}", e);
                }
                break;
            }
            Ok(_) => {
                println!("Message: {}", message);
                if message.len() > 1 {
                    let msg_obj = Message {
                        sender: client_id,
                        message,
                    };
                    match message_chan.send(msg_obj).await {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error sending message: {}", e);
                        }
                    };
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
}

async fn handle_message(msg: Message, clients: &mut HashMap<i32, TcpStream>) {
    let recipients: Vec<i32> = clients.keys().cloned().collect();
    for id in recipients {
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
    let clients = Arc::new(Mutex::new(HashMap::<i32, TcpStream>::new()));



    let connection_chan_sender = connection_chan.sender.clone(); 
    tokio::task::spawn(async move {
        accept_conn(ln, connection_chan_sender).await;
    });

    loop {
        tokio::select! {
            Some(conn) = connection_chan.receiver.recv() => {
                // Handle new connection
                let client_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as i32; 
                let clients_clone = Arc::clone(&clients);  
                let message_chan_clone = message_chan.sender.clone(); 

                // Spawn task to handle client
                tokio::spawn(async move {
                    let mut clients_guard = clients_clone.lock().await;
                    clients_guard.insert(client_id, conn);
                    println!("Number of clients connected: {}, Client ID: {}", clients_guard.len(), client_id);

                    handle_client(&mut clients_guard, client_id, message_chan_clone).await;
                });

            },
            Some(msg) = message_chan.receiver.recv() => {
                println!("{}", msg.message);
                let clients_clone = Arc::clone(&clients);

                tokio::spawn(async move {
                    let mut clients_guard = clients_clone.lock().await;
                    handle_message(msg, &mut clients_guard).await;
                });
            }
        }
    }
}

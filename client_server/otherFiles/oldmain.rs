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
#[derive(Debug)]
pub struct Message {
    sender: i32,
    message: String,
}
#[derive(Debug)]
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
pub struct Clients {
    listener: net::TcpListener,
    clients: HashMap<i32, TcpStream>,
    client_channel: BiChan<Arc<(i32, TcpStream)>>,
    msg_channel: BiChan<Message>,
}

impl Clients {
    async fn new() -> Arc<Mutex<Clients>> {
        let port = "127.0.0.1:8030";

        let listener = net::TcpListener::bind(port).await.unwrap();
        let clients: HashMap<i32, TcpStream> = HashMap::new();
        let client_channel: BiChan<Arc<(i32, TcpStream)>> = BiChan::new(400);
        let msg_channel: BiChan<Message> = BiChan::new(400);

        Arc::new(Mutex::new(Clients {
            listener,
            clients,
            client_channel,
            msg_channel,
        }))
    }

    async fn handle_client(client_obj: Arc<Mutex<Self>>) {
        let clients = Arc::clone(&client_obj);
        let client_channel_sender = client_obj.lock().await.client_channel.sender.clone();
        let msg_channel_sender = client_obj.lock().await.msg_channel.sender.clone();

        let clients_clone = Arc::clone(&clients);
        tokio::spawn(async move {
            loop {
                match clients_clone.lock().await.listener.accept().await {
                    Ok((connection, _)) => {
                        println!("new connection!");
                        let client_id = SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_micros() as i32;
                        let connection = Arc::new(connection);
                        let client = Arc::clone(&connection);
                        let client_b = Arc::clone(&connection);

                        clients_clone
                            .lock()
                            .await
                            .clients
                            .insert(client_id, Arc::try_unwrap(client).unwrap());

                        println!("Number of clients connected: {}, Client ID: {}", clients_clone.lock().await.clients.len(), client_id);

                        if let Err(e) = client_channel_sender
                            .send(Arc::new((client_id, Arc::try_unwrap(client_b).unwrap())))
                            .await
                        {
                            eprintln!("Error sending client to channel: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                };
            }
        });

        let clients_clone = Arc::clone(&client_obj);
        tokio::spawn(async move {
            while let Some(client) = clients_clone.lock().await.client_channel.receiver.recv().await {
                let (client_id, mut client_b) = Arc::try_unwrap(client).unwrap();
                let mut reader = tokio::io::BufReader::new(&mut client_b);

                loop {
                    let mut message = String::new();

                    // Read the message from the client
                    match reader.read_line(&mut message).await {
                        Ok(0) => {
                            eprintln!("Client disconnected");

                            let client = reader.into_inner();

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
                                if let Err(e) = msg_channel_sender.send(msg_obj).await {
                                    eprintln!("Error sending message: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading message: {}", e);
                        }
                    };
                }
            }
        });

        // let c = client_obj.lock().await
        let clients_clone = Arc::clone(&client_obj);
        tokio::spawn(async move {
            loop {
                if let Some(msg) = clients_clone.lock().await.msg_channel.receiver.recv().await {
                    let recipients: Vec<i32> = clients_clone.lock().await.clients.keys().cloned().collect();
                    for id in recipients {
                        if id != msg.sender {
                            let mut clients = clients_clone.lock().await;
                            if let Some(conn) = clients.clients.get_mut(&id) {
                                if let Err(e) = conn.write_all(msg.message.as_bytes()).await {
                                    eprintln!("Failed to send message to client {}: {}", id, e);
                                }
                            }
                        }
                    }
                }
            }
        });
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

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let clients = Clients::new().await;

    // Start handling clients
    loop {
        let clients_clone = Arc::clone(&clients);
        Clients::handle_client(clients_clone).await;
    }
    

    Ok(())
}

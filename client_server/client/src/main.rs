use core::fmt;
use tokio::{
    self,
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::{
        self,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

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

async fn read(conn: &mut OwnedReadHalf, should_exit: &mut bool) {
    //reader to read from connection
    let mut reader = tokio::io::BufReader::new(conn);
    loop {
        let mut message = String::new();
        match reader.read_line(&mut message).await {
            Ok(_) => match message.as_str() {
                "exit\n" => {
                    *should_exit = true;
                    break;
                }
                _ => {
                    println!("Received: {}", message);
                }
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        };
    }
}

async fn write(conn: OwnedWriteHalf) {
    // get input from command line and write to connection
    println!("Enter message:");
    let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
    let mut writer = tokio::io::BufWriter::new(conn);
    
    loop {
        let mut message = String::new();
        
        match reader.read_line(&mut message).await {
            Ok(0) => {
                println!("Connection closed.");
                break;
            }
            Ok(_) => {
                if !message.ends_with('\n') {
                    message.push('\n');
                }
                match writer.write_all(message.as_bytes()).await {
                    Ok(_) => {
                        println!("Sent: {}", message);
                    }
                    Err(e) => {
                        eprintln!("Failed to send message: {}", e);
                        break;
                    }
                };

                if let Err(e) = writer.flush().await {
                    eprintln!("Failed to flush the writer: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), AppError> {
    // remove lifetime and change error to custom error type
    // server connection

    let conn = match net::TcpStream::connect("127.0.0.1:8030").await {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(AppError::Io(e));
        }
    };

    let (mut conn_read, conn_write) = conn.into_split();

    let mut should_exit = false;

    tokio::task::spawn(async move { write(conn_write).await });
    loop {
        if should_exit {
            break;
        }

        // TODO read()
        read(&mut conn_read, &mut should_exit).await
    }

    Ok(())
}

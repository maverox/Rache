use crate::{
    common_enums::{Request, Response},
    storage::LSMTree,
};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    pub lsm_tree: Arc<Mutex<LSMTree>>,
}

impl Server {
    pub fn new(lsm_tree: LSMTree) -> Self {
        Server {
            lsm_tree: Arc::new(Mutex::new(lsm_tree)),
        }
    }
    
    async fn handle_client(&self, mut socket: TcpStream) {
        let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);
        let mut buffer = Vec::new();

        loop {
            buffer.clear();
            let bytes_read = reader.read_buf(&mut buffer).await.unwrap();
            if bytes_read == 0 {
                break;
            }

            let mut de = Deserializer::new(&buffer[..]);
            let command: Result<Request, _> = Deserialize::deserialize(&mut de);
            let response = match command {
                Ok(Request::Read { key }) => {
                    let lsm_tree = self.lsm_tree.lock().unwrap();
                    match lsm_tree.read(&key) {
                        Ok(Some(value)) => Response::Success(Some(value)),
                        Ok(None) => Response::Success(None),
                        Err(e) => Response::Error(e.to_string()),
                    }
                }
                Ok(Request::Write { key, value }) => {
                    let mut lsm_tree = self.lsm_tree.lock().unwrap();
                    match lsm_tree.write(key, value) {
                        Ok(_) => Response::Success(None),
                        Err(e) => Response::Error(e.to_string()),
                    }
                }
                Ok(Request::Delete { key }) => {
                    let mut lsm_tree = self.lsm_tree.lock().unwrap();
                    match lsm_tree.write(key, "".to_string()) {
                        Ok(_) => Response::Success(None),
                        Err(e) => Response::Error(e.to_string()),
                    }
                }
                Err(e) => Response::Error(e.to_string()),
            };

            let mut buf = Vec::new();
            response.serialize(&mut Serializer::new(&mut buf)).unwrap();
            writer.write_all(&buf).await.unwrap();
        }
    }

    pub async fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Server running on {}", addr);

        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let server = self.clone();
            tokio::spawn(async move {
                server.handle_client(socket).await;
            });
        }
    }
}

impl Clone for Server {
    fn clone(&self) -> Self {
        Server {
            lsm_tree: Arc::clone(&self.lsm_tree),
        }
    }
}

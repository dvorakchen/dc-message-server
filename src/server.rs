use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;

use dvorak_message::message::{Message, MessageType};

type ClientStore = Arc<RwLock<HashMap<String, TcpStream>>>;

pub(crate) struct Server {
    keeping_clients: ClientStore,
    listener: TcpListener,
}

impl Server {
    pub async fn new(host: &str) -> Self {
        let listener = TcpListener::bind(host).await.unwrap();
        Self {
            keeping_clients: Arc::new(RwLock::new(HashMap::new())),
            listener,
        }
    }

    pub async fn listen(&mut self) {
        loop {
            let (mut client_stream, _) = self.listener.accept().await.unwrap();

            let first = Server::check_login(&mut client_stream).await;
            if first.is_err() {
                continue;
            }

            let username = first.unwrap();
            self.keeping_clients
                .write()
                .await
                .insert(username.clone(), client_stream);
            let store = Arc::clone(&self.keeping_clients);

            tokio::spawn(async move {
                _ = Server::listen_client(username, store);
            });
        }
    }

    async fn listen_client(username: String, store: ClientStore) -> Result<(), ()> {
        loop {
            let message = {
                let mut hm = store.write().await;
                let stream = hm.get_mut(&username).ok_or_else(|| ())?;

                Message::read_from(stream)
                    .await
                    .map_err(|_| ())?
                    .ok_or_else(|| ())?
            };

            match &message.message_type {
                MessageType::Text(_) => {
                    let receiver = &message.receiver.clone();

                    let mut hm = store.write().await;
                    let receiver_stream = hm.get_mut(receiver);
                    if receiver_stream.is_none() {
                        println!("{} offline!", receiver);
                        let offline_message = Message::new(
                            MessageType::Text(format!("{} offline!", receiver)),
                            "<Server>".to_string(),
                            username.clone(),
                        );

                        let stream = hm.get_mut(&username).ok_or_else(|| ())?;
                        Message::send(stream, offline_message).await.unwrap();
                        continue;
                    }

                    let receiver_stream = receiver_stream.unwrap();
                    Message::send(receiver_stream, message).await.unwrap();
                }
                MessageType::Logout => {
                    let mut hm = store.write().await;
                    hm.remove(&username);
                    println!("{} logged out!", username);
                    break;
                }
                _ => continue,
            }
        }
        Ok(())
    }

    async fn check_login(tcp_stream: &mut TcpStream) -> Result<String, ()> {
        let message = Message::read_from(tcp_stream).await.unwrap();
        if message.is_none() {
            return Err(());
        }
        let message = message.unwrap();
        if message.message_type != MessageType::Login {
            return Err(());
        }

        Ok(message.username)
    }
}

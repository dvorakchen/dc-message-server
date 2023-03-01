use once_cell::sync::Lazy;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

static mut CONNECTING_CLIENT: Lazy<Mutex<HashMap<String, TcpStream>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn keep_client(username: String, tcp_stream: TcpStream) {
    unsafe {
        CONNECTING_CLIENT
            .lock()
            .await
            .insert(username, tcp_stream);
    }
}

pub async fn release_client(username: String) {
    unsafe {
        CONNECTING_CLIENT.lock().await.remove(&username);
    }
}
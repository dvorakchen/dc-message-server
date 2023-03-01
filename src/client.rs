use tokio::net::TcpStream;
use dvorak_message::message::{Message, MessageType};

/// check the next message type is [`MessageType::Login`]
/// 
/// returns username that [`MessageType::Login`], otherwise Err(())
pub(crate) async fn check_login(tcp_stream: &mut TcpStream) -> Result<String, ()> {
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
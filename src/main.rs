use dvorak_message::message::{Message, MessageType};
use once_cell::sync::OnceCell;
use tokio::net::{TcpListener, TcpStream};

mod args;
mod client;
mod global;

static ARGS: OnceCell<args::Args> = OnceCell::new();

#[tokio::main]
async fn main() {
    ARGS.set(args::Args::parse()).unwrap();

    let host = ARGS.get().unwrap().host.clone();
    let tcp = TcpListener::bind(host).await.unwrap();

    loop {
        let (mut stream, _) = tcp.accept().await.unwrap();

        let first = client::check_login(&mut stream).await;
        if first.is_err() {
            continue;
        }

        let username = first.unwrap();
        
        global::keep_client(username, stream).await;

        // tokio::spawn(async move {
        //     handle_client(message.username).await;
        // });
    }
}

// async fn handle_client(username: String) {
//     loop {
//         let mut tt = unsafe { CONNECTING_CLIENT.lock().await};
//         let v = tt.get_mut(&username);
//         if v.is_none() {
//             return;
//         }

//         let stream = v.unwrap();

//         let message = Message::read_from(stream).await.unwrap();

//         if message.is_none() {
//             return;
//         }

//         let message = message.unwrap();

//         let data = String::from_utf8(message.message_type.as_bytes().to_vec()).unwrap();
//         println!("Received: {data}",);

//         Message::send(stream, message).await.unwrap();
//     }
// }

// use futures_util::sink::SinkExt;
// use std::error::Error;
// use std::net::SocketAddr;
// use tokio::net::{TcpListener, TcpStream};
// use tokio::sync::broadcast::{channel, Sender};
// use tokio_websockets::{Message, ServerBuilder, WebsocketStream};
//
// /// E55.2 - Broadcast Chat Application - Server
// ///
//
// async fn handle_connection(
//     addr: SocketAddr,
//     mut ws_stream: WebsocketStream<TcpStream>,
//     bcast_tx: Sender<String>,
// ) -> Result<(), Box<dyn Error + Send + Sync>> {
//     ws_stream.send(Message::text(String::from("Welcome to the chat! Please type a message"))).await?;
//     let mut bcast_rx = bcast_tx.subscribe();
//
//     // A continuous loop for concurrently performing two tasks: (1) receiving
//     // messages from `ws_stream` and broadcasting them, and (2) receiving
//     // messages on `bcast_rx` and sending them to the client
//     loop {
//         tokio::select! {
//             incoming = ws_stream.next() => {
//                 match incoming {
//                     Some(Ok(msg)) => {
//                         let msg = msg.as_text()?;
//                         println!("From  client {addr:?} {msg:?}");
//                         bcast_tx.send(msg.into())?;
//                         // Broadcast messages to all clients, but the sender of the message.
//                         bcast_rx.recv().await?;
//                     }
//                     Some(Err(err)) => return Err(err.into()),
//                     None => return Ok(()),
//                 }
//             }
//             outgoing = bcast_rx.recv() => {
//                 ws_stream.send(Message::text(outgoing?)).await?;
//             }
//         }
//     }
//     // NOTE: For the broadcast MPMC channel we are working with: All data sent on Sender will
//     // become available on every active Receiver in the same order as it was sent.
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
//     let (bcast_tx, _) = channel(16);
//
//     let listener = TcpListener::bind("127.0.0.1:2000").await?;
//     println!("listening on port 2000");
//
//     loop {
//         let (socket, addr) = listener.accept().await?;
//         println!("New connection from {addr:?}");
//         let bcast_tx = bcast_tx.clone();
//         tokio::spawn(async move {
//             // Wrap the raw TCP stream into a websocket.
//             let ws_stream = ServerBuilder::new().accept(socket).await?;
//
//             handle_connection(addr, ws_stream, bcast_tx).await
//         });
//     }
// }

fn main() {
    println!("Hello, world!");
}
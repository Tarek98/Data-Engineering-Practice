use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::select;
use tokio_websockets::{ClientBuilder, Message};

/// E55.2 - Broadcast Chat Application - Client
///

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let mut ws_stream = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    // Continuous loop for concurrently sending and receiving messages.
    loop {
        select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => println!("From server: {}", msg.as_text()?),
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }
            outgoing = stdin.next_line() => {
                match outgoing {
                    Ok(Some(line)) => ws_stream.send(Message::text(line.to_string())).await?,
                    Err(err) => return Err(err.into()),
                    Ok(None) => return Ok(()),
                }
            }
        }
    }
}
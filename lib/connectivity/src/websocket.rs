use futures_util::{SinkExt, StreamExt};
use log::{trace, warn};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error(transparent)]
    SendError(#[from] mpsc::error::SendError<Message>),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Client {
    send_tx: mpsc::UnboundedSender<Message>,
    recv_rx: mpsc::UnboundedReceiver<Message>,
    cancel: CancellationToken,
}

impl Client {
    pub async fn new(url: &'static str) -> Result<Self> {
        // TODO: add connection timeout

        let stream = connect_async(url).await?.0;
        let (sink, stream) = stream.split();

        let (recv_tx, recv_rx) = mpsc::unbounded_channel();
        let (send_tx, send_rx) = mpsc::unbounded_channel();

        let cancel = CancellationToken::new();

        tokio::spawn(run(cancel.clone(), send_rx, sink, stream, recv_tx));

        Ok(Self {
            send_tx,
            recv_rx,
            cancel,
        })
    }

    pub fn disconnect(&self) {
        self.cancel.cancel();
    }

    /// Returns error if the connection is closed
    pub fn send(&self, message: String) -> Result<()> {
        self.send_tx.send(Message::Text(message))?;
        Ok(())
    }

    pub fn send_binary(&self, message: Vec<u8>) -> Result<()> {
        self.send_tx.send(Message::Binary(message))?;
        Ok(())
    }

    /// Returns None if the connection is closed
    pub async fn receive(&mut self) -> Option<Message> {
        self.recv_rx.recv().await
    }
}

async fn run(
    cancel: CancellationToken,
    mut send_rx: mpsc::UnboundedReceiver<Message>,
    mut sink: futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        Message,
    >,
    mut stream: futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
    >,
    recv_tx: mpsc::UnboundedSender<Message>,
) {
    loop {
        tokio::select! {
            msg = send_rx.recv() => match msg {
                Some(msg) => {
                    if let Err(error) = sink.send(msg).await {
                        warn!("Error while transmitting message: {}", error);
                    }
                }
                None => {
                    trace!("Received None from send channel");
                    break;
                }
            },
            msg = stream.next() => match msg {
                Some(Ok(msg)) => match msg {
                    Message::Text(_) | Message::Binary(_) => {
                        if let Err(error) = recv_tx.send(msg) {
                            warn!("Error passing message to receive channel: {}", error);
                        }
                    }
                    Message::Ping(data) => {
                        if let Err(error) = sink.send(Message::Pong(data)).await {
                            warn!("Error sending pong: {}", error);
                        }
                    }
                    Message::Close(_) => {
                        trace!("Received close message");
                        break;
                    }
                    _ => {
                        warn!("Received unexpected message: {:?}", msg);
                    }
                },
                Some(Err(error)) => {
                    warn!("Error receiving message: {}", error);
                }
                None => {
                    trace!("Stream closed by server");
                    break;
                }
            },
            _ = cancel.cancelled() => {
                trace!("Cancelled");
                break;
            }
        }
    }
    trace!("Cleaning up");
    let _ = sink.send(Message::Close(None)).await;
    let _ = sink.close().await;
    drop(recv_tx);
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[tokio::test]
    async fn test() {
        // TODO: replace test with local server later

        let mut client = Client::new("wss://stream.binance.com:9443/ws")
            .await
            .unwrap();

        // Subscribe to btcusdt@depth@100ms
        client
            .send(r#"{"method": "SUBSCRIBE","params": ["btcusdt@depth@100ms"],"id": 1}"#.to_owned())
            .unwrap();

        // Should get result of subscription
        let msg = client.receive().await.unwrap();
        assert_eq!(msg, Message::Text(r#"{"result":null,"id":1}"#.to_owned()));

        // Give some time to receive messages and then disconnect
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        client.disconnect();

        let mut received = false;
        loop {
            let msg = client.receive().await;
            if msg.is_none() {
                break;
            }

            let msg: Value = serde_json::from_str(msg.unwrap().to_text().unwrap()).unwrap();
            assert_eq!(msg["e"], "depthUpdate");
            assert_eq!(msg["s"], "BTCUSDT");

            received = true;
        }
        assert!(received);

        // try to send message after disconnect
        let result = client.send("test".to_owned());
        assert!(result.is_err());
    }
}

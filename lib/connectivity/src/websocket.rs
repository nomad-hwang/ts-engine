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
        let (mut sink, mut stream) = stream.split();

        let (recv_tx, recv_rx) = mpsc::unbounded_channel();
        let (send_tx, mut send_rx) = mpsc::unbounded_channel();

        let cancel = CancellationToken::new();
        let cancle_clone = cancel.clone();

        // TODO: think later: what would be better(directly use the sink and stream or use the channels)

        tokio::spawn(async move {
            let cancel = cancle_clone;

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
                        Some(Ok(Message::Text(_))) | Some(Ok(Message::Binary(_))) => {
                            if let Err(error) = recv_tx.send(msg.unwrap().unwrap()) {
                                warn!("Error passing message to receive channel: {}", error);
                            }
                        }
                        Some(Ok(Message::Ping(ping))) => {
                            if let Err(error) = sink.send(Message::Pong(ping)).await {
                                warn!("Error sending pong: {}", error);
                            }
                        }
                        Some(Ok(Message::Close(_))) => {
                            trace!("Received close message");
                            break;
                        }
                        Some(Ok(_)) => { // Pong, Frame, etc.
                            warn!("Received unexpected message: {:?}", msg);
                        }
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

            // TODO: Do i need to send remaining messages in send_tx to sink and wait for response?

            let _ = sink.send(Message::Close(None)).await;
            let _ = sink.close().await;

            // drop channels
            drop(recv_tx);
        });

        Ok(Self {
            send_tx,
            recv_rx,
            cancel,
        })
    }

    pub async fn disconnect(&self) {
        self.cancel.cancel();
    }

    pub async fn send(&self, message: &str) -> Result<()> {
        self.send_tx.send(Message::Text(message.to_owned()))?;
        Ok(())
    }

    pub async fn receive(&mut self) -> Option<Message> {
        self.recv_rx.recv().await
    }
}

// Test with wss://echo.websocket.org

#[cfg(test)]
mod tests {
    use super::*;
    // use std::time::Duration;
    // use tokio::time::sleep;

    #[tokio::test]
    async fn test() {
        // binance wss
        let mut client = Client::new("wss://stream.binance.com:9443/ws/btcusdt@trade")
            .await
            .unwrap();

        // receive
        let msg = client.receive().await.unwrap();
        println!("Received: {:?}", msg);

        // TODO: add more tests
    }
}

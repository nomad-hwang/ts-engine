pub mod crypto;
pub mod error;

use async_trait::async_trait;
use domain::security::Security;
use error::Result;

pub enum Event {
    // Trade(Trade),
    Trade(),
    // OrderBookUpdate(OrderBookUpdate),
    OrderBookUpdate(),
}

#[async_trait]
pub trait Client {
    async fn get_all_pairs(&self) -> Result<Vec<Security>>;
}

#[async_trait]
pub trait Stream {
    async fn subscribe(&mut self, pairs: Vec<Security>) -> Result<()>;
    async fn unsubscribe(&mut self, pairs: Vec<Security>) -> Result<()>;
    async fn get_subscription(&self) -> Result<Vec<Security>>;

    async fn next_event(&mut self) -> Result<Event>;
}

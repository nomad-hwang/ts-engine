pub mod crypto;
pub mod error;

use async_trait::async_trait;
use domain::{orderbook::OrderBookUpdate, pair::Pair, trade::Trade};
use error::Result;

pub enum Event {
    Trade(Trade),
    OrderBookUpdate(OrderBookUpdate),
}

#[async_trait]
pub trait Client {
    async fn get_all_pairs(&self) -> Result<Vec<Pair>>;
}

#[async_trait]
pub trait Stream {
    async fn subscribe(&mut self, pairs: Vec<Pair>) -> Result<()>;
    async fn unsubscribe(&mut self, pairs: Vec<Pair>) -> Result<()>;
    async fn get_subscription(&self) -> Result<Vec<Pair>>;

    async fn next_event(&mut self) -> Result<Event>;
}

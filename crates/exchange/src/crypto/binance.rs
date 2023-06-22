use async_trait::async_trait;
use domain::pair::Pair;

use crate::{error::Result, Event, Stream};

pub struct Binance {}

#[async_trait]
impl Stream for Binance {
    async fn subscribe(&mut self, pairs: Vec<Pair>) -> Result<()> {
        // not implemented
        Ok(())
    }

    async fn unsubscribe(&mut self, pairs: Vec<Pair>) -> Result<()> {
        // not implemented
        Ok(())
    }

    async fn get_subscription(&self) -> Result<Vec<Pair>> {
        // not implemented
        Ok(Vec::new())
    }

    async fn next_event(&mut self) -> Result<Event> {
        panic!("not implemented")
    }
}

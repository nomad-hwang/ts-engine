use async_trait::async_trait;
use domain::security::Security;

use crate::{error::Result, Event, Stream};

pub struct Binance {}

#[async_trait]
impl Stream for Binance {
    async fn subscribe(&mut self, security: Vec<Security>) -> Result<()> {
        panic!("not implemented")
    }

    async fn unsubscribe(&mut self, security: Vec<Security>) -> Result<()> {
        panic!("not implemented")
    }

    async fn get_subscription(&self) -> Result<Vec<Security>> {
        panic!("not implemented")
    }

    async fn next_event(&mut self) -> Result<Event> {
        panic!("not implemented")
    }
}

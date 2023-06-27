use crate::pair::Pair;

pub enum Side {
    Buy,
    Sell,
}

pub struct Trade {
    pub pair: Pair,
    pub price: f64,
    pub quantity: f64,
    pub timestamp: u64,
    pub trade_id: u32,
    pub side: Side,
}

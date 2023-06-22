pub type Price = f64;
pub type Quantity = f64;

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Entry {
    price: Price,
    quantity: Quantity,
}

pub struct OrderBookUpdate {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

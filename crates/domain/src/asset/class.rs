use std::fmt;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum AssetClass {
    Equity,
    Currency,
    Cryptocurrency,
}

impl fmt::Display for AssetClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssetClass::Equity => write!(f, "Equity"),
            AssetClass::Currency => write!(f, "Currencies"),
            AssetClass::Cryptocurrency => write!(f, "Cryptocurrencies"),
        }
    }
}

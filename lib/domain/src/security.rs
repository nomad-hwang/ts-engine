use smartstring::alias::CompactString as CString;

use crate::{currency::Currency, exchange::Exchange};

pub enum SecurityType {
    Base,
    Equity,
    Option,
    Commodity,
    Future,
    Forex,
    Cfd,
    Index,
    Crypto,
    CryptoFuture,
}

pub type Symbol = CString;

pub struct Security {
    symbol: Symbol,
    name: CString,
    quote: Currency,
    market: Exchange,
}

impl Security {
    pub fn new(symbol: &str, name: &str, quote: Currency, market: Exchange) -> Self {
        Self {
            symbol: CString::from(symbol),
            name: CString::from(name),
            quote,
            market,
        }
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn name(&self) -> &CString {
        &self.name
    }

    pub fn quote(&self) -> &Currency {
        &self.quote
    }

    pub fn market(&self) -> &Exchange {
        &self.market
    }
}

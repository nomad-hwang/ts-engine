use once_cell::sync::Lazy;
use smartstring::alias::CompactString;

use crate::macros;

pub type CurrencyCode = CompactString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Currency {
    code: CurrencyCode,
    name: String,
    symbol: String,
}

impl Currency {
    pub fn new(code: &str, name: &str, symbol: &str) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            symbol: symbol.into(),
        }
    }

    macros::get!(code, CurrencyCode);
    macros::get!(name, String);
    macros::get!(symbol, String);

    // TODO: Add more currencies
    macros::currency!("USD", "United States Dollar", "$");
    macros::currency!("KRW", "South Korean Won", "₩");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency() {
        assert_eq!(
            Currency::USD(),
            &Currency::new("USD", "United States Dollar", "$")
        );

        assert_eq!(Currency::USD().get_code(), "USD");
        assert_eq!(Currency::USD().get_name(), "United States Dollar");
        assert_eq!(Currency::USD().get_symbol(), "$");
    }
}

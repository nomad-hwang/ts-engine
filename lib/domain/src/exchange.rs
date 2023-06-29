use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use smartstring::alias::CompactString as CString;

use crate::country::Country;
use crate::macros;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExchangeCode {
    Mic(CString),    // Market Identifier Code // TODO: Define type for this
    Crypto(CString), // Cryptocurrency exchange code - which is not defined by ISO 10383
}

pub struct Exchange {
    name: CString,
    code: ExchangeCode,
    country: Country,
    timezone: Tz,
}

impl Exchange {
    pub fn new(name: &str, code: ExchangeCode, country: Country, timezone: Tz) -> Self {
        Self {
            name: CString::from(name),
            code,
            country,
            timezone,
        }
    }

    macros::get!(name, CString);
    macros::get!(code, ExchangeCode);
    macros::get!(country, Country);
    macros::get!(timezone, Tz);

    pub fn time(&self, now: DateTime<Utc>) -> DateTime<Tz> {
        now.with_timezone(&self.timezone)
    }
}

#[cfg(test)]
mod tests {
    use crate::country::US;

    use super::*;

    #[test]
    fn test_market() {
        let market = Exchange {
            name: "New York Stock Exchange".into(),
            code: ExchangeCode::Mic("NYSE".into()),
            country: US.clone(),
            timezone: "America/New_York".parse().unwrap(),
        };

        assert_eq!(market.get_name(), "New York Stock Exchange");
        assert_eq!(market.get_code(), ExchangeCode::Mic("NYSE".into()));
        assert_eq!(market.get_country(), US);
        assert_eq!(market.get_timezone(), "America/New_York".parse().unwrap());

        let now = Utc::now();

        // convert back to UTC to compare
        assert_eq!(market.time(now).with_timezone(&Utc), now);
    }
}

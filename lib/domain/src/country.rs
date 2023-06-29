use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Country(&'static str);

impl From<&'static str> for Country {
    fn from(country: &'static str) -> Self {
        Self(country)
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

macro_rules! create_country {
    ($($country:ident),*) => {
        $(
            pub const $country: Country = Country(stringify!($country));
        )*
    };
}

// TODO: Add more countries.
// ISO 3166-1 alpha-2: https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2
create_country!(KR, US, JP, XX);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_country() {
        assert_eq!(US, "US".into()); // From<&str>

        // PartialEq
        assert_ne!(US, JP);

        // Clone
        assert_eq!(US, US.clone());

        // Display
        assert_eq!(format!("{}", US), "US");
    }
}

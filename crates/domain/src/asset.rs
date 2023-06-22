use self::class::AssetClass;
pub mod class;
pub mod table;

pub type AssetId = u64;

#[derive(Debug, Clone)]
pub struct Asset {
    id: AssetId, // Unique identifier for the asset //!(valid only for the lifetime of the application)
    name: String, // Full name of the asset
    symbol: String, // Symbol of the asset
    class: AssetClass,
}

impl PartialEq for Asset {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Asset {
    pub fn new(id: AssetId, name: String, symbol: String, class: AssetClass) -> Self {
        Self {
            id,
            name,
            symbol,
            class,
        }
    }

    pub fn id(&self) -> AssetId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn class(&self) -> AssetClass {
        self.class
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset() {
        let asset = Asset::new(
            0,
            "Bitcoin".to_string(),
            "BTC".to_string(),
            AssetClass::Cryptocurrency,
        );

        assert_eq!(asset.id(), 0);
        assert_eq!(asset.name(), "Bitcoin");
        assert_eq!(asset.symbol(), "BTC");
        assert_eq!(asset.class(), AssetClass::Cryptocurrency);

        assert_eq!(
            asset,
            Asset::new(
                0,
                "Bitcoin".to_string(),
                "BTC".to_string(),
                AssetClass::Cryptocurrency
            )
        );
    }
}

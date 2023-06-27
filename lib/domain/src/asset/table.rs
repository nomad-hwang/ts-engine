use std::collections::HashMap;

use parking_lot::RwLock;

use super::{class::AssetClass, Asset, AssetId};

pub struct AssetTable {
    currencies: Vec<Asset>,
    currency_ids: HashMap<String, AssetId>,
    rwlock: RwLock<()>,
}

impl AssetTable {
    // TODO: I am not sure about using async on the table.
    // TODO: it might block whole event loop on adding new asset(but it's not common i guess?)

    pub fn new() -> Self {
        Self {
            currencies: Vec::new(),
            currency_ids: HashMap::new(),
            rwlock: RwLock::new(()),
        }
    }

    pub fn add(&mut self, name: String, symbol: String, class: AssetClass) -> AssetId {
        let key = name.clone() + &class.to_string();
        let _ = self.rwlock.write();

        if let Some(id) = self.currency_ids.get(&key) {
            return *id;
        }

        let id = self.currencies.len() as AssetId;

        self.currencies.push(Asset::new(id, name, symbol, class));
        self.currency_ids.insert(key, id);

        id
    }

    pub fn get(&self, id: AssetId) -> Option<&Asset> {
        let _ = self.rwlock.read();
        self.currencies.get(id as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_asset() {
        let mut table = AssetTable::new();

        let id = table.add(
            "United States Dollar".to_string(),
            "USD".to_string(),
            AssetClass::Currency,
        );
        assert_eq!(id, 0);

        // Euro
        let id = table.add("Euro".to_string(), "EUR".to_string(), AssetClass::Currency);
        assert_eq!(id, 1);

        // United States Dollar
        let id = table.add(
            "United States Dollar".to_string(),
            "USD".to_string(),
            AssetClass::Currency,
        );
        assert_eq!(id, 0);
    }

    #[test]
    fn test_get_asset() {
        let mut table = AssetTable::new();

        // Add United States Dollar
        let id = table.add(
            "United States Dollar".to_string(),
            "USD".to_string(),
            AssetClass::Currency,
        );
        let asset = table.get(id).unwrap();
        assert_eq!(asset.id(), id);
        assert_eq!(asset.name(), "United States Dollar");
        assert_eq!(asset.symbol(), "USD");
        assert_eq!(asset.class(), AssetClass::Currency);

        // Add Euro
        let id = table.add("Euro".to_string(), "EUR".to_string(), AssetClass::Currency);
        let asset = table.get(id).unwrap();
        assert_eq!(asset.id(), id);
        assert_eq!(asset.name(), "Euro");
        assert_eq!(asset.symbol(), "EUR");
        assert_eq!(asset.class(), AssetClass::Currency);

        // Try to get invalid asset id
        let asset = table.get(2);
        assert!(asset.is_none());
    }
}

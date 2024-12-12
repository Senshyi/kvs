use std::{collections::HashMap, path::PathBuf};

use crate::errors::KvsError;

/// Custom result type
pub type Result<T> = std::result::Result<T, KvsError>;

/// KvStore stores key/value string pairs.
/// Key/value pair is stored in hashMap
///```rust
///# use kvs::KvStore;
///let mut store = KvStore::new();
///store.set("key1".to_string(), "value1".to_string());
///let value = store.get("key1".to_string());
///assert_eq!(value, Some("value1".to_string()));
///```
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// creates a new instance of KvStore
    pub fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Store key-value pair, does not return anything for now
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.insert(key, value);
        unimplemented!()
    }

    /// get value by key, returns Some if value exists else None
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.store.get(&key).map(|value| value.to_string());
        unimplemented!()
    }

    /// Removes value from store
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        unimplemented!()
    }

    /// Opens KvStore at given path
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        unimplemented!()
    }
}

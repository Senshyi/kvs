#![deny(missing_docs)]
//! A key value store that enables persistant storage
use std::collections::HashMap;

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
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// get value by key, returns Some if value exists else None
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).map(|value| value.to_string())
    }

    /// Removes value from store
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}

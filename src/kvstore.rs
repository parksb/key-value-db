use std::collections::HashMap;

/// A key-value store
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Adds the value of a string key to a string
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Returns the string value of a given string key
    pub fn get(&mut self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }

    /// Removes a given key
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
}

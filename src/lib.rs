use core::panic;

pub struct KvStore {}

impl KvStore {
    pub fn new() -> Self {
        panic!()
    }

    pub fn set(&self, key: String, value: String) -> Option<String> {
        panic!()
    }

    pub fn get(&self, key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(&self, key: String) -> Option<String> {
        panic!()
    }
}

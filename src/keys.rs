
use ketos::Value;

#[derive(Debug, Clone)]
pub struct Key {
    pub key: String,
    pub callback: Value,
}

impl Key {
    pub fn new(key: String, call: Value) -> Key {
        Key {
            key: key,
            callback: call,
        }
    }
}


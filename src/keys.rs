
use ketos::function::Lambda;

#[derive(Debug)]
pub struct Key {
    pub key: String,
    pub callback: Lambda,
}

impl Key {
    pub fn new(key: String, call: Lambda) -> Key {
        Key {
            key: key,
            callback: call,
        }
    }
}



use ketos::ForeignValue;

use keys::Key;

use std::cell::RefCell;

#[derive(Debug)]
pub struct Mode {
    name: String,
    desc: String,
    keys: Vec<Key>
}

impl Mode {
    pub fn new(name: String, desc: String) -> Mode {
        Mode {
            name: name,
            desc: desc,
            keys: vec![],
        }
    }
}

//pub type ModeList = Vec<Mode>;

#[derive(Debug)]
pub struct ModeList {
    inner: RefCell<Vec<Mode>>,
}

impl ModeList {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(vec![]),
        }
    }

    pub fn add_mode(&self, name: String, desc: String) {
        self.inner.borrow_mut().push(Mode::new(name, desc));
    }

    pub fn list(&self) {
        println!("{:?}", self.inner.borrow_mut());
    }
}

impl ForeignValue for ModeList {
    fn type_name(&self) -> &'static str { "ModeList" }
}

foreign_type_conversions! {
    ModeList => "ModeList"
}



use ketos::{ForeignValue, Error};

use keys::Key;

use std::cell::RefCell;
use std::fmt;

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

#[derive(Debug)]
pub struct ModeList {
    inner: RefCell<Vec<Mode>>,
    kind: String,
}

impl ModeList {
    pub fn new(kind: &str) -> Self {
        Self {
            inner: RefCell::new(vec![]),
            kind: kind.to_owned(),
        }
    }

    pub fn add_mode(&self, name: String, desc: String) {
        self.inner.borrow_mut().push(Mode::new(name, desc));
    }

    pub fn list(&self) {
        println!("{}:", self.kind);
        println!("    {:?}", self.inner.borrow());
    }
}

impl ForeignValue for ModeList {
    fn type_name(&self) -> &'static str { "ModeList" }
}

foreign_type_conversions! {
    ModeList => "ModeList"
}

pub fn new_mode(list: &ModeList, name: &str, desc: &str) -> Result<(), Error> {
    list.add_mode(name.to_owned(), desc.to_owned());
    Ok(())
}


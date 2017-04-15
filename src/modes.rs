
use ketos::{ForeignValue, Error, Value, ExecError};
use ketos::function::Lambda;

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

    pub fn add_key(&mut self, test: String, call: Lambda) {
        self.keys.push(Key::new(test, call));
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

    pub fn add_key(&self, name: String, key: String, call: &Value) -> Result<(), Error> {
        match *call {
            Value::Lambda(ref l) => {
                let mut borrowed = self.inner.borrow_mut();
                let modes = borrowed.iter_mut();
                for m in modes {
                    if m.name == name {
                        (*m).add_key(key.clone(), l.clone());
                        break;
                    }
                }
                Ok(())
            },
            _ => Err(Error::ExecError(ExecError::expected("lambda", &call)))
        }
    }
}

impl ForeignValue for ModeList {
    fn type_name(&self) -> &'static str { "ModeList" }
}

foreign_type_conversions! {
    ModeList => "ModeList"
}


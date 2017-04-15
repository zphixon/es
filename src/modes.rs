
use ketos::{ForeignValue, Error, Value, ExecError, Interpreter};
use ketos::function::Lambda;

use keys::Key;

use std::cell::RefCell;

#[derive(Debug, Clone)]
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

    pub fn get_callback(&self, key: String) -> Option<Lambda> {
        for k in self.keys.iter() {
            if k.key == key {
                return Some(k.callback.clone());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct ModeList {
    inner: RefCell<Vec<Mode>>,
    kind: String,
    current: Option<Mode>,
}

impl ModeList {
    pub fn new(kind: &str) -> Self {
        Self {
            inner: RefCell::new(vec![]),
            kind: kind.to_owned(),
            current: None,
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

    pub fn call_key_callback(&self, interp: &Interpreter, mode_name: String, key_pressed: String) -> Result<(), Error> {
        // TODO: sadness                      :(
        // TODO: a lot of it               vvvvvvvv
        for mode in self.inner.borrow_mut().clone().into_iter() {
            if mode.name == mode_name {
                for key in mode.keys {
                    if key.key == key_pressed {
                        interp.execute_code(key.callback.code).unwrap(); // TODO: error handle
                    }
                }
            }
        }
        Ok(())
    }
}

impl ForeignValue for ModeList {
    fn type_name(&self) -> &'static str { "ModeList" }
}

foreign_type_conversions! {
    ModeList => "ModeList"
}


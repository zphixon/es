
use modes::ModeList;
use ketos::{Error, Value};

pub fn new_mode(list: &ModeList, name: &str, desc: &str) -> Result<(), Error> {
    list.add_mode(name.to_owned(), desc.to_owned());
    Ok(())
}

pub fn add_key(list: &ModeList, name: &str, key: &str, call: &Value) -> Result<(), Error> {
    list.add_key(name.to_owned(), key.to_owned(), call)
}


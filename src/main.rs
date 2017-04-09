
#[macro_use]
extern crate ketos;
use ketos::*;

extern crate es;
use es::modes::{ModeList, Mode};

use std::rc::Rc;
use std::path::Path;

fn new_mode(list: &ModeList, name: &str, desc: &str) -> Result<(), Error> {
    list.add_mode(name.to_owned(), desc.to_owned());
    Ok(())
}

fn main() {
    let interp = Interpreter::new();
    let globals = Rc::new(ModeList::new());

    ketos_fn! {
        interp.scope() => "new-mode" =>
        fn new_mode(list: &ModeList, name: &str, desc: &str) -> ()
    }

    interp.run_file(Path::new("test.ket"));
    interp.call("main", vec![Value::Foreign(globals.clone())]).unwrap();

    globals.list();
}

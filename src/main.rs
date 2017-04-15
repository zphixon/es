
#[macro_use]
extern crate ketos;
use ketos::*;

extern crate es;
use es::modes::ModeList;
use es::exposed::*;

use std::rc::Rc;
use std::path::Path;

fn main() {
    let global = Rc::new(ModeList::new("global"));
    let major = Rc::new(ModeList::new("major"));
    let minor = Rc::new(ModeList::new("minor"));

    let interp = Interpreter::new();

    ketos_fn! {
        interp.scope() => "new-mode" =>
        fn new_mode(list: &ModeList, name: &str, desc: &str) -> ()
    }

    ketos_fn! {
        interp.scope() => "add-key" =>
        fn add_key(list: &ModeList, name: &str, key: &str, call: &Value) -> ()
    }

    let res = interp.run_file(Path::new("test.ket"));
    match res {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
        }
    }

    interp.call("es-main",
                vec![Value::Foreign(global.clone()),
                     Value::Foreign(major.clone()),
                     Value::Foreign(minor.clone())]).unwrap();

    global.call_key_callback(&interp, "default".into(), "j".into()).unwrap();

    global.list();
    major.list();
    minor.list();
}

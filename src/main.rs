extern crate ketos;
extern crate pancurses;
extern crate argparse;

use pancurses::*;
use argparse::*;

extern crate es;
use es::*;

fn main() {
    let mut filename: String = "".into();

    {
        let mut args = ArgumentParser::new();
        args.set_description("Edit a file with es");
        args.refer(&mut filename)
            .add_argument("file", Store, "File to edit");
        // more state later
        args.parse_args_or_exit();
    }

    let interp = ketos::Interpreter::new();
    match interp.run_file(std::path::Path::new("src/test.ket")) {
        Ok(_) => {},
        Err(e) => {
            println!("oh noes error {}", e);
        }
    }

    let mut es = EsEditor::new();

    //if filename.len() == 0 {
    //    es.
    //}
    es.vertical_split();
    println!("{:?}", es);
}


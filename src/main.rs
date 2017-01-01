extern crate ketos;

fn main() {
    let interp = ketos::Interpreter::new();
    interp.run_file(std::path::Path::new("src/test.ket")).unwrap();
}


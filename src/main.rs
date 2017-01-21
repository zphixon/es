extern crate ketos;
extern crate pancurses;

extern crate es;

use pancurses::*;
use es::EsBuffer;

fn main() {
    let interp = ketos::Interpreter::new();
    match interp.run_file(std::path::Path::new("src/test.ket")) {
        Ok(_) => {},
        Err(e) => {
            println!("oh noes error {}", e);
        }
    }

    let mut es_buffer = EsBuffer::from_filename("src/test.ket");
    es_buffer.append_line("; test".into());
    es_buffer.append_text("arino\n");

    for line in &es_buffer.lines {
        print!("{}", line);
    }

    match es_buffer.save() {
        Ok(_) => {},
        Err(e) => {
            println!("Didn't work: {:?}", e);
        }
    }

    let (x, y) = es_buffer.pos_to_xy(77);
    println!("{}, {}", x, y);
}


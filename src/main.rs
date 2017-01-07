extern crate ketos;
extern crate pancurses;

extern crate es;

use pancurses::*;
use es::EsBuffer;

fn main() {
    let interp = ketos::Interpreter::new();
    interp.run_file(std::path::Path::new("src/test.ket")).unwrap();

    let mut es_buffer = EsBuffer::from_filename(&String::from("src/test.ket"));
    &es_buffer.append_line("test".into());
    &es_buffer.append_text("dick\n");

    for line in &es_buffer.lines {
        print!("{}", line);
    }
}


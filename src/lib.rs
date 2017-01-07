use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

pub struct EsBuffer {
    pub filename: String,    // holy moly that's awful
    pub lines: Vec<String>,  // like really really gross
    pub length: usize,
    pub pos: usize,
}

impl EsBuffer {
    pub fn from_filename(filename: &String) -> EsBuffer {
        let path = Path::new(filename);

        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                println!("File doesn't exist: {}", path.display());
                std::process::exit(1);
            }
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Ok(_) => {},
            Err(e) => {
                println!("Error reading {}: {}", path.display(), e.description());
            }
        }

        let mut lines: Vec<String> = vec![];
        for line in s.split("\n") {
            lines.push(String::from(line) + "\n");
        }
        lines.pop();

        return EsBuffer {
            filename: filename.clone(),
            lines: lines,
            length: s.len(),
            pos: 0
        };
    }

    pub fn append_line(&mut self, line: String) {
        self.lines.push(line);
    }

    // FIXME holy fuck this is awful
    // also why would somthing borrow itself? that makes no sense
    // I would assume it would have ownership of itself
    pub fn append_text(&mut self, text: &str) {
        let mut tmp = String::new();
        for b in self.lines[&self.lines.len() - 1].clone().bytes() {
            if b == '\n' as u8 {
                self.append_line(String::from(text));
                return;
            }
            tmp.push(b as char);
        }
        let len = self.lines.len();
        self.lines[len - 1] = tmp + text;
    }
}


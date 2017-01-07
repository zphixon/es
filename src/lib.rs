use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::io::Write;

pub struct EsCursor {
    pub x: usize,
    pub y: usize,
}

pub struct EsEditor {
    pub buffers: Vec<EsBuffer>,
    pub current_buffer: usize,
}

pub struct EsBuffer {
    pub filename: String,
    pub lines: Vec<String>,
    pub length: usize,
    pub pos: usize,
    pub saved: bool,
}

#[derive(Debug)]
pub enum EsError {
    WrapperErrorBecauseImTooLazy,
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
            pos: 0,
            saved: true,
        };
    }

    pub fn save(&self) -> Result<(), EsError> {
        let mut file = match File::create(&self.filename) {
            Ok(f) => f,
            Err(_) => {
                return Err(EsError::WrapperErrorBecauseImTooLazy);
            }
        };
        for line in &self.lines {
            match file.write_all(line.as_bytes()) {
                Ok(_) => continue,
                Err(_) => {
                    return Err(EsError::WrapperErrorBecauseImTooLazy);
                }
            }
        }
        Ok(())
    }

    pub fn append_line(&mut self, line: String) {
        self.lines.push(line);
        self.saved = false;
    }

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
        self.saved = false;
    }
}


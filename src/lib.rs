use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;
use std::io::Write;

pub struct EsCursor {
    pub x: usize,
    pub y: usize,
}

pub struct EsEditor<'a> {
    pub buffers: Vec<EsBuffer<'a>>,
    pub current_buffer: usize,
}

pub struct EsBuffer<'a> {
    pub filename: &'a str,
    pub lines: Vec<String>,
    pub length: usize,
    pub pos: usize,
    pub saved: bool,
    pub cursor: EsCursor,
}

#[derive(Debug)]
pub enum EsError {
    FileError,
}

impl<'a> EsEditor<'a> {
    pub fn new(filename: &'a str) -> EsEditor {
        EsEditor {
            buffers: vec![EsBuffer::from_filename(filename)],
            current_buffer: 0,
        }
    }
}

impl<'a> EsBuffer<'a> {
    pub fn from_filename(filename: &'a str) -> EsBuffer {
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
        for line in s.split('\n') {
            lines.push(String::from(line) + "\n");
        }
        lines.pop();

        EsBuffer {
            filename: filename,
            lines: lines,
            length: s.len(),
            pos: 0,
            saved: true,
            cursor: EsCursor { x: 0, y: 0 },
        }
    }

    pub fn save(&mut self) -> Result<(), EsError> {
        let mut file = match File::create(&self.filename) {
            Ok(f) => f,
            Err(_) => {
                return Err(EsError::FileError);
            }
        };

        for line in &self.lines {
            match file.write_all(line.as_bytes()) {
                Ok(_) => continue,
                Err(_) => {
                    return Err(EsError::FileError);
                }
            }
        }

        self.saved = true;
        Ok(())
    }

    pub fn append_line(&mut self, line: &str) {
        self.lines.push(String::from(line));
        self.saved = false;
    }

    pub fn append_text(&mut self, text: &str) {
        if self.lines[self.lines.len() - 1].ends_with('\n') {
            self.append_line(text);
        }

        let tmp = self.lines[self.lines.len() - 1].clone();
        let len = self.lines.len();
        self.lines[len - 1] = tmp + text;
        self.saved = false;
    }

    pub fn pos_to_xy(&self, pos: usize) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        let mut bc = 0;
        for line in &self.lines {
            for _ in 0..(line.bytes().len() - 1) {
                if bc == pos {
                    return (x, y);
                }
                bc += 1;
                x += 1;
            }
            x = 0;
            y += 1;
        }
        (-1, -1)
    }
}


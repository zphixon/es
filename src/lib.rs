
extern crate xi_rope;

mod cursor;
mod buffer;
mod editor;
mod layout;

pub use cursor::*;
pub use buffer::*;
pub use editor::*;
pub use layout::*;

#[derive(Debug)]
pub enum Error {
    FileError,
}



extern crate binary_tree;
extern crate xi_rope;

use xi_rope::Rope;
use binary_tree::count;

mod cursor;
mod buffer;
mod editor;

pub use cursor::*;
pub use buffer::*;
pub use editor::*;

#[derive(Debug)]
pub enum EsError {
    FileError,
}


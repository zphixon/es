
extern crate xi_rope;

mod cursor;
mod buffer;
mod editor;
//mod layout;
// TODO: add layout stuff
// I'm too bad at data structures to imlement this
// pull request pls

pub use cursor::*;
pub use buffer::*;
pub use editor::*;
//pub use layout::*;

#[derive(Debug)]
pub enum Error {
    FileError,
}


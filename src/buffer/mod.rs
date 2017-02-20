
use xi_rope::Rope;

#[derive(Debug)]
pub struct Buffer {
    content: String,
    filename: String
    // more state stuff later
}

impl Buffer {
    pub fn empty() -> Buffer {
        Buffer {
            //content: Rope::from(""),
            content: "".into(),
            filename: "".into()
        }
    }
    pub fn test() -> Buffer {
        Buffer {
            content: "hello!".into(),
            filename: "testaroony".into()
        }
    }
}


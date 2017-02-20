
use xi_rope::Rope;

#[derive(Debug)]
pub struct EsBuffer {
    content: String,
    filename: String
    // more state stuff later
}

impl EsBuffer {
    pub fn empty() -> EsBuffer {
        EsBuffer {
            //content: Rope::from(""),
            content: "".into(),
            filename: "".into()
        }
    }
    pub fn test() -> EsBuffer {
        EsBuffer {
            content: "hello!".into(),
            filename: "testaroony".into()
        }
    }
}



use xi_rope::Rope;

#[derive(Debug)]
pub struct Buffer {
    content: String,
    filename: String,
    id: u64,
    position: (u64, u64),
    // more state stuff later
}

impl Buffer {
    pub fn empty() -> Buffer {
        Buffer {
            //content: Rope::from(""),
            content: "".into(),
            filename: "untitled".into(),
            id: 0,
            position: (1, 1)
        }
    }

    pub fn status(&self) -> String {
        let (x, y) = self.position();
        self.filename.clone() + " "
            + self.id.to_string().as_str()
            + " "
            + x.to_string().as_str()
            + ":"
            + y.to_string().as_str()
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn position(&self) -> (u64, u64) {
        self.position
    }

    pub fn move_to(&mut self, x: u64, y: u64) {
        self.position = (x, y);
    }

    pub fn current_line(&self) -> &str {
        let y = self.position().1;
        for (i, line) in self.content.lines().enumerate() {
            if i as u64 == y - 1 {
                return line;
            }
        }
        unreachable!()
    }
}


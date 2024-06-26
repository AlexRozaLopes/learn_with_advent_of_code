#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub id: i32,
    pub red: i32,
    pub green: i32,
    pub blue: i32,
    pub valid: bool
}

impl Game {
    pub fn new(id: i32, red: i32, green: i32, blue: i32, valid: bool) -> Self {
        Self { id, red, green, blue, valid }
    }
}



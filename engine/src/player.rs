#[derive(Debug)]
pub struct Player {
    pub y: i32,
}

impl Player {
    pub fn new() -> Self {
        Player { y: 0 }
    }

    pub fn move_up(&mut self) {
        self.y -= 1;
    }

    pub fn move_down(&mut self) {
        self.y += 1;
    }
}

#[derive(Debug)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    pub size: u32, // width/height of the square
}

impl Player {
    pub fn new() -> Self {
        Player { x: 50, y: 50, size: 10 }
    }

    pub fn move_up(&mut self) {
        self.y -= 2;
    }
    pub fn move_down(&mut self) {
        self.y += 2;
    }
    pub fn move_left(&mut self) {
        self.x -= 2;
    }
    pub fn move_right(&mut self) {
        self.x += 2;
    }
}

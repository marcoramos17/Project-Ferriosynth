pub struct Renderable {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: [u8; 4], // RGBA
}

impl Renderable {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: [u8; 4]) -> Self {
        Self { x, y, width, height, color }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn size(&self) -> u32 {
        self.width
    }

    pub fn color(&self) -> [u8; 4] {
        self.color
    }
}
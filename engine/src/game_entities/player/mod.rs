pub mod render;

use entities::stats::Stats;
use entities::renderable::Renderable;
use entities::controls::{Controllable, Controls};


pub struct Player {
    pub stats: Stats,
    pub renderable: Renderable,
}

impl Player {
    pub fn new() -> Self {
        Self {
            stats: Stats::new(100, 50),
            renderable: Renderable::new(50, 50, 32, 32, [0, 0, 255, 255]),
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.renderable.x += dx;
        self.renderable.y += dy;
    }

    pub fn position(&self) -> (i32, i32) {
        self.renderable.position()
    }

    pub fn size(&self) -> u32 {
        self.renderable.size()
    }

    pub fn color(&self) -> [u8; 4] {
        self.renderable.color()
    }
}

impl Controllable for Player {
    fn handle_controls(&mut self, controls: Controls) {
        match controls {
            Controls::MoveUp => self.move_by(0, -5),
            Controls::MoveDown => self.move_by(0, 5),
            Controls::MoveLeft => self.move_by(-5, 0),
            Controls::MoveRight => self.move_by(5, 0),
            _ => {}
        }
    }
}

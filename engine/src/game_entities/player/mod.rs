pub mod render;

use entities::stats::Stats;
use entities::renderable::Renderable;
use entities::controls::{Controls, Controllable};

pub struct Player {
    pub stats: Stats,
    pub renderable: Renderable,
}

impl Player {
    pub fn new() -> Self {
        Self {
            stats: Stats::new(100, 50),
            renderable: Renderable {
                x: 50,
                y: 50,
                width: 32,
                height: 32,
                color: [0, 0, 255, 255], // blue
            },
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.renderable.x, self.renderable.y)
    }

    pub fn size(&self) -> u32 {
        self.renderable.width
    }

    pub fn color(&self) -> [u8; 4] {
        self.renderable.color
    }

    pub fn move_up(&mut self) { self.renderable.y -= 5; }
    pub fn move_down(&mut self) { self.renderable.y += 5; }
    pub fn move_left(&mut self) { self.renderable.x -= 5; }
    pub fn move_right(&mut self) { self.renderable.x += 5; }
}

impl Controllable for Player {
    fn handle_controls(&mut self, controls: Controls) {
        match controls {
            Controls::MoveUp => self.move_up(),
            Controls::MoveDown => self.move_down(),
            Controls::MoveLeft => self.move_left(),
            Controls::MoveRight => self.move_right(),
            _ => {}
        }
    }
}


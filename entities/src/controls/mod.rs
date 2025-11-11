use winit::keyboard::{KeyCode, PhysicalKey};
use winit::event::KeyEvent;

pub enum Controls {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    None,
}

impl Controls {
    pub fn from_key_event(event: &KeyEvent) -> Self {
        match event.physical_key {
            PhysicalKey::Code(KeyCode::ArrowUp) => Controls::MoveUp,
            PhysicalKey::Code(KeyCode::ArrowDown) => Controls::MoveDown,
            PhysicalKey::Code(KeyCode::ArrowLeft) => Controls::MoveLeft,
            PhysicalKey::Code(KeyCode::ArrowRight) => Controls::MoveRight,
            _ => Controls::None,
        }
    }
}

pub trait Controllable {
    fn handle_controls(&mut self, controls: Controls);
}

use winit::event::{KeyEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug)]
pub enum Controls {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    None,
}


impl Controls {
    pub fn from_key_event(event: &KeyEvent) -> Self {
        if let PhysicalKey::Code(code) = event.physical_key {
            match (code, event.state) {
                (KeyCode::ArrowUp, ElementState::Pressed) => Controls::MoveUp,
                (KeyCode::ArrowDown, ElementState::Pressed) => Controls::MoveDown,
                (KeyCode::ArrowLeft, ElementState::Pressed) => Controls::MoveLeft,
                (KeyCode::ArrowRight, ElementState::Pressed) => Controls::MoveRight,
                _ => Controls::None,
            }
        } else {
            Controls::None
        }
    }
}

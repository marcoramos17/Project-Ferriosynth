use winit::keyboard::KeyCode;
use crate::controls::Controls;

pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub menu: KeyCode,
}

impl KeyBindings {
    pub fn default() -> Self {
        Self {
            up: KeyCode::ArrowUp,
            down: KeyCode::ArrowDown,
            left: KeyCode::ArrowLeft,
            right: KeyCode::ArrowRight,
            menu: KeyCode::Escape,
        }
    }

    pub fn map_key(&self, key: KeyCode) -> Controls {
        match key {
            k if k == self.up => Controls::MoveUp,
            k if k == self.down => Controls::MoveDown,
            k if k == self.left => Controls::MoveLeft,
            k if k == self.right => Controls::MoveRight,
            k if k == self.menu => Controls::OpenMenu,
            _ => Controls::None,
        }
    }
}

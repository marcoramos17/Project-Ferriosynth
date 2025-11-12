pub mod input;
pub mod bindings;

pub use input::map_key_event;
pub use bindings::KeyBindings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controls {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    OpenMenu,
    Pause,
    None,
}

pub trait Controllable {
    fn handle_controls(&mut self, controls: Controls);
}

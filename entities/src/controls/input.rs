use winit::event::KeyEvent;
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::controls::{Controls, KeyBindings};

pub fn map_key_event(event: &KeyEvent, bindings: &KeyBindings) -> Controls {
    match event.physical_key {
        PhysicalKey::Code(code) => bindings.map_key(code),
        _ => Controls::None,
    }
}

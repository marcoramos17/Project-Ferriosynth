//! Main entry point for the `render` crate.
//!
//! This crate handles window creation, rendering logic, and scene drawing.

pub mod window;
pub mod renderer;
pub mod draw;

pub use renderer::Renderer;
pub use draw::draw_scene;
pub use window::{GameWindow, handle_window_event};

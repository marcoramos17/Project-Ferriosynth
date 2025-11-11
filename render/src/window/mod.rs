//! The `window` module handles all window-related functionality for the game.
//!
//! It includes:
//! - [`GameWindow`] for creating and managing the game window
//! - [`handle_window_event`] for processing window events like input and redraw
//!
//! This module abstracts away the `winit` setup and event handling so that the main game loop remains clean.


pub mod manager;
pub mod events;

pub use manager::GameWindow;
pub use events::handle_window_event;

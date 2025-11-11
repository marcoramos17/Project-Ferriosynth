//! Responsible for creating and configuring the game window.
//!
//! This module wraps `winit`'s window creation logic and exposes a [`GameWindow`] struct
//! that stores a `'static` reference to the window and its ID.

use winit::{
    dpi::LogicalSize,
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Window, WindowId},
};

/// Represents the main game window and its associated ID.
///
/// This struct is created via [`GameWindow::new`] and holds a `'static` reference
/// to the window so it can be used across the lifetime of the game.
pub struct GameWindow {
    /// A leaked `'static` reference to the window.
    pub window: &'static Window,
    /// The unique ID of the window.
    pub window_id: WindowId,
}

impl GameWindow {
    /// Creates a new game window with the given dimensions and fullscreen setting.
    ///
    /// # Parameters
    /// - `event_loop`: The active event loop used to create the window
    /// - `width`: The logical width of the window
    /// - `height`: The logical height of the window
    /// - `fullscreen`: Whether the window should start in fullscreen mode
    ///
    /// # Returns
    /// A [`GameWindow`] containing a `'static` reference to the created window
    pub fn new(event_loop: &ActiveEventLoop, width: u32, height: u32, fullscreen: bool) -> Self {
        let mut attributes = Window::default_attributes()
            .with_title("Ferriosynth Prototype")
            .with_inner_size(LogicalSize::new(width as f64, height as f64));

        attributes = attributes.with_fullscreen(if fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        });

        let window = event_loop.create_window(attributes).unwrap();
        let window_ref: &'static Window = Box::leak(Box::new(window));

        Self {
            window: window_ref,
            window_id: window_ref.id(),
        }
    }
}

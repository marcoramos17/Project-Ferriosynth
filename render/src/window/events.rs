//! Handles window events such as input and redraw requests.
//!
//! This module defines [`handle_window_event`], which is called from the game loop
//! to respond to events dispatched by `winit`.

use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::WindowId,
};

use engine::world::World;
use entities::controls::{Controls, Controllable};
use crate::Renderer;

/// Processes a single window event and updates the game state accordingly.
///
/// This function is called from the main game loop whenever a window event is received.
///
/// # Parameters
/// - `event_loop`: The active event loop
/// - `id`: The ID of the window that received the event
/// - `expected_id`: The ID of the main game window
/// - `event`: The window event to process
/// - `world`: The mutable reference to the game world
/// - `renderer`: The mutable reference to the renderer
/// - `window`: A reference to the game window
pub fn handle_window_event(
    event_loop: &ActiveEventLoop,
    id: WindowId,
    expected_id: WindowId,
    event: WindowEvent,
    world: &mut World,
    renderer: &mut Renderer<'static>,
    window: &winit::window::Window,
) {
    if id != expected_id {
        return;
    }

    match event {
        WindowEvent::CloseRequested => event_loop.exit(),
        WindowEvent::KeyboardInput { event: key_event, .. } => {
            let controls = Controls::from_key_event(&key_event);
            world.player.handle_controls(controls);
            window.request_redraw();
        }
        WindowEvent::RedrawRequested => {
            renderer.draw(world);
            if renderer.render().is_err() {
                event_loop.exit();
            }
        }
        _ => {}
    }
}

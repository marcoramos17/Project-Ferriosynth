// render/src/window.rs
use winit::{
    dpi::LogicalSize,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId, Fullscreen},
};

pub struct GameWindow {
    pub window: &'static Window,
    pub window_id: WindowId,
}

impl GameWindow {
    pub fn new(event_loop: &ActiveEventLoop, width: u32, height: u32, fullscreen: bool) -> Self {
        // Create window attributes
        let mut attributes = Window::default_attributes()
            .with_title("Ferriosynth Prototype")
            .with_inner_size(LogicalSize::new(width as f64, height as f64));

        attributes = attributes.with_fullscreen(
            if fullscreen {
                Some(Fullscreen::Borderless(None))
            } else {
                None
            }
        );

        // Create the window
        let window = event_loop.create_window(attributes).unwrap();

        // Leak the window to get a 'static reference (same trick you used in main.rs)
        let window_ref: &'static Window = Box::leak(Box::new(window));

        Self {
            window: window_ref,
            window_id: window_ref.id(),
        }
    }
}

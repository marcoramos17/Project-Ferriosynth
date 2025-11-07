use engine::controls::Controls;
use engine::world::World;
use render::Renderer;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

struct GameApp {
    world: World,
    renderer: Option<Renderer<'static>>,
    window: Option<&'static Window>,
    window_id: Option<WindowId>,
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create the window
        let window = event_loop
            .create_window(
                Window::default_attributes()
                    .with_title("Ferriosynth Prototype")
                    .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64)),
            )
            .unwrap();

        // Leak the window to get a 'static reference for the whole app lifetime
        let window_ref: &'static Window = Box::leak(Box::new(window));

        self.window = Some(window_ref);
        self.window_id = Some(window_ref.id());
        self.renderer = Some(Renderer::new(WIDTH, HEIGHT, window_ref));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        // Only handle events for our window
        if Some(id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                let controls = Controls::from_key_event(&event);
                self.world.handle_controls(controls);

                // trigger redraw after input
                if let Some(w) = self.window {
                    w.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.draw(&self.world);
                    if renderer.render().is_err() {
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = GameApp {
        world: World::new(),
        renderer: None,
        window: None,
        window_id: None,
    };
    event_loop.run_app(&mut app).unwrap();
}

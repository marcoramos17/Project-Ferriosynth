mod settings;

use entities::controls::{Controls, Controllable};
use engine::world::World;
use render::{Renderer, window::GameWindow}; // ✅ Import GameWindow
use settings::GameSettings;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};



const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const FULLSCREEN: bool = false; // ✅ You can later load this from settings.toml

struct GameApp {
    world: World,
    renderer: Option<Renderer<'static>>,
    window: Option<&'static Window>,
    window_id: Option<WindowId>,
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let settings = GameSettings::load();
        // ✅ Use GameWindow abstraction
        let game_window = GameWindow::new(
            &event_loop,
            settings.window.width,
            settings.window.height,
            settings.window.fullscreen,
        );


        self.window = Some(game_window.window);
        self.window_id = Some(game_window.window_id);
        self.renderer = Some(Renderer::new(WIDTH, HEIGHT, game_window.window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if Some(id) != self.window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event: key_event, .. } => {
                let controls = Controls::from_key_event(&key_event);
                self.world.player.handle_controls(controls);

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

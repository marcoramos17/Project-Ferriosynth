use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use engine::world::World;
use render::Renderer;
use render::window::GameWindow;
use crate::settings::GameSettings;

pub struct GameApp {
    world: World,
    renderer: Option<Renderer<'static>>,
    window: Option<&'static Window>,
    window_id: Option<WindowId>,
    settings: GameSettings,
}

impl GameApp {
    pub fn new(settings: GameSettings) -> Self {
        Self {
            world: World::new(),
            renderer: None,
            window: None,
            window_id: None,
            settings,
        }
    }
}

impl ApplicationHandler for GameApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let game_window = GameWindow::new(
            event_loop,
            self.settings.window.width,
            self.settings.window.height,
            self.settings.window.fullscreen,
        );

        self.window = Some(game_window.window);
        self.window_id = Some(game_window.window_id);
        self.renderer = Some(Renderer::new(
            self.settings.window.width,
            self.settings.window.height,
            game_window.window,
        ));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let (Some(renderer), Some(window), Some(expected_id)) =
            (&mut self.renderer, self.window, self.window_id)
        {
            render::window::handle_window_event(
                event_loop,
                id,
                expected_id,
                event,
                &mut self.world,
                renderer,
                window,
            );
        }
    }
}
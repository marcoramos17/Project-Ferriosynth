mod settings;
mod app;

use app::GameApp;
use settings::GameSettings;
use winit::event_loop::EventLoop;

fn main() {
    let settings = GameSettings::load();
    let event_loop = EventLoop::new().unwrap();
    let mut app = GameApp::new(settings);
    event_loop.run_app(&mut app).unwrap();
}

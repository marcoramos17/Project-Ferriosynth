use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, KeyEvent, ElementState},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowAttributes,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

fn main() {
    // Create event loop + window
    let event_loop = EventLoop::new().unwrap();
    let window = event_loop
        .create_window(
            WindowAttributes::new()
                .with_title("Ferriosynth Prototype")
                .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64)),
        )
        .unwrap();

    // Create pixel buffer
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

    // Capture a reference instead of moving the window
    let window_ref = &window;

    // Run loop
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),

                WindowEvent::KeyboardInput {
                    event: KeyEvent { physical_key, state, .. },
                    ..
                } => {
                    if let PhysicalKey::Code(code) = physical_key {
                        match code {
                            KeyCode::ArrowUp if state == ElementState::Pressed => {
                                println!("Up pressed!");
                            }
                            KeyCode::ArrowDown if state == ElementState::Pressed => {
                                println!("Down pressed!");
                            }
                            _ => {}
                        }
                    }
                }

                WindowEvent::RedrawRequested => {
                    let frame = pixels.frame_mut();
                    for chunk in frame.chunks_exact_mut(4) {
                        chunk.copy_from_slice(&[0x00, 0x80, 0x00, 0xff]); // green
                    }

                    if pixels.render().is_err() {
                        elwt.exit();
                    }
                }

                _ => {}
            },

            Event::AboutToWait => {
                window_ref.request_redraw(); // use the borrowed reference
            }

            _ => {}
        }
    });
}

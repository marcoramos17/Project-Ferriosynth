use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use engine::world::World;

pub struct Renderer<'win> {
    pixels: Pixels<'win>,
}

impl<'win> Renderer<'win> {
    pub fn new(width: u32, height: u32, window: &'win Window) -> Self {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window);
        let pixels = Pixels::new(width, height, surface_texture).unwrap();
        Renderer { pixels }
    }

    pub fn draw(&mut self, _world: &World) {
        let frame = self.pixels.frame_mut();
        for chunk in frame.chunks_exact_mut(4) {
            chunk.copy_from_slice(&[0x00, 0x80, 0x00, 0xff]); // green background
        }
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}

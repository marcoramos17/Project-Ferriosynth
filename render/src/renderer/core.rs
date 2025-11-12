use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Renderer<'win> {
    pub pixels: Pixels<'win>,
    pub width: u32,
    pub height: u32,
}

impl<'win> Renderer<'win> {
    pub fn new(width: u32, height: u32, window: &'win Window) -> Self {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window);
        let pixels = Pixels::new(width, height, surface_texture).unwrap();
        Self { pixels, width, height }
    }

    pub fn frame_mut(&mut self) -> &mut [u8] {
        self.pixels.frame_mut()
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}

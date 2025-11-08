use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;
use engine::world::World;

pub struct Renderer<'win> {
    pixels: Pixels<'win>,
    width: u32,
    height: u32,
}

impl<'win> Renderer<'win> {
    pub fn new(width: u32, height: u32, window: &'win Window) -> Self {
        let size = window.inner_size();
        let surface_texture = SurfaceTexture::new(size.width, size.height, window);
        let pixels = Pixels::new(width, height, surface_texture).unwrap();
        Self { pixels, width, height }
    }

    pub fn draw(&mut self, world: &World) {
        let frame = self.pixels.frame_mut();

        // Fill background green
        for chunk in frame.chunks_exact_mut(4) {
            chunk.copy_from_slice(&[0x00, 0x80, 0x00, 0xff]);
        }

        // Draw player as blue square
        let player = &world.player;
        let size = player.size as usize;
        let w = self.width as usize;
        let h = self.height as usize;

        for dy in 0..size {
            for dx in 0..size {
                let px = (player.x as usize + dx).clamp(0, w - 1);
                let py = (player.y as usize + dy).clamp(0, h - 1);

                let idx = (py * w + px) * 4;
                frame[idx..idx + 4].copy_from_slice(&[0x00, 0x00, 0xff, 0xff]); // blue
            }
        }
    }

    pub fn render(&mut self) -> Result<(), pixels::Error> {
        self.pixels.render()
    }
}

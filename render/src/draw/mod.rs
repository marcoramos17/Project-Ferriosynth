use engine::world::World;
use crate::renderer::Renderer;
use engine::game_entities::player::render::draw_player;

pub fn draw_scene(renderer: &mut Renderer, world: &World) {
    let w = renderer.width as usize;
    let h = renderer.height as usize;
    let frame = renderer.frame_mut();

    for chunk in frame.chunks_exact_mut(4) {
        chunk.copy_from_slice(&[0x00, 0x80, 0x00, 0xff]);
    }

    draw_player(frame, w, h, &world.player);
}

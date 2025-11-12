use crate::game_entities::player::Player;

pub fn draw_player(frame: &mut [u8], width: usize, height: usize, player: &Player) {
    let (x, y) = player.position();
    let size = player.size();
    let color = player.color();

    for dy in 0..size {
        for dx in 0..size {
            let px = ((x + dx as i32).clamp(0, width as i32 - 1)) as usize;
            let py = ((y + dy as i32).clamp(0, height as i32 - 1)) as usize;
            let idx = (py * width + px) * 4;
            frame[idx..idx + 4].copy_from_slice(&color);
        }
    }
}

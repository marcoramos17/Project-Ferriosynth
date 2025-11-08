use crate::controls::Controls;
use crate::player::Player;

pub struct World {
    pub player: Player,
}

impl World {
    pub fn new() -> Self {
        World { player: Player::new() }
    }

    pub fn handle_controls(&mut self, controls: Controls) {
        match controls {
            Controls::MoveUp => self.player.move_up(),
            Controls::MoveDown => self.player.move_down(),
            Controls::MoveLeft => self.player.move_left(),
            Controls::MoveRight => self.player.move_right(),
            Controls::None => {}
        }
    }

}

use crate::game_entities::player::Player;
use entities::controls::{Controls, Controllable};

pub struct World {
    pub player: Player,
}

impl World {
    pub fn new() -> Self {
        World { player: Player::new() }
    }

    pub fn update(&mut self, controls: Controls) {
        self.player.handle_controls(controls);
    }
}

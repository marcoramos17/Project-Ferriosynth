pub struct Stats {
    pub health: u32,
    pub mana: u32,
}
impl Stats {
    pub fn new(health: u32, mana: u32) -> Self {
        Self { health, mana }
    }
}

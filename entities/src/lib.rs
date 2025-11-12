pub mod renderable;
pub use renderable::Renderable;

pub mod stats;
pub use stats::Stats;

pub mod controls;
pub use controls::{Controls, Controllable};


pub struct Entity {
    pub id: u32,
    pub stats: Stats,
    pub renderable: Renderable,
}

pub trait EntityTrait {
    fn id(&self) -> u32;
    fn renderable(&self) -> &Renderable;
}

mod goblin;
mod orc;

use rand::{thread_rng, Rng};
use std::fmt::Debug;

use goblin::Goblin;
use orc::Orc;

// The Monster trait requires that implementers also implement Debug
pub trait Monster: Debug {
    fn attack(&self) -> &str;
    fn defend(&self) -> &str;
}

// Enum to represent the factory that creates monsters
#[derive(Debug)]
pub enum MonsterFactory {
    Goblin,
    Orc,
}

impl MonsterFactory {
    pub fn random() -> Box<dyn Monster> {
        let mut rng = thread_rng();
        match rng.gen_range(0..=1) {
            0 => Box::new(Goblin::new()),
            1 => Box::new(Orc::new()),
            _ => unreachable!(),
        }
    }
}

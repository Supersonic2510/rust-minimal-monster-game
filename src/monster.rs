mod goblin;
mod orc;

use rand::{thread_rng, Rng};

use goblin::Goblin;
use orc::Orc;

trait Monster {
    fn attack(&self) -> &str;
    fn defend(&self) -> &str;
}

#[derive(Debug)]
pub enum MonsterFactory {
    Goblin(Goblin),
    Orc(Orc),
}

impl MonsterFactory {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..=1) {
            0 => MonsterFactory::Goblin(Goblin::new()),
            1 => MonsterFactory::Orc(Orc::new()),
            _ => unreachable!(),
        }
    }
}

mod goblin;
mod orc;

use rand::{thread_rng, Rng};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

use goblin::Goblin;
use orc::Orc;

// The Monster trait requires that implementers also implement Debug
pub trait Monster: Debug {
    fn attack(&mut self, enemy: &mut dyn Monster) -> &str;
    fn defend(&self) -> &str;
    fn damage(&mut self, value: i8);
    fn is_dead(&self) -> bool;
    fn get_speed(&self) -> i8;
}

// Enum to represent the factory that creates monsters
#[derive(Debug)]
pub enum MonsterFactory {
    Goblin,
    Orc,
}

impl MonsterFactory {
    pub fn random() -> Rc<RefCell<dyn Monster>> {
        let mut rng = thread_rng();
        match rng.gen_range(0..=1) {
            0 => Rc::new(RefCell::new(Goblin::new())),
            1 => Rc::new(RefCell::new(Orc::new())),
            _ => unreachable!(),
        }
    }
}

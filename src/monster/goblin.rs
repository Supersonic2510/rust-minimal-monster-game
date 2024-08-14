use super::Monster;
use std::rc::Rc;

const GOBLIN_NAME: &str = "Goblin";

#[derive(Debug)]
pub struct Goblin {
    name: Rc<str>,
    hit_points: u32,
}

impl Goblin {
    pub fn new() -> Self {
        Self {
            name: Rc::from(GOBLIN_NAME),
            hit_points: 10,
        }
    }
}

impl MonsterActions for Goblin {
    fn attack(&self) -> &str {
        "The goblin swings its rusty sword!"
    }

    fn defend(&self) -> &str {
        "The goblin hides behind its shield!"
    }
}

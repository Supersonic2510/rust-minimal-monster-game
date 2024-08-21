use super::Monster;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

const GOBLIN_NAME: &str = "Goblin";

#[derive(Debug, PartialEq)]
pub struct Goblin {
    name: Rc<str>,
    hit_points: i8,
    current_hit_points: i8,
    is_defending: bool,
    speed: i8,
}

impl Goblin {
    pub fn new() -> Self {
        Self {
            name: Rc::from(GOBLIN_NAME),
            hit_points: 10,
            current_hit_points: 10,
            is_defending: false,
            speed: 5,
        }
    }
}

impl Monster for Goblin {
    fn attack(&mut self, enemy: &mut dyn Monster) {
        enemy.damage(1);
    }

    fn attack_self(&mut self) {
        self.damage(1);
    }

    fn defend(&self) -> &'static str {
        return "The goblin hides behind its shield!";
    }

    fn damage(&mut self, value: i8) {
        self.hit_points -= value;
    }

    fn is_dead(&self) -> bool {
        return self.hit_points <= 0;
    }

    fn get_speed(&self) -> i8 {
        return self.speed;
    }
}

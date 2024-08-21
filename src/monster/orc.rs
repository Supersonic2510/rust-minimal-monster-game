use super::Monster;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

const ORC_NAME: &str = "Orc";

#[derive(Debug, PartialEq)]
pub struct Orc {
    name: Rc<str>,
    hit_points: i8,
    current_hit_points: i8,
    is_defending: bool,
    speed: i8,
}

impl Orc {
    pub fn new() -> Self {
        Self {
            name: Rc::from(ORC_NAME),
            hit_points: 20,
            current_hit_points: 20,
            is_defending: false,
            speed: 3,
        }
    }
}

impl Monster for Orc {
    fn attack(&mut self, enemy: &mut dyn Monster) {
        enemy.damage(1);
    }

    fn attack_self(&mut self) {
        self.damage(1);
    }

    fn defend(&self) -> &str {
        "The orc hides behind its shield!"
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

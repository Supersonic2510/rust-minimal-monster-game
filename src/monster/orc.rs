use super::Monster;
use std::rc::Rc;

const ORC_NAME: &str = "Orc";

#[derive(Debug)]
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
    fn attack(&mut self, enemy: &mut dyn Monster) -> &str {
        // Do damage to the enemy

        enemy.damage(1);
        if enemy.is_dead() {
            return "The enemy is already dead!";
        }

        return "The orc swings its rusty sword!";
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

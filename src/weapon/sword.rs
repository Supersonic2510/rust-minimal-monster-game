use super::Weapon;
use crate::distributions::damage::DamageDistribution;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

const SWORD_NAME: &str = "Sword";

#[derive(Debug)]
pub struct Sword {
    name: Rc<str>,
    damage_dist: DamageDistribution,
}

impl Sword {
    pub fn new() -> Self {
        Self {
            name: Rc::from(SWORD_NAME),
            damage_dist: DamageDistribution::new(3i8, 5i8, 6i8, 7i8, 7i8, 7i8, 0f32).unwrap(),
        }
    }
}

impl Weapon for Sword {
    fn get_name(&self) -> Rc<str> {
        return self.name.clone();
    }

    fn get_damage(&self) -> i8 {
        return self.damage_dist.get_damage_value();
    }
}

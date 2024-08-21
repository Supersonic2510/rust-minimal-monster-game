mod sword;

use rand::{thread_rng, Rng};
use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

use sword::Sword;

pub trait Weapon: Debug {
    fn get_name(&self) -> Rc<str>;
    fn get_damage(&self) -> i8;
}

// Enum to represent the factory that creates monsters
#[derive(Debug, PartialEq)]
pub enum WeaponFactory {
    Sword,
}

impl WeaponFactory {
    pub fn random() -> Rc<RefCell<dyn Weapon>> {
        let mut rng = thread_rng();
        match rng.gen_range(0..=0) {
            0 => Rc::new(RefCell::new(Sword::new())),
            _ => unreachable!(),
        }
    }

    pub fn create(&self) -> Rc<RefCell<dyn Weapon>> {
        match self {
            WeaponFactory::Sword => Rc::new(RefCell::new(Sword::new())),
        }
    }
}

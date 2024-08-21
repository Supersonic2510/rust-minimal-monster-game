use super::monster::Monster;
use std::{cell::RefCell, rc::Rc};

const GRID_SIZE: usize = 4;

#[derive(Debug)]
pub struct EnemyGrid {
    pub upper_left: Option<Rc<RefCell<dyn Monster>>>,
    pub upper_right: Option<Rc<RefCell<dyn Monster>>>,
    pub lower_left: Option<Rc<RefCell<dyn Monster>>>,
    pub lower_right: Option<Rc<RefCell<dyn Monster>>>,
}

impl EnemyGrid {
    pub fn new() -> Self {
        Self {
            upper_left: None,
            upper_right: None,
            lower_left: None,
            lower_right: None,
        }
    }

    // Simple function to insert a monster into the grid when no position is specified
    pub fn insert(&mut self, monster: Rc<RefCell<dyn Monster>>) {
        if self.upper_left.is_none() {
            self.upper_left = Some(monster);
        } else if self.upper_right.is_none() {
            self.upper_right = Some(monster);
        } else if self.lower_left.is_none() {
            self.lower_left = Some(monster);
        } else if self.lower_right.is_none() {
            self.lower_right = Some(monster);
        }
    }

    pub fn as_array(&self) -> Vec<Rc<RefCell<dyn Monster>>> {
        let mut monster_array = Vec::with_capacity(GRID_SIZE);
        if let Some(monster) = &self.upper_left {
            monster_array.push(monster.clone());
        }
        if let Some(monster) = &self.upper_right {
            monster_array.push(monster.clone());
        }
        if let Some(monster) = &self.lower_left {
            monster_array.push(monster.clone());
        }
        if let Some(monster) = &self.lower_right {
            monster_array.push(monster.clone());
        }

        return monster_array;
    }

    pub fn remove(&mut self, target: &Rc<RefCell<dyn Monster>>) {
        let positions = [
            &mut self.upper_left,
            &mut self.upper_right,
            &mut self.lower_left,
            &mut self.lower_right,
        ];

        for position in positions {
            if let Some(monster_rc) = position {
                if Rc::ptr_eq(monster_rc, target) {
                    *position = None;
                    return;
                }
            }
        }
    }
}

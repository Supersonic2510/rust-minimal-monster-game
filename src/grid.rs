use super::monster::Monster;
use std::{cell::RefCell, rc::Rc};

const GRID_SIZE: usize = 4;

#[derive(Debug)]
pub struct EnemyGrid {
    pub upperLeft: Option<Rc<RefCell<dyn Monster>>>,
    pub upperRight: Option<Rc<RefCell<dyn Monster>>>,
    pub lowerLeft: Option<Rc<RefCell<dyn Monster>>>,
    pub lowerRight: Option<Rc<RefCell<dyn Monster>>>,
}

impl EnemyGrid {
    pub fn new() -> Self {
        Self {
            upperLeft: None,
            upperRight: None,
            lowerLeft: None,
            lowerRight: None,
        }
    }

    // Simple function to insert a monster into the grid when no position is specified
    pub fn insert(&mut self, monster: Rc<RefCell<dyn Monster>>) {
        if self.upperLeft.is_none() {
            self.upperLeft = Some(monster);
        } else if self.upperRight.is_none() {
            self.upperRight = Some(monster);
        } else if self.lowerLeft.is_none() {
            self.lowerLeft = Some(monster);
        } else if self.lowerRight.is_none() {
            self.lowerRight = Some(monster);
        }
    }

    pub fn as_array(&self) -> Vec<Rc<RefCell<dyn Monster>>> {
        let mut monster_array = Vec::with_capacity(GRID_SIZE);
        if let Some(monster) = &self.upperLeft {
            monster_array.push(monster.clone());
        }
        if let Some(monster) = &self.upperRight {
            monster_array.push(monster.clone());
        }
        if let Some(monster) = &self.lowerLeft {
            monster_array.push(monster.clone());
        }
        if let Some(monster) = &self.lowerRight {
            monster_array.push(monster.clone());
        }

        return monster_array;
    }
}

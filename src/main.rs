mod grid;
mod monster;

use rand::distributions::{Distribution, Uniform};
use std::{cell::RefCell, rc::Rc};

use grid::EnemyGrid;
use monster::{Monster, MonsterFactory};

fn main() {
    let mut rng = rand::thread_rng(); // Create a random number generator

    // Create the monster grid
    let mut grid = EnemyGrid::new();

    // Insert some monsters into the grid
    for _ in 0..4 {
        grid.insert(MonsterFactory::random());
    }

    println!("The enemy grid is {:#?}", grid);

    // Simulate a turn
    let monster_array = grid.as_array();
    let player_array = Vec::new();

    let entity_array: Vec<Rc<RefCell<dyn Monster>>> = monster_array
        .iter()
        .chain(player_array.iter())
        .cloned()
        .collect();

    let entity_range = Uniform::new(0, entity_array.len()); // Create a uniform distribution in the range

    for entity in &entity_array {
        let mut entity = entity.borrow_mut();
        println!("Entity: {{{:#?}}} has taken a turn!", entity);

        // Select a random target from monster_array
        let target_index = entity_range.sample(&mut rng);
        let mut target = entity_array[target_index].borrow_mut();

        entity.attack(&mut *target); // Perform the attack
    }
}

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

    //println!("The enemy grid is {:#?}", grid);

    // Simulate a turn
    let monster_array = grid.as_array();
    let player_array = Vec::new();

    let mut entity_array: Vec<Rc<RefCell<dyn Monster>>> = monster_array
        .iter()
        .chain(player_array.iter())
        .cloned()
        .collect();

    // Sort the entities by speed in descending order
    entity_array.sort_by(|a, b| {
        let speed_a = a.borrow().get_speed();
        let speed_b = b.borrow().get_speed();
        speed_b.cmp(&speed_a) // For descending order
    });

    'turn_loop: for entity in entity_array.clone().into_iter() {
        {
            let entity = entity.borrow();
            if entity.is_dead() {
                continue 'turn_loop;
            }
        }

        // Collect alive targets, including the current entity (if you want to attack yourself)
        let alive_targets: Vec<Rc<RefCell<dyn Monster>>> = entity_array
            .iter()
            .filter(|target_pointer| !target_pointer.borrow().is_dead())
            .cloned()
            .collect();

        // This is because yourself is included in the list of targets
        if alive_targets.len() <= 1 {
            println!("No more entities alive or only one entity left (self).");
            break 'turn_loop;
        }

        println!("Entity: {:#?}", entity);

        let entity_range = Uniform::new(0, alive_targets.len());
        let target_index = entity_range.sample(&mut rng);

        let target = alive_targets[target_index].clone();

        println!("Target: {:#?}", target);

        // Attack the target
        if Rc::ptr_eq(&entity, &target) {
            entity.borrow_mut().attack_self();
        } else {
            entity.borrow_mut().attack(&mut *target.borrow_mut());
        }

        if target.borrow().is_dead() {
            grid.remove(&target);
            println!("{:#?} has died!", target);
        }
    }
}

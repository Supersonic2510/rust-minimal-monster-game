mod monster;

use monster::{Monster, MonsterFactory};

fn main() {
    // Create the Monster Factory
    let mut monster_array: Vec<Box<dyn Monster>> = Vec::new();
    for _ in 0..10 {
        let random_monster = MonsterFactory::random();
        monster_array.push(random_monster);
    }

    println!("The monster array is {:#?}", monster_array);

    // Simulate a turn
    for monster in monster_array.iter() {
        println!("Monster: {{{:#?}}} has taken a turn!", monster);
    }
}

mod monster;

fn main() {
    // Create the Monster Factory
    let random_monster = monster::MonsterFactory::random();
    println!("The monster is {:#?}", random_monster);
}

use super::Monster;
use std::rc::Rc;

const ORC_NAME: &str = "Orc";

#[derive(Debug)]
pub struct Orc {
    name: Rc<str>,
    hit_points: u32,
}

impl Orc {
    pub fn new() -> Self {
        Self {
            name: Rc::from(ORC_NAME),
            hit_points: 20,
        }
    }
}

impl Monster for Orc {
    fn attack(&self) -> &str {
        "The orc swings its rusty sword!"
    }

    fn defend(&self) -> &str {
        "The orc hides behind its shield!"
    }
}

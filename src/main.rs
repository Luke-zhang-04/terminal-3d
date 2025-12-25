pub mod matrix3;
pub mod terminal;
pub mod vector3;
pub mod world;
pub mod world_object;

use std::rc::Rc;

use crate::{terminal::Terminal, vector3::Vector3, world::World, world_object::WorldObject};

struct Square {
    vertices: Vec<Vector3>,
}

impl Square {
    pub fn new() -> Square {
        Square {
            vertices: vec![
                vector3!(0, 0, 0),
                vector3!(10, 0, 0),
                vector3!(10, 10, 0),
                vector3!(0, 10, 0),
            ],
        }
    }
}

impl WorldObject for Square {
    fn vectices(&self) -> Vec<Vector3> {
        self.vertices.clone()
    }
}

fn main() {
    let mut world = World::new();

    world.add_world_object(Rc::new(Square::new()));

    let mut terminal = Terminal::new(&mut world);
    terminal.render(1);
}

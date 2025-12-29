use crate::{terminal, vector3::Vector3, world_object::WorldObject};

pub struct Point {
    coordinate: Vector3,
}

impl Point {
    pub fn new(coordinate: Vector3) -> Point {
        Point { coordinate }
    }
}

impl WorldObject for Point {
    fn vectices(&self) -> Vec<Vector3> {
        vec![self.coordinate]
    }

    fn vertex_style(&self) -> terminal::Style {
        (
            '+',
            terminal::Color::Green,
            terminal::Decor::BoldHighIntensity,
        )
    }
}

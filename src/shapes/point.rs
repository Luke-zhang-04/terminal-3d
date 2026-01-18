use crate::{terminal, vector3::Vector3, world_object::WorldObject};

pub struct Point {
    coordinate: Vector3,
    color: terminal::Color,
}

impl Point {
    pub fn new(coordinate: Vector3, color: terminal::Color) -> Point {
        Point { coordinate, color }
    }
}

impl WorldObject for Point {
    fn vectices(&self) -> Vec<Vector3> {
        vec![self.coordinate]
    }

    fn vertex_style(&self) -> terminal::Style {
        ('+', self.color, terminal::Decor::BoldHighIntensity)
    }
}

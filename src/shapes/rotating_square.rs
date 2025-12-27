use std::f64::consts::PI;

use crate::{matrix3, vector3};
use crate::{matrix3::Matrix3, terminal, vector3::Vector3, world_object::WorldObject};

pub struct RotatingSquare {
    vertices: Vec<Vector3>,
    edges: Vec<(usize, usize)>,
}

impl RotatingSquare {
    pub fn new() -> RotatingSquare {
        RotatingSquare {
            vertices: vec![
                vector3!(0, 0, 0),
                vector3!(10, 0, 0),
                vector3!(10, 10, 0),
                vector3!(0, 10, 0),
            ],
            edges: vec![(0, 1), (1, 2), (2, 3), (3, 0)],
        }
    }
}

impl WorldObject for RotatingSquare {
    fn vectices(&self) -> Vec<Vector3> {
        self.vertices.clone()
    }

    fn vertex_style(&self) -> terminal::Style {
        (
            'X',
            terminal::Color::Blue,
            terminal::Decor::BoldHighIntensity,
        )
    }

    fn edges(&self) -> Vec<(usize, usize)> {
        self.edges.clone()
    }

    fn update(&mut self, frame: u64) {
        if frame == 1 {
            return;
        };
        let angle = PI / 36.0; // 5 degrees
        let mat = matrix3!(
            (angle.cos(), angle.sin(), 0),
            (-angle.sin(), angle.cos(), 0),
            (0, 0, 1)
        );
        let rotation_center = vector3!(10, 10, 0);

        for vertex in &mut self.vertices {
            *vertex = mat * (*vertex - rotation_center) + rotation_center;
        }
    }
}

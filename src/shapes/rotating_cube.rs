use std::f64::consts::PI;

use crate::{matrix3, vector3};
use crate::{matrix3::Matrix3, terminal, vector3::Vector3, world_object::WorldObject};

pub struct RotatingCube {
    vertices: Vec<Vector3>,
    edges: Vec<(usize, usize)>,
}

impl RotatingCube {
    pub fn new() -> RotatingCube {
        RotatingCube {
            vertices: vec![
                vector3!(0, 0, 0),
                vector3!(10, 0, 0),
                vector3!(10, 10, 0),
                vector3!(0, 10, 0),
                vector3!(0, 0, -10),
                vector3!(10, 0, -10),
                vector3!(10, 10, -10),
                vector3!(0, 10, -10),
            ],
            edges: vec![
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 0),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 4),
                (0, 4),
                (1, 5),
                (2, 6),
                (3, 7),
            ],
        }
    }
}

impl WorldObject for RotatingCube {
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
            (angle.cos(), 0, -angle.sin()),
            (0, 1, 0),
            (angle.sin(), 0, angle.cos())
        );
        let rotation_center = vector3!(5, 5, -5);

        for vertex in &mut self.vertices {
            *vertex = mat * (*vertex - rotation_center) + rotation_center;
        }
    }
}

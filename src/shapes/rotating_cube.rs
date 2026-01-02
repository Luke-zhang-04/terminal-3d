use std::f64::consts::PI;

use crate::{matrix3, vector3};
use crate::{matrix3::Matrix3, terminal, vector3::Vector3, world_object::WorldObject};

pub struct RotatingCube {
    vertices: Vec<Vector3>,
    edges: Vec<(usize, usize)>,
    rotation_point: Vector3,
}

impl RotatingCube {
    pub fn new(middle: Vector3, size: u16) -> RotatingCube {
        let half_size = size as f64 / 2.0;

        RotatingCube {
            vertices: vec![
                middle + vector3!(-half_size, -half_size, half_size),
                middle + vector3!(half_size, -half_size, half_size),
                middle + vector3!(half_size, half_size, half_size),
                middle + vector3!(-half_size, half_size, half_size),
                middle + vector3!(-half_size, -half_size, -half_size),
                middle + vector3!(half_size, -half_size, -half_size),
                middle + vector3!(half_size, half_size, -half_size),
                middle + vector3!(-half_size, half_size, -half_size),
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
            rotation_point: middle,
        }
    }

    pub fn default() -> RotatingCube {
        RotatingCube::new(Vector3::zero(), 10)
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
        if frame == 0 {
            return;
        };
        let angle = PI / 36.0; // 5 degrees
        let mat = matrix3!(
            (angle.cos(), 0, -angle.sin()),
            (0, 1, 0),
            (angle.sin(), 0, angle.cos())
        );
        // let mat = matrix3!(
        //     (1, 0, 0),
        //     (0, angle.cos(), angle.sin()),
        //     (0, -angle.sin(), angle.cos())
        // );
        for vertex in &mut self.vertices {
            *vertex = mat * (*vertex - self.rotation_point) + self.rotation_point;

            // if (frame / 10) % 2 == 0 {
            //     *vertex += vector3!(0, 1, 0);
            //     self.rotation_point += vector3!(0, 1, 0);
            // } else {
            //     *vertex += vector3!(0, -1, 0);
            //     self.rotation_point += vector3!(0, -1, 0);
            // }
        }
    }
}

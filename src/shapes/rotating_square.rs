use std::f64::consts::PI;

use crate::{matrix3, vector3};
use crate::{matrix3::Matrix3, terminal, vector3::Vector3, world_object::WorldObject};

pub struct RotatingSquare {
    vertices: Vec<Vector3>,
    edges: Vec<(usize, usize)>,
    triangles: Vec<(usize, usize, usize)>,
    rotation_point: Vector3,
}

impl RotatingSquare {
    pub fn new(middle: Vector3, size: u16) -> RotatingSquare {
        let half_size = size as f64 / 2.0;

        RotatingSquare {
            vertices: vec![
                middle + vector3!(-half_size, -half_size, half_size),
                middle + vector3!(half_size, -half_size, half_size),
                middle + vector3!(half_size, half_size, half_size),
                middle + vector3!(-half_size, half_size, half_size),
            ],
            edges: vec![(0, 1), (1, 2), (2, 3), (3, 0)],
            triangles: vec![(0, 2, 3), (0, 1, 2)],
            rotation_point: middle,
        }
    }

    pub fn default() -> RotatingSquare {
        RotatingSquare::new(Vector3::zero(), 10)
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

    fn triangles(&self) -> Vec<(usize, usize, usize)> {
        self.triangles.clone()
    }

    fn update(&mut self, frame: u64) {
        if frame == 0 {
            return;
        };
        let angle = PI / 36.0; // 5 degrees
        let mat = matrix3!(
            (angle.cos(), angle.sin(), 0),
            (-angle.sin(), angle.cos(), 0),
            (0, 0, 1)
        );

        for vertex in &mut self.vertices {
            *vertex = mat * (*vertex - self.rotation_point) + self.rotation_point;
        }
    }
}

pub mod matrix3;
pub mod render;
pub mod terminal;
pub mod vector3;
pub mod world;
pub mod world_object;

use std::{
    f64::consts::PI,
    ops::{Deref, DerefMut},
    thread,
    time::{self, Duration},
};

use crate::{
    matrix3::Matrix3, terminal::Terminal, vector3::Vector3, world::World, world_object::WorldObject,
};

struct Square {
    vertices: Vec<Vector3>,
    edges: Vec<(usize, usize)>,
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
            edges: vec![(0, 1), (1, 2), (2, 3), (3, 0)],
        }
    }
}

impl WorldObject for Square {
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

static FPS: u16 = 24;

fn main() {
    let mut world = World::new();

    world.add_world_object(Box::new(Square::new()));

    let mut terminal = Terminal::new();
    let frame_time = Duration::from_secs_f64(1.0 / FPS as f64);

    for frame in 1..=200 {
        let start = time::Instant::now();
        for obj in world.values_mut() {
            obj.deref_mut().update(frame);
        }
        terminal.pre_render();
        for obj in world.values() {
            terminal.buffer_world_object(obj.deref(), frame);
        }
        let end = time::Instant::now();
        if end - start < frame_time {
            thread::sleep(frame_time - (end - start));
        }

        terminal.render();
    }
}

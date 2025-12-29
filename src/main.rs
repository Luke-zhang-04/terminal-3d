pub mod camera;
pub mod matrix3;
pub mod render;
pub mod shapes;
pub mod terminal;
pub mod vector3;
pub mod world;
pub mod world_object;

use std::{
    ops::{Deref, DerefMut},
    thread,
    time::{self, Duration},
};

use crate::{terminal::Terminal, vector3::Vector3, world::World};

static FPS: u16 = 12;

fn main() {
    let mut world = World::new();

    world.add_world_object(Box::new(shapes::RotatingCube::default()));
    world.add_world_object(Box::new(shapes::Point::new(vector3!(0, 0, -5))));
    world.add_world_object(Box::new(shapes::Point::new(vector3!(20, 20, 0))));

    let mut terminal = Terminal::new();
    let camera = camera::PerspectiveCamera::new(
        90,
        vector3!(0, 30, 30),
        vector3!(0, -1, -1),
        vector3!(0, 1, -1),
        terminal.get_term_size(),
    );
    let frame_time = Duration::from_secs_f64(1.0 / FPS as f64);

    for frame in 1..=200 {
        let start = time::Instant::now();
        for obj in world.values_mut() {
            obj.deref_mut().update(frame);
        }
        terminal.pre_render();
        for obj in world.values() {
            terminal.buffer_world_object(obj.deref(), &camera, frame);
        }
        let end = time::Instant::now();
        if end - start < frame_time {
            thread::sleep(frame_time - (end - start));
        }

        terminal.render();
    }
}

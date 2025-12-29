pub mod iso_camera;
pub mod perspective_camera;

pub use crate::camera::iso_camera::IsoCamera;
pub use crate::camera::perspective_camera::PerspectiveCamera;

use crate::vector3::Vector3;

pub trait Camera {
    fn update_screen_size(&mut self, screen_size: (u16, u16));
    fn get_screen_size(&self) -> (u16, u16);
    fn update_observation_point(&mut self, point: Vector3, direction: Vector3);
    fn get_observation_point(&self) -> (Vector3, Vector3);

    /// Make recalculations based on camera parameters. Called when camera parameters are updated.
    fn recalculate(&mut self);
    /// Take a vector in the world, and return a vector relative to the screen,
    /// where x = 0, y = 0 is the top left of the screen, and z is the distance away from the
    /// observation point.
    /// In addition, for points outside the screen, the function should still return the projected
    /// coordinate relative to the screen, to allow for line/face drawing items partially inside
    /// the screen. If a point is behind the observer, the returned depth should be negative.
    fn project_vector(&self, vec: Vector3) -> Vector3;
}

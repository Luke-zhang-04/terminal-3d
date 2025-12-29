use crate::camera::Camera;
use crate::vector3;
use crate::vector3::Vector3;

/// Perspective camera
pub struct PerspectiveCamera {
    /// Field of view, in degrees
    fov: u16,
    /// Point we are observing from
    observation_point: Vector3,
    /// Direction we are looking
    observation_direction: Vector3,
    /// Upwards direction relative to camera
    orientation: Vector3,
    /// Size of screen the world should be projected onto, (width, height)
    screen_size: (u16, u16),
    screen_top_left: Vector3,
}

impl Camera for PerspectiveCamera {
    fn update_screen_size(&mut self, screen_size: (u16, u16)) {
        self.screen_size = screen_size;
        self.recalculate();
    }

    fn get_screen_size(&self) -> (u16, u16) {
        self.screen_size
    }

    fn update_observation_point(&mut self, point: Vector3, direction: Vector3) {
        self.observation_point = point;
        self.observation_direction = direction;
        self.recalculate();
    }

    fn get_observation_point(&self) -> (Vector3, Vector3) {
        (self.observation_point, self.observation_direction)
    }

    fn recalculate(&mut self) {
        let screen_distance =
            self.screen_size.0 as f64 / (2.0 * ((self.fov as f64).to_radians() / 2.0).tan());

        let up = self.orientation;
        let forward = self.observation_direction;
        let left = up * self.observation_direction;

        self.screen_top_left = self.observation_point
            + (self.screen_size.0 as f64 / 2.0) * left
            + (self.screen_size.1 as f64 / 2.0) * up
            + screen_distance * forward;
    }

    fn project_vector(&self, vec: Vector3) -> Vector3 {
        let project_direction = (vec - self.observation_point).normalize();
        let normal_vector = self.observation_direction;
        let incoming = (vec - self.screen_top_left).dot(normal_vector);

        if incoming.abs() < f64::EPSILON {
            (vec - self.screen_top_left).neg_y()
        } else {
            //TODO: why subtract???
            let pt = vec - project_direction * (incoming / (project_direction.dot(normal_vector)));

            (pt - self.screen_top_left).neg_y() + Vector3::new(0.0, 0.0, vec.distance_to(pt))
        }
    }
}

impl PerspectiveCamera {
    pub fn new(
        fov: u16,
        observation_point: Vector3,
        observation_direction: Vector3,
        orientation: Vector3,
        screen_size: (u16, u16),
    ) -> PerspectiveCamera {
        let mut camera = PerspectiveCamera {
            fov,
            observation_point,
            observation_direction: observation_direction.normalize(),
            orientation: orientation.normalize(),
            screen_size,
            screen_top_left: Vector3::zero(),
        };

        camera.recalculate();

        camera
    }

    pub fn default(screen_size: (u16, u16)) -> PerspectiveCamera {
        let mut camera = PerspectiveCamera {
            fov: 90,
            observation_point: vector3!(0, 0, 30),
            observation_direction: vector3!(0, 0, -1).normalize(),
            orientation: vector3!(0, 1, 0).normalize(),
            screen_size,
            screen_top_left: Vector3::zero(),
        };

        camera.recalculate();

        camera
    }

    pub fn update_fov(&mut self, fov: u16) {
        self.fov = fov;
        self.recalculate();
    }

    pub fn get_fov(&self) -> u16 {
        self.fov
    }
}

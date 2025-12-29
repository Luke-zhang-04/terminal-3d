use crate::camera::Camera;
use crate::vector3;
use crate::vector3::Vector3;

/// Isometric camera
pub struct IsoCamera {
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

impl Camera for IsoCamera {
    fn update_screen_size(&mut self, screen_size: (u16, u16)) {
        self.screen_size = screen_size;
    }

    fn get_screen_size(&self) -> (u16, u16) {
        self.screen_size
    }

    fn update_observation_point(&mut self, point: Vector3, direction: Vector3) {
        self.observation_point = point;
        self.observation_direction = direction;
    }

    fn get_observation_point(&self) -> (Vector3, Vector3) {
        (self.observation_point, self.observation_direction)
    }

    fn recalculate(&mut self) {
        let up = self.orientation;
        let left = up * self.observation_direction;

        self.screen_top_left = self.observation_point
            + (self.screen_size.0 as f64 / 2.0) * left
            + (self.screen_size.1 as f64 / 2.0) * up;
    }

    fn project_vector(&self, vec: Vector3) -> Vector3 {
        let normal_vector = self.observation_direction;
        let incoming = (vec - self.observation_point).dot(normal_vector);

        if incoming < f64::EPSILON {
            (vec - self.screen_top_left).neg_y()
        } else {
            let pt = vec + normal_vector * (incoming / (-normal_vector.dot(normal_vector)));

            (pt - self.screen_top_left).neg_y() + Vector3::new(0.0, 0.0, vec.distance_to(pt))
        }
    }
}

impl IsoCamera {
    pub fn new(
        observation_point: Vector3,
        observation_direction: Vector3,
        orientation: Vector3,
        screen_size: (u16, u16),
    ) -> IsoCamera {
        let mut camera = IsoCamera {
            observation_point,
            observation_direction: observation_direction.normalize(),
            orientation: orientation.normalize(),
            screen_size,
            screen_top_left: Vector3::zero(),
        };

        camera.recalculate();

        camera
    }

    pub fn default(screen_size: (u16, u16)) -> IsoCamera {
        let mut camera = IsoCamera {
            observation_point: vector3!(0, 0, 10),
            observation_direction: vector3!(0, 0, -1).normalize(),
            orientation: vector3!(0, 1, 0).normalize(),
            screen_size,
            screen_top_left: Vector3::zero(),
        };

        camera.recalculate();

        camera
    }
}

use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(vfov_deg: f64, aspect_ratio: f64) -> Self {
        let origin = Vec3::init(0.0, 0.0, 0.0);

        let theta = vfov_deg.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;

        Camera {
            lower_left_corner: Vec3::init(-half_width, -half_height, -1.0),
            origin,
            horizontal: Vec3::init(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::init(0.0, 2.0 * half_height, 0.0),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: &self.lower_left_corner + &self.horizontal * u + &self.vertical * v
                - &self.origin,
        }
    }
}

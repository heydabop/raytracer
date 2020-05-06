use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Vec3::init(0.0, 0.0, 0.0);
        let horizontal = Vec3::init(4.0, 0.0, 0.0);
        let vertical = Vec3::init(0.0, 2.25, 0.0);
        Camera {
            lower_left_corner: &origin
                - &(&horizontal / 2.0)
                - &vertical / 2.0
                - Vec3::init(0.0, 0.0, 1.0),
            origin,
            horizontal,
            vertical,
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

impl Default for Camera {
    fn default() -> Self {
        Camera::new()
    }
}

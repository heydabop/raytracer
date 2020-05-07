use super::ray::Ray;
use super::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, target: &Vec3, up: &Vec3, vfov_deg: f64, aspect_ratio: f64) -> Self {
        let theta = vfov_deg.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;
        let w = (&origin - target).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: &origin - &(&u * half_width) - &v * half_height - w,
            origin,
            horizontal: u * half_width * 2.0,
            vertical: v * half_height * 2.0,
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

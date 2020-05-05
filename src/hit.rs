use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Debug, PartialEq)]
pub enum Hit {
    Hit {
        point: Vec3,
        normal: Vec3,
        t: f64,
        front_face: bool,
    },
    Miss,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Hit;
}

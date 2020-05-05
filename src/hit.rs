use super::ray::Ray;
use super::vec3::Vec3;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq)]
pub struct HitData {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Debug, PartialEq)]
pub enum Hit {
    Hit(HitData),
    Miss,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Hit;
}

use super::material::MaterialWritable;
use super::ray::Ray;
use super::vec3::Vec3;
use std::rc::Rc;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn MaterialWritable>,
}

impl PartialEq for Hit {
    fn eq(&self, rhs: &Self) -> bool {
        self.point == rhs.point
            && self.normal == rhs.normal
            && self.t == rhs.t
            && self.front_face == rhs.front_face
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

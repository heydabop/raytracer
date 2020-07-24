use super::aabb::AABB;
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
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Rc<dyn MaterialWritable>,
}

impl PartialEq for Hit {
    fn eq(&self, rhs: &Self) -> bool {
        self.point == rhs.point
            && self.normal == rhs.normal
            && self.t == rhs.t
            && self.u == rhs.u
            && self.v == rhs.v
            && self.front_face == rhs.front_face
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = None;
        let mut closest = t_max;

        for obj in self {
            if let Some(obj_hit) = obj.hit(r, t_min, closest) {
                closest = obj_hit.t;
                hit = Some(obj_hit);
            }
        }

        hit
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.is_empty() {
            // nothing to box
            return None;
        }

        // build up a bounding box of entire scene
        let mut bounding_box: Option<AABB> = None;

        for o in self {
            // compute this object's bounding box
            if let Some(o_box) = o.bounding_box(t0, t1) {
                // if this object has a BB, add it to the scene's box (if we have one), otherwise this first box is now our scene's box
                bounding_box = if let Some(t_box) = bounding_box {
                    Some(t_box.surrounding_box(&o_box))
                } else {
                    Some(o_box)
                };
            } else {
                // if any object doesn't have a bounding box then neither does this scene
                return None;
            }
        }

        bounding_box
    }
}

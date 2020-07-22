use super::aabb::AABB;
use super::hit::{Hit, Hittable};
use super::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BVH {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bounding_box: AABB,
}

impl BVH {
    pub fn new<T: Rng>(rng: &mut T, mut objects: Vec<Rc<dyn Hittable>>, t0: f64, t1: f64) -> Self {
        let axis: u8 = rng.gen_range(0, 3);
        let comparator = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| compare(a, b, axis);
        let len = objects.len();

        let (left, right): (Rc<dyn Hittable>, Rc<dyn Hittable>) = match len.cmp(&2) {
            Ordering::Less => (objects[0].clone(), objects[0].clone()),
            Ordering::Equal => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            Ordering::Greater => {
                objects.sort_by(comparator);

                let right_half = objects.split_off(len / 2);
                (
                    Rc::new(BVH::new(rng, objects, t0, t1)),
                    Rc::new(BVH::new(rng, right_half, t0, t1)),
                )
            }
        };

        let box_left = left.bounding_box(t0, t1);
        let box_right = right.bounding_box(t0, t1);
        if box_left.is_none() || box_right.is_none() {
            panic!("No bounding box in BVH construction");
        }

        BVH {
            left,
            right,
            bounding_box: box_left.unwrap().surrounding_box(&box_right.unwrap()),
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self
            .right
            .hit(r, t_min, if let Some(l) = &hit_left { l.t } else { t_max });

        if let Some(r) = hit_right {
            Some(r)
        } else {
            hit_left
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}

fn compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: u8) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);
    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in BVH construction");
    }

    box_b.unwrap().min[axis].total_cmp(&box_a.unwrap().min[axis])
}

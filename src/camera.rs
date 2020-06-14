use super::ray::Ray;
use super::vec3::Vec3;
use rand::Rng;
use rand_pcg::Pcg64Mcg;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    time0: f64,
    time1: f64,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        origin: Vec3,
        target: &Vec3,
        up: &Vec3,
        vfov_deg: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = vfov_deg.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect_ratio;
        let w = (&origin - target).unit_vector();
        let u = up.cross(&w).unit_vector();
        let v = w.cross(&u);

        Camera {
            lower_left_corner: &origin
                - &(&u * half_width * focus_dist)
                - &v * half_height * focus_dist
                - &w * focus_dist,
            origin,
            horizontal: &u * half_width * 2.0 * focus_dist,
            vertical: &v * half_height * 2.0 * focus_dist,
            lens_radius: aperture / 2.0,
            u,
            v,
            time0,
            time1,
        }
    }

    pub fn ray(&mut self, mut rng: &mut Pcg64Mcg, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk(&mut rng) * self.lens_radius;
        let offset = &self.u * rd.x + &self.v * rd.y;
        Ray {
            origin: &self.origin + &offset,
            direction: &self.lower_left_corner + &self.horizontal * s + &self.vertical * t
                - &self.origin
                - offset,
            time: rng.gen_range(self.time0, self.time1),
        }
    }
}

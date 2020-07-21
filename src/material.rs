use super::hit::Hit;
use super::ray::Ray;
use super::vec3::Vec3;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use std::fmt;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    // Returns (if ray scatters) new scattered ray and attenuation of ray
    fn scatter(&self, r_in: &Ray, rng: &mut Pcg64Mcg, hit: &Hit) -> Option<Scatter>;
}

#[allow(clippy::module_name_repetitions)]
pub trait MaterialWritable: Material + fmt::Debug {}

// Lambertian

#[derive(Debug)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, mut rng: &mut Pcg64Mcg, hit: &Hit) -> Option<Scatter> {
        let scatter_direction = &hit.normal + Vec3::random_unit_vector(&mut rng);
        let scattered_ray = Ray {
            origin: hit.point.clone(),
            direction: scatter_direction,
            time: r_in.time,
        };

        Some(Scatter {
            ray: scattered_ray,
            attenuation: self.albedo.clone(),
        })
    }
}

impl MaterialWritable for Lambertian {}

// Metal

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    #[allow(dead_code)]
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, mut rng: &mut Pcg64Mcg, hit: &Hit) -> Option<Scatter> {
        let reflected = r_in.direction.unit_vector().reflect(&hit.normal);
        if reflected.dot(&hit.normal) > 0.0 {
            let scattered = Ray {
                origin: hit.point.clone(),
                direction: reflected + Vec3::random_in_unit_sphere(&mut rng) * self.fuzz,
                time: r_in.time,
            };
            return Some(Scatter {
                ray: scattered,
                attenuation: self.albedo.clone(),
            });
        }

        None
    }
}

impl MaterialWritable for Metal {}

// Dielectric

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn schlick(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 *= r0;
        (1.0 - r0).mul_add((1.0 - cosine).powi(5), r0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rng: &mut Pcg64Mcg, hit: &Hit) -> Option<Scatter> {
        let attenuation = Vec3::from_xyz(1.0, 1.0, 1.0);
        let eta_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = (-&unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (-cos_theta.mul_add(cos_theta, -1.0)).sqrt();
        if eta_ratio * sin_theta > 1.0 || rng.gen::<f64>() < Self::schlick(cos_theta, eta_ratio) {
            let reflected = unit_direction.reflect(&hit.normal);
            let scattered = Ray {
                origin: hit.point.clone(),
                direction: reflected,
                time: r_in.time,
            };
            return Some(Scatter {
                ray: scattered,
                attenuation,
            });
        }

        let refracted = unit_direction.refract(&hit.normal, eta_ratio);
        let scattered = Ray {
            origin: hit.point.clone(),
            direction: refracted,
            time: r_in.time,
        };

        Some(Scatter {
            ray: scattered,
            attenuation,
        })
    }
}

impl MaterialWritable for Dielectric {}

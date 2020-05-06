#![feature(clamp)]

mod camera;
mod hit;
mod material;
mod ppm;
mod ray;
mod scene;
mod sphere;
mod vec3;

use camera::Camera;
use material::Lambertian;
use rand::prelude::*;
use scene::Scene;
use sphere::Sphere;
use std::io::{self, Write};
use std::rc::Rc;
use vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 1280;
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let image_height = (f64::from(image_width) / aspect_ratio).round() as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut stdout = io::stdout();

    let camera = Camera::new();

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere {
        center: Vec3::init(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian::new(Vec3::init(0.5, 0.5, 0.5))),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::init(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Lambertian::new(Vec3::init(0.5, 0.5, 0.5))),
    }));

    let mut colors: Vec<Vec<Vec3>> = vec![];

    let mut rng = thread_rng();

    for j in (0..image_height).rev() {
        let mut row: Vec<Vec3> = vec![];

        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Vec3::init(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen::<f64>()) / f64::from(image_width - 1);
                let v = (f64::from(j) + rng.gen::<f64>()) / f64::from(image_height - 1);
                let r = camera.ray(u, v);
                pixel_color += &r.color(&scene, max_depth);
            }
            row.push(pixel_color);
        }
        colors.push(row);
    }

    stdout
        .write_all(ppm::p6_image(&colors, samples_per_pixel).as_slice())
        .unwrap();

    eprintln!("\nDone.");
}

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
use material::{Lambertian, Metal};
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use scene::Scene;
use sphere::Sphere;
use std::io::{self, Write};
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use vec3::Vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 1280;
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let image_height = (f64::from(image_width) / aspect_ratio).round() as u16;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut stdout = io::stdout();

    let cam_center = Vec3::from_xyz(1.5, 1.0, 1.0);
    let cam_target = Vec3::from_xyz(0.0, 0.2, -1.0);
    let cam_up = Vec3::from_xyz(0.0, 1.0, 0.0);

    let camera = Camera::new(cam_center, &cam_target, &cam_up, 45.0, aspect_ratio);

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.7, 0.3, 0.3))),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.8, 0.8, 0.0))),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal::new(Vec3::from_xyz(0.8, 0.8, 0.8), 0.2)),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal::new(Vec3::from_xyz(0.8, 0.6, 0.2), 0.8)),
    }));

    let mut colors: Vec<Vec<Vec3>> = vec![];

    let mut rng = Pcg64Mcg::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );

    for j in (0..image_height).rev() {
        let mut row: Vec<Vec3> = vec![];

        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Vec3::from_xyz(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen_range(0.0, 1.0)) / f64::from(image_width - 1);
                let v = (f64::from(j) + rng.gen_range(0.0, 1.0)) / f64::from(image_height - 1);
                let r = camera.ray(u, v);
                pixel_color += &r.color(&scene, &mut rng, max_depth);
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

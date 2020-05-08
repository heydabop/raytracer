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
use material::{Dielectric, Lambertian, Metal};
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use scene::Scene;
use sphere::Sphere;
use std::io::{self, Write};
use std::rc::Rc;
use std::thread::{self, JoinHandle};
use std::time::{SystemTime, UNIX_EPOCH};
use vec3::Vec3;

fn main() {
    let num_threads = 8;

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1920;
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let image_height = (f64::from(image_width) / aspect_ratio).round() as u32;
    let samples_per_pixel = 200;
    let max_depth = 50;

    if image_width % num_threads != 0 {
        // TODO
        panic!("image width % threads != 0");
    }
    let rows_per_thread = image_height / num_threads;

    let mut stdout = io::stdout();

    let mut last_handle: Option<JoinHandle<Vec<Vec3>>> = None;

    for n in (0..num_threads).rev() {
        let this_last_handle = if last_handle.is_some() {
            last_handle.take()
        } else {
            None
        };
        let handle = thread::spawn(move || {
            let mut thread_colors = render_scene_slice(
                aspect_ratio,
                image_width,
                image_height,
                rows_per_thread,
                n,
                samples_per_pixel,
                max_depth,
            );

            eprintln!("Thread {} finished", n);

            if let Some(h) = this_last_handle {
                let mut last_colors = h.join().unwrap();
                last_colors.append(&mut thread_colors);
                last_colors
            } else {
                thread_colors
            }
        });
        last_handle = Some(handle);
    }

    let colors = last_handle.unwrap().join().unwrap();

    stdout
        .write_all(ppm::p6_image(image_width, image_height, &colors, samples_per_pixel).as_slice())
        .unwrap();

    eprintln!("\nDone.");
}

fn render_scene_slice(
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    slice_width: u32,
    slice_num: u32,
    samples_per_pixel: u16,
    max_depth: u16,
) -> Vec<Vec3> {
    let cam_center = Vec3::from_xyz(-1.5, 0.8, 1.0);
    let cam_target = Vec3::from_xyz(0.0, 0.1, -1.0);
    let cam_up = Vec3::from_xyz(0.0, 1.0, 0.0);
    let cam_focus_dist = (&cam_center - &cam_target).length();
    let cam_aperture = 0.1;

    let mut camera = Camera::new(
        cam_center,
        &cam_target,
        &cam_up,
        50.0,
        aspect_ratio,
        cam_aperture,
        cam_focus_dist,
    );

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.1, 0.2, 0.5))),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(0.0, -500.5, -1.0),
        radius: 500.0,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.8, 0.8, 0.0))),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Dielectric::new(1.5)),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal::new(Vec3::from_xyz(0.8, 0.6, 0.2), 0.1)),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(-0.55, -0.302_263, -2.4),
        radius: 0.2,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.9, 0.07, 0.05))),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(-1.4, 0.0868, 1.2),
        radius: 0.58,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.8, 0.8, 0.8))),
    }));

    let mut colors: Vec<Vec3> = Vec::with_capacity(slice_height as usize * image_width as usize);

    let mut rng = Pcg64Mcg::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );

    for j in (slice_height * slice_num..slice_height * (slice_num + 1)).rev() {
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let mut pixel_color = Vec3::new();
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen_range(0.0, 1.0)) / f64::from(image_width - 1);
                let v = (f64::from(j) + rng.gen_range(0.0, 1.0)) / f64::from(image_height - 1);
                let r = camera.ray(u, v);
                pixel_color += &r.color(&scene, &mut rng, max_depth);
            }
            colors.push(pixel_color);
        }
    }

    colors
}

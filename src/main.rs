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
use std::time::{Instant, SystemTime, UNIX_EPOCH};
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
    let columns_per_thread = image_width / num_threads;

    let scene_seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut last_handle: Option<JoinHandle<Vec<Vec<Vec3>>>> = None;

    for n in 0..num_threads {
        // split image up into vertical slices and render each one on its own thread

        // the algorithm for splitting/combining horizontally instead of vertically is more striaght forward
        // but it can lead to less even workloads when the upper 3rd of an image is blank horizon/sky
        let this_last_handle = if last_handle.is_some() {
            last_handle.take()
        } else {
            None
        };
        let handle = thread::spawn(move || {
            let now = Instant::now();

            let thread_colors = render_scene_slice(
                aspect_ratio,
                image_width,
                image_height,
                columns_per_thread,
                n,
                samples_per_pixel,
                max_depth,
                scene_seed,
            );

            eprintln!(
                "Thread {} finished in {:.3}s",
                n,
                now.elapsed().as_secs_f64()
            );

            if let Some(h) = this_last_handle {
                let mut last_colors = h.join().unwrap();
                last_colors.push(thread_colors);
                last_colors
            } else {
                vec![thread_colors]
            }
        });
        last_handle = Some(handle);
    }

    let colors_by_slice = last_handle.unwrap().join().unwrap();

    // Right now we have a vector of vertical slices, we need to flatten this into a single vector by taking chunks of slice_width from each vector and concating them together

    let mut colors = Vec::with_capacity(image_width as usize * image_height as usize);

    for n in 0..image_height {
        for v in &colors_by_slice {
            colors.extend_from_slice(
                &v[(columns_per_thread * n) as usize..(columns_per_thread * (n + 1)) as usize],
            );
        }
    }

    io::stdout()
        .write_all(ppm::p6_image(image_width, image_height, &colors, samples_per_pixel).as_slice())
        .unwrap();

    eprintln!("\nDone.");
}

#[allow(clippy::too_many_arguments)]
fn render_scene_slice(
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    slice_width: u32,
    slice_num: u32,
    samples_per_pixel: u16,
    max_depth: u16,
    scene_seed: u128,
) -> Vec<Vec3> {
    // Generating the camera, scene, and its contents thread local is much easier than sharing it, even for read only

    let cam_center = Vec3::from_xyz(13.0, 2.5, 3.5);
    let cam_target = Vec3::from_xyz(0.0, 0.0, 0.0);
    let cam_up = Vec3::from_xyz(0.0, 1.0, 0.0);
    let cam_focus_dist = 10.0;
    let cam_aperture = 0.1;
    let cam_vfov = 20.0;

    let mut camera = Camera::new(
        cam_center,
        &cam_target,
        &cam_up,
        cam_vfov,
        aspect_ratio,
        cam_aperture,
        cam_focus_dist,
    );

    let scene = random_spheres(scene_seed);

    let mut colors: Vec<Vec3> = Vec::with_capacity(slice_width as usize * image_height as usize);

    let mut rng = Pcg64Mcg::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            + u128::from(slice_num),
    );

    for j in (0..image_height).rev() {
        io::stderr().flush().unwrap();

        for i in slice_width * slice_num..slice_width * (slice_num + 1) {
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

fn random_spheres(scene_seed: u128) -> Scene {
    let mut scene = Scene::new();
    let ground_y = -1000.0;
    let ground_radius = 1000.0;
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(0.0, ground_y, 0.0),
        radius: ground_radius,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.5, 0.5, 0.5))),
    }));

    let mut rng = Pcg64Mcg::new(scene_seed);

    let intersection_check = Vec3::from_xyz(4.0, 0.2, 0.0);

    let gen_width = 11;

    for a in -gen_width..gen_width {
        for b in -gen_width..gen_width {
            let choose_material: f64 = rng.gen();
            let x = rng.gen::<f64>().mul_add(0.9, f64::from(a));
            let z = rng.gen::<f64>().mul_add(0.9, f64::from(b));
            let radius = 0.2;
            let y = surface_y(x, z, ground_radius + radius, ground_y);

            let center = Vec3::from_xyz(x, y, z);
            if (&center - &intersection_check).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo =
                        Vec3::random(&mut rng, 0.0, 1.0) * Vec3::random(&mut rng, 0.0, 1.0);
                    scene.add(Box::new(Sphere {
                        center,
                        radius,
                        material: Rc::new(Lambertian::new(albedo)),
                    }));
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    scene.add(Box::new(Sphere {
                        center,
                        radius,
                        material: Rc::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    scene.add(Box::new(Sphere {
                        center,
                        radius,
                        material: Rc::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Dielectric::new(1.5)),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(
            -4.0,
            surface_y(-4.0, 0.0, ground_radius + 1.0, ground_y),
            0.0,
        ),
        radius: 1.0,
        material: Rc::new(Lambertian::new(Vec3::from_xyz(0.4, 0.2, 0.1))),
    }));
    scene.add(Box::new(Sphere {
        center: Vec3::from_xyz(4.0, surface_y(4.0, 0.0, ground_radius + 1.0, ground_y), 0.0),
        radius: 1.0,
        material: Rc::new(Metal::new(Vec3::from_xyz(0.7, 0.6, 0.5), 0.0)),
    }));

    scene
}

fn surface_y(x: f64, z: f64, combined_radius: f64, ground_y: f64) -> f64 {
    ground_y + (x.mul_add(-x, z.mul_add(-z, combined_radius * combined_radius))).sqrt()
}

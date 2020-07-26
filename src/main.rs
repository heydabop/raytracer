#![feature(new_uninit)]
#![feature(total_cmp)]
#![feature(clamp)]

mod aabb;
mod bvh;
mod camera;
mod hit;
mod material;
mod moving_sphere;
mod ppm;
mod ray;
mod sphere;
mod texture;
mod vec3;

use camera::Camera;
use hit::Hittable;
use material::{Dielectric, Lambertian, Metal};
use moving_sphere::MovingSphere;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use sphere::Sphere;
use std::io::{self, Write};
use std::rc::Rc;
use std::thread::{self, JoinHandle};
use std::time::{SystemTime, UNIX_EPOCH};
use texture::{CheckerTexture, SolidColor};
use vec3::Vec3;

fn main() {
    let num_threads = 8;

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1280;
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let image_height = (f64::from(image_width) / aspect_ratio).round() as u32;
    let samples_per_thread = 50 / num_threads;
    let max_depth = 50;

    let scene_seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let mut last_handle: Option<JoinHandle<Vec<Vec3>>> = None;

    for n in 0..num_threads {
        // render image once per thread and average out the images

        // each thread adds its colors to the running vector of colors from the last thread
        // we then divide colors by total samples during image gen to get averaged color
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
                n,
                samples_per_thread,
                max_depth,
                scene_seed,
            );

            if let Some(h) = this_last_handle {
                let last_colors = h.join().unwrap();
                for i in 0..thread_colors.len() {
                    thread_colors[i] += last_colors[i]
                }
            }
            thread_colors
        });
        last_handle = Some(handle);
    }

    let colors = last_handle.unwrap().join().unwrap();

    io::stdout()
        .write_all(
            ppm::p6_image(
                image_width,
                image_height,
                &colors,
                samples_per_thread * num_threads,
            )
            .as_slice(),
        )
        .unwrap();

    eprintln!("\nDone.");
}

#[allow(clippy::too_many_arguments)]
fn render_scene_slice(
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    slice_num: u32,
    samples_per_pixel: u32,
    max_depth: u16,
    scene_seed: u128,
) -> Vec<Vec3> {
    // Generating the camera, scene, and its contents thread local is much easier than sharing it, even for read only

    let cam_center = Vec3::from_xyz(13.0, 2.5, 3.5);
    let cam_target = Vec3::from_xyz(0.0, 0.0, 0.0);
    let cam_up = Vec3::from_xyz(0.0, 1.0, 0.0);
    let cam_focus_dist = 10.0;
    let cam_aperture = 0.0;
    let cam_vfov = 20.0;
    let cam_t1 = 0.0;
    let cam_t2 = 1.0;

    let mut camera = Camera::new(
        cam_center,
        cam_target,
        cam_up,
        cam_vfov,
        aspect_ratio,
        cam_aperture,
        cam_focus_dist,
        cam_t1,
        cam_t2,
    );

    let scene = random_spheres(scene_seed);

    let mut colors: Vec<Vec3> = Vec::with_capacity(image_width as usize * image_height as usize);

    let mut rng = Pcg64Mcg::new(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            + u128::from(slice_num),
    );

    for j in (0..image_height).rev() {
        if slice_num == 0 {
            eprint!("\rScanlines remaining: {} ", j);
        }

        for i in 0..image_width {
            let mut pixel_color = Vec3::new();
            for _ in 0..samples_per_pixel {
                let u = (f64::from(i) + rng.gen_range(0.0, 1.0)) / f64::from(image_width - 1);
                let v = (f64::from(j) + rng.gen_range(0.0, 1.0)) / f64::from(image_height - 1);
                let r = camera.ray(&mut rng, u, v);
                pixel_color += r.color(&scene, &mut rng, max_depth);
            }
            colors.push(pixel_color);
        }
    }

    colors
}

fn random_spheres(scene_seed: u128) -> bvh::BVH {
    let mut scene: Vec<Rc<dyn Hittable>> = vec![];
    let ground_y = -1000.0;
    let ground_radius = 1000.0;
    scene.push(Rc::new(Sphere {
        center: Vec3::from_xyz(0.0, ground_y, 0.0),
        radius: ground_radius,
        material: Rc::new(Lambertian::new(Box::new(CheckerTexture {
            even: Box::new(SolidColor::from_rgb(0.2, 0.3, 0.1)),
            odd: Box::new(SolidColor::from_rgb(0.9, 0.9, 0.9)),
        }))),
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
            if (center - intersection_check).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo =
                        Vec3::random(&mut rng, 0.0, 1.0) * Vec3::random(&mut rng, 0.0, 1.0);
                    let center1 = center + Vec3::from_xyz(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    scene.push(Rc::new(MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.0,
                        time1: 1.0,
                        radius,
                        material: Rc::new(Lambertian::new(Box::new(SolidColor { color: albedo }))),
                    }));
                } else if choose_material < 0.95 {
                    let albedo = Vec3::random(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    scene.push(Rc::new(Sphere {
                        center,
                        radius,
                        material: Rc::new(Metal::new(albedo, fuzz)),
                    }));
                } else {
                    scene.push(Rc::new(Sphere {
                        center,
                        radius,
                        material: Rc::new(Dielectric::new(1.5)),
                    }));
                }
            }
        }
    }

    scene.push(Rc::new(Sphere {
        center: Vec3::from_xyz(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Rc::new(Dielectric::new(1.5)),
    }));
    scene.push(Rc::new(Sphere {
        center: Vec3::from_xyz(
            -4.0,
            surface_y(-4.0, 0.0, ground_radius + 1.0, ground_y),
            0.0,
        ),
        radius: 1.0,
        material: Rc::new(Lambertian::new(Box::new(SolidColor::from_rgb(
            0.4, 0.2, 0.1,
        )))),
    }));
    scene.push(Rc::new(Sphere {
        center: Vec3::from_xyz(4.0, surface_y(4.0, 0.0, ground_radius + 1.0, ground_y), 0.0),
        radius: 1.0,
        material: Rc::new(Metal::new(Vec3::from_xyz(0.7, 0.6, 0.5), 0.0)),
    }));

    bvh::BVH::new(&mut rng, scene, 0.0, 1.0)
}

fn surface_y(x: f64, z: f64, combined_radius: f64, ground_y: f64) -> f64 {
    ground_y + (x.mul_add(-x, z.mul_add(-z, combined_radius * combined_radius))).sqrt()
}

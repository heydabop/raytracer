use std::io::{self, Write};

mod vec3;
use vec3::Vec3;

mod ray;
use ray::Ray;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 384;
    let image_height = (f64::from(image_width) / aspect_ratio).round() as u16;

    println!("P3\n{} {}\n255", image_width, image_height);

    let origin = Vec3::new();
    let horizontal = Vec3::init(4.0, 0.0, 0.0);
    let vertical = Vec3::init(0.0, 2.25, 0.0);
    let lower_left_corner =
        &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::init(0.0, 0.0, 1.0);

    let mut stdout = io::stdout();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let u = f64::from(i) / f64::from(image_width - 1);
            let v = f64::from(j) / f64::from(image_height - 1);
            let r = Ray {
                origin: origin.clone(),
                direction: &lower_left_corner + &horizontal * u + &vertical * v,
            };
            stdout.write_all(r.color().ppm_pixel().as_bytes()).unwrap();
        }
    }

    eprintln!("\nDone.");
}

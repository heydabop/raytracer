use std::io::{self, Write};

mod vec3;
use vec3::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut stdout = io::stdout();

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let pixel = Vec3(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            stdout.write_all(pixel.ppm_pixel().as_bytes()).unwrap();
        }
    }

    eprintln!("\nDone.");
}

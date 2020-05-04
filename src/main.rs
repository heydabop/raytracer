use std::io::{self, Write};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        io::stderr().flush().unwrap();

        for i in 0..image_width {
            let r = (255.999 * (i as f32 / (image_width - 1) as f32)) as i32;
            let g = (255.999 * (j as f32 / (image_height - 1) as f32)) as i32;
            let b = (255.899 * 0.25) as i32;

            println!("{} {} {}", r, g, b);
        }
    }

    eprintln!("\nDone.");
}

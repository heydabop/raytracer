use super::vec3::Vec3;

#[allow(dead_code)]
pub fn p3_header(width: u16, height: u16) -> String {
    format!("P3\n{} {}\n255\n", width, height)
}

#[allow(dead_code)]
pub fn p3_pixel(color: &Vec3, samples_per_pixel: u32) -> String {
    if !color.is_valid_color(samples_per_pixel) {
        panic!("Color {} {} out of range", &color, samples_per_pixel)
    }

    let scale = 1.0 / f64::from(samples_per_pixel);
    let r = color.r() * scale;
    let g = color.g() * scale;
    let b = color.b() * scale;

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    {
        format!(
            "{} {} {}\n",
            (256.0 * r.sqrt().clamp(0.0, 0.999)) as u8,
            (256.0 * g.sqrt().clamp(0.0, 0.999)) as u8,
            (256.0 * b.sqrt().clamp(0.0, 0.999)) as u8,
        )
    }
}

#[allow(dead_code)]
pub fn p6_image(width: u32, height: u32, colors: &[Vec3], samples_per_pixel: u32) -> Vec<u8> {
    if colors.is_empty() || colors.len() != width as usize * height as usize {
        // TODO: error
        let mut image = vec![];
        image.extend_from_slice(b"P6 0 0 255\n");
        return image;
    }
    let mut image = Vec::from(format!("P6 {} {} 255\n", width, height).as_bytes());

    for color in colors {
        if !color.is_valid_color(samples_per_pixel) {
            panic!("Color {} {} out of range", &color, samples_per_pixel)
        }

        let scale = 1.0 / f64::from(samples_per_pixel);
        let r = color.r() * scale;
        let g = color.g() * scale;
        let b = color.b() * scale;

        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        {
            image.push((256.0 * r.sqrt().clamp(0.0, 0.999)) as u8);
            image.push((256.0 * g.sqrt().clamp(0.0, 0.999)) as u8);
            image.push((256.0 * b.sqrt().clamp(0.0, 0.999)) as u8);
        }
    }

    image
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn p3_header() {
        assert_eq!(super::p3_header(100, 200), "P3\n100 200\n255\n");
    }

    #[test]
    fn p3_pixel() {
        assert_eq!(
            super::p3_pixel(&Vec3::from_xyz(0.0, 0.77, 1.0), 1),
            "0 224 255\n"
        );
    }

    #[test]
    fn p6_image() {
        let colors = vec![
            Vec3::from_xyz(1.0, 0.0, 0.0),
            Vec3::from_xyz(0.0, 1.0, 0.0),
            Vec3::from_xyz(0.0, 0.0, 1.0),
            Vec3::from_xyz(0.5, 0.0, 0.0),
            Vec3::from_xyz(0.0, 0.5, 0.0),
            Vec3::from_xyz(0.0, 0.0, 0.5),
        ];
        assert_eq!(
            super::p6_image(3, 2, &colors, 1),
            [
                80, 54, 32, 51, 32, 50, 32, 50, 53, 53, 10, 255, 0, 0, 0, 255, 0, 0, 0, 255, 181,
                0, 0, 0, 181, 0, 0, 0, 181
            ]
        );
    }
}

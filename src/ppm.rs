use super::vec3::Vec3;

#[allow(dead_code)]
pub fn p3_header(width: u16, height: u16) -> String {
    format!("P3\n{} {}\n255\n", width, height)
}

#[allow(dead_code)]
pub fn p3_pixel(color: Vec3) -> String {
    if !color.is_valid_color() {
        panic!("Color {} out of range", &color)
    }
    format!(
        "{} {} {}\n",
        (255.999 * color.r()) as u8,
        (255.999 * color.g()) as u8,
        (255.999 * color.b()) as u8
    )
}

#[allow(dead_code)]
pub fn p6_image(colors: &[Vec<Vec3>]) -> Vec<u8> {
    if colors.is_empty() || colors[0].is_empty() {
        let mut image = vec![];
        image.extend_from_slice(b"P6 0 0 255\n");
        return image;
    }
    let mut image = Vec::from(format!("P6 {} {} 255\n", colors[0].len(), colors.len()).as_bytes());

    for row in colors {
        for color in row {
            if !color.is_valid_color() {
                panic!("Color {} out of range", &color)
            }
            image.push((255.999 * color.r()) as u8);
            image.push((255.999 * color.g()) as u8);
            image.push((255.999 * color.b()) as u8);
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
        assert_eq!(super::p3_pixel(Vec3::init(0.0, 0.77, 1.0)), "0 197 255\n");
    }

    #[test]
    fn p6_image() {
        let colors = vec![
            vec![
                Vec3::init(1.0, 0.0, 0.0),
                Vec3::init(0.0, 1.0, 0.0),
                Vec3::init(0.0, 0.0, 1.0),
            ],
            vec![
                Vec3::init(0.5, 0.0, 0.0),
                Vec3::init(0.0, 0.5, 0.0),
                Vec3::init(0.0, 0.0, 0.5),
            ],
        ];
        assert_eq!(
            super::p6_image(&colors),
            [
                80, 54, 32, 51, 32, 50, 32, 50, 53, 53, 10, 255, 0, 0, 0, 255, 0, 0, 0, 255, 127,
                0, 0, 0, 127, 0, 0, 0, 127
            ]
        );
    }
}

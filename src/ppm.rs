use super::vec3::Vec3;

pub fn p3_header(width: u16, height: u16) -> String {
    format!("P3\n{} {}\n255\n", width, height)
}

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

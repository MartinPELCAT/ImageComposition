use image::{GenericImageView, Rgba};

pub fn avg_image_color(path: &str) -> (u8, u8, u8) {
    let image = image::open(path).unwrap();

    let (width, height) = image.dimensions();

    let pixels_count: i32 = (width * height).try_into().unwrap();

    let mut r_sum: i32 = 0;
    let mut g_sum: i32 = 0;
    let mut b_sum: i32 = 0;

    for pixel in image.pixels() {
        let Rgba(rgba) = pixel.2;

        let r: i32 = rgba.get(0).unwrap().to_owned().try_into().unwrap();
        let g: i32 = rgba.get(1).unwrap().to_owned().try_into().unwrap();
        let b: i32 = rgba.get(2).unwrap().to_owned().try_into().unwrap();

        r_sum += r;
        g_sum += g;
        b_sum += b;
    }

    let r_val: u8 = (r_sum / pixels_count).try_into().unwrap();
    let g_val: u8 = (g_sum / pixels_count).try_into().unwrap();
    let b_val: u8 = (b_sum / pixels_count).try_into().unwrap();

    return (r_val, g_val, b_val);
}

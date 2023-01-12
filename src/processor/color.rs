use image::Rgba;

pub type Color = (u8, u8, u8);

pub fn get_color_diff_basic(color1: Color, color2: Color) -> f64 {
    let c1r: i32 = color1.0.into();
    let c2r: i32 = color2.0.into();
    let c1g: i32 = color1.1.into();
    let c2g: i32 = color2.1.into();
    let c1b: i32 = color1.2.into();
    let c2b: i32 = color2.2.into();

    let r: f64 = (c1r - c2r).pow(2).into();
    let g: f64 = (c1g - c2g).pow(2).into();
    let b: f64 = (c1b - c2b).pow(2).into();

    let value = 0.3 * r + 0.59 * g + 0.11 * b;
    value
}

pub fn get_closest_image(values: Vec<(String, (u8, u8, u8))>, rbga: Rgba<u8>) -> String {
    let rgb = get_rbg(rbga);

    let first_value = values.get(0).unwrap();

    let mut closest = first_value.clone();
    let mut closest_score = get_color_diff_basic(rgb, closest.1);

    for value in values {
        let score = get_color_diff_basic(rgb, value.1);

        if score < closest_score {
            closest = value;
            closest_score = score;
        }
    }

    return closest.0;
}

fn get_rbg(rgba: Rgba<u8>) -> Color {
    let r = rgba.0.get(0).unwrap();
    let g = rgba.0.get(1).unwrap();
    let b = rgba.0.get(2).unwrap();

    (*r, *g, *b)
}

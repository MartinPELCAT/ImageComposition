use std::fs::{create_dir_all, read_dir};

use image::{GenericImageView, RgbaImage};

use crate::processor::color::get_closest_image;

mod processor;

const IMG_WIDTH: u32 = 200;

#[tokio::main]
async fn main() {
    create_dir_all("images").unwrap();
    // processor::request_images::fetch_images(1000).await;

    let base_image = image::open("image.jpeg").unwrap();

    let (width, height) = base_image.dimensions();

    let image_w = IMG_WIDTH * width;
    let image_h = IMG_WIDTH * height;

    let mut new_image = RgbaImage::new(image_w, image_h);

    let dir = read_dir("images").unwrap();

    let mut handles = Vec::new();

    for img in dir {
        let img = img.unwrap();

        let path = img.path().clone();
        let path = &path.to_str().unwrap();

        let path = format!("{}", path);

        let handle = tokio::spawn(async move {
            let avg_color = processor::avg_image_color::avg_image_color(path.as_str());
            (path, avg_color)
        });

        handles.push(handle);
    }

    let paths_with_avg_colors = futures::future::join_all(handles).await;

    let paths_with_avg_colors = paths_with_avg_colors
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let mut pixels_handles = Vec::new();

    for (b_x, b_y, b_rgb) in base_image.pixels() {
        let paths_with_avg_colors = paths_with_avg_colors.clone();

        let pixel_handle = tokio::spawn(async move {
            let closest_image = get_closest_image(paths_with_avg_colors, b_rgb);

            let found_image = image::open(closest_image).unwrap();

            let mut pixels = Vec::new();
            for (f_x, f_y, f_rgb) in found_image.pixels() {
                let pixel_place = (f_x + IMG_WIDTH * b_x, f_y + IMG_WIDTH * b_y, f_rgb);
                pixels.push(pixel_place)
            }
            println!("getting pixel x:{} y:{}", b_x, b_y);

            pixels
        });

        pixels_handles.push(pixel_handle);
    }

    let values = futures::future::join_all(pixels_handles).await;

    for (i, value) in values.iter().enumerate() {
        let pixels = value.as_ref().unwrap();

        for pixel in pixels {
            let pix = new_image.get_pixel_mut(pixel.0, pixel.1);
            *pix = pixel.2;
        }

        println!("writing pixel i:{}", i + 1);
    }

    new_image.save("composition.jpeg").unwrap();

    println!("finished");
}
// https://picsum.photos/500/500

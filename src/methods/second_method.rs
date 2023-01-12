use std::{cmp, collections::HashMap, fs::create_dir_all};

use image::{GenericImageView, RgbaImage};
use uuid::Uuid;

use crate::processor::{
    color::get_closest_image, get_all_images_colors::get_images_avg_colors_with_pixels,
};

const IMG_WIDTH: u32 = 50;
const MAX_SIZE: u32 = 65535 / IMG_WIDTH;

pub async fn method_two() {
    create_dir_all("images").unwrap();
    // processor::request_images::fetch_images(1000).await;

    let base_image = image::open("image.jpeg").unwrap();

    let (mut width, mut height) = base_image.dimensions();
    println!("{}, {}", width, height);

    let max_side = cmp::max(width, height);

    if max_side > MAX_SIZE {
        let aspect_ratio = width as f32 / height as f32;
        if max_side == width {
            width = MAX_SIZE;
            height = (height as f32 * aspect_ratio).round() as u32;
        } else {
            height = MAX_SIZE;
            width = (width as f32 * aspect_ratio).round() as u32;
        }
    }

    let base_image = base_image.resize(width, height, image::imageops::FilterType::Nearest);

    let (width, height) = base_image.dimensions();

    println!("{}, {}", width, height);

    let image_w: u32 = IMG_WIDTH * width;
    let image_h: u32 = IMG_WIDTH * height;

    let mut new_image = RgbaImage::new(image_w, image_h);

    let (paths_with_avg_colors_as_vec, paths_with_pixels) =
        get_images_avg_colors_with_pixels().await;

    println!("Got all the paths");

    let mut pixels_lines_handles = Vec::new();

    let path_with_colors = paths_with_avg_colors_as_vec.clone();

    for y in 0..height {
        let base_image = base_image.clone();
        let path_with_colors = path_with_colors.clone();

        let pixel_handle = tokio::spawn(async move {
            let mut base_image_pixels_with_images = HashMap::new();
            for x in 0..width {
                let rbga = base_image.get_pixel(x, y);
                let closest_image = get_closest_image(path_with_colors.clone(), rbga);
                base_image_pixels_with_images.insert((x, y), closest_image);
            }

            base_image_pixels_with_images
        });
        println!("getting pixel line y:{}", y);

        pixels_lines_handles.push(pixel_handle);
    }

    let values = futures::future::join_all(pixels_lines_handles).await;

    for (base_img_line_num, value) in values.iter().enumerate() {
        let line_pixels = value.as_ref().unwrap();

        for line_pixel in line_pixels {
            let file_path = line_pixel.1;
            let (base_pix_x, base_pix_y) = line_pixel.0.clone();

            let found_image_pixels = paths_with_pixels.get(file_path).unwrap();

            for found_image_pixel in found_image_pixels {
                let x = found_image_pixel.0;
                let y = found_image_pixel.1;
                let color = found_image_pixel.2;

                let pos_x = base_pix_x * IMG_WIDTH + x;
                let pos_y = base_pix_y * IMG_WIDTH + y;

                let new_img_pixel = new_image.get_pixel_mut(pos_x, pos_y);
                *new_img_pixel = color;
            }
        }
        println!("writing pixel line {}", base_img_line_num + 1);
    }

    let uuid = Uuid::new_v4().to_string();

    let path = format!("{}.jpeg", uuid);

    new_image.save(path).unwrap();
}

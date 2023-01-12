use std::{collections::HashMap, fs::read_dir};

use image::{GenericImageView, Rgba};

pub async fn get_images_avg_colors() -> Vec<(String, (u8, u8, u8))> {
    let dir = read_dir("images").unwrap();

    let mut handles = Vec::new();

    for img in dir {
        let img = img.unwrap();

        let path = img.path().clone();
        let path = &path.to_str().unwrap();

        let path = format!("{}", path);

        let handle = tokio::spawn(async move {
            let avg_color = super::avg_image_color::avg_image_color(path.as_str());
            (path, avg_color)
        });

        handles.push(handle);
    }

    let paths_with_avg_colors = futures::future::join_all(handles).await;

    let paths_with_avg_colors = paths_with_avg_colors
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    paths_with_avg_colors
}

type Test = (
    Vec<(String, (u8, u8, u8))>,
    HashMap<String, Vec<(u32, u32, Rgba<u8>)>>,
);

pub async fn get_images_avg_colors_with_pixels() -> Test {
    let dir = read_dir("images").unwrap();

    let mut handles = Vec::new();

    for img in dir {
        let img = img.unwrap();

        let path = img.path().clone();
        let path = &path.to_str().unwrap();

        let path = format!("{}", path);

        let handle = tokio::spawn(async move {
            let image = image::open(&path).unwrap();
            let avg_color = super::avg_image_color::avg_image_color(path.as_str());
            let pixels = image.pixels().collect::<Vec<_>>();

            (path, (avg_color, pixels))
        });

        handles.push(handle);
    }

    let paths_with_avg_colors = futures::future::join_all(handles).await;

    let mut path_with_color_vec = Vec::new();

    let mut path_with_pixels = HashMap::new();

    paths_with_avg_colors.iter().for_each(|val| {
        let val = &*val;
        let val = val.as_ref().unwrap().clone();

        let path = val.0;
        let val = val.1.clone();

        let avg_color = val.0;
        path_with_color_vec.push((path.clone(), avg_color));
        path_with_pixels.insert(path, val.1);
    });

    (path_with_color_vec, path_with_pixels)
}

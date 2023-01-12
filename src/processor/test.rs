fn process_test() {
    let image = image::open("image.jpeg").unwrap();

    let (width, height) = image.dimensions();

    let mut new_image = RgbaImage::new(width, height);

    for (x, y, rgb) in image.pixels() {
        let pixel = new_image.get_pixel_mut(x, y);
        *pixel = rgb;
    }
    new_image.save("clone.jpeg").unwrap();
}

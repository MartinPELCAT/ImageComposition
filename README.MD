# Setup

- Create a folder next to `src` called `images`

Uncomment the fetch_image line to get images and run the program multiples times to load images.
There is about 1084 images in the api, runnin the program more than 10 times should be enough.

```rust
// src/main.rs
#[tokio::main]
async fn main() {
    let time = Instant::now();
    processor::request_images::fetch_images(100).await;
    // methods::second_method::method_two().await;
    let elapsed = time.elapsed();
    println!("Time elapsed is: {:?}", elapsed);

    println!("finished");
}
```

Then add a `jpeg` image in the root directory named `image.jpeg`.
Comment the fetch_image method and run the method_two to create the files.

```rust
// src/main.rs
#[tokio::main]
async fn main() {
    let time = Instant::now();
    //processor::request_images::fetch_images(100).await;
    methods::second_method::method_two().await;
    let elapsed = time.elapsed();
    println!("Time elapsed is: {:?}", elapsed);

    println!("finished");
}
```

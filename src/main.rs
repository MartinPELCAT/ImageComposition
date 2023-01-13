use std::time::Instant;

mod methods;
mod processor;

#[tokio::main]
async fn main() {
    let time = Instant::now();
    // processor::request_images::fetch_images(100).await;
    methods::second_method::method_two().await;
    let elapsed = time.elapsed();
    println!("Time elapsed is: {:?}", elapsed);

    println!("finished");
}

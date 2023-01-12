use std::{
    fs::File,
    io::{copy, Cursor},
};

use bytes::Bytes;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, USER_AGENT},
    Client,
};

pub async fn fetch_images(number: i32) {
    let mut handlers = Vec::new();

    for _ in 0..number {
        let handle = tokio::spawn(async {
            let client = Client::new();
            let image = client
                .get("https://picsum.photos/200")
                .header(USER_AGENT, "PostmanRuntime/7.30.0")
                .send()
                .await
                .unwrap();

            let headers = &image.headers().clone();

            let value = image.bytes().await.unwrap();

            self::download_image(headers, value)
        });

        handlers.push(handle);
    }

    futures::future::join_all(handlers).await;
}

fn download_image(headers: &HeaderMap, bytes: Bytes) {
    let content_dispo = headers.get("content-disposition").unwrap();

    let filename_regex = Regex::new(r#""(.*?)*""#).unwrap();

    let content_dispo = content_dispo.to_str().unwrap();
    let filename = filename_regex
        .captures(content_dispo)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let path = format!("images/{filename}");

    let mut file = File::create(path.as_str()).unwrap();

    let mut content = Cursor::new(bytes);

    copy(&mut content, &mut file).unwrap();
}

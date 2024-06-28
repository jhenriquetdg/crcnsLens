use std::sync::{Arc, Mutex};

use futures_util::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::types::state::SrPair;

pub async fn write_response(file: &mut File, response: reqwest::Response) {
    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")
        .unwrap();

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.unwrap();
        file.write_all(&chunk).await.unwrap();
        downloaded += chunk.len() as u64;

        // Log the download progress
        println!(
            "Downloaded {} of {} bytes ({:.2}%)",
            downloaded,
            total_size,
            (downloaded as f64 / total_size as f64) * 100.0
        );
    }
}

pub async fn write_response_with_sender(
    file: &mut File,
    response: reqwest::Response,
    sr: Arc<Mutex<SrPair>>,
) {
    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")
        .unwrap();

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.unwrap();
        file.write_all(&chunk).await.unwrap();
        downloaded += chunk.len() as u64;

        // Log the download progress
        let _ = sr
            .lock()
            .unwrap()
            .0
            .send((downloaded as f32 / total_size as f32) * 100.0);
    }
}

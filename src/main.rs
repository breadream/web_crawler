use reqwest;
use reqwest::get as async_get;
use std::thread;
use std::time::Instant;
use tokio::runtime::Runtime;

const URLS: [&str; 3] = [
    "https://www.rust-lang.org",
    "https://www.google.com",
    "https://www.github.com",
];

fn main() {
    download_with_threads();
    download_with_async();
}

fn download_with_threads() {
    println!("Downloading web pages using multi-threading...");

    let start_time = Instant::now();

    let handles: Vec<_> = URLS.iter().map(|&url| {
        thread::spawn(move || {
            let response = reqwest::blocking::get(url).unwrap();
            println!("Downloaded {} ({} bytes)", url, response.content_length().unwrap_or(0));
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    println!("Multi-threaded download completed in {:.2} seconds", elapsed);
}

async fn download_async(url: &str) {
    let response = async_get(url).await.unwrap();
    println!("Downloaded {} ({} bytes)", url, response.content_length().unwrap_or(0));
}

fn download_with_async() {
    println!("Downloading web pages using asynchronous tasks...");

    let start_time = Instant::now();

    let rt = Runtime::new().unwrap();

    for &url in &URLS {
        rt.spawn(download_async(url));
    }

    rt.block_on(async {
        println!("Async download completed");
    });

    let elapsed = start_time.elapsed().as_secs_f64();
    println!("Asynchronous download completed in {:.2} seconds", elapsed);
}

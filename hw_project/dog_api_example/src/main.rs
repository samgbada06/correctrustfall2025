use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::Read; 

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String,
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";
    
    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        },
        Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
    }
}

fn download_image(url: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let response = ureq::get(url).call()?;
    let bytes = response.into_reader().bytes().collect::<Result<Vec<u8>, _>>()?;
    
    let mut file = File::create(file_name)?;
    file.write_all(&bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Downloader");
    println!("========================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success! Downloading...");

                let file_name = format!("dog_image_{}.jpg", i);
                
                match download_image(&dog_image.message, &file_name) {
                    Ok(_) => println!("Saved as '{}'\n", file_name),
                    Err(e) => println!("Failed to download: {}\n", e),
                }
            },
            ApiResult::ApiError(e) => println!("API Error: {}\n", e),
            ApiResult::NetworkError(e) => println!("Network Error: {}\n", e),
        }
    }

    Ok(())
}

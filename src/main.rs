use std::{error::Error, fs::File, time::Instant};

fn main() {
    let images = vec![
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
        "https://picsum.photos/200/300?grayscale",
    ];

    let now = Instant::now();

    for (index, url) in images.iter().enumerate() {
        println!("Downloading image {}", index + 1);

        match download_image(url, index) {
            Ok(_) => println!("Successfully saved image as image_{}.jpg", index + 1),
            Err(e) => eprintln!("Failed to download image {}: {}", index + 1, e),
        }
    }

    let duration = now.elapsed();
    println!("Took {:?}", duration);
}

fn download_image(url: &str, index: usize) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;

    let file_name = format!("asset/image_{}.jpg", index + 1);
    let mut file = File::create(file_name)?;

    response.copy_to(&mut file)?;

    Ok(())
}

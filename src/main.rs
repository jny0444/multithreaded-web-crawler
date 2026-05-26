use std::{
    error::Error,
    fs::File,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use crossbeam_channel::unbounded;

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
    let images: Vec<String> = images.into_iter().map(|a| a.to_string()).collect();

    let now = Instant::now();

    let (tx, rx) = unbounded::<(usize, String)>();
    for (index, url) in images.into_iter().enumerate() {
        tx.send((index, url)).unwrap();
    }

    drop(tx);

    let total_bytes_downloaded = Arc::new(Mutex::new(0));

    let mut handles = Vec::new();
    let num_workers = 4;

    for worker_id in 0..num_workers {
        let rx_clone = rx.clone();

        let counter_clone = Arc::clone(&total_bytes_downloaded);

        let handle = thread::spawn(move || {
            while let Ok((index, url)) = rx_clone.recv() {
                println!("Worker {} downloading image {}", worker_id, index + 1);

                match download_image(&url, index) {
                    Ok(bytes) => {
                        println!(
                            "Worker {} saved image_{}.jpg ({:?} bytes)",
                            worker_id,
                            index + 1,
                            bytes
                        );

                        let mut guard = counter_clone.lock().unwrap();
                        *guard += bytes;
                    }
                    Err(e) => eprintln!("Worker {} failed image {}: {}", worker_id, index + 1, e),
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_bytes = *total_bytes_downloaded.lock().unwrap();

    let duration = now.elapsed();
    println!("Took {:?}", duration);
    println!(
        "Total MB downloaded: {:.2} MB",
        final_bytes as f64 / 1_000_000.0
    );
}

fn download_image(url: &String, index: usize) -> Result<u64, Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;

    let file_name = format!("asset/image_{}.jpg", index + 1);
    let mut file = File::create(file_name)?;

    let bytes_written = response.copy_to(&mut file)?;

    Ok(bytes_written)
}

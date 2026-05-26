use std::{error::Error, fs::File, thread, time::Instant};

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

    let (tx, rx) = crossbeam_channel::unbounded::<(usize, String)>();

    for (index, url) in images.into_iter().enumerate() {
        tx.send((index, url)).unwrap();
    }

    drop(tx);

    let mut handles = Vec::new();
    let num_workers = 4;

    for worker_id in 0..num_workers {
        let rx_clone = rx.clone();

        let handle = thread::spawn(move || {
            println!("Worker {} started, waiting for jobs...", worker_id);

            while let Ok((index, url)) = rx_clone.recv() {
                println!("Worker {} picked up image {}", worker_id, index + 1);

                match download_image(&url, index) {
                    Ok(_) => println!(
                        "Worker {} successfully saved image_{}.jpg",
                        worker_id,
                        index + 1
                    ),
                    Err(e) => eprintln!("Worker {} failed image {}: {}", worker_id, index + 1, e),
                }
            }

            println!("Worker {} has no more work. Shutting down.", worker_id);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = now.elapsed();
    println!("Took {:?}", duration);
}

fn download_image(url: &String, index: usize) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;

    let file_name = format!("asset/image_{}.jpg", index + 1);
    let mut file = File::create(file_name)?;

    response.copy_to(&mut file)?;

    Ok(())
}

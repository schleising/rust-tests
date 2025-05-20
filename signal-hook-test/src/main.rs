use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    println!("Starting signal-hook test...");

    // Create a new Signals instance
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))
        .expect("Failed to register SIGTERM");
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))
        .expect("Failed to register SIGINT");

    let handle = std::thread::spawn(move || {
        println!("Thread started");
        // Simulate some work
        while !term.load(Ordering::Relaxed) {
            // Sleep to simulate work
            std::thread::sleep(std::time::Duration::from_secs(2));

            println!("Finished sleeping");
        }
        println!("Thread finished");
    });

    println!("Thread spawned, waiting for it to finish...");

    // Wait for the thread to finish
    match handle.join() {
        Ok(_) => println!("Thread joined successfully"),
        Err(e) => println!("Thread panicked: {:?}", e),
    }
    println!("Main thread finished");
}

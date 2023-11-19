use std::any::Any;

// Create a processor intensive task that takes input seconds to run
fn sleep(seconds: u64) -> u64 {
    let start = std::time::Instant::now();
    loop {
        if start.elapsed().as_secs() >= seconds {
            break;
        }
    }
    seconds
}

fn main() {
    // Create a vector of input ints to be processed
    let input_vec: Vec<u64> = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    // Get the start time
    let start = std::time::Instant::now();

    // Create threads to calculate the factorial of each input int
    let mut thread_vec = Vec::new();
    for input in input_vec {
        let thread = std::thread::spawn(move || {
            println!("Thread started, sleeping for {} seconds", input);
            let result = sleep(input);
            // Print how long the thread took to run
            println!(
                "Thread finished, slept for {} seconds, took {:?} to run",
                result,
                start.elapsed()
            );
        });
        thread_vec.push(thread);
    }

    // Wait for all threads to finish
    for thread in thread_vec {
        let thread_result: Result<(), Box<dyn Any + Send>> = thread.join();

        // Handle thread join error
        match thread_result {
            Ok(_) => (),
            Err(e) => println!("Error joining thread: {:?}", e),
        }
    }
}

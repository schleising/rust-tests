use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print the available parallelism
    let available_parallelism = thread::available_parallelism()?;
    println!("Available parallelism: {:?}", available_parallelism);

    let mut vector = vec![1, 2, 3, 4, 5];
    println!("Main thread {:?}", vector);

    // Get the current time
    let start_time = time::Instant::now();

    thread::scope(|scope| {
        scope.spawn(|| {
            vector.push(6);
            println!("Thread {:?}", vector);
            thread::sleep(time::Duration::from_secs(2));
        });

        scope.spawn(|| {
            thread::sleep(time::Duration::from_secs(4));
        });
    });

    // Calculate the elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {:.0?}", elapsed_time);

    // Vector is still accessible here
    vector.push(7);
    println!("Main thread after scope {:?}", vector);

    Ok(())
}

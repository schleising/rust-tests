use std::{fmt::Pointer, thread, time};

#[derive(Debug)]
enum MyVec<T> {
    Vector(Vec<T>)
}

impl<T> MyVec<T> {
    fn push(&mut self, value: T) {
        match self {
            MyVec::Vector(vec) => vec.push(value),
        }
    }
}

impl<T> Pointer for MyVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:p}")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Print the available parallelism
    let available_parallelism = thread::available_parallelism()?;
    println!("Available parallelism: {available_parallelism:?}");

    let vector = vec![1, 2, 3, 4, 5];
    let mut my_vector = MyVec::Vector(vector);
    println!("Main thread {my_vector:?}");

    // Get the current time
    let start_time = time::Instant::now();

    thread::scope(|scope| {
        scope.spawn(|| {
            my_vector.push(6);
            println!("Thread {my_vector:?}");
            thread::sleep(time::Duration::from_secs(2));
        });

        scope.spawn(|| {
            thread::sleep(time::Duration::from_secs(4));
        });
    });

    // Calculate the elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Elapsed time: {elapsed_time:.0?}");

    // Vector is still accessible here
    my_vector.push(7);
    println!("Main thread after scope {my_vector:?}");

    Ok(())
}

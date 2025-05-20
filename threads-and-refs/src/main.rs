struct TestRef {
    value: &'static str,
}

fn main() {
    let r  = TestRef { value: "Hello, world!" };
    
    // Spawn a thread to print the value
    let handle = std::thread::spawn(move || {
        println!("{}", r.value);
    });
    // Wait for the thread to finish
    handle.join().unwrap();
    // The main thread can still use `s` here
    // println!("Main thread: {}", s);
}

use std::io::{self, Write};

macro_rules! foo {
    ($($name:expr),+) => {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        $(
            let output_string = "Hello, ".to_string() + &$name.to_string() + "! ";
            handle.write_all(output_string.as_bytes()).unwrap();
        )+
        handle.write_all(b"\n").unwrap();
        handle.flush().unwrap();
    };
}

fn main() {
    foo!(1, "Two", 3);
    foo!("Test");
    foo!("World");
}

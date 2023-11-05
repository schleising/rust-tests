use std::fs::File;

mod shapes;

mod mod_a {
    pub mod mod_b {
        pub mod mod_c {
            pub fn foo() {
                println!("Hello from mod_a::mod_b::mod_c::foo()");
            }
        }
    }
}

fn main() {
    let radius = 4.0;
    println!("The circumference of a circle with radius {} is {}",
            radius, shapes::circle::circumference(radius));
    println!("The area of a circle with radius {} is {}",
            radius, shapes::circle::area(radius));
    mod_a::mod_b::mod_c::foo();

    let result: Result<File, std::io::Error> = File::open("src/main.rs");

    match result {
        Ok(file) => {
            println!("File opened successfully: {:?}", file);

            // Read the file line by line and print the lines
            let reader = std::io::BufReader::new(file);

            for line in std::io::BufRead::lines(reader) {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => println!("Failed to read line: {:?}", error),
                    
                }
            }
        },
        Err(error) => {
            println!("Failed to open file: {:?}", error);
            // Exit the program with a non-zero exit code
            std::process::exit(1);
        }
    }
}

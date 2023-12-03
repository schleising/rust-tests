use std::fmt::Display;

trait Exclaim where Self: Display {
    fn exclaim (&self) -> String;
}

impl<T: Display> Exclaim for T {
    fn exclaim (&self) -> String {
        format!("{}!", self)
    }
}

fn main() {
    println!("{}", "&str".exclaim());
    // Create another type that implements Display
    let s: String = String::from("String");
    println!("{}", s.exclaim());

    // Create another type that implements Display
    let s: i32 = 5;
    println!("{}", s.exclaim());

    // Create another type that implements Display
    let s: f64 = 5.5;
    println!("{}", s.exclaim());

    // Create a type that does not implement Display
    // let s: Vec<i32> = vec![1, 2, 3];
    // println!("{}", s.exclaim()); // This will not compile
}

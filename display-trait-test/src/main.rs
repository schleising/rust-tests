use std::fmt::Display;

trait Exclaim: Display {
    fn exclaim (&self) -> String;
}

impl<T: Display> Exclaim for T {
    fn exclaim (&self) -> String {
        format!("{}!", self)
    }
}

#[derive(Debug)]
struct MyStruct {
    name: String,
}

impl Display for MyStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
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

    // Create a my struct
    let s: MyStruct = MyStruct {
        name: String::from("MyStruct"),
    };

    println!("{}", s.exclaim());
}

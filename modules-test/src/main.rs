mod library;

use library::exclaim::Exclaim;

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", format!("{}", library::maths::add(1, 2)).exclaim());
}

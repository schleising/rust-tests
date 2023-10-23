mod shapes;

use shapes::circle;

fn main() {
    let radius = 4.0;
    println!("The circumference of a circle with radius {} is {}",
             radius, circle::circumference(radius));
    println!("The area of a circle with radius {} is {}",
             radius, circle::area(radius));
}

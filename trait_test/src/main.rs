// Use the Shapes from lib.rs
extern crate shape_lib;
use shape_lib::shapes::{Shape, Rectangle, Triangle};

fn main() {
    // Create a rectangle
    let rectangle: Rectangle = Shape::new(2, 3);

    // Create a triangle
    let triangle: Triangle = Shape::new(2, 3);

    // Clone the rectangle
    let mut rectangle2 = rectangle.clone();
    
    rectangle2.set_width(4);
    
    println!("Shape 1: {}, Area: {}", rectangle, rectangle.area());
    println!("Shape 2: {}, Area: {}", rectangle2, rectangle2.area());
    println!("Shape 3: {}, Area: {}", triangle, triangle.area());
}

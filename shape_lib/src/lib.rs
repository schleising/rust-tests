// Module: shapes
pub mod shapes {
    pub trait Shape {
        fn new(width: i32, height: i32) -> Self;
        fn area(&self) -> i32;

        fn set_width(&mut self, width: i32);
        fn set_height(&mut self, height: i32);
    }

    // Rectangle definition
    #[derive(Clone)]
    pub struct Rectangle {
        name: String,
        width: i32,
        height: i32,
    }

    // Implement the Display trait for Rectangle
    impl std::fmt::Display for Rectangle {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{} {{ width: {}, height: {} }}", self.name, self.width, self.height)
        }
    }

    // Implement the Shape trait for Rectangle
    impl Shape for Rectangle {
        fn new(width: i32, height: i32) -> Rectangle {
            Rectangle { name: "Rectangle".to_string(), width, height }
        }

        fn set_height(&mut self, height: i32) {
            self.height = height;
        }

        fn set_width(&mut self, width: i32) {
            self.width = width;
        }

        fn area(&self) -> i32 {
            self.width * self.height
        }
    }

    // Triangle definition
    #[derive(Clone)]
    pub struct Triangle {
        name: String,
        width: i32,
        height: i32,
    }

    // Implement the Display trait for Triangle
    impl std::fmt::Display for Triangle {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{} {{ width: {}, height: {} }}", self.name, self.width, self.height)
        }
    }

    // Implement the Shape trait for Triangle
    impl Shape for Triangle {
        fn new(width: i32, height: i32) -> Triangle {
            Triangle { name: "Triangle".to_string(), width, height }
        }

        fn set_width(&mut self, width: i32) {
            self.width = width;
        }

        fn set_height(&mut self, height: i32) {
            self.height = height;
        }

        fn area(&self) -> i32 {
            (self.width * self.height) / 2
        }
    }
}

// Write tests for the shapes module
#[cfg(test)]
mod tests {
    use super::shapes::{Shape, Rectangle, Triangle};

    // Test the Rectangle area
    #[test]
    fn test_rectangle_area() {
        let rectangle: Rectangle = Shape::new(2, 3);
        assert_eq!(rectangle.area(), 6);
    }

    // Test the Rectangle Display trait
    #[test]
    fn test_rectangle_display() {
        let rectangle: Rectangle = Shape::new(2, 3);
        assert_eq!(format!("{rectangle}"), "Rectangle { width: 2, height: 3 }");
    }

    // Test the Triangle area
    #[test]
    fn test_triangle_area() {
        let triangle: Triangle = Shape::new(2, 3);
        assert_eq!(triangle.area(), 3);
    }

    // Test the Triangle Display trait
    #[test]
    fn test_triangle_display() {
        let triangle: Triangle = Shape::new(2, 3);
        assert_eq!(format!("{triangle}"), "Triangle { width: 2, height: 3 }");
    }
}

pub trait Exclaim {
    fn exclaim(&self) -> String;
}

// Implement Exclaim for any type that implements Display
impl<T: std::fmt::Display> Exclaim for T {
    fn exclaim(&self) -> String {
        format!("{}!", self)
    }
}

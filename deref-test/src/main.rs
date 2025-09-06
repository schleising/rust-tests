struct MyStruct {
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Self {
        MyStruct { value }
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let my_struct = MyStruct::new(10);
    let boxed_struct: Box<MyStruct> = Box::new(my_struct);

    // Can call methods on the Box directly because of Deref coercion
    // Deref coercion allows us to call methods on the Box as if it were a reference to MyStruct
    // This is possible because Box implements Deref, which allows it to be treated as a reference
    // to the value it contains.
    // The compiler automatically dereferences the Box to call the method on MyStruct
    // without needing to explicitly dereference it.
    // This is a demonstration of how Rust's deref coercion works.
    // The following line is equivalent to:
    // let value = (*boxed_struct).get_value();
    let value = boxed_struct.get_value();
    println!("Value: {value}");
}

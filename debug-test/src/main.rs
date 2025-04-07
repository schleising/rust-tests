#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: i32,
}

impl MyStruct {
    fn new(a: i32, b: i32) -> MyStruct {
        MyStruct { a, b }
    }
}

#[derive(Debug)]
struct Container {
    version: i32,
    my_struct: MyStruct,
}

impl Container {
    fn new(version: i32, my_struct: MyStruct) -> Container {
        Container { version, my_struct }
    }
}

fn main() {
    let my_struct: MyStruct = MyStruct::new(1, 2);
    let container: Container = Container::new(3, my_struct);

    // Print the struct fields
    println!(
        "a: {}, b: {}, version: {}",
        container.my_struct.a, container.my_struct.b, container.version
    );

    // Pretty print the struct
    println!("{:#?}", container);
}

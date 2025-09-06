struct TestStruct {
    x: i32,
    y: f64,
}

enum TestEnum {
    Variant1(i64),
    Variant2(String),
    Variant3(TestStruct),
}

fn return_some_value() -> Option<i32> {
    Some(42)
}

fn return_no_value() -> Option<i32> {
    None
}

fn main() {
    let some_value: Option<i32> = return_some_value();
    let no_value: Option<i32> = return_no_value();

    // Match on the Option<i32> returned by return_some_value()
    match some_value {
        Some(x) => println!("Return Some Value Got a value: {x}"),
        None => println!("Return Some Value Got no value"),
    }

    // Match on the Option<i32> returned by return_no_value()
    match no_value {
        Some(x) => println!("Return No Value Got a value: {x}"),
        None => println!("Return No Value Got no value"),
    }

    // Populate the enum variants with values
    let variant1 = TestEnum::Variant1(42);
    let variant2 = TestEnum::Variant2(String::from("Hello"));
    let variant3 = TestEnum::Variant3(TestStruct { x: 3, y: 5.4 });

    // Match on the enum variants
    match variant1 {
        TestEnum::Variant1(x) => println!("Variant1: {x}"),
        TestEnum::Variant2(x) => println!("Variant2: {x}"),
        TestEnum::Variant3(x) => println!("Variant3: x: {}, y: {}", x.x, x.y),
    }

    match variant2 {
        TestEnum::Variant1(x) => println!("Variant1: {x}"),
        TestEnum::Variant2(x) => println!("Variant2: {x}"),
        TestEnum::Variant3(x) => println!("Variant3: x: {}, y: {}", x.x, x.y),
    }

    match variant3 {
        TestEnum::Variant1(x) => println!("Variant1: {x}"),
        TestEnum::Variant2(x) => println!("Variant2: {x}"),
        TestEnum::Variant3(x) => println!("Variant3: x: {}, y: {}", x.x, x.y),
    }
}

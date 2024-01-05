fn take_test() {
    let mut some_i32: Option<i32> = Some(5);
    let mut none_i32: Option<i32> = None;

    println!("Before take()");
    println!("some_i32: {:?}", some_i32);
    println!("none_i32: {:?}", none_i32);

    let the_i32 = some_i32.take();
    println!("After take() on some_i32");
    println!("some_i32: {:?}", some_i32);
    println!("the_i32: {:?}", the_i32);

    let the_i32 = none_i32.take();
    println!("After take() on none_i32");
    println!("none_i32: {:?}", none_i32);
    println!("the_i32: {:?}", the_i32);
}

fn main() {
    take_test();
}

fn take_test() {
    let mut some_i32: Option<i32> = Some(5);
    let mut none_i32: Option<i32> = None;

    println!("Before take()");
    dbg!(some_i32);
    dbg!(none_i32);

    let the_i32 = some_i32.take();
    println!("After take() on some_i32");
    dbg!(some_i32);
    dbg!(the_i32);

    let the_i32 = none_i32.take();
    println!("After take() on none_i32");
    dbg!(none_i32);
    dbg!(the_i32);
    println!();
}

fn replace_test() {
    let mut some_i32: Option<i32> = Some(5);
    let mut none_i32: Option<i32> = None;

    println!("Before replace()");
    dbg!(some_i32);
    dbg!(none_i32);

    let the_i32 = some_i32.replace(6);
    println!("After replace() on some_i32");
    dbg!(some_i32);
    dbg!(the_i32);

    let the_i32 = none_i32.replace(7);
    println!("After replace() on none_i32");
    dbg!(none_i32);
    dbg!(the_i32);
    println!();
}

fn main() {
    take_test();
    replace_test();
}

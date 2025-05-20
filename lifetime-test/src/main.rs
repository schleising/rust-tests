struct Foo<'a> {
    x: &'a i32,
}

fn return_largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let x = 15;
    println!("The value of x is     {}", x);
    let z: &i32;
    {
        let y = 6;
        println!("The value of y is     {}", y);
        z = return_largest(&x, &y);
        println!("The largest number is {}", z);
    }

    // println!("The value of z is     {}", z); // error[E0597]: `y` does not live long enough

    let mut foo_struct: Foo = Foo { x: &x };
    println!("The value of foo_struct.x is {}", foo_struct.x);

    {
        let y = 5;
        foo_struct.x = &y;

        println!("The value of foo_struct.x is {}", foo_struct.x);
    }

    // println!("The value of foo_struct.x is {}", foo_struct.x); // error[E0597]: `y` does not live long enough

}

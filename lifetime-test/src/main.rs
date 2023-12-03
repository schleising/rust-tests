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
    
}

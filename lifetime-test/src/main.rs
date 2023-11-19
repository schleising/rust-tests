fn return_largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let x = 5;
    let z: &i32;
    {
        let y = 6;
        z = return_largest(&x, &y);
        println!("{}", z);
    }

}

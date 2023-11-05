struct IntStruct {
    x: i32,
}

fn int_ref_inc(x: &mut i32) {
    // Prints the value of x
    println!("x in int_ref_inc = {}", x);
    // Increments the value of x
    *x += 1;
}

fn modift_int_struct(x: &mut IntStruct) {
    // Prints the value of x
    println!("x start modift_int_struct = {}", x.x);
    // Increments the value of x
    x.x += 1;
    // Prints the value of x
    println!("x end modift_int_struct = {}", x.x);
}

fn main() {
    let mut x = 1;
    println!("x before call = {}", x);
    int_ref_inc(&mut x);
    println!("x after call = {}", x);

    let mut x = IntStruct { x: 1 };
    println!("x before call = {}", x.x);
    modift_int_struct(&mut x);
    println!("x after call = {}", x.x);

}

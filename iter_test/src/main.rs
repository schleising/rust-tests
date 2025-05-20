// Filter the even numbers from a slice of numbers
fn filter_even_numbers(numbers: &[i32]) -> Vec<i32> {
    // Create an iterator over the slice
    let iter: std::slice::Iter<'_, i32> = numbers.iter();

    // Filter the iterator to only include even numbers
    let even_numbers: Vec<i32> = iter.filter(|&&x| x % 2 == 0).copied().collect();

    // Return the even numbers
    even_numbers
}

// Filter the odd numbers from a slice of numbers
fn filter_odd_numbers(numbers: &[i32]) -> Vec<i32> {
    // Create an iterator over the slice
    let iter = numbers.iter();

    // Filter the iterator to only include odd numbers
    let odd_numbers: Vec<i32> = iter.filter(|&x| x % 2 != 0).copied().collect();

    // Return the odd numbers
    odd_numbers
}

// Skip the first n numbers from a slice of numbers
fn skip_numbers(numbers: &[i32], n: usize) -> Vec<i32> {
    // Create an iterator over the slice
    let iter = numbers.iter();

    // Skip the first n numbers
    let skip_numbers: Vec<i32> = iter.skip(n).copied().collect();

    // Return the skipped numbers
    skip_numbers
}

fn inspect_numbers(numbers: &[i32]) -> Vec<i32> {
    // Create an iterator over the slice
    let iter = numbers.iter();

    // Inspect before and after a skip, filter, and map
    let inspect_numbers: Vec<i32> = iter
        .inspect(|&x| println!("before skip, filter, and map: {}", x))
        .skip(2)
        .inspect(|&x| println!("after skip: {}", x))
        .filter(|&x| x % 3 == 0)
        .inspect(|&x| println!("after filter: {}", x))
        .map(|&x| x * 2)
        .inspect(|&x| println!("after map: {}", x))
        .collect();

    // Return the inspected numbers
    inspect_numbers
}

fn main() {
    // Create a vector of numbers from 1 to 10
    let numbers: Vec<i32> = (1..11).collect();

    // Filter the even numbers
    let even_numbers = filter_even_numbers(&numbers);

    // Filter the odd numbers
    let odd_numbers = filter_odd_numbers(&numbers);

    // Skip the first 5 numbers
    let skip_numbers = skip_numbers(&numbers, 5);

    // Inspect the numbers
    let inspect_numbers = inspect_numbers(&numbers);

    // Print the original numbers
    println!("numbers = {:?}", numbers);

    // Print the even numbers
    println!("even_numbers = {:?}", even_numbers);

    // Print the odd numbers
    println!("odd_numbers = {:?}", odd_numbers);

    // Print the skipped numbers
    println!("skip_numbers = {:?}", skip_numbers);

    // Print the inspected numbers
    println!("inspect_numbers = {:?}", inspect_numbers);
}



fn main() {
    let results_vec: Vec<Result<&str, &str>> = vec![Ok("This is OK"), Err("WTF !!"), Ok("This is also OK")];

    // Handle the results turning resukts_vec into an iterator and not using a for loop
    let results_iter = results_vec.into_iter();

    // Use map to convert the results into a list of string slices
    let results_strings = results_iter.map(|result| match result {
        Ok(string) => string,
        Err(string) => string,
    });

    // Print the results
    for result in results_strings {
        println!("{result}");
    }
}

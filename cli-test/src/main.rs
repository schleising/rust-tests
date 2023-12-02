// Import the get_number_from_user function from the cli module
use cli_test::user_input::{get_number_from_user, error_should_exit};

fn main() {
    // Loop forever
    loop {
        // Get a number from the user
        let input_number: i32 = match get_number_from_user() {
            Ok(number) => number,
            Err(input_err) => {
                println!("{}", input_err);
                // Check if the error should cause the program to exit
                if error_should_exit(input_err) {
                    // If the error should cause the program to exit, break the loop
                    break;
                } else {
                    // If the error should not cause the program to exit, continue the loop
                    continue;
                }
            },
        };

        // Print the number
        println!("The user entered: {}", input_number);
    }
}

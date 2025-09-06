// Import the get_number_from_user function and the CliError enum from the cli_test crate
use cli_test::user_input::{get_number_from_user, CliError};

// Import the colored crate
use colored::*;

fn main() {
    // Loop forever
    loop {
        // Get a number from the user
        let input_number: u8 = match get_number_from_user() {
            Ok(number) => number,
            Err(input_err) => {
                match input_err {
                    // Check if the error is a CliError::UserQuit and break the loop, otherwise continue
                    CliError::UserQuit => {
                        // Print quit message to stdout and break the loop
                        println!("{input_err}");
                        break
                    },
                    _ => {
                        // Print the error to stderr and continue the loop
                        eprintln!("{input_err}");
                        continue
                    },
                }
            },
        };

        // Print the number
        let output_string = format!("The user entered: {input_number}");
        println!("{}", output_string.blue());
    }
}

pub mod user_input {
    use std::io::{stdin, stdout, BufRead, Write};
    use std::error::Error;
    use std::fmt;

    use colored::Colorize;

    // Create a custom error type for the cli module that implements the Error trait
    #[derive(Debug, PartialEq)]
    pub enum CliError {
        NumberOverflow,
        EmptyInput,
        InvalidDigit,
        InvalidUtf8,
        UserQuit,
    }
    
    impl Error for CliError {}
    
    impl fmt::Display for CliError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CliError::NumberOverflow => write!(f, "{}", "Error: Number overflow, please enter a different number".red()),
                CliError::EmptyInput => write!(f, "{}", "Error: Empty input, please enter a number".red()),
                CliError::InvalidDigit => write!(f, "{}", "Error: Invalid digit, please enter a number".red()),
                CliError::InvalidUtf8 => write!(f, "{}", "Error: Invalid UTF-8, please enter a number".red()),
                CliError::UserQuit => write!(f, "{}", "Error: User quit, exiting...".green()),
            }
        }
    }

    impl From<std::num::ParseIntError> for CliError {
        fn from(error: std::num::ParseIntError) -> Self {
            match error.kind() {
                std::num::IntErrorKind::PosOverflow => CliError::NumberOverflow,
                std::num::IntErrorKind::NegOverflow => CliError::NumberOverflow,
                std::num::IntErrorKind::Empty => CliError::EmptyInput,
                std::num::IntErrorKind::InvalidDigit => CliError::InvalidDigit,
                _ => CliError::InvalidDigit,
                
            }
        }
    }

    impl From<std::io::Error> for CliError {
        fn from(error: std::io::Error) -> Self {
            match error.kind() {
                std::io::ErrorKind::InvalidData => CliError::InvalidUtf8,
                _ => CliError::InvalidDigit,
            }
        }
    }

    fn get_number_from_user_wrapped<R: BufRead, W: Write> (input: &mut R, output: &mut W) -> Result<u8, CliError> {
        // Print a prompt to output
        write!(output, "Please enter a number: ")?;

        // Flush the output to the screen
        output.flush()?;

        // Get a number from the user
        let mut user_input: String = String::new();
        input.read_line(&mut user_input)?;

        // Trim the input
        let trimmed_input: String = user_input.trim().to_string();

        // Check if the input is 'q' to quit
        if trimmed_input == "q" {
            // If the input was 'q', print a message and break the loop
            return Err(CliError::UserQuit);
        }
        
        // Parse the number
        let parsed_number: u8 = trimmed_input.parse::<u8>()?;
        
        // Return the number
        Ok(parsed_number)
    }

    pub fn get_number_from_user () -> Result<u8, CliError> {
        let mut input: std::io::StdinLock<'_> = stdin().lock();
        let mut output: std::io::Stdout = stdout();
        get_number_from_user_wrapped(&mut input, &mut output)
    }

    // Test the cli module
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_get_number_from_user () {
            // Test all possible u8 values
            for i in u8::MIN..u8::MAX {
                // Create a string from the number
                let current_number: String = format!("{i}\n");

                // Test the get_number_from_user function simulating the user entering a number
                let mut input: &[u8] = current_number.as_bytes();
                let mut output: Vec<u8> = Vec::new();
        
                // Get the number from the user
                let number: Result<u8, CliError> = get_number_from_user_wrapped(&mut input, &mut output);
        
                // Check if the number is correct
                assert!(number.is_ok());
                assert_eq!(number.unwrap(), i);
            }
        }

        // Test all the error cases for the get_number_from_user function
        #[test]
        fn test_get_number_from_user_quit () {
            // Test the get_number_from_user function simulating the user entering 'q'
            let mut input: &[u8] = "q\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<u8, CliError> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert!(number.is_err());
            // Check if the error is CliError::UserQuit
            assert_eq!(number.unwrap_err(), CliError::UserQuit);
        }

        // Test that a number that is too large will return an error
        #[test]
        fn test_get_number_from_user_number_too_large () {
            // Test the get_number_from_user function simulating the user entering a number that is too large
            let mut input: &[u8] = "256\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<u8, CliError> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert!(number.is_err());
            // Check if the error is std::num::ParseIntError
            assert_eq!(number.unwrap_err(), CliError::NumberOverflow);
        }

        // Test that a non-number will return an error
        #[test]
        fn test_get_number_from_user_non_number () {
            // Test the get_number_from_user function simulating the user entering a non-number
            let mut input: &[u8] = "abc\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<u8, CliError> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert!(number.is_err());
            // Check if the error is std::num::ParseIntError
            assert_eq!(number.unwrap_err(), CliError::InvalidDigit);
        }

        // Test that a non-UTF8 character will return an error
        #[test]
        fn test_get_number_from_user_non_utf8 () {
            // Test the get_number_from_user function simulating the user entering a non-UTF8 character
            let mut input: &[u8] = &[0, 159, 146, 150];
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<u8, CliError> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert!(number.is_err());
            // Check if the error is a std::io::Error
            assert_eq!(number.unwrap_err(), CliError::InvalidUtf8);
        }

        // Test that an empty string will return an error
        #[test]
        fn test_get_number_from_user_empty_string () {
            // Test the get_number_from_user function simulating the user entering an empty string
            let mut input: &[u8] = "\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<u8, CliError> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert!(number.is_err());
            // Check if the error is std::num::ParseIntError
            assert_eq!(number.unwrap_err(), CliError::EmptyInput);
        }
    }
}

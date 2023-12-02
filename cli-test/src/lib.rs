pub mod user_input {
    use std::io::{stdin, stdout, BufRead, Write};
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub enum CliError {
        UserQuit,
    }
    
    impl fmt::Display for CliError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                CliError::UserQuit => write!(f, "User quit, exiting..."),
            }
        }
    }
    
    impl Error for CliError {}
    
    fn get_number_from_user_wrapped<R: BufRead, W: Write> (input: &mut R, output: &mut W) -> Result<i32, Box<dyn Error>> {
        // Print a prompt
        print!("Enter a number or (q) to quit: ");

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
            return Err(Box::new(CliError::UserQuit));
        }
        
        // Parse the number
        let parsed_number: i32 = trimmed_input.parse::<i32>()?;
        
        // Return the number
        Ok(parsed_number)
    }

    pub fn get_number_from_user () -> Result<i32, Box<dyn Error>> {
        let mut input: std::io::StdinLock<'_> = stdin().lock();
        let mut output: std::io::Stdout = stdout();
        get_number_from_user_wrapped(&mut input, &mut output)
    }

    pub fn error_should_exit (input_err: Box<dyn Error>) -> bool {
        // Check if the error is a CliError::UserQuit and print the error
        if let Some(_) = input_err.downcast_ref::<CliError>() {
            return true;
        } else {
            return false;
        }
    }

    // Test the cli module
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_get_number_from_user () {
            // Test the get_number_from_user function simulating the user entering 1
            let mut input = "1\n".as_bytes();
            let mut output = Vec::new();
    
            // Get the number from the user
            let number: Result<i32, Box<dyn Error>> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the number is 1
            assert_eq!(number.unwrap(), 1);
        }

        // Test all the error cases for the get_number_from_user function
        #[test]
        fn test_get_number_from_user_quit () {
            // Test the get_number_from_user function simulating the user entering 'q'
            let mut input: &[u8] = "q\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<i32, Box<dyn Error>> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert_eq!(number.is_err(), true);
            // Check if the error is CliError::UserQuit
            assert_eq!(number.unwrap_err().downcast_ref::<CliError>().is_some(), true);
        }

        // Test that a number that is too large will return an error
        #[test]
        fn test_get_number_from_user_number_too_large () {
            // Test the get_number_from_user function simulating the user entering a number that is too large
            let mut input: &[u8] = "2147483648\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<i32, Box<dyn Error>> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert_eq!(number.is_err(), true);
            // Check if the error is std::num::ParseIntError
            let error = number.unwrap_err();
            let downcast_error = error.downcast_ref::<std::num::ParseIntError>().unwrap();
            assert_eq!(error.downcast_ref::<std::num::ParseIntError>().is_some(), true);
            // Check is the kind of the error is std::num::IntErrorKind::PosOverflow
            let error_kind = downcast_error.kind();
            assert_eq!(*error_kind, std::num::IntErrorKind::PosOverflow);
        }

        // Test that a non-number will return an error
        #[test]
        fn test_get_number_from_user_non_number () {
            // Test the get_number_from_user function simulating the user entering a non-number
            let mut input: &[u8] = "abc\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<i32, Box<dyn Error>> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert_eq!(number.is_err(), true);
            // Check if the error is std::num::ParseIntError
            let error = number.unwrap_err();
            assert_eq!(error.downcast_ref::<std::num::ParseIntError>().is_some(), true);
            // Check is the kind of the error is std::num::IntErrorKind::InvalidDigit
            let downcast_error = error.downcast_ref::<std::num::ParseIntError>().unwrap();
            let error_kind = downcast_error.kind();
            assert_eq!(*error_kind, std::num::IntErrorKind::InvalidDigit);
        }

        // Test that a non-UTF8 character will return an error
        #[test]
        fn test_get_number_from_user_non_utf8 () {
            // Test the get_number_from_user function simulating the user entering a non-UTF8 character
            let mut input: &[u8] = &[0, 159, 146, 150];
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<i32, Box<dyn Error>> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert_eq!(number.is_err(), true);
            // Check if the error is a std::io::Error
            let error = number.unwrap_err();
            assert_eq!(error.downcast_ref::<std::io::Error>().is_some(), true);
            // Check if the error is a std::io::ErrorKind::InvalidData
            let downcast_error = error.downcast_ref::<std::io::Error>().unwrap();
            let error_kind = downcast_error.kind();
            assert_eq!(error_kind, std::io::ErrorKind::InvalidData);
        }

        // Test that an empty string will return an error
        #[test]
        fn test_get_number_from_user_empty_string () {
            // Test the get_number_from_user function simulating the user entering an empty string
            let mut input: &[u8] = "\n".as_bytes();
            let mut output: Vec<u8> = Vec::new();
    
            // Get the number from the user
            let number: Result<i32, Box<dyn Error>> = get_number_from_user_wrapped(&mut input, &mut output);
    
            // Check if the error is CliError::UserQuit
            assert_eq!(number.is_err(), true);
            // Check if the error is std::num::ParseIntError
            let error = number.unwrap_err();
            assert_eq!(error.downcast_ref::<std::num::ParseIntError>().is_some(), true);
            // Check is the kind of the error is std::num::IntErrorKind::Empty
            let downcast_error = error.downcast_ref::<std::num::ParseIntError>().unwrap();
            let error_kind = downcast_error.kind();
            assert_eq!(*error_kind, std::num::IntErrorKind::Empty);
        }

        #[test]
        fn test_error_should_exit () {
            // Test the error_should_exit function
            assert_eq!(error_should_exit(Box::new(CliError::UserQuit)), true);
        }

        #[test]
        fn test_error_should_not_exit () {
            use std::io::{Error, ErrorKind};
            // Test the error_should_exit function
            assert_eq!(error_should_exit(Box::new(Error::new(ErrorKind::Other, "Test error"))), false);
        }
    }
}

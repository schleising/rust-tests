use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use chrono::NaiveTime;

// Create an error type for the program
#[derive(Debug)]
enum FfmpegError {
    // An error occurred while spawning the process
    SpawnError(std::io::Error),
    // An error occurred while parsing the output
    ParseError(String),
}

// Implement the Display trait for FfmpegError
impl std::fmt::Display for FfmpegError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FfmpegError::SpawnError(e) => write!(f, "Spawn Error: {:?}", e),
            FfmpegError::ParseError(e) => write!(f, "Parse Error: {:?}", e),
        }
    }
}

// Implement the Error trait for FfmpegError
impl std::error::Error for FfmpegError {}

// Convert a parse int error to an FfmpegError
impl From<std::num::ParseIntError> for FfmpegError {
    fn from(e: std::num::ParseIntError) -> Self {
        FfmpegError::ParseError(format!("ParseIntError: {}", e))
    }
}

// Convert a parse float error to an FfmpegError
impl From<std::num::ParseFloatError> for FfmpegError {
    fn from(e: std::num::ParseFloatError) -> Self {
        FfmpegError::ParseError(format!("ParseFloatError: {}", e))
    }
}

// Convert a chrono parse error to an FfmpegError
impl From<chrono::ParseError> for FfmpegError {
    fn from(e: chrono::ParseError) -> Self {
        FfmpegError::ParseError(format!("ParseError: {}", e))
    }
}

// Create a struct to hold the output
struct FfmpegOutput {
    // The current frame
    current_frame: u32,

    // Current frames per second
    current_fps: f32,

    // Quality
    quality: f32,

    // Size
    size: i32,

    // Time
    time: NaiveTime,

    // Bitrate
    bitrate: f32,

    // Speed
    speed: f32,
}

impl FfmpegOutput {
    // Create a new FfmpegOutput
    fn new() -> FfmpegOutput {
        if let Some(time) = NaiveTime::from_hms_opt(0, 0, 0) {
            FfmpegOutput {
                current_frame: 0,
                current_fps: 0.0,
                quality: 0.0,
                size: 0,
                time,
                bitrate: 0.0,
                speed: 0.0,
            }
        } else {
            panic!("Could not create NaiveTime");
        }
    }
}

// Implement the Display trait for FfmpegOutput
impl std::fmt::Display for FfmpegOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Decoded: frame={:5} fps={:5.2} q={:5.2} size={:5}kB time={} bitrate={:5.2}kbits/s speed={:5.2}x", self.current_frame, self.current_fps, self.quality, self.size, self.time.format("%H:%M:%S%.f"), self.bitrate, self.speed)
    }
}

// Create an enum to label the different parts of the output
enum FfmpegOutputPart {
    Frame = 1,
    Fps,
    Quality,
    Size,
    Time,
    Bitrate,
    Speed,
}

// Parse a string into an FfmpegOutput
fn parse_string(string: &str) -> Result<(), FfmpegError> {
    // Check if the string starts with "frame="
    if string.starts_with("frame=") {
        // Use a regular expression to parse the string
        let re = regex::Regex::new(r"frame= *(\d+) fps= *(\d+)\.*\d* q= *(-*\d+\.\d+) L*size= *(\d+)kB time= *(\d{2}:\d{2}:\d{2}\.\d{2}) bitrate= *(\d+\.\d+)kbits/s dup= *\d+ drop= *\d+ speed= *(\d+\.*\d*) *x").unwrap();

        // Get the captures from the regular expression
        let captures = re.captures(string);

        // Check if the captures are valid
        if let Some(captures) = captures {
            // Create a new FfmpegOutput
            let mut output = FfmpegOutput::new();

            // Get the current frame
            output.current_frame = captures.get(FfmpegOutputPart::Frame as usize)
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .ok_or(FfmpegError::ParseError("Could not get current frame".to_string()))?;

            // Get the current fps
            output.current_fps = captures.get(FfmpegOutputPart::Fps as usize)
                .and_then(|m| m.as_str().parse::<f32>().ok())
                .ok_or(FfmpegError::ParseError("Could not get current fps".to_string()))?;

            // Get the quality
            output.quality = captures.get(FfmpegOutputPart::Quality as usize)
                .and_then(|m| m.as_str().parse::<f32>().ok())
                .ok_or(FfmpegError::ParseError("Could not get quality".to_string()))?;

            // Get the size
            output.size = captures.get(FfmpegOutputPart::Size as usize)
                .and_then(|m| m.as_str().parse::<i32>().ok())
                .ok_or(FfmpegError::ParseError("Could not get size".to_string()))?;

            // Get the time
            output.time = captures.get(FfmpegOutputPart::Time as usize)
                .and_then(|m| NaiveTime::parse_from_str(m.as_str(), "%H:%M:%S%.f").ok())
                .ok_or(FfmpegError::ParseError("Could not get time".to_string()))?;

            // Get the bitrate
            output.bitrate = captures.get(FfmpegOutputPart::Bitrate as usize)
                .and_then(|m| m.as_str().parse::<f32>().ok())
                .ok_or(FfmpegError::ParseError("Could not get bitrate".to_string()))?;

            // Get the speed
            output.speed = captures.get(FfmpegOutputPart::Speed as usize)
                .and_then(|m| m.as_str().parse::<f32>().ok())
                .ok_or(FfmpegError::ParseError("Could not get speed".to_string()))?;

            // Print the output
            println!("{}", output);
        } else {
            // Return an error
            return Err(FfmpegError::ParseError("Could not parse string".to_string()));
        }
    }

    // Return Ok
    Ok(())
} 

fn main() -> std::io::Result<()> {
    let mut command = Command::new("ffmpeg");
    println!("Command Created");

    // Create a path to the user's Downloads directory
    let mut downloads = PathBuf::new();

    // Get the user's home directory
    if let Some(home) = dirs::home_dir() {
        // Add the Downloads directory to the path
        downloads.push(home);
        downloads.push("Downloads");
    } else {
        // Return an error
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find home directory"));
    }

    // Set the input and output files
    let input_file = downloads.join("DH.mp4");
    let output_file = downloads.join("DH2.mp4");

    // Set the command arguments
    command.arg("-i");
    command.arg(&input_file);
    command.args(["-y", "-c:v", "libx264", "-crf", "23", "-c:a", "copy"]);
    command.arg(&output_file);
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command.stdin(Stdio::piped());
    println!("Command Built");

    match command.spawn() {
        Ok(mut cmd) => {
            println!("Command Executed");

            if let Some(stderr) = cmd.stderr.as_mut() {
                // Create a buffer for the output
                let mut buffer: Vec<u8> = Vec::new();

                // Create a reader for the output
                let mut reader = BufReader::new(stderr);

                // Loop through the output and print it to the screen
                loop {
                    // Clear the buffer
                    buffer.clear();

                    let mut byte_buffer: [u8; 1] = [0; 1];

                    // Read byte by byte until a \r or \n
                    while let Ok(bytes_read) = reader.read(&mut byte_buffer) {
                        // If no bytes were read, break out of the loop
                        if bytes_read == 0 {
                            break;
                        }

                        // Get the byte
                        let byte = byte_buffer[0];

                        // If the byte is a \r or \n, break out of the loop
                        if byte == b'\r' || byte == b'\n' {
                            break;
                        }

                        // Push the byte to the buffer
                        buffer.push(byte);
                    }

                    // If the buffer is empty, break out of the loop
                    if buffer.is_empty() {
                        break;
                    }

                    // Convert the buffer to a string
                    if let Ok(s) = String::from_utf8(buffer.clone()) {
                        // Print the string
                        match parse_string(s.as_str()) {
                            Ok(_) => {
                            },
                            Err(e) => {
                                println!("Parse Error: {} {}", e, s);
                            }
                        }
                    }
                }
            } else {
                println!("Could not get stdout");
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Could not get stdout"));
            }

            // Wait for the process to finish
            if let Ok(status) = cmd.wait() {
                println!("Process ended with {}", status);
            }
        },
        Err(e) => {
            let error = FfmpegError::SpawnError(e);
            println!("Error: {}", error);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, error));
        }
    }

    Ok(())
}

/* Allows for coercion of std::io::Error into UserError */

// Third Party Dependencies
use std::io::Error as IOError;

// Intra Library Imports
use super::UserError;

/// Convert a std::io::Error into a UserError
///
/// # Example
/// ```
/// use user_error::UserError;
///
/// use std::fs::File;
///	fn open_file(path: &str) -> Result<File, UserError> {
///		let f = File::open(path)?;
///		Ok(f)
///	}
///
///	match open_file("does_not_exist.txt") {
///		Err(e) => eprintln!("{}", e),
///		Ok(_) => ()
///	}
/// ```
/// This results in the following being printed to stderr:
/// ```bash
/// Error: No such file or directory (os error 2)
/// ```
impl From<IOError> for UserError {
    fn from(error: IOError) -> Self {
        let summary = String::from("I/O Error");
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("File not found")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::PermissionDenied => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Insufficient permissions")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::ConnectionRefused => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Connection refused by the remote server")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::ConnectionReset => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Connection reset by the remote server")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::ConnectionAborted => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Connection aborted by the remote server")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::NotConnected => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("The network operation failed because it was not connected yet")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::AddrInUse => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Socket could not be bound because the address is already in use")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::AddrNotAvailable => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Address not available")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::BrokenPipe => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Requested pipe was broken")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::AlreadyExists => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("File already exists")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::WouldBlock => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Operation needs to block to complete, but the blocking operation was requested to not occur")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::InvalidInput => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Incorrect parameter provided")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::InvalidData => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Invalid or malformed data")]),
                    subtleties: Some(vec![String::from("For example, a function that reads a file into a string will error with InvalidData if the file's contents are not valid UTF-8")]),
                    original_errors: None,
                }
            },
            std::io::ErrorKind::TimedOut => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Operation timed out")]),
                    subtleties: None,
                    original_errors: None,
                }
            },
            std::io::ErrorKind::WriteZero => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Call to `write` returned `Ok(0)`")]),
                    subtleties: Some(vec![String::from("This typically means that an operation could only succeed if it wrote a particular number of bytes but only a smaller number of bytes could be written.")]),
                    original_errors: None,
                }
            },
            std::io::ErrorKind::Interrupted => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Operation was interrupted")]),
                    subtleties: Some(vec![String::from("Interrupted operations can typically be retried.")]),
                    original_errors: None,
                }
            },
            std::io::ErrorKind::UnexpectedEof => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Encountered 'EOF' prematurely")]),
                    subtleties: Some(vec![String::from("This typically means that an operation could only succeed if it read a particular number of bytes but only a smaller number of bytes could be read.")]),
                    original_errors: None,
                }
            },
            _ => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Operation encountered an unexpected error")]),
                    subtleties: None,
                    original_errors: None,
                }
            }
        }
    }
}

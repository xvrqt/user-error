
use definition::UserError;

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
impl From<std::io::Error> for UserError {
    fn from(error: std::io::Error) -> Self {
        UserError {
        	summary: error.to_string(),
        	reasons: None,
        	subtleties: None,
        	original_errors: Some(vec![Box::new(error)]),
        }
    }
}
/* Allows for the conversion of String, &str Error types to be coerced into a
 * UserError.
*/

use super::UserError;

/// Convert an Err(String) into a UserError
///
/// # Example
/// ```
/// use user_error::UserError;
///
/// fn string_error(e: &str) -> Result<(), String> {
///		Err(String::from(e))
///	}
///
///	fn caller() -> Result<(), UserError> {
///		string_error("broken")?;
///		Ok(())
///	}
///
///	match caller() {
///		Err(e) => eprintln!("{}", e),
///		Ok(_) => ()
///	}
/// ```
/// This results in the following being printed to stderr:
/// ```bash
/// Error: broken
/// ```
impl From<String> for UserError {
    fn from(error: String) -> Self {
        UserError {
        	summary: error,
        	reasons: None,
        	subtleties: None,
        	original_errors: None,
        }
    }
}

/// Convert an Err(&'static str) into a UserError
///
/// # Example
/// ```
/// use user_error::UserError;
///
/// fn str_error(path: &str) -> Result<(), &'static str> {
///		Err("Failed!")
///	}
///
///	fn caller() -> Result<(), UserError> {
///		str_error("does_not_exist.txt")?;
///		Ok(())
///	}
///
///	match caller() {
///		Err(e) => eprintln!("{}", e),
///		Ok(_) => ()
///	}
/// ```
/// This results in the following being printed to stderr:
/// ```bash
/// Error: Failed!
/// ```
impl From<&str> for UserError {
    fn from(error: &str) -> Self {
        UserError {
        	summary: String::from(error),
        	reasons: None,
        	subtleties: None,
        	original_errors: None,
        }
    }
}

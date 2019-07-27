/* Allows for coercion of rusqlite::Error into UserError */

// Third Party Dependencies
use scrawl::error::ScrawlError as ScrawlError;

// Intra Library Imports
use super::UserError;

/// Convert a ScrawlError into a UserError
///
/// # Example
/// ```should_panic
/// use scrawl;
/// use user_error::UserError;
/// use std::path::Path;
///
/// fn bad_scrawl() -> Result<String, UserError> {
///     let file = Path::new("does_not_exist.txt");
///     let output = scrawl::open(file)?;
///     Ok(output)
/// }
///
/// match bad_scrawl() {
///     Ok(s)  => println!("{}", s),
///     Err(e) => e.print_and_exit()
/// }
/// 
/// ```
impl From<ScrawlError> for UserError {
    fn from(error: ScrawlError) -> Self {
        const SUMMARY: &str = "Scrawl Error";
        match error {
            ScrawlError::FailedToCreateTempfile => UserError::hardcoded(SUMMARY,
                    &["Could not create a temporary file to use as a buffer"],
                    &[]),

            ScrawlError::FailedToOpenEditor(editor) => UserError::hardcoded(SUMMARY,
                    &[&format!("Could not open {} as a text editor", editor)],
                    &[]),

            ScrawlError::FailedToCaptureInput=> UserError::hardcoded(SUMMARY,
                    &["Failed to capture user input."],
                    &[]),

            ScrawlError::FailedToCopyToTempFile(filename) => UserError::hardcoded(SUMMARY,
                &[&format!("Failed to copy the contents of the `{}` to the temporary buffer for editing.", filename)],
                &["Make sure the file exists."])
        }
    }
}


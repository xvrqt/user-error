//! # User Error
//! A library for conveniently displaying well-formatted, and good looking errors to users of CLI applications. Useful for bubbling up unrecoverable errors to inform the user what they can do to fix them.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

// Standard Library Dependencies
use std::fmt;
use std::path::Path;
use std::error::Error;

// Third Party Dependencies
use colorful::Color;
use colorful::Colorful;

/********************
 * Helper Functions *
 ********************/

// Returns the application name (falls back on "The Application")
#[inline]
fn get_application_name() -> String {
	// Pull the name from the first command line argument
	String::from(std::env::args().next().as_ref()
				.map(|s| Path::new(s))
				.and_then(|p| p.file_stem())
				.and_then(|s| s.to_str())
				.unwrap_or("The application"))
}

// Keep the default string formatting DRY
#[inline]
fn default_summary() -> String {
	format!("{} encountered an unknown error.", get_application_name())
}

/// The eponymous struct.
#[derive(Debug)]
pub struct UserError {
	/// These fields are used to print the error. Title should be a summary of the error (e.g. "Failed to process files"). Reasons should be a list of reasons for the summary (e.g. "Direction 'foo' doesn't exist"). Subtlties is dimly printed text that can be used to provide more verbose solutions the use can take to resolve the error, (e.g. "Try running the following command to create the directory: mkdir foo"). 
	summary:    String,
	reasons:    Option<Vec<String>>,
	subtleties: Option<Vec<String>>,
}

impl UserError {

	/// Generates an error by consuming heap allocated arguemnts. This method is better if you're dynamically creating error messages. If you're using hardcoded error messages, see UserError::hardcoded below.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	/// let error_summary    = String::from("Failed to build project");
	/// let error_reasons    = vec![String::from("Database could not be parsed"), String::from("File \"main.db\" not found")];
	/// let error_subtleties = vec![String::from("Try: touch main.db"), String::from("This command will create and empty database file the program can use ")];
	/// let e = UserError::new(error_summary, error_reasons, error_subtleties);
	/// ```
	pub fn new(mut summary: String, reasons: Vec<String>, subtleties: Vec<String>) -> UserError {
		if summary.is_empty() {
			summary = default_summary();
		}
		UserError {
			summary,
			reasons: Some(reasons),
			subtleties: Some(subtleties),
		}
	}

	/// Generate an error using hardcoded string references (&str), making it a quick and dirty way to create a pretty printed error message.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	/// let e = UserError::hardcoded("Failed to build project", 
	///								&[	"Database could not be parsed", 
	///									"File \"main.db\" not found"], 
	///								&[	"Try: touch main.db", 
	///									"This command will create and empty database file the program can use "]);
	/// ```
	pub fn hardcoded(summary: &str, reasons: &[&str], subtleties: &[&str]) -> UserError {
		let reasons = reasons.into_iter().map(|s| String::from(*s)).collect();
		let subtleties = subtleties.into_iter().map(|s| String::from(*s)).collect();
		UserError {
			summary: String::from(summary),
			reasons: Some(reasons),
			subtleties: Some(subtleties),
		}
	}

	/// Generate an error with only a summary using a hardcoded string reference (&str). A quick and dirty way to create a simple pretty printed error message.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	/// let e = UserError::simple("Failed to build project");
	/// ```
	pub fn simple(summary: &str) -> UserError {
		UserError {
			summary: String::from(summary),
			reasons: None,
			subtleties: None,
		}
	}

	/// Returns a formatted, possibly colored String listing the error summary, prepended by an error header ("Error: "). If the terminal supports color, the error header will be printed boldly in white upon a red background; the error summary will be printed boldly in red upon a black background. If no error summary is provided the default is: <application name> encountered an unknown error.
	/// Format: "Error: <summary>"
	/// Example Output:
	/// Error: Failed to build project
	///
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	/// let e = UserError::hardcoded("Failed to build project", 
	///								&[	"Database could not be parsed", 
	///									"File \"main.db\" not found"], 
	///								&[	"Try: touch main.db", 
	///									"This command will create and empty database file the program can use "]);
	/// eprintln!("{}", e.summary()); // Error: Failed to build project
	/// ```
	pub fn summary(&self) -> String {
		format!("{} {}",
			String::from("Error:").color(Color::White).bg_color(Color::Red).bold(),
			self.summary.clone().color(Color::Red).bold()
		)
	}

	/// Returns a formatted, possibly colored String listing the reasons for the error. If the terminal supports color, the bullet point ('-') will be colored yellow. Each String in the Vec<String> will be printed on its own line.
	/// Format: - <reason>
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	/// let e = UserError::hardcoded("Failed to build project", 
	///								&[	"Database could not be parsed", 
	///									"File \"main.db\" not found"], 
	///								&[	"Try: touch main.db", 
	///									"This command will create and empty database file the program can use "]);
	///
	/// eprintln!("{}", e.reasons()); // - Database could not be parsed
	///								  // - File "main.db" no found
	/// ```
	pub fn reasons(&self) -> String {
		match &self.reasons {
			Some(v) => {
				let mut b = String::with_capacity(v.len() * 32);
				v.iter().for_each(|s| {
					b.push_str(format!("{} {}\n", 
								"-".color(Color::Yellow),
								s.as_str()).as_str());
				});
				b.pop();
				b
			},
			None => String::from("")
		}
	}

	/// Modifies the UserError by adding in additional reasons. Useful if you're passing this error up the call stack while cleaning up and want each calling function to be able to annotate what went wrong before displaying to the user.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	/// // An untested function that may fail to process a number
    ///	fn unstable_function(n: isize) -> Result<(), &'static str> {
    ///		if n >= 0 { Ok(()) }
    ///     else { Err("Negative numbers cannot be processed!") }
    ///	}
    ///
    ///	// Processes a vec of numbers. Reports errors but doesn't halt the processing upon encountering one.
    ///	fn process_numbers(numbers: Vec<isize>) -> Result<(), UserError> {
    ///		let mut bad_nums = Vec::new();
    ///		
    ///		// Process a list of numbers
    ///		for (i, n) in numbers.iter().enumerate() {
    ///			match unstable_function(*n) {
    ///				Err(_) => { bad_nums.push((i, n)); },
    ///				Ok(_) => ()
    ///			}
    ///		}
    ///
    ///		// Check if any of the numbers failed to process
    ///		match  bad_nums.len() {
    ///			0 => Ok(()),
    ///			_ => {
    ///				// Return an error that tells the user which inputs failed
    ///				let summary = format!("Failed to process {} inputs", bad_nums.len());
    ///				let mut e = UserError::simple(&summary);
    ///				for (i, n) in bad_nums {
	///					let reason = format!("Failed input #{} with value ({})", i, n);
    ///					e.add_reason(&reason);
    ///				}
    ///				Err(e)
    ///			}
    ///		}
    ///	}
    ///
    ///	// Process the numbers
    ///	let numbers = vec![0, 1, 2, -2, 50, -1, -100, 22, 2, 2];
	///	match process_numbers(numbers) {
	///     Err(e) => eprintln!("{}", e),
	///     _ => println!("List processed successfully!"),
	/// }
	/// ```
	/// This results in the following being printed to stderr:
	/// ```bash
	/// Error: Failed to process 3 inputs
    /// - Failed input #3 with value (-2)
    /// - Failed input #5 with value (-1)
    /// - Failed input #6 with value (-100)
    /// ```
	pub fn add_reason(&mut self, reason: &str) {
		let reason = String::from(reason);
		match &mut self.reasons {
			Some(v) => { v.push(reason); },
			None => {
				self.reasons = Some(vec![reason]);
			}
		}
	}

	/// Returns a formatted, possibly colored String listing additional subtleties to the error. If the terminal supports color, subtleties will be printed dimly. Each String in the Vec<String> will be printed on its own line.
	/// Format: <subtleties>
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	/// let e = UserError::hardcoded("Failed to build project", 
	///								&[	"Database could not be parsed", 
	///									"File \"main.db\" not found"], 
	///								&[	"Try: touch main.db", 
	///									"This command will create and empty database file the program can use "]);
	///
	/// eprintln!("{}", e.subtleties()); // Try: touch main.db 
	///								     // This command will create and empty database file the program can use
	/// ```
	pub fn subtleties(&self) -> String {
		match &self.subtleties {
			Some(v) => {
				let mut b = String::with_capacity(v.len() * 32);
				v.iter().for_each(|s| {
					b.push_str(&format!("{}\n", 
								s.as_str()
									.color(Color::White)
									.dim()));
				});
				// Remove the last linebreak
				b.pop();
				b
			},
			None => String::from("")
		}
	}

	/// Modifies the UserError by adding in additional subtly. Useful if you're passing this error up the call stack while cleaning up and want each calling function to be able to annotate what went wrong before displaying to the user.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///	// A mock system call to open a file
    ///	fn open_file(path: &str) -> Result<(), String> {
    ///		Err(format!("Failed to open file: {}", path))
    ///	}
    ///
    ///	// A mock system call to check if path requires root permissions
    ///	fn needs_root(path: &str) -> bool {
    ///		true
    ///	}
    ///
    ///	// Builds an unspecified project
    ///	fn build_project(path: &str) -> Result<(), UserError> {
    ///		match open_file(path) {
    ///			Ok(_) => Ok(()),
    ///			Err(e) => {
    ///				let mut error = UserError::new(String::from("Failed to build project"),
    ///														vec![e],
    ///														vec![format!("Try: touch {}", path)]);let mut error = UserError::simple("Failed to build project");
    ///				// Conditionally give the user an additional hint
    ///				if needs_root(path) {
    ///					error.add_subtly("You may need to ask your administrator to run this command for you");
    ///				}
    ///				Err(error)
    ///			}
    ///		}
    ///	}
    ///
	///	match build_project("/user_data.db") {
	///        Err(e) => eprintln!("{}", e),
	///        _ => println!("Project built successfully!"),
	///    }
	/// 
    /// ```
	pub fn add_subtly(&mut self, subtly: &str) {
		let subtly = String::from(subtly);
		match &mut self.subtleties {
			Some(v) => { v.push(subtly); },
			None => {
				self.subtleties = Some(vec![subtly]);
			}
		}
	}
}

/// Required to implement the Error trait
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut summary = self.summary();
    	let mut reasons = self.reasons();
    	let subtleties = self.subtleties();

    	// Concatenate line breaks if necessary
    	if !reasons.is_empty() || !subtleties.is_empty() {
    		summary.push('\n');
    	}

    	if !reasons.is_empty() && !subtleties.is_empty() {
    		reasons.push('\n');
    	}

        f.write_str(&format!("{}{}{}", summary, reasons, subtleties))
    }
}

/// Default implementation for UserError attempts to print: <application name> has encountered an unknown error.
impl Default for UserError {
	fn default() -> Self {
		UserError {
			summary: default_summary(),
			reasons: None,
			subtleties: None
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn produce_error(ue: UserError) -> Result<(), UserError> {
		Err(ue)
	}

    #[test]
    fn test() {

    	// A mock system call to open a file
    	fn open_file(path: &str) -> Result<(), String> {
    			Err(format!("Failed to open file: {}", path))
    	}

    	// A mock system call to check if path requires root permissions
    	fn needs_root(path: &str) -> bool {
    		true
    	}

    	// Builds an unspecified project
    	fn build_project(path: &str) -> Result<(), UserError> {

    		match open_file(path) {
    			Ok(_) => Ok(()),
    			Err(e) => {
    				let mut error = UserError::new(String::from("Failed to build project"),
    														vec![e],
    														vec![format!("Try: touch {}", path)]);
    				// error.add_reason(&e);
    				// error.add_subtly(&format!("Try: touch {}", path));
    				// Conditionally give the user an additional hint
    				if needs_root(path) {
    					error.add_subtly("You may need to ask your administrator to run this command for you")
    				}
    				Err(error)
    			}
    		}
    	}

		match build_project("/user_data.db") {
	        Err(e) => eprintln!("{}", e),
	        _ => println!("Project built successfully!"),
	    }
    }
}

//! # User Error
//! A library for conveniently displaying well-formatted, and good looking errors to users of CLI applications. Useful for bubbling up unrecoverable errors to inform the user what they can do to fix them.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

// Standard Library Dependencies
use std::fmt;
use std::path::Path;
use std::error::Error;

// Third Party Dependencies
// use colorful::HSL;
// use colorful::RGB;
use colorful::Color;
use colorful::Colorful;

// Print an error summary, reasons for the error, ways to correct and other info
// Keep track of original errors, and error codes for internal processing 
// Have multiple convenient constructors
// Print & (Print & Exit) commands
// Have a dynamic ColorScheme

/// The eponymous struct.
#[derive(Debug)]
pub struct UserError {
	// These fields are used to print the error. Title should be a summary of the error (e.g. "Failed to process files"). Reasons should be a list of reasons for the summary (e.g. "Direction 'foo' doesn't exist"). Subtlties is dimly printed text that can be used to provide more verbose solutions the use can take to resolve the error, (e.g. "Try running the following command to create the directory: mkdir foo"). 
	summary: String,
	reasons: Option<Vec<String>>,
	sublties: Option<Vec<String>>,

	// These optional fields can make be used internally for dealing with errors
	code: Option<usize>,
	original_error: Option<Box<dyn Error>>,

	// This optional field can be supplied to change the color scheme for this error only
	color_scheme: Option<ColorScheme>
}

impl UserError {

	/// Generate a new user facing error
	pub fn new(summary: &str) -> UserError {
		UserError {
			summary: String::from(summary),
			..Default::default()
		}
	}

	// These combine the data of Error instances with the functions of the default ColorScheme
	// fn print_summary(&self) -> colorful::core::color_string::CString {
		// (*DEFAULT_COLOR_SCHEME.summary)(self.summary.as_str())
	// }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", "Error:".color(Color::White).bg_color(Color::Red).bold(), self.summary.as_str())
    }
}

impl Default for UserError {
	fn default() -> Self {
		let name = String::from(std::env::args().next().as_ref()
						.map(|s| Path::new(s))
						.and_then(|p| p.file_stem())
						.and_then(|s| s.to_str())
						.unwrap_or("The application"));
		
		UserError {
			summary: format!("{} has stopped working.", name),
			reasons: None,
			sublties: None,

			code: None,
			original_error: None,

			color_scheme: None
		}
	}
}

/// You can provide an alternative color scheme for the error printing using this struct
#[derive(Debug)]
struct ColorScheme {
	// summary: Box<Fn(&str) -> colorful::core::color_string::CString>
	
	// reasons: Color,
	// reasons_bullet: Color,

	// sublties: Color
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test() {
		fn produce_error() -> Result<(), UserError> {
			Err(UserError::new("Failed to build project"))
		}

		match produce_error() {
	        Err(e) => eprintln!("{}", e),
	        _ => println!("No error"),
	    }
    }

    #[test]
    fn default() {
    	fn produce_error() -> Result<(), UserError> {
			Err(UserError {
				..Default::default()
			})
		}

		match produce_error() {
	        Err(e) => eprintln!("{}", e),
	        _ => println!("No error"),
	    }
    }
}

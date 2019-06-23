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

// Third Party Dependencies
use colorful::HSL;
use colorful::RGB;
use colorful::Color;
use colorful::Colorful;

// Print an error summary, reasons for the error, ways to correct and other info
// Keep track of original errors, and error codes for internal processing 
// Have multiple convenient constructors
// Print & (Print & Exit) commands
// Have a dynamic ColorScheme

/// The eponymous struct.
#[derive(Debug)]
struct Error {
	// These fields are used to print the error. Title should be a summary of the error (e.g. "Failed to process files"). Reasons should be a list of reasons for the summary (e.g. "Direction 'foo' doesn't exist"). Subtlties is dimly printed text that can be used to provide more verbose solutions the use can take to resolve the error, (e.g. "Try running the following command to create the directory: mkdir foo"). 
	summary: String,
	// reasons: Option<&[&str]>,
	// sublties: Option<&[&str]>,

	// These optional fields can make be used internally for dealing with errors
	// code: Option<usize>,
	// original_error: Option<Error>,

	// This optional field can be supplied to change the color scheme for this error only
	// color_scheme: Option<ColorScheme>
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", "Error:".color(Color::White).bg_color(Color::Red).bold(), self.summary.as_str().color(Color::Red))
    }
}

impl Error {
	pub fn new(summary: &str) -> Error {
		Error {
			summary: String::from(summary)
		}
	}
}

// impl fmt::Debug for Error {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{{ file: {}, line: {} }}", file!(), line!())
//     }
// }

// You can provide an alternative color scheme for the error printing using this struct
// struct ColorScheme {
// 	title: Color,
	
// 	reasons: Color,
// 	reasons_bullet: Color,

// 	sublties: Color
// }

// The default ColorScheme for error printing
// let default_color_scheme = ColorScheme {
// 	title: Color::Red,

// 	reasons: Color::White,
// 	reasons_bullet: Color::Yellow,

// 	subtle: Color::Gray
// };


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test() {
		fn produce_error() -> Result<(), Error> {
			Err(Error::new("Failed to build project"))
		}

		match produce_error() {
	        Err(e) => eprintln!("{}", e),
	        _ => println!("No error"),
	    }

        // let e = Error {
        // 	title: "Help, I'm gay!"
        // 	// reasons: None,
        // 	// sublties: None,
        // 	// code: None,
        // 	// original_error: None,
        // 	// color_scheme: None
        // };

        // eprintln!("{}", produce_error;
    }
}

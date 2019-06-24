//! # User Error
//! A library for conveniently displaying well-formatted, and good looking errors to users of CLI applications. Useful for bubbling up unrecoverable errors to inform the user what they can do to fix them.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features,
        unused_import_braces, unused_qualifications)]

// Standard Library Dependencies
use std::fmt;
use std::path::Path;
use std::sync::Mutex;
use std::error::Error;

// Third Party Dependencies
// use colorful::HSL;
// use colorful::RGB;
use colorful::Color;
use colorful::Colorful;
use colorful::core::color_string::CString;

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

	/// Generate a simple user facing error
	pub fn new(summary: &str) -> UserError {
		UserError {
			summary: String::from(summary),
			..Default::default()
		}
	}

	/// Used to print "Error:" followed by the UserError's summary
	fn print_summary(&self) -> String {
		format!("{} {}{}", 
			unsafe { (DEFAULT_COLOR_SCHEME.summary_header)("Error:") }, 
			unsafe { (DEFAULT_COLOR_SCHEME.summary)(self.summary.as_str()) }, 
			"\n"
		)
	}

	/// Used to print bulleted lists of detailed reasons for the error listed in summary
	fn print_reasons(&self) -> String {
		match &self.reasons {
			Some(v) => {
				let t: Vec<String> = v.iter().map(|s| { let mut ss = s.clone(); ss.insert_str(0, "- "); ss }).collect();
				format!("{}", t.join("\n"))
			},
			None => format!("")
		}
	}
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.print_summary())
        	.and(f.write_str(&self.print_reasons()))
    }
}

/// Default implementation for UserError attempts to print: {application name} has stopped working.
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

/// Contains function pointers to functions that apply the appropriate styling to the different parts of the error message
struct ColorScheme {
	summary_header: &'static Fn(&str)-> CString,
	summary: &'static Fn(&str)-> CString
	
	// reasons: Color,
	// reasons_bullet: Color,

	// sublties: Color
}

impl Default for ColorScheme {
	fn default() -> Self {
		ColorScheme {
			summary_header: &color_summary_header,
			summary: &color_summary
		}
	}
}

/// Static function pointers to the default styling functions. Can be overwritten using the ColorScheme interface if you don't want to pass your own ColorScheme with every call.  
static mut DEFAULT_COLOR_SCHEME: ColorScheme = ColorScheme {
	summary_header: &color_summary_header,
	summary: &color_summary
};

/// List of static function pointers used by DEFAULT_COLOR_SCHEME that make up the default styling implementation
///
/// Default styling for the "Error" part of the summary
fn color_summary_header(s: &str) -> CString {
	s.color(Color::White).bg_color(Color::Red).bold()
}

/// Default styling for the error summary
fn color_summary(s: &str) -> CString {
	s.color(Color::Red).bold()
}

/// Required override so that UserError (which contains and Option<ColorScheme> can implement the trait Debug which is reequired for std::Error type.
impl fmt::Debug for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "gay")
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	fn produce_error(ue: UserError) -> Result<(), UserError> {
		Err(ue)
	}

	fn color(s: &str) -> CString {
		s.color(Color::Red).bg_color(Color::White).bold()
	}

    #[test]
    fn test() {
		let ue = UserError::new("Failed to build project");
		match produce_error(ue) {
	        Err(e) => eprintln!("{}", e),
	        _ => println!("No error"),
	    }
    }

    #[test]
    fn default() {
    	let ue = UserError {
			..Default::default()
		};
		match produce_error(ue) {
	        Err(e) => eprintln!("{}", e),
	        _ => println!("No error"),
	    }
    }

    #[test]
    fn reasons() {
    	let ue = UserError {
    		summary: String::from("This error has actual reasons!"),
    		reasons: Some(vec![String::from("I'm gay"), String::from("I love girls")]),
    		..Default::default()
    	};

		match produce_error(ue) {
	        Err(e) => eprintln!("{}", e),
	        _ => println!("No error"),
	    }
    }
}

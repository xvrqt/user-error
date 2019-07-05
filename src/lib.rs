//! # User Error
//! A library for conveniently displaying well-formatted, and good looking errors to users of CLI applications. Useful for bubbling up unrecoverable errors to inform the user what they can do to fix them.
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

// Standard Library Dependencies
use std::error::Error;

mod traits;
mod helper;
mod sqlite_errors;
mod string_errors;
mod implementation;

/// The eponymous struct.
#[derive(Debug)]
pub struct UserError {
	/// These fields are used to print the error. Title should be a summary of the error (e.g. "Failed to process files"). Reasons should be a list of reasons for the summary (e.g. "Direction 'foo' doesn't exist"). Subtlties is dimly printed text that can be used to provide more verbose solutions the use can take to resolve the error, (e.g. "Try running the following command to create the directory: mkdir foo"). 
	summary:    String,
	reasons:    Option<Vec<String>>,
	subtleties: Option<Vec<String>>,

	/// Original Error (if any) used when converted from another error type
	original_error: Option<Box<Error>>,
}

/* Test that you can construct an error. Additional test are in the
 * tests/ folder and in the documentation of the other files.
*/
#[cfg(test)]
mod tests {
	use super::*;
	// Testing is done via document examples and would be redundant here
	#[test]
	fn example() {
		let e = UserError::hardcoded("Failed to build project", 
                                &[  "Database could not be parsed", 
                                    "File \"main.db\" not found"], 
                                &[  "Try: touch main.db", 
                                    "This command will create an empty database file the program can use"]);
		eprintln!("{}", e);
	}

	// Tests that rusqlite::error::Error is correctyl coerced into a UserError
	#[test]
	fn sqlite_coercion() {
		use std::path::Path;
		use rusqlite::{Connection, OpenFlags};

		fn bad_connection() -> Result<Connection, UserError> {
			let c = Connection::open_with_flags(Path::new("nonexistent.db"), OpenFlags::SQLITE_OPEN_READ_WRITE)?;
			Ok(c)    
		}
		let r = bad_connection();
		assert!(r.is_err());
		let mut r = r.unwrap_err();
		println!("{}", r);
		r.update_and_push_summary("Failed to create project");
		println!("----\n{}", r);
	}
}

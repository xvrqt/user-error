/* Useful functions used by more than one module to keep things DRY */

// Standard Library Dependencies
use std::path::Path;

/// Returns a string saying the application has stopped working.
#[inline]
pub fn default_summary() -> String {
	// Pull the name from the first command line argument
	let name = String::from(std::env::args().next().as_ref()
				.map(|s| Path::new(s))
				.and_then(std::path::Path::file_stem)
				.and_then(std::ffi::OsStr::to_str)
				.unwrap_or("The application"));
	format!("{} encountered an unknown error.", name)
}


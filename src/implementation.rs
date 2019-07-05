#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features, unsafe_code,
        unused_import_braces, unused_qualifications)]

// Third Party Dependencies
use colorful::Color;
use colorful::Colorful;

// Intra Library Imports
use crate::helper;
use crate::UserError;

impl UserError {
	/// Generates an error by consuming heap allocated arguemnts. This method is better if you're dynamically creating error messages. If you're using hardcoded error messages, see UserError::hardcoded below.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	/// let error_summary    = String::from("Failed to build project");
	/// let error_reasons    = vec![String::from("Database could not be parsed"), 
	///								String::from("File \"main.db\" not found")];
	/// let error_subtleties = vec![String::from("Try: touch main.db"), 
	///								String::from("This command will create and empty database file the program can use ")];
	/// let e = UserError::new(error_summary, error_reasons, error_subtleties);
	/// ```
	pub fn new(mut summary: String, reasons: Vec<String>, subtleties: Vec<String>) -> UserError {
		if summary.is_empty() {
			summary = helper::default_summary();
		}
		UserError {
			summary,
			reasons: Some(reasons),
			subtleties: Some(subtleties),
			original_errors: None
		}
	}

	/// Generate an error using hardcoded string references (&str), making it a quick and dirty way to create a pretty printed error message.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	/// let e = UserError::hardcoded("Failed to build project", 
	///								&[	"Database could not be parsed", 
	///									"File \"main.db\" not found"], 
	///								&[	"Try: touch main.db", 
	///									"This command will create and empty database file the program can use "]);
	/// ```
	pub fn hardcoded(summary: &str, reasons: &[&str], subtleties: &[&str]) -> UserError {
		let reasons = reasons.iter().map(|s| String::from(*s)).collect();
		let subtleties = subtleties.iter().map(|s| String::from(*s)).collect();
		UserError {
			summary: String::from(summary),
			reasons: Some(reasons),
			subtleties: Some(subtleties),
			original_errors: None,
		}
	}

	/// Generate an error with only a summary using a hardcoded string reference (&str). A quick and dirty way to create a simple pretty printed error message.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	/// let e = UserError::simple("Failed to build project");
	/// ```
	pub fn simple(summary: &str) -> UserError {
		UserError {
			summary: String::from(summary),
			reasons: None,
			subtleties: None,
			original_errors: None,
		}
	}

	/// Prints the error to stderr
	///
	/// # Exapmle
	/// ```
	/// use user_error::UserError;
	///
	/// let e = UserError::simple("Failed to build project");
	/// e.print();
	/// ```
	/// This results in the following being printed to stderr:
	/// ```bash
	/// Error: Failed to build project
	/// ```
	pub fn print(&self) {
		eprintln!("{}", self);
	}

	/// Prints the error to stderr and terminates the process with exit code 1
	///
	/// # Exapmle
	/// ```should_panic
	/// use user_error::UserError;
	///
	/// let e = UserError::simple("Failed to build project");
	/// e.print_and_exit(); // Program exits here
	/// eprintln!("I will not be printed!");
	/// ```
	/// This results in the following being printed to stderr:
	/// ```bash
	/// Error: Failed to build project
	/// ```
	pub fn print_and_exit(&self) {
		self.print();
		std::process::exit(1);
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
	///
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

	/// Updates the summary of the UserError. Useful for taking UserErrors that were coerced from other Error types and changing the summary before displaying them to the user.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	/// let mut e = UserError::simple("Failed to build project");
	/// e.update_summary("Failed to create database");
	/// eprintln!("{}", e.summary()); // Error: Failed to create database
	/// ```
	pub fn update_summary(&mut self, s: &str) {
		self.summary = String::from(s);
	}

	/// Updates the summary of the UserError and adds the old summary as a reason for the error. Useful for taking UserErrors that were coerced from other Error types, changing the summary but remembering the underlying cause before displaying them to the user.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	/// use std::path::Path;
	///	use rusqlite::{Connection, OpenFlags};
	///
	///	fn bad_connection() -> Result<Connection, UserError> {
	///		let c = Connection::open_with_flags(Path::new("nonexistent.db"), OpenFlags::SQLITE_OPEN_READ_WRITE)?;
	///		Ok(c)    
	///	}
	///	let r = bad_connection();
	///	assert!(r.is_err());
	/// let mut r = r.unwrap_err();
	///	eprintln!("{}", r);
	/// r.update_and_push_summary("Failed to create new project");
	/// eprintln!("-----\n{}", r);
	/// ```
	/// This results in the following being printed to stderr:
	/// ```text
	/// Error: SQLite has encountered an issue
	///	- Underlying SQLite call failed
	///	- unable to open database file
	/// -----
	/// Error: Failed to create new project
	/// - SQLite has encountered an issue
	///	- Underlying SQLite call failed
	///	- unable to open database file
    /// ```
	pub fn update_and_push_summary(&mut self, s: &str) {
		let old_summary = self.summary.clone();
		self.add_reason(&old_summary);
		self.summary = String::from(s);
	}

	/// Returns a formatted, possibly colored String listing the reasons for the error. If the terminal supports color, the bullet point ('-') will be colored yellow. Each String in the Vec<String> will be printed on its own line.
	/// Format: - <reason>
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	/// let e = UserError::hardcoded("Failed to build project", 
	///								&[	"Database could not be parsed", 
	///									"File \"main.db\" not found"], 
	///								&[	"Try: touch main.db", 
	///									"This command will create and empty database file the program can use "]);
	///
	/// eprintln!("{}", e.reasons()); // - Database could not be parsed
	///								  // - File "main.db" not found
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

	/// Modifies the UserError by adding in additional reasons. Additional reasons are inserted into the front of the reasons lists. This allows the call stack to list the reasons in inscreasing specificity. Useful if you're passing this error up the call stack while cleaning up and want each calling function to be able to annotate what went wrong before displaying to the user.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
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
    /// - Failed input #6 with value (-100)
    /// - Failed input #5 with value (-1)
    /// - Failed input #3 with value (-2)
    /// ```
	pub fn add_reason(&mut self, reason: &str) {
		let reason = String::from(reason);
		match &mut self.reasons {
			Some(v) => { v.insert(0, reason); },
			None => {
				self.reasons = Some(vec![reason]);
			}
		}
	}

	/// Modifies the UserError by removing all reasons.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	///	let mut e = UserError::hardcoded("Failed to build project",
    ///									&["Reason #1",
    ///									  "Reason #2",
    ///									  "Reason #3"],
    ///									&["Try again?"]);
    ///	e.clear_reasons();
    ///	eprintln!("{}", e);
	/// ```
    /// This results in the following being printed to stderr:
	/// ```bash
	/// Error: Failed to build project
	/// Try again?
    /// ```
	pub fn clear_reasons(&mut self) {
		self.reasons = None;
	}

	/// Returns a formatted, possibly colored String listing additional subtleties to the error. If the terminal supports color, subtleties will be printed dimly. Each String in the Vec<String> will be printed on its own line.
	/// Format: <subtleties>
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
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
	///
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
	/// 	Err(e) => eprintln!("{}", e),
	///     _ => println!("Project built successfully!"),
	/// }
	/// 
    /// ```
    /// This results in the following being printed to stderr:
	/// ```bash
	/// Error: Failed to build project
	///	- Failed to open file: /user_data.db
	///	Try: touch /user_data.db
	///	You may need to ask your administrator to run this command for you
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

	/// Modifies the UserError by removing all subtly.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	///	let mut e = UserError::hardcoded("Failed to build project",
	///									&["Reasons!"],
    ///									&["Tip #1",
    ///									  "Tip #2",
    ///									  "Tip #3"]);
    ///	e.clear_subtleties();
    ///	eprintln!("{}", e);
	/// ```
    /// This results in the following being printed to stderr:
	/// ```bash
	/// Error: Failed to build project
	/// - Reasons!
    /// ```
	pub fn clear_subtleties(&mut self) {
		self.subtleties = None;
	}

	/// Prints all the other errors (if present) to stderr. Does nothing if there are no other errors.
	///
	/// # Example
	/// ```
	/// use user_error::UserError;
	///
	/// use std::path::Path;
	///	use rusqlite::{Connection, OpenFlags};
	///
	///	fn bad_connection() -> Result<Connection, UserError> {
	///		let c = Connection::open_with_flags(Path::new("nonexistent.db"), OpenFlags::SQLITE_OPEN_READ_WRITE)?;
	///		Ok(c)    
	///	}
	///	let r = bad_connection();
	///	assert!(r.is_err());
	/// let r = r.unwrap_err();
	/// eprintln!("{}\n-----", r);
	///	r.print_other_errors();
	/// ```
	/// This results in the following being printed to stderr:
	/// ```text
	/// Error: SQLite has encountered an issue
	///	- Underlying SQLite call failed
	///	- unable to open database file
	/// -----
	/// Error code 14: Unable to open the database file
    /// ```
	pub fn print_other_errors(& self) {
		if let Some(v) = &self.original_errors {
			for e in v {
				eprintln!("{}", e);
			}
		}
	}
}

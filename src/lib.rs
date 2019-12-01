//! # User Facing Error
//! A library for conveniently displaying well-formatted, and good looking errors to users of CLI applications. Useful for bubbling up unrecoverable errors to inform the user what they can do to fix them. Error messages you'd be proud to show your mom.
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

/* Standard Library Dependencies */
use std::error::Error;
use std::fmt;


/// Marker trait to ensure valid state transitions.
pub trait UFEState {}

/// The eponymous struct.
/// # Example
/// ```
/// use user_error::UserFacingError;
///
/// let err = UserFacingError::new("File failed to open")
///                             .reason("File not found")
///                             .helptext("Make sure foo.txt exists");
/// ```
#[derive(Debug)]
pub struct UserFacingError<S: UFEState> {
    summary: String,
    state: S,
}

// Initial state of our Error builder
#[derive(Debug, Clone, Copy)]
/// Marker traits indicating the start state of the UFE Builder sequence.
pub struct Start;
impl UFEState for Start {}

impl UserFacingError<Start> {
    /// This is how users create a new User Facing Error. The value passed to new() will be used as an error summary. Error summaries are displayed first, prefixed by 'Error: '.
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open");
    /// ```
    pub fn new(s: &str) -> UserFacingError<Start> {
        UserFacingError {
            summary: s.into(),
            state: Start,
        }
    }

    /// Prints the error
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open");
    /// err.print();
    /// ```
    pub fn print(&self) {
        eprintln!("{}", self);
    }

    /// Prints the error and then exits the program
    /// # Example
    /// ```should_panic
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open");
    /// err.print_and_exit();
    pub fn print_and_exit(&self) -> ! {
        eprintln!("{}", self);
        std::process::exit(1)
    }

    /// Add a reason to the UserFacingError. Reasons are displayed in a bulleted list below the summary.
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .reason("Directory cannot be entered");
    /// ```
    pub fn reason(self, r: &str) -> UserFacingError<Reason> {
        UserFacingError {
            summary: self.summary,
            state: Reason {
                reasons: vec![r.into()],
            },
        }
    }

    /// Add help text to the error. Help text is displayed last, in a muted fashion. Once you add help text you cannot add additional reasons.
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .helptext("Check if the file exists.");
    /// ```
    /// ## This will not compile
    /// ```compile_fail
    /// # use user_error::UserFacingError;
    /// let err = UserFacingError::new("File failed to open")
    ///                             .helptext("Check if the file exists.")
    ///                             .reason("File not found");
    /// ```
    pub fn helptext(self, h: &str) -> UserFacingError<HelpText> {
        UserFacingError {
            summary: self.summary,
            state: HelpText {
                help_text: h.into(),
            },
        }
    }
}

static SUMMARY_PREFIX: &str = "\u{001b}[41;1mError:\u{001b}[0m";

impl fmt::Display for UserFacingError<Start> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", SUMMARY_PREFIX, self.summary)
    }
}

impl Error for UserFacingError<Start> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// A User Facing Error that has help text but no reasons
#[derive(Debug)]
/// Marker type for valid state transitions. Holds the helptext String.
pub struct HelpText {
    help_text: String,
}
impl UFEState for HelpText {}

impl UserFacingError<HelpText> {
    /// Prints the error
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .helptext("Check that it exists");
    /// err.print();
    /// ```
    pub fn print(&self) {
        eprintln!("{}", self);
    }

    /// Prints the error and then exits the program
    /// # Example
    /// ```should_panic
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .helptext("Check that it exists");
    /// err.print_and_exit();
    pub fn print_and_exit(&self) -> ! {
        eprintln!("{}", self);
        std::process::exit(1)
    }
}

impl fmt::Display for UserFacingError<HelpText> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}\n\u{001b}[37m{}\u{001b}[0m",
            SUMMARY_PREFIX, self.summary, self.state.help_text
        )
    }
}

impl Error for UserFacingError<HelpText> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// A User Facing Error that has reasons for the error
#[derive(Debug)]
/// Marker type for valid state transitions. Holds a Vec<String> of reasons that the error occurred.
pub struct Reason {
    reasons: Vec<String>,
}
impl UFEState for Reason {}

impl UserFacingError<Reason> {
    /// Allows the creation of a UserFacingError from a Error. Checks the source() of the error and will list underlying errors as reasons for the error. You may add additional reasons and helptext.
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let ioe = std::io::Error::new(std::io::ErrorKind::Other, "Error");
    /// let ufe = UserFacingError::from(&ioe);    let ioe = std::io::Error::new(std::io::ErrorKind::Other, "Error");          
    /// let ufe = ufe.reason("No good reason");
    /// ```
    pub fn from(error: &dyn Error) -> Self {
        /* Check for additional errors from source() */
        let mut reasons = Vec::new();
        let mut e = error.source();
        while let Some(err) = e {
            reasons.push(err.to_string());
            e = err.source();
        }
        reasons.reverse();

        UserFacingError {
            summary: error.to_string(),
            state: Reason { reasons },
        }
    }

    /// Prints the error
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found");
    /// err.print();
    /// ```
    pub fn print(&self) {
        eprintln!("{}", self);
    }

    /// Prints the error and then exits the program
    /// # Example
    /// ```should_panic
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found");
    /// err.print_and_exit();
    pub fn print_and_exit(&self) -> ! {
        eprintln!("{}", self);
        std::process::exit(1)
    }

    /// Add a reason to the UserFacingError. Reasons are displayed in a bulleted list below the summary.
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .reason("Directory cannot be entered");
    /// ```
    pub fn reason(mut self, r: &str) -> Self {
        self.state.reasons.push(r.into());
        self
    }

    /// Add help text to the error. Help text is displayed last, in a muted fashion. Once you add help text you cannot add additional reasons.
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .helptext("Check if the file exists.");
    /// ```
    /// ## This will not compile
    /// ```compile_fail
    /// # use user_error::UserFacingError;
    /// let err = UserFacingError::new("File failed to open")
    ///                             .helptext("Check if the file exists.")
    ///                             .reason("File not found");
    /// ```
    pub fn helptext(self, h: &str) -> UserFacingError<ReasonsAndHelp> {
        UserFacingError {
            summary: self.summary,
            state: ReasonsAndHelp {
                reasons: self.state.reasons,
                help_text: h.into(),
            },
        }
    }
}

/* Helper function to keep things DRY */
fn format_reasons(reasons: &[String]) -> String {
    reasons.join("\u{001b}[0m\n\u{001b}[33;1m - \u{001b}[37;1m")
}

impl fmt::Display for UserFacingError<Reason> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* Reasons should always be greater than 0 unless converting from
           another Error type. Then they maybe not be present.
        */
        if !self.state.reasons.is_empty() {
            let reasons = format_reasons(&self.state.reasons);
            write!(
                f,
                "{} {}\n\u{001b}[33;1m - \u{001b}[37;1m{}\n",
                SUMMARY_PREFIX, self.summary, reasons
            )
        } else {
            write!(f, "{} {}", SUMMARY_PREFIX, self.summary)
        }
    }
}

impl Error for UserFacingError<Reason> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// A User Facing Error that has subtleties
#[derive(Debug)]
/// Marker type for valid state transitions. Holds both a Vec<String> of reasons the error happened and a String of helptext.
pub struct ReasonsAndHelp {
    reasons: Vec<String>,
    help_text: String,
}
impl UFEState for ReasonsAndHelp {}

impl UserFacingError<ReasonsAndHelp> {
    /// Prints the error
    /// # Example
    /// ```
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .helptext("Make sure the file exists");
    /// err.print();
    /// ```
    pub fn print(&self) {
        eprintln!("{}", self);
    }

    /// Prints the error and then exits the program
    /// # Example
    /// ```should_panic
    /// use user_error::UserFacingError;
    ///
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .helptext("Make sure the file exists");
    /// err.print_and_exit();
    pub fn print_and_exit(&self) -> ! {
        eprintln!("{}", self);
        std::process::exit(1)
    }
}

impl fmt::Display for UserFacingError<ReasonsAndHelp> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reasons = format_reasons(&self.state.reasons);

        write!(
            f,
            "{} {}\n\u{001b}[33;1m - \u{001b}[37;1m{}\n\u{001b}[37m{}\u{001b}[0m",
            SUMMARY_PREFIX, self.summary, reasons, self.state.help_text
        )
    }
}

impl Error for UserFacingError<ReasonsAndHelp> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /* Statics to keep the testing DRY/cleaner */
    static s: &'static str = "Test Error";
    static r: &'static str = "Reason 1";
    static h: &'static str = "Try Again";

    #[test]
    fn new_test() {
        eprintln!("{}", UserFacingError::new("Test Error"));
    }

    #[test]
    fn summary_test() {
        let e = UserFacingError::new(s);
        let expected = [SUMMARY_PREFIX, " ", s].concat();
        assert_eq!(e.to_string(), String::from(expected));
        eprintln!("{}", e);
    }

    #[test]
    fn helptext_test() {
        let e = UserFacingError::new(s).helptext(h);
        let expected = format!("{} {}\n\u{001b}[37m{}\u{001b}[0m", SUMMARY_PREFIX, s, h);
        assert_eq!(e.to_string(), String::from(expected));
        eprintln!("{}", e);
    }

    #[test]
    fn reason_test() {
        let e = UserFacingError::new(s).reason(r);
        let reasons = format_reasons(&vec![String::from(r)]);
        let expected = format!(
            "{} {}\n\u{001b}[33;1m - \u{001b}[37;1m{}\n",
            SUMMARY_PREFIX, s, reasons
        );
        assert_eq!(e.to_string(), String::from(expected));
        eprintln!("{}", e);
    }

    #[test]
    fn reasons_test() {
        let rr = "Reason 2";
        let e = UserFacingError::new(s).reason(r).reason(rr);
        let reasons = format_reasons(&vec![r.into(), rr.into()]);
        let expected = format!(
            "{} {}\n\u{001b}[33;1m - \u{001b}[37;1m{}\n",
            SUMMARY_PREFIX, s, reasons
        );
        assert_eq!(e.to_string(), String::from(expected));
        eprintln!("{}", e);
    }

    #[test]
    fn reason_and_helptext_test() {
        let e = UserFacingError::new(s).reason(r).helptext(h);
        let reasons = format_reasons(&vec![r.into()]);
        let expected = format!(
            "{} {}\n\u{001b}[33;1m - \u{001b}[37;1m{}\n\u{001b}[37m{}\u{001b}[0m",
            SUMMARY_PREFIX, s, reasons, h
        );
        assert_eq!(e.to_string(), String::from(expected));
        eprintln!("{}", e);
    }
}

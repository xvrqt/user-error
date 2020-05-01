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
use core::fmt::{self, Debug, Display};
use std::error::Error;

/*************
 * CONSTANTS *
 *************/

// 'Error:' with a red background and white, bold, text
const SUMMARY_PREFIX: &str = "\u{001b}[97;41;22mError:\u{001b}[91;49;1m ";
// ' - ' bullet point in yellow and text in bold white
const REASON_PREFIX: &str = "\u{001b}[93;49;1m - \u{001b}[97;49;1m";
// Muted white helptext
const HELPTEXT_PREFIX: &str = "\u{001b}[37;49;2m";
// ASCII Reset formatting escape code
const RESET: &str = "\u{001b}[0m";

// Helper function to keep things DRY
// Takes a dyn Error.source() and returns a Vec of Strings representing all the .sources() in the error chain (if any)
fn error_sources(mut source: Option<&(dyn Error + 'static)>) -> Option<Vec<String>> {
    /* Check if we have any sources to derive reasons from */
    if source.is_some() {
        /* Add all the error sources to a list of reasons for the error */
        let mut reasons = Vec::new();
        while let Some(error) = source {
            reasons.push(error.to_string());
            source = error.source();
        }
        Some(reasons)
    } else {
        None
    }
}

/*********
 * TRAIT *
 *********/

/// Convenience function that converts the summary into pretty String.
fn pretty_summary(summary: &str) -> String {
    [SUMMARY_PREFIX, summary, RESET].concat()
}

/// Convenience function that converts the reasons into pretty String.
fn pretty_reasons(reasons: Reasons) -> Option<String> {
    /* Print list of Reasons (if any) */
    if let Some(reasons) = reasons {
        /* Vector to store the intermediate bullet point strings */
        let mut reason_strings = Vec::with_capacity(reasons.len());
        for reason in reasons {
            let bullet_point = [REASON_PREFIX, &reason].concat();
            reason_strings.push(bullet_point);
        }
        /* Join the buller points with a newline, append a RESET ASCII escape code to the end */
        Some([&reason_strings.join("\n"), RESET].concat())
    } else {
        None
    }
}

/// Convenience function that converts the help text into pretty String.
fn pretty_helptext(helptext: Helptext) -> Option<String> {
    if let Some(helptext) = helptext {
        Some([HELPTEXT_PREFIX, &helptext, RESET].concat())
    } else {
        None
    }
}

/// You can implement UFE on your error types pretty print them. The default implementation will print Error: <your error .to_string()> followed by a list of reasons that are any errors returned by .source()
/// You should only override the summary, reasons and helptext functions. The pretty print versions of these are used by print(), print_and_exit() and contain the formatting. If you wish to change the formatting you should update it with the formatting functions.
pub trait UFE: Error {
    /**************
     * IMPLENT ME *
     **************/

    /// Returns a summary of the error. This will be printed in red, prefixed by "Error: ", at the top of the error message.
    fn summary(&self) -> String {
        self.to_string()
    }

    /// Returns a vector of Strings that will be listed as bullet points below the summary. By default, lists any errors returned by .source() recursively.
    fn reasons(&self) -> Option<Vec<String>> {
        /* Helper function to keep things DRY */
        error_sources(self.source())
    }

    /// Returns help text that is listed below the reasons in a muted fashion. Useful for additional details, or suggested next steps.
    fn helptext(&self) -> Option<String> {
        None
    }

    /**********
     * USE ME *
     **********/

    /// Prints the formatted error.
    /// # Example
    /// ```
    /// use user_error::{UserFacingError, UFE};
    /// UserFacingError::new("File failed to open")
    ///         .reason("File not found")
    ///         .help("Try: touch file.txt")
    ///         .print();
    /// ```
    fn print(&self) {
        /* Print Summary */
        eprintln!("{}", pretty_summary(&self.summary()));

        /* Print list of Reasons (if any) */
        if let Some(reasons) = pretty_reasons(self.reasons()) {
            eprintln!("{}", reasons);
        }

        /* Print help text (if any) */
        if let Some(helptext) = pretty_helptext(self.helptext()) {
            eprintln!("{}", helptext);
        }
    }

    /// Convenience function that pretty prints the error and exits the program.
    /// # Example
    /// ```should_panic
    /// use user_error::{UserFacingError, UFE};
    /// UserFacingError::new("File failed to open")
    ///         .reason("File not found")
    ///         .help("Try: touch file.txt")
    ///         .print_and_exit();
    /// ```
    fn print_and_exit(&self) {
        self.print();
        std::process::exit(1)
    }
}

/**********
 * STRUCT *
 **********/
type Summary = String;
type Reasons = Option<Vec<String>>;
type Helptext = Option<String>;
type Source = Option<Box<(dyn Error)>>;

/// The eponymous struct. You can create a new one from using user_error::UserFacingError::new()
/// I recommend you use your own error types and have them implement UFE instead of useing UserFacingError directly. This is more of an example type, or a way to construct a pretty message.
#[derive(Debug)]
pub struct UserFacingError {
    summary: Summary,
    reasons: Reasons,
    helptext: Helptext,
    source: Source,
}

/******************
 * IMPLEMENTATION *
 ******************/

// Implement our own trait for our example struct
// Cloning is not super efficient but this should be the last thing a program does, and it will only do it once so... ¯\_(ツ)_/¯
impl UFE for UserFacingError {
    fn summary(&self) -> Summary {
        self.summary.clone()
    }
    fn reasons(&self) -> Reasons {
        self.reasons.clone()
    }
    fn helptext(&self) -> Helptext {
        self.helptext.clone()
    }
}

// Implement Display so our struct also implements std::error::Error
impl Display for UserFacingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let summary = pretty_summary(&self.summary());
        let reasons = pretty_reasons(self.reasons());
        let helptext = pretty_helptext(self.helptext());

        /* Love this - thanks Rust! */
        match (summary, reasons, helptext) {
            (summary, None, None) => writeln!(f, "{}", summary),
            (summary, Some(reasons), None) => writeln!(f, "{}\n{}", summary, reasons),
            (summary, None, Some(helptext)) => writeln!(f, "{}\n{}", summary, helptext),
            (summary, Some(reasons), Some(helptext)) => {
                writeln!(f, "{}\n{}\n{}", summary, reasons, helptext)
            }
        }
    }
}

// Implement std::error::Error
impl Error for UserFacingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self.source {
            Some(_) => self.source.as_deref(),
            None => None,
        }
    }
}

// Helper function to keep things DRY
fn get_ufe_struct_members(error: &(dyn Error)) -> (Summary, Reasons) {
    /* Error Display format is the summary */
    let summary = error.to_string();
    /* Form the reasons from the error source chain */
    let reasons = error_sources(error.source());
    (summary, reasons)
}

//
// Instead of converting from std Error, convert from types that implement UFE :)
// Don't have my type implement std Error
//

/// Allows you to create UserFacingErrors From std::io::Error for convenience
/// You should really just implement UFE for your error type, but if you wanted to convert before quitting so you could add helptext of something you can use this.
impl From<std::io::Error> for UserFacingError {
    fn from(error: std::io::Error) -> UserFacingError {
        let (summary, reasons) = get_ufe_struct_members(&error);

        UserFacingError {
            summary,
            reasons,
            helptext: None,
            source: Some(Box::new(error)),
        }
    }
}

/// Allows you to create UserFacingErrors From std Errors.
/// You should really just implement UFE for your error type, but if you wanted to convert before quitting so you could add helptext of something you can use this.
impl From<Box<(dyn Error)>> for UserFacingError {
    fn from(error: Box<(dyn Error)>) -> UserFacingError {
        let (summary, reasons) = get_ufe_struct_members(error.as_ref());

        UserFacingError {
            summary,
            reasons,
            helptext: None,
            source: Some(error),
        }
    }
}

/// Allows you to create UserFacingErrors From std Errors.
/// You should really just implement UFE for your error type, but if you wanted to convert before quitting so you could add helptext or something you can use this.
impl From<&(dyn Error)> for UserFacingError {
    fn from(error: &(dyn Error)) -> UserFacingError {
        let (summary, reasons) = get_ufe_struct_members(error);

        UserFacingError {
            summary,
            reasons,
            helptext: None,
            source: None,
        }
    }
}

/// Allows you to create UserFacingErrors From std Errors wrapped in a Result
/// You should really just implement UFE for your error type, but if you wanted to convert before quitting so you could add helptext or something you can use this.
impl<T: Debug> From<Result<T, Box<dyn Error>>> for UserFacingError {
    fn from(error: Result<T, Box<dyn Error>>) -> UserFacingError {
        /* Panics if you try to convert an Ok() Result to a UserFacingError */
        let error = error.unwrap_err();
        let (summary, reasons) = get_ufe_struct_members(error.as_ref());

        UserFacingError {
            summary,
            reasons,
            helptext: None,
            source: Some(error),
        }
    }
}

// Implement convenience functions to modify the UserFacingError struct
impl UserFacingError {
    /// This is how users create a new User Facing Error. The value passed to new() will be used as an error summary. Error summaries are displayed first, prefixed by 'Error: '.
    /// # Example
    /// ```
    /// # use user_error::UserFacingError;
    /// let err = UserFacingError::new("File failed to open");
    /// ```
    pub fn new(summary: &str) -> UserFacingError {
        UserFacingError {
            summary: summary.to_string(),
            reasons: None,
            helptext: None,
            source: None,
        }
    }

    /// Replace the error summary.
    /// # Example
    /// ```
    /// # use user_error::UserFacingError;
    /// let mut err = UserFacingError::new("File failed to open");
    /// err.update("Failed Task");
    /// ```
    pub fn update(&mut self, summary: &str) {
        self.summary = summary.into();
    }

    /// Replace the error summary and add the previous error summary to the list of reasons
    /// # Example
    /// ```
    /// # use user_error::UserFacingError;
    /// let mut err = UserFacingError::new("File failed to open");
    /// err.push("Failed Task");
    /// ```
    pub fn push(&mut self, new_summary: &str) {
        // Add the old summary to the list of reasons
        let old_summary = self.summary();
        match self.reasons.as_mut() {
            Some(reasons) => reasons.insert(0, old_summary),
            None => self.reasons = Some(vec![old_summary.into()]),
        }

        // Update the summary
        self.summary = new_summary.to_string();
    }

    /// Add a reason to the UserFacingError. Reasons are displayed in a bulleted list below the summary, in the reverse order they were added.
    /// # Example
    /// ```
    /// # use user_error::UserFacingError;
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .reason("Directory cannot be entered");
    /// ```
    pub fn reason(mut self, reason: &str) -> UserFacingError {
        self.reasons = match self.reasons {
            Some(mut reasons) => {
                reasons.push(reason.into());
                Some(reasons)
            }
            None => Some(vec![reason.into()]),
        };
        self
    }

    // Return ref to previous?

    /// Clears all reasons from a UserFacingError.
    /// # Example
    /// ```
    /// # use user_error::UserFacingError;
    /// let mut err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .reason("Directory cannot be entered");
    /// err.clear_reasons();
    /// ```
    pub fn clear_reasons(&mut self) {
        self.reasons = None;
    }

    /// Add help text to the error. Help text is displayed last, in a muted fashion.
    /// # Example
    /// ```
    /// # use user_error::UserFacingError;
    /// let err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .help("Check if the file exists.");
    /// ```
    pub fn help(mut self, helptext: &str) -> UserFacingError {
        self.helptext = Some(helptext.into());
        self
    }

    /// Clears all the helptext from a UserFacingError.
    /// # Example
    /// ```
    /// # use user_error::UserFacingError;
    /// let mut err = UserFacingError::new("File failed to open")
    ///                             .reason("File not found")
    ///                             .reason("Directory cannot be entered")
    ///                             .help("Check if the file exists.");
    /// err.clear_helptext();
    /// ```
    pub fn clear_helptext(&mut self) {
        self.helptext = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /* Statics to keep the testing DRY/cleaner */
    static S: &'static str = "Test Error";
    static R: &'static str = "Reason 1";
    static H: &'static str = "Try Again";

    #[test]
    fn new_test() {
        eprintln!("{}", UserFacingError::new("Test Error"));
    }

    #[test]
    fn summary_test() {
        let e = UserFacingError::new(S);
        let expected = [SUMMARY_PREFIX, S, RESET, "\n"].concat();
        assert_eq!(e.to_string(), String::from(expected));
        eprintln!("{}", e);
    }

    #[test]
    fn helptext_test() {
        let e = UserFacingError::new(S).help(H);
        let expected = format!(
            "{}{}{}\n{}{}{}\n",
            SUMMARY_PREFIX, S, RESET, HELPTEXT_PREFIX, H, RESET
        );
        assert_eq!(e.to_string(), expected);
        eprintln!("{}", e);
    }

    #[test]
    fn reason_test() {
        let e = UserFacingError::new(S).reason(R).reason(R);

        /* Create Reasons String */
        let reasons = vec![String::from(R), String::from(R)];
        let mut reason_strings = Vec::with_capacity(reasons.len());
        for reason in reasons {
            let bullet_point = [REASON_PREFIX, &reason].concat();
            reason_strings.push(bullet_point);
        }
        /* Join the buller points with a newline, append a RESET ASCII escape code to the end */
        let reasons = [&reason_strings.join("\n"), RESET].concat();

        let expected = format!("{}{}{}\n{}\n", SUMMARY_PREFIX, S, RESET, reasons);
        assert_eq!(e.to_string(), expected);
        eprintln!("{}", e);
    }

    #[test]
    fn push_test() {
        let mut e = UserFacingError::new(S).reason("R1");
        e.push("R2");

        /* Create Reasons String */
        let reasons = vec![String::from(S), String::from("R1")];
        let mut reason_strings = Vec::with_capacity(reasons.len());
        for reason in reasons {
            let bullet_point = [REASON_PREFIX, &reason].concat();
            reason_strings.push(bullet_point);
        }
        /* Join the buller points with a newline, append a RESET ASCII escape code to the end */
        let reasons = [&reason_strings.join("\n"), RESET].concat();

        let expected = format!("{}{}{}\n{}\n", SUMMARY_PREFIX, "R2", RESET, reasons);
        assert_eq!(e.to_string(), expected);
        eprintln!("{}", e);
    }

    #[test]
    fn push_test_empty() {
        let mut e = UserFacingError::new(S);
        e.push("S2");

        /* Create Reasons String */
        let reasons = vec![String::from(S)];
        let mut reason_strings = Vec::with_capacity(reasons.len());
        for reason in reasons {
            let bullet_point = [REASON_PREFIX, &reason].concat();
            reason_strings.push(bullet_point);
        }
        /* Join the buller points with a newline, append a RESET ASCII escape code to the end */
        let reasons = [&reason_strings.join("\n"), RESET].concat();

        let expected = format!("{}{}{}\n{}\n", SUMMARY_PREFIX, "S2", RESET, reasons);
        assert_eq!(e.to_string(), expected);
        eprintln!("{}", e);
    }

    #[test]
    fn reason_and_helptext_test() {
        let e = UserFacingError::new(S).reason(R).reason(R).help(H);

        /* Create Reasons String */
        let reasons = vec![String::from(R), String::from(R)];
        let mut reason_strings = Vec::with_capacity(reasons.len());
        for reason in reasons {
            let bullet_point = [REASON_PREFIX, &reason].concat();
            reason_strings.push(bullet_point);
        }
        /* Join the buller points with a newline, append a RESET ASCII escape code to the end */
        let reasons = [&reason_strings.join("\n"), RESET].concat();

        let expected = format!(
            "{}{}{}\n{}\n{}{}{}\n",
            SUMMARY_PREFIX, S, RESET, reasons, HELPTEXT_PREFIX, H, RESET
        );
        assert_eq!(e.to_string(), expected);
        eprintln!("{}", e);
    }

    #[test]
    fn from_error_test() {
        let error_text = "Error";
        let ioe = std::io::Error::new(std::io::ErrorKind::Other, error_text);

        /* Lose the type */
        fn de(ioe: std::io::Error) -> Box<dyn Error> {
            Box::new(ioe)
        }
        /* Convert to UFE */
        let ufe: UserFacingError = de(ioe).into();

        let expected = [SUMMARY_PREFIX, error_text, RESET, "\n"].concat();
        assert_eq!(ufe.to_string(), expected);
    }

    #[test]
    fn from_error_source_test() {
        let ufe: UserFacingError = get_super_error().into();
        let expected = [
            SUMMARY_PREFIX,
            "SuperError",
            RESET,
            "\n",
            REASON_PREFIX,
            "Sidekick",
            RESET,
            "\n",
        ]
        .concat();

        assert_eq!(ufe.to_string(), expected);
    }

    /* Used for to test that source is working correctly */
    #[derive(Debug)]
    struct SuperError {
        side: SuperErrorSideKick,
    }

    impl Display for SuperError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "SuperError")
        }
    }

    impl Error for SuperError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.side)
        }
    }

    #[derive(Debug)]
    struct SuperErrorSideKick;

    impl Display for SuperErrorSideKick {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Sidekick")
        }
    }

    impl Error for SuperErrorSideKick {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

    fn get_super_error() -> Result<(), Box<dyn Error>> {
        Err(Box::new(SuperError {
            side: SuperErrorSideKick,
        }))
    }
}

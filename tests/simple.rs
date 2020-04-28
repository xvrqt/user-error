/* Duh */
use std::fmt::{self, Display};
use user_error::{UserFacingError, UFE};

/* Standard Library */
use std::error::Error;

#[test]
fn simple_constructor_test() {
    let _ufe = UserFacingError::new("Too gay to live");
}

#[test]
fn complex_builder_test() {
    let _ufe = UserFacingError::new("Too cool for cats")
        .reason("Neato shades")
        .reason("Fashionable jacket")
        .help("There is no help coming");
}

#[test]
fn to_error_coercion_test() {
    fn returns_err() -> Result<(), Box<dyn Error>> {
        Err(Box::new(UserFacingError::new("Error")))
    }

    match returns_err() {
        Ok(_) => panic!(),
        Err(e) => eprintln!("{}", e),
    }
}

// Dummy Error type to ensure that we can implement UFE on it
#[derive(Debug)]
struct MyError;

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError")
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl UFE for MyError {}

#[test]
fn custom_error_implements_ufe() {
    let me = MyError {};
    me.summary();
    me.reasons();
    me.helptext();
    me.pretty_summary();
    me.pretty_reasons();
    me.pretty_helptext();
    me.print();
}

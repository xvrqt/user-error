/* Duh */
use user_error::UserFacingError;

/* Standard Library */
use std::error::Error;
use std::fmt;
use std::io;

#[test]
fn simple_constructor_test() {
    let ufe = UserFacingError::new("Too gay to live");
}

#[test]
fn complex_builder_test() {
    let ue = UserFacingError::new("Too cool for cats")
        .reason("Neato shades")
        .reason("Fashionable jacket")
        .helptext("There is no help coming");
}

#[test]
fn to_error_coercion_test() {
    fn returns_err() -> Result<(), Box<Error>> {
        Err(Box::new(UserFacingError::new("Error")))
    }

    match returns_err() {
        Ok(_) => panic!(),
        Err(e) => eprintln!("{}", e),
    }
}

#[test]
fn from_error_test() {
    let ioe = std::io::Error::new(std::io::ErrorKind::Other, "Error");
    let ufe = UserFacingError::from(&ioe);

    let expected = "\u{001b}[41;1mError:\u{001b}[0m Error";

    assert_eq!(ufe.to_string(), expected);
}

#[test]
fn from_error_source_test() {
    let err = get_super_error().unwrap_err();
    let ufe = UserFacingError::from(&err);

    let expected =
        "\u{001b}[41;1mError:\u{001b}[0m SuperError\n\u{001b}[33;1m - \u{001b}[37;1mSidekick\n";

    assert_eq!(ufe.to_string(), expected);
}

/* Used for to test that source is working correctly */
#[derive(Debug)]
struct SuperError {
    side: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
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

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sidekick")
    }
}

impl Error for SuperErrorSideKick {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError {
        side: SuperErrorSideKick,
    })
}

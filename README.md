# UserFacingError

![build-status-shield](https://img.shields.io/github/workflow/status/xvrqt/user-error/Rust)
[![github-issues-open-shield](https://img.shields.io/github/issues-raw/xvrqt/user-error)](https://github.com/xvrqt/user-error/issues)
[![crates-io-version-shield](https://img.shields.io/crates/v/user-error)](https://crates.io/crates/user-error)
[![crates-io-downloads-shield](https://img.shields.io/crates/d/user-error)](https://crates.io/crates/user-error)
[![license-shield](https://img.shields.io/github/license/xvrqt/user-error)](https://github.com/xvrqt/user-error/blob/master/LICENSE.txt)
-
[![discord-status-shield](https://img.shields.io/discord/524687904522371072)](https://discord.xvrqt.com)
[![twitter-shield](https://img.shields.io/twitter/follow/xvrqt?label=%40xvrqt&style=social)](https://twitter.com/xvrqt)

Pretty printed errors for your CLI application.

This repository contains:

1. A new trait, **UFE**, that you can implement on your Error types to pretty print them
2. A new type, UserFacingError, that you can use to construct pretty CLI error messages
3. Handy macros to implemnt UFE on your type, and to convert `Box<dyn Error>` to UserFacing Errors

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
    - [UFE Trait](#ufe-trait)
        - [Default Implementations](#default-implementations)
            - [Summary](#summary)
            - [Reasons](#reasons)
            - [Helptext](#helptext)
        - [Other Methods](#other-methods)
            - [Print](#print)
            - [Print & Exit](#print-&-exit)
    - [UserFacingError](#userfacingerror)
        - [Construction](#constructing)
            - [Builder Pattern](#builder-pattern)
            - [From Other Errors](#from-other-error-types)
        -[Methods](#methods)
            - [Update](#update)
            - [Push](#push)
            - [Clear Reasons](#clear-reasons)
            - [Clear Help Text](#clear-help-text)
- [Related Efforts](#related-efforts)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Background

UserFacingError is intended to print errors to users of command line applications in a sensible, pretty format.
I love Rust's Result types, and using enums for matching and &str for errors and matching is great for development but less great for end users of CLI applications. For this I made a `UserFacingError` which can be used to quickly construct a pretty error suitable to help users what went wrong and what they can do about it.

## Install

This project uses [node](http://nodejs.org) and [npm](https://npmjs.com). Go check them out if you don't have them locally installed.

Add the following to your Cargo.toml:
```yaml
[dependencies]
user-error = "1.2.4"
```

## Usage

### UFE Trait

You can trivially implement the UFE trait on your custom error types, allowing you to pretty print them to stderr. The UFE trait requires your trait also implement Error.

```rust
#[derive(Debug)]
struct MyError {}

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

// Trivial implementation
impl UFE for MyError {}
```

#### Default Implementations
There are three functions you may optionally override:

1. `.summary() -> String` - returns a string to be used as the error summary
2. `.reasons() -> Option<Vec<String>>` - optionally return a Vec of Strings representing the cause of the error
3. `.helptext() -> Option<String>` - optionally return a String representing follow up advice on how to resolve the error

##### Summary

By default, the error summary is the String provided by calling `my_error_type.to_string()` prefixed by "Error: ". 

##### Reasons

By default the list of reasons is created by recursively calling `.source()` and prefixing each reason with a bullet point.

##### Helptext

By default no helptext is added to type that implement UFE. You'll have to provide an implementation yourself. It is probably easier to convert your error type to a UserFacingError and use the `.help()` function provided by that type if you want to add help text before printing.

```rust
use user_error::{UserFacingError, UFE};

// Custom Error Type
#[derive(Debug)]
struct MyError { mssg: String, src: Option<MyError> }

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, mssg.to_string())
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.src.as_ref()
    }
}

impl UFE for MyError {}

#[test]
fn custom_error_implements_ufe() {
    let me = MyError {
        mssg: "Program Failed".into(),
        src: Some(MyError {
            mssg: "Reason 1".into(),
            sub: Some(MyError {
                mssg: "Reason 2".into(),
                sub: None,
            }),
        },
    };
    me.print();
}
```

This prints:
```text
Error: Program Failed
- Reason 1
- Reason 2
```

#### Other Methods

UFE provides two additional methods:

1. `.print()` - Pretty print the error
2. `.print_and_exit()` - Pretty print the error and terminate the process

You could override these methods but then there is not much point in using this crate :p

This prints:
```text
Error: Program Failed!
 - Bad luck
```

#### Print
Pretty prints the UserFacingError to stderr

```rust
use user_error::UserFacingError;

fn main() {
    UserFacingError::new("Failed to build project")
        .reason("Database could not be parsed")
        .print_and_exit();
}
```

This prints:
```text
Error: Failed to build project
 - Database could not be parsed
```

#### Print & Exit
Since constructing this error is likely the last thing your program will do, you can also call `.print_and_exit()` to print the error and then terminate the process with status code 1.

```rust
use user_error::UserFacingError;

fn main() {
    UserFacingError::new("Failed to build project")
        .reason("Database could not be parsed")
        .print_and_exit();
}
```

This prints:
```text
Error: Failed to build project
 - Database could not be parsed
```

### UserFacingError

UserFacingError is an error type that helps you format and print good looking error messages for users of your CLI application. These errors are intended for consumption by a human, not your program. They are divided into 3 parts: summary, reasons and help text.

**Summary:** A String representing a one-line description of your error. A summary is mandatory and is printed boldly in red.

**Reasons:** A vector of Strings explaining in more detail _why_ this error occured. Reasons are optional and if the terminal supports color, the bullet point ('-') will be colored yellow. Each reason will be printed on its own line.

**Help Text:** A String explaining additional information, including what the user can do about the error, or where to file a bug. Help text is optional and if the terminal supports color it will be printed _dimly_.

#### Construction

There are two ways to create a new UserFacingError:

1. Using a builder pattern
2. From other std Errors

##### Builder Pattern

```rust
use user_error::UserFacingError;

fn main() {
    UserFacingError::new("Failed to build project") 
        .reason("Database could not be parsed")
        .reason("File \"main.db\" not found") 
        .help("Try: touch main.db")
        .print()
}
```

This prints:
```text
Error: Failed to build project
- Database could not be parsed
- File "main.db" not found
Try: touch main.db
```

If the user has colors enabled on their terminal, it may look something like this:
![Quickstart example of user-error library for Rust](https://xvrqt.sfo2.cdn.digitaloceanspaces.com/image-cache/user-error-output.png)

##### From Other Error Types
You can also create a UserFacingError from other types that implement std::error::Error. 

The summary will be the result of error.to_string() and the list of reasons will be any errors in the error chain constructed by recursively calling .source()

```rust
use user_error::UserFacingError;
use std::io::(Error, ErrorKind);

fn main() {
    /* Lose the type */
    fn dyn_error() -> Box<dyn Error> {
        let ioe = Error::new(ErrorKind::Other, "MyError");
        Box::new(ioe)
    }

    /* Convert to UFE */
    let ufe: UserFacingError = dyn_error().into();
}
```
#### Methods

UserFacingErrors have 6 non-builder methods:

1. `.update(&str)` - Change the error summary
2. `.push(&str)` - Change the error summary, add the previous summary to the list of reasons
3. `.clear_reasons()` - Remove all reasons
4. `.clear_help()` - Remove the helptext
5. `.print()` - Pretty print the error (uses the default UFE implementation)
6. `.print_and_exit()` - Pretty print the error and terminate the process (uses the default UFE implementation)

##### Update

You can call `.update(&str)` on a UserFacingError to change the error summary.

```rust
use user_error::UserFacingError;

fn do_thing() -> Result<(), UserFacingError> {
    Err(UserFacingError::new("Didn't do the thing!")
        .reason("Just didn't happen"))
}

fn main() {
    match do_thing() {
        Ok(_) => println!("Success!"),
        Err(E) => {
            e.update("Program Failed!").print()
        }
    }
}
```

This prints:
```text
Error: Program Failed!
 - Just didn't happen
```

##### Push

You can call `.push(&str)` on a UserFacingError to change the error summary and add the old error summary to the list of reasons. It adds the summary to the front of the list of reasons.

```rust
use user_error::UserFacingError;

fn do_thing() -> Result<(), UserFacingError> {
    Err(UserFacingError::new("Didn't do the thing!")
        .reason("Just didn't happen"))
}

fn main() {
    match do_thing() {
        Ok(_) => println!("Success!"),
        Err(E) => {
            e.update("Program Failed!").print()
        }
    }
}
```

This prints:
```text
Error: Program Failed!
 - Didn't do the thing!
 - Just didn't happen
```

##### Clear Reasons

Calling this removes all reasons from a UserFacingError.

```rust
use user_error::UserFacingError;

fn main() {
    let ufe = UserFacingError::new("Program Failed!")
                .reason("Program internal error message");
    /* --- */

    ufe.clear_reasons();
    ufe.print_and_exit();
}
```

This prints:
```text
Error: Program Failed!
```

##### Clear Help Text

Calling this removes the help text from a UserFacingError.

```rust
use user_error::UserFacingError;

fn main() {
    let ufe = UserFacingError::new("Program Failed!")
                .reason("Bad luck")
                .help("Try running it again?");
    /* --- */

    ufe.clear_help();
    ufe.print_and_exit();
}
```

This prints:
```text
Error: Program Failed!
 - Bad luck
```

## Related Efforts

- Need to look up related human readable error crates. If you have a recommendation, open a PR :D

## Maintainers

- Amy Jie [@xvrqt](https://twitter.com/xvrqt)

## Contributing

Feel free to dive in! [Open an issue](https://github.com/xvrqt/user-error/issues/new) or submit PRs.

### Contributors

- Amy Jie [@xvrqt](https://twitter.com/xvrqt)

## License

[MIT](LICENSE) © Amy Jie

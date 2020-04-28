# UserFacingError
UserError is an error type that helps you format and print good looking error messages for users of your CLI application. These errors are intended for consumption by a human, not your program. They are divided into 3 parts: summary, reasons and help text.

**Summary:** A String representing a one-line description of your error. A summary is mandatory and is printed boldly in red.

**Reasons:** A vector of Strings explaining in more detail _why_ this error occured. Reasons are optional and if the terminal supports color, the bullet point ('-') will be colored yellow. Each reason will be printed on its own line.

**Help Text:** A String explaining additional information, including what the user can do about the error, or where to file a bug. Help text is optional and if the terminal supports color it will be printed _dimly_.

# Quickstart
Add the following to your Cargo.toml:
```yaml
[dependencies]
user-error = "1.2.0"
```

Add the following to your main.rs/lib.rs:
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

### Print
If for some reason you don't want to follow the format!() convention, you can call print() on a UserError and it will pretty print itself to stderr
```rust
use user_error::UserError;

fn main() {
    UserFacingError::new("Critical Failure!").print();
}
```
This prints:
```text
Error: Critical Failure!
```

### Print and Exit
Since this error is likely the last thing your program will run you can use this shortcut to print the error and exit the process in an immediate, albeit ungraceful manner. Returns error code 1 to the OS.
```rust
use user_error::UserError;

fn main() {
    let e = UserFacingError::new("Critical Failure!").print_and_exit();
    eprintln("I am never printed!");
}
```
This prints:
```text
Error: Critical Failure!
```

### UFE
You can implement the UFE trait for your own error types, instead of using the UserFacingError type. This is the preferred usage of the User-Error crate. By default, the summary will be the result of your_error.to_string() and the reasons will be created from following the error chain provided by .source().
```rust
// Dummy Error type to ensure that we can implement UFE on it
#[derive(Debug)]
struct MyError { sub: MySubError }

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError")
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
       Some(&self.sub)
    }
}

// Dummy sub error to represent the error chain
#[derive(Debug)]
struct MySubError { sub: MySubSubError }

impl Display for MySubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MySubError")
    }
}

impl Error for MySubError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.sub)
    }
}

// Dummy sub-sub error to represent the error chain
#[derive(Debug)]
struct MySubSubError;

impl Display for MySubSubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MySubSubError")
    }
}

impl Error for MySubSubError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl UFE for MyError {}

fn main() {
    let me = MyError { sub: MySubError { sub: MySubSubError {}}};
    me.print_and_exit();
}
```
This will print something that looks like this:
![UFE default trait implementation printout](https://xvrqt.sfo2.cdn.digitaloceanspaces.com/image-cache/error_example.png)

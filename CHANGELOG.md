# Latest

Added in exhaustive Error coercion for:

- std::io::Error
- rusqlite::Error

## New Functions

**print_other_errors()** the original_error field is not original_errors and is a vector of errors that caused or were coerced into this UserError. Prints the other errors to stderr.

**update_summary(new_summary: &str)** updates the summary of the UserError. Useful for updating UserErrors that were coerced from other Error types to make it more personalized for your app.

**upate_summary_and_push(new_summary: &str)** updates the summary while inserting the current summary into the list of reasons for the error. Useful for unwinding the call stack.

## Previous

## v1.0.1
README updated to better explain functionality

## v1.0.0
Initial publication


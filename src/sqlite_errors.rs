/* Allows for coercion of rusqlite::Error into UserError */

// Third Party Dependencies
use rusqlite::Error as SQLError;

// Intra Library Imports
use super::UserError;

/// Convert a rusqlite::Error into a UserError
///
/// # Example
/// ```
/// use user_error::UserError;
/// use std::path::Path;
/// use rusqlite::{Connection, Result as SQLResult, NO_PARAMS, OpenFlags};
/// 
/// fn bad_connection() -> Result<Connection, UserError> {
///     let c = Connection::open_with_flags(Path::new("nonexistent.db"), OpenFlags::SQLITE_OPEN_READ_WRITE)?;
///     Ok(c)    
/// }
/// 
/// assert!(bad_connection().is_err());
/// ```
impl From<SQLError> for UserError {
    fn from(error: SQLError) -> Self {
        let summary = String::from("SQLite has encountered an issue");
        match error {
            SQLError::SqliteFailure(e, r) => {
                let mut reasons = vec![String::from("Underlying SQLite call failed")];
                if let Some(s) = r { reasons.push(s) }
                UserError {
                    summary,
                    reasons: Some(reasons),
                    subtleties: None,
                    original_error: Some(Box::new(e)),
                }
            },
            SQLError::SqliteSingleThreadedMode => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Attempted to open an additional connection when SQLite was configured to allow single-threaded use only")]),
                    subtleties: None,
                    original_error: None 
                }
            },
            SQLError::FromSqlConversionFailure(c, t, e) => {
                UserError {
                    summary, 
                    reasons: Some(vec![format!("Failed to convert value of column {} to Rust type {}", c, t)]),
                    subtleties: None,
                    original_error: Some(e) 
                }
            },
            SQLError::IntegralValueOutOfRange(c, n) => {
                UserError {
                    summary, 
                    reasons: Some(vec![format!("Cannot fit integral value '{}' from column {} into requested type without overflow", n, c)]),
                    subtleties: Some(vec![String::from("e.g., trying to get the value 1000 into a u8")]),
                    original_error: None 
                }
            },
            SQLError::Utf8Error(e) => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Failed to convert string value to UTF-8")]),
                    subtleties: None,
                    original_error: Some(Box::new(e)) 
                }
            },
            SQLError::NulError(e) => {
                let bad_string = match std::str::from_utf8(&(e.clone().into_vec())) {
                    Ok(s)  => format!("'{}'", s),
                    Err(_) => String::from("String")
                };

                UserError {
                    summary, 
                    reasons: Some(vec![format!("Failed to convert {} to a C-Compatible String", bad_string), 
                                       format!("{} contains a nul byte at position: {}", bad_string, e.nul_position())]),
                    subtleties: Some(vec![String::from("While strings may contain nul bytes in the middle, C strings can't, as that byte would effectively truncate the string.")]),
                    original_error: Some(Box::new(e)) 
                }
            },
            SQLError::InvalidParameterName(s) => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Invalid parameter name"),
                                       format!("Parameter: {} not present in the SQL", s)]),
                    subtleties: None,
                    original_error: None
                }
            },
            SQLError::InvalidPath(_p) => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Invalid path")]),
                    subtleties: Some(vec![String::from("Could not convert the file path to a string.")]),
                    original_error: None
                }
            },
            SQLError::ExecuteReturnedResults => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Execute call returned rows")]),
                    subtleties: None,
                    original_error: None
                }
            },
            SQLError::QueryReturnedNoRows => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Query returned no rows")]),
                    subtleties: Some(vec![String::from("Query was expected to return at least one row (e.g., for query_row) but did not return any.")]),
                    original_error: None
                }
            },
            SQLError::InvalidColumnIndex(c) => {
                UserError {
                    summary, 
                    reasons: Some(vec![format!("Column index: {} is out of range for a statement", c)]),
                    subtleties: None,
                    original_error: None
                }
            },
            SQLError::InvalidColumnName(s) => {
                UserError {
                    summary, 
                    reasons: Some(vec![format!("No column matching '{}' in statement", s)]),
                    subtleties: None,
                    original_error: None
                }
            },
            SQLError::InvalidColumnType(c, _s, t) => {
                UserError {
                    summary, 
                    reasons: Some(vec![format!("Failed to convert value of column {} to Rust type {}", c, t)]),
                    subtleties: None,
                    original_error: None
                }
            },
            SQLError::StatementChangedRows(_c) => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Statement failed to insert row(s)")]),
                    subtleties: None,
                    original_error: None
                }
            },
            SQLError::InvalidQuery => {
                UserError {
                    summary, 
                    reasons: Some(vec![String::from("Invalid query")]),
                    subtleties: None,
                    original_error: None
                }
            },
            SQLError::ToSqlConversionFailure(e) => {
                UserError {
                    summary,
                    reasons: Some(vec![String::from("Failed to convert to SQL")]),
                    subtleties: None,
                    original_error: Some(e)
                }
            }
        }
    }
}

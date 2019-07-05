/* Trait implemenation for UserError
 * - Display
 * - Default
*/

// Standard Library Dependencies
use std::fmt;

// Intra Library Imports
use crate::helper;
use crate::UserError;

/// Display and Debug are required to satisfy the Error trait. Debug has been derived for UserError.
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut summary = self.summary();
    	let mut reasons = self.reasons();
    	let subtleties  = self.subtleties();

    	// Concatenate line breaks if necessary
    	if !reasons.is_empty() || !subtleties.is_empty() {
    		summary.push('\n');
    	}

    	if !reasons.is_empty() && !subtleties.is_empty() {
    		reasons.push('\n');
    	}

        f.write_str(&format!("{}{}{}", summary, reasons, subtleties))
    }
}

/// Default implementation for UserError prints: <application name> has encountered an unknown error.
impl Default for UserError {
	fn default() -> Self {
		UserError {
			summary: helper::default_summary(),
			reasons: None,
			subtleties: None,
			original_errors: None,
		}
	}
}

//! Pack validation schema (v0.6.0).
use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::i18n_text::I18nText;

/// Validation diagnostic entry.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    /// Diagnostic code.
    pub code: String,
    /// User-facing diagnostic message.
    pub message: I18nText,
    /// Optional severity string (info/warn/error).
    pub severity: Option<String>,
}

/// Validation result payload.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackValidationResult {
    /// Overall success flag.
    pub ok: bool,
    /// Diagnostics collected during validation.
    pub issues: Vec<Diagnostic>,
}

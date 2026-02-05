//! Legacy QA schema (v0.5.0) supported for migration.
//!
//! This represents the legacy QA shape supported for migration.
//! It is not guaranteed to match all historical formats.
use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ciborium::value::Value;

/// Legacy component QA spec (v0.5.0).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct LegacyComponentQaSpec {
    /// Title shown for the QA flow.
    pub title: String,
    /// Optional description shown to the user.
    pub description: Option<String>,
    /// Questions included in the QA flow.
    pub questions: Vec<LegacyQuestion>,
}

/// Legacy question entry.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct LegacyQuestion {
    /// Question identifier.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Optional help/description string.
    pub help: Option<String>,
    /// Question kind.
    pub kind: LegacyQuestionKind,
    /// Whether the question is required.
    pub required: bool,
    /// Optional default value.
    pub default: Option<Value>,
    /// Optional choice list.
    pub choices: Option<Vec<LegacyChoice>>,
}

/// Legacy choice option.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct LegacyChoice {
    /// Choice value (raw value encoded as string).
    pub value: String,
    /// Label presented to the user.
    pub label: String,
}

/// Legacy question kinds.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LegacyQuestionKind {
    /// Free-form text.
    Text,
    /// Choice list.
    Choice,
    /// Numeric input.
    Number,
    /// Boolean input.
    Bool,
}

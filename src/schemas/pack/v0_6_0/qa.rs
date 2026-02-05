//! Pack QA schema (v0.6.0).
use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use ciborium::value::Value;

use crate::i18n_text::I18nText;

/// QA mode.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum QaMode {
    /// Default mode.
    Default,
    /// Setup mode.
    Setup,
    /// Upgrade mode.
    Upgrade,
    /// Remove mode.
    Remove,
}

/// QA spec for a pack.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct PackQaSpec {
    /// QA mode.
    pub mode: QaMode,
    /// Human-readable title.
    pub title: I18nText,
    /// Optional description.
    pub description: Option<I18nText>,
    /// Questions to present.
    pub questions: Vec<Question>,
    /// Default values (canonical order).
    pub defaults: BTreeMap<String, Value>,
}

impl PackQaSpec {
    /// Collect all i18n keys referenced by this spec.
    pub fn i18n_keys(&self) -> alloc::collections::BTreeSet<String> {
        let mut keys = alloc::collections::BTreeSet::new();
        keys.insert(self.title.key.clone());
        if let Some(desc) = &self.description {
            keys.insert(desc.key.clone());
        }
        for question in &self.questions {
            question.collect_i18n_keys(&mut keys);
        }
        keys
    }
}

/// Question entry.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Question {
    /// Question identifier.
    pub id: String,
    /// Label shown to the user.
    pub label: I18nText,
    /// Optional help text.
    pub help: Option<I18nText>,
    /// Optional error message (validation feedback).
    pub error: Option<I18nText>,
    /// Kind of question.
    pub kind: QuestionKind,
    /// Whether the question is required.
    pub required: bool,
    /// Optional default value.
    pub default: Option<Value>,
}

impl Question {
    fn collect_i18n_keys(&self, keys: &mut alloc::collections::BTreeSet<String>) {
        keys.insert(self.label.key.clone());
        if let Some(help) = &self.help {
            keys.insert(help.key.clone());
        }
        if let Some(error) = &self.error {
            keys.insert(error.key.clone());
        }
        if let QuestionKind::Choice { options } = &self.kind {
            for option in options {
                keys.insert(option.label.key.clone());
            }
        }
    }
}

/// Question type.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case", tag = "type"))]
pub enum QuestionKind {
    /// Free-form text input.
    Text,
    /// Choice list.
    Choice {
        /// Choice options presented to the user.
        options: Vec<ChoiceOption>,
    },
    /// Numeric input.
    Number,
    /// Boolean input.
    Bool,
}

/// Choice option.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ChoiceOption {
    /// Value returned for this option.
    pub value: String,
    /// Label shown to the user.
    pub label: I18nText,
}

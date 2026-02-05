//! Pack schemas (v0.6.0).
pub mod describe;
pub mod qa;
pub mod validation;

pub use crate::i18n_text::I18nText;
pub use describe::{CapabilityDescriptor, CapabilityMetadata, PackDescribe, PackInfo};
pub use qa::{ChoiceOption, PackQaSpec, QaMode, Question, QuestionKind};
pub use validation::{Diagnostic, PackValidationResult};

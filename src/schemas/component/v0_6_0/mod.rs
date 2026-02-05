//! Component schemas (v0.6.0).
pub mod describe;
pub mod qa;

pub use crate::i18n_text::I18nText;
pub use describe::{ComponentDescribe, ComponentInfo, ComponentRunInput, ComponentRunOutput};
pub use qa::{ChoiceOption, ComponentQaSpec, QaMode, Question, QuestionKind};

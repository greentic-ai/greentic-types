//! Internationalization tags, IDs, and minimal profiles.
use alloc::{format, string::String};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use unic_langid::{LanguageIdentifier, LanguageIdentifierError};

use crate::cbor::canonical;

const I18N_ID_PREFIX: &str = "i18n:v1:";

/// Normalized locale tag (BCP 47 + -u- extensions) used for ID generation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct I18nTag(String);

impl I18nTag {
    /// Normalize an arbitrary locale tag into canonical casing.
    pub fn normalize_tag(input: &str) -> Result<Self, I18nTagError> {
        let langid: LanguageIdentifier = input.parse()?;
        Ok(Self(langid.to_string()))
    }

    /// Canonical tag string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for I18nTag {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Errors encountered while normalizing locale tags.
#[derive(Debug, Error)]
pub enum I18nTagError {
    /// Input string is not a valid BCP 47 tag.
    #[error("invalid locale tag: {0}")]
    Invalid(#[from] LanguageIdentifierError),
}

/// Stable identifier for a normalized locale tag or profile.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct I18nId(String);

impl I18nId {
    /// Parse a string produced by `id_for_tag`.
    pub fn parse(value: &str) -> Result<Self, I18nIdError> {
        if !value.starts_with(I18N_ID_PREFIX) {
            return Err(I18nIdError::InvalidPrefix);
        }
        let encoded = &value[I18N_ID_PREFIX.len()..];
        canonical::decode_base32_crockford(encoded)?;
        Ok(Self(value.to_owned()))
    }

    /// Borrow the canonical identifier string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for I18nId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Compute the ID for a normalized tag.
pub fn id_for_tag(tag: &I18nTag) -> Result<I18nId, I18nIdError> {
    let canonical_bytes = canonical::to_canonical_cbor(&tag.as_str())?;
    let digest = canonical::blake3_128(&canonical_bytes);
    let encoded = canonical::encode_base32_crockford(&digest);
    Ok(I18nId(format!("{I18N_ID_PREFIX}{encoded}")))
}

/// Errors emitted when working with I18n IDs.
#[derive(Debug, Error)]
pub enum I18nIdError {
    /// Identifier does not have the required `i18n:v1:` prefix.
    #[error("i18n ID must begin with {I18N_ID_PREFIX}")]
    InvalidPrefix,
    /// Base32 payload could not be decoded.
    #[error("invalid base32 payload: {0}")]
    Base32(#[from] canonical::Base32Error),
    /// Canonicalization or hashing failed while generating the ID.
    #[error(transparent)]
    Canonical(#[from] canonical::CanonicalError),
}

/// Directionality of text (`ltr` / `rtl`).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    /// Left-to-right text.
    Ltr,
    /// Right-to-left text.
    Rtl,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Ltr
    }
}

/// Minimal profile used during setup-time localization.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MinimalI18nProfile {
    /// Primary language subtag (e.g., `en`).
    pub language: String,
    /// Optional region subtag (e.g., `GB`).
    pub region: Option<String>,
    /// Optional script subtag (e.g., `Latn`).
    pub script: Option<String>,
    /// Text direction.
    pub direction: Direction,
    /// Calendar system (e.g., `gregory`).
    pub calendar: String,
    /// Currency code (ISO 4217).
    pub currency: String,
    /// Decimal separator symbol (e.g., `.` or `,`).
    pub decimal_separator: String,
    /// Optional timezone identifier.
    pub timezone: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_tag_case() {
        let tag = match I18nTag::normalize_tag("en-gb") {
            Ok(tag) => tag,
            Err(err) => panic!("normalize failed: {err:?}"),
        };
        assert_eq!(tag.as_str(), "en-GB");
    }

    #[test]
    fn tag_id_roundtrip() {
        let tag = match I18nTag::normalize_tag("en-US") {
            Ok(tag) => tag,
            Err(err) => panic!("normalize failed: {err:?}"),
        };
        let id = match id_for_tag(&tag) {
            Ok(id) => id,
            Err(err) => panic!("id generation failed: {err:?}"),
        };
        let roundtrip = match I18nId::parse(id.as_str()) {
            Ok(value) => value,
            Err(err) => panic!("parse failed: {err:?}"),
        };
        assert_eq!(roundtrip.as_str(), id.as_str());
    }
}

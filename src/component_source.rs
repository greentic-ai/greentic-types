//! Canonical component source references for packs.

use alloc::string::{String, ToString};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Supported component source references.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub enum ComponentSourceRef {
    /// Public OCI reference (`oci://repo/name:tag` or `oci://repo/name@sha256:...`).
    Oci(String),
    /// Private repository reference (`repo://...`).
    Repo(String),
    /// Store-licensed component reference (`store://...`).
    Store(String),
    /// File-based component reference (`file://...`).
    File(String),
}

impl ComponentSourceRef {
    /// Returns the scheme name for this reference.
    pub fn scheme(&self) -> &'static str {
        match self {
            ComponentSourceRef::Oci(_) => "oci",
            ComponentSourceRef::Repo(_) => "repo",
            ComponentSourceRef::Store(_) => "store",
            ComponentSourceRef::File(_) => "file",
        }
    }

    /// Returns the raw reference portion without the scheme prefix.
    pub fn reference(&self) -> &str {
        match self {
            ComponentSourceRef::Oci(value) => value,
            ComponentSourceRef::Repo(value) => value,
            ComponentSourceRef::Store(value) => value,
            ComponentSourceRef::File(value) => value,
        }
    }

    /// Returns `true` when this is an OCI reference using a tag suffix (`:tag`).
    pub fn is_tag(&self) -> bool {
        matches!(self.oci_reference_kind(), Some(OciReferenceKind::Tag))
    }

    /// Returns `true` when this is an OCI reference using a digest suffix (`@sha256:...`).
    pub fn is_digest(&self) -> bool {
        matches!(self.oci_reference_kind(), Some(OciReferenceKind::Digest))
    }

    /// Returns a canonical string form of the reference.
    pub fn normalized(&self) -> String {
        match self {
            ComponentSourceRef::Oci(reference) => normalize_oci_reference(reference),
            _ => self.to_string(),
        }
    }
}

impl core::fmt::Display for ComponentSourceRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}://{}", self.scheme(), self.reference())
    }
}

impl core::str::FromStr for ComponentSourceRef {
    type Err = ComponentSourceRefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            return Err(ComponentSourceRefError::EmptyReference);
        }
        if value.chars().any(char::is_whitespace) {
            return Err(ComponentSourceRefError::ContainsWhitespace);
        }
        if value.starts_with("oci://") {
            return parse_with_scheme(value, "oci://").map(ComponentSourceRef::Oci);
        }
        if value.starts_with("repo://") {
            return parse_with_scheme(value, "repo://").map(ComponentSourceRef::Repo);
        }
        if value.starts_with("store://") {
            return parse_with_scheme(value, "store://").map(ComponentSourceRef::Store);
        }
        if value.starts_with("file://") {
            return parse_with_scheme(value, "file://").map(ComponentSourceRef::File);
        }
        Err(ComponentSourceRefError::InvalidScheme)
    }
}

impl From<ComponentSourceRef> for String {
    fn from(value: ComponentSourceRef) -> Self {
        value.to_string()
    }
}

impl TryFrom<String> for ComponentSourceRef {
    type Error = ComponentSourceRefError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

/// Errors produced when parsing component source references.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ComponentSourceRefError {
    /// Reference cannot be empty.
    #[error("component source reference cannot be empty")]
    EmptyReference,
    /// Reference must not contain whitespace.
    #[error("component source reference must not contain whitespace")]
    ContainsWhitespace,
    /// Reference must use a supported scheme.
    #[error("component source reference must use oci://, repo://, store://, or file://")]
    InvalidScheme,
    /// Reference is missing the required locator after the scheme.
    #[error("component source reference is missing a locator")]
    MissingLocator,
}

fn parse_with_scheme(value: &str, scheme: &str) -> Result<String, ComponentSourceRefError> {
    if let Some(rest) = value.strip_prefix(scheme) {
        if rest.is_empty() {
            return Err(ComponentSourceRefError::MissingLocator);
        }
        return Ok(rest.to_string());
    }
    Err(ComponentSourceRefError::InvalidScheme)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OciReferenceKind {
    Tag,
    Digest,
}

struct OciReferenceParts<'a> {
    name: &'a str,
    tag: Option<&'a str>,
    digest: Option<&'a str>,
}

impl ComponentSourceRef {
    fn oci_reference_kind(&self) -> Option<OciReferenceKind> {
        let ComponentSourceRef::Oci(reference) = self else {
            return None;
        };
        let parts = split_oci_reference(reference);
        if parts.digest.is_some() {
            Some(OciReferenceKind::Digest)
        } else if parts.tag.is_some() {
            Some(OciReferenceKind::Tag)
        } else {
            None
        }
    }
}

fn split_oci_reference(reference: &str) -> OciReferenceParts<'_> {
    let (name_with_tag, digest) = match reference.split_once('@') {
        Some((name, digest)) => (name, Some(digest)),
        None => (reference, None),
    };
    let (name, tag) = split_oci_tag(name_with_tag);
    OciReferenceParts { name, tag, digest }
}

fn split_oci_tag(reference: &str) -> (&str, Option<&str>) {
    let last_slash = reference.rfind('/');
    let last_colon = reference.rfind(':');
    if let Some(colon) = last_colon {
        if last_slash.is_none_or(|slash| colon > slash) {
            let tag = &reference[colon + 1..];
            if !tag.is_empty() {
                return (&reference[..colon], Some(tag));
            }
        }
    }
    (reference, None)
}

fn normalize_oci_reference(reference: &str) -> String {
    let parts = split_oci_reference(reference);
    if let Some(digest) = parts.digest {
        format!("oci://{}@{}", parts.name, digest)
    } else if let Some(tag) = parts.tag {
        format!("oci://{}:{}", parts.name, tag)
    } else {
        format!("oci://{}", reference)
    }
}

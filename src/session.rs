//! Session identity and cursor helpers.

use alloc::borrow::ToOwned;
use alloc::string::String;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Unique key referencing a persisted session.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct SessionKey(pub String);

impl SessionKey {
    /// Returns the session key as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates a new session key from the supplied string.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Generates a random session key using [`uuid`], when enabled.
    #[cfg(feature = "uuid")]
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl From<String> for SessionKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for SessionKey {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl core::fmt::Display for SessionKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for SessionKey {
    fn from(value: uuid::Uuid) -> Self {
        Self(value.to_string())
    }
}

/// Cursor pointing at a session's position in a flow graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SessionCursor {
    /// Identifier of the node currently owning the session.
    pub node_pointer: String,
    /// Optional wait reason emitted by the node.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub wait_reason: Option<String>,
    /// Optional marker describing pending outbox operations.
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub outbox_marker: Option<String>,
}

impl SessionCursor {
    /// Creates a new cursor pointing at the provided node identifier.
    pub fn new(node_pointer: impl Into<String>) -> Self {
        Self {
            node_pointer: node_pointer.into(),
            wait_reason: None,
            outbox_marker: None,
        }
    }

    /// Assigns a wait reason to the cursor.
    pub fn with_wait_reason(mut self, reason: impl Into<String>) -> Self {
        self.wait_reason = Some(reason.into());
        self
    }

    /// Assigns an outbox marker to the cursor.
    pub fn with_outbox_marker(mut self, marker: impl Into<String>) -> Self {
        self.outbox_marker = Some(marker.into());
        self
    }
}

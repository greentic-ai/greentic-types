//! Shared DTOs for the universal messaging operator-provider protocol.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "schemars")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ChannelMessageEnvelope;

/// HTTP header name/value pair.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Header {
    /// Header name.
    pub name: String,
    /// Header value.
    pub value: String,
}

/// Normalized HTTP ingress payload (v1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct HttpInV1 {
    /// HTTP method.
    pub method: String,
    /// Request path.
    pub path: String,
    /// Optional raw query string.
    #[cfg_attr(feature = "serde", serde(default))]
    pub query: Option<String>,
    /// Request headers.
    #[cfg_attr(feature = "serde", serde(default))]
    pub headers: Vec<Header>,
    /// Base64-encoded request body.
    #[cfg_attr(feature = "serde", serde(default))]
    pub body_b64: String,
    /// Optional route hint for dispatching.
    #[cfg_attr(feature = "serde", serde(default))]
    pub route_hint: Option<String>,
    /// Optional binding identifier for routing.
    #[cfg_attr(feature = "serde", serde(default))]
    pub binding_id: Option<String>,
    /// Optional provider configuration payload.
    #[cfg_attr(feature = "serde", serde(default))]
    pub config: Option<Value>,
}

/// Normalized HTTP egress response (v1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct HttpOutV1 {
    /// Response status code.
    pub status: u16,
    /// Response headers.
    #[cfg_attr(feature = "serde", serde(default))]
    pub headers: Vec<Header>,
    /// Base64-encoded response body.
    #[cfg_attr(feature = "serde", serde(default))]
    pub body_b64: String,
    /// Emitted inbound events.
    #[cfg_attr(feature = "serde", serde(default))]
    pub events: Vec<ChannelMessageEnvelope>,
}

/// Render planning input for provider payloads.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RenderPlanInV1 {
    /// Message to be rendered.
    pub message: ChannelMessageEnvelope,
    /// Optional metadata for renderer hints.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: BTreeMap<String, Value>,
}

/// Render planning output containing the serialized plan.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct RenderPlanOutV1 {
    /// Serialized plan JSON.
    pub plan_json: String,
}

/// Provider-encoded payload plus metadata.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct ProviderPayloadV1 {
    /// MIME content type of the payload.
    pub content_type: String,
    /// Base64-encoded payload body.
    #[cfg_attr(feature = "serde", serde(default))]
    pub body_b64: String,
    /// Provider-specific metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: BTreeMap<String, Value>,
}

/// Encode request combining the message and render plan.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct EncodeInV1 {
    /// Message to be encoded.
    pub message: ChannelMessageEnvelope,
    /// Render plan to apply.
    pub plan: RenderPlanInV1,
}

/// Authenticated user reference used for provider operations.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct AuthUserRefV1 {
    /// Provider user identifier.
    pub user_id: String,
    /// Token key reference.
    pub token_key: String,
    /// Optional tenant identifier.
    #[cfg_attr(feature = "serde", serde(default))]
    pub tenant_id: Option<String>,
    /// Optional user email address.
    #[cfg_attr(feature = "serde", serde(default))]
    pub email: Option<String>,
    /// Optional display name.
    #[cfg_attr(feature = "serde", serde(default))]
    pub display_name: Option<String>,
}

/// Send request for a provider payload.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SendPayloadInV1 {
    /// Provider type identifier.
    pub provider_type: String,
    /// Optional tenant identifier override.
    #[cfg_attr(feature = "serde", serde(default))]
    pub tenant_id: Option<String>,
    /// Optional auth user reference.
    #[cfg_attr(feature = "serde", serde(default))]
    pub auth_user: Option<AuthUserRefV1>,
    /// Provider payload to deliver.
    pub payload: ProviderPayloadV1,
}

/// Send result status and retry hint.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SendPayloadResultV1 {
    /// Whether the send succeeded.
    pub ok: bool,
    /// Optional error or status message.
    #[cfg_attr(feature = "serde", serde(default))]
    pub message: Option<String>,
    /// Whether the operation is retryable.
    #[cfg_attr(feature = "serde", serde(default))]
    pub retryable: bool,
}

/// Subscription ensure request (v1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SubscriptionEnsureInV1 {
    /// Protocol version.
    pub v: u32,
    /// Provider identifier.
    pub provider: String,
    /// Optional tenant hint.
    #[cfg_attr(feature = "serde", serde(default))]
    pub tenant_hint: Option<String>,
    /// Optional team hint.
    #[cfg_attr(feature = "serde", serde(default))]
    pub team_hint: Option<String>,
    /// Optional binding identifier.
    #[cfg_attr(feature = "serde", serde(default))]
    pub binding_id: Option<String>,
    /// Resource to subscribe to.
    pub resource: String,
    /// Change types to subscribe to.
    #[cfg_attr(feature = "serde", serde(default))]
    pub change_types: Vec<String>,
    /// Notification URL for callbacks.
    pub notification_url: String,
    /// Optional expiration in minutes.
    #[cfg_attr(feature = "serde", serde(default))]
    pub expiration_minutes: Option<u32>,
    /// Optional target expiration timestamp (ms since epoch).
    #[cfg_attr(feature = "serde", serde(default))]
    pub expiration_target_unix_ms: Option<u64>,
    /// Optional client state token.
    #[cfg_attr(feature = "serde", serde(default))]
    pub client_state: Option<String>,
    /// Optional provider metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Option<Value>,
    /// Authenticated user reference.
    #[cfg_attr(feature = "serde", serde(default))]
    pub user: AuthUserRefV1,
}

/// Subscription ensure response (v1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SubscriptionEnsureOutV1 {
    /// Protocol version.
    pub v: u32,
    /// Provider subscription identifier.
    pub subscription_id: String,
    /// Expiration timestamp (ms since epoch).
    pub expiration_unix_ms: u64,
    /// Resource subscribed to.
    pub resource: String,
    /// Change types in the subscription.
    pub change_types: Vec<String>,
    /// Optional client state token.
    #[cfg_attr(feature = "serde", serde(default))]
    pub client_state: Option<String>,
    /// Optional provider metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Option<Value>,
    /// Optional binding identifier.
    #[cfg_attr(feature = "serde", serde(default))]
    pub binding_id: Option<String>,
    /// Authenticated user reference.
    #[cfg_attr(feature = "serde", serde(default))]
    pub user: AuthUserRefV1,
}

/// Subscription renewal request (v1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SubscriptionRenewInV1 {
    /// Protocol version.
    pub v: u32,
    /// Provider identifier.
    pub provider: String,
    /// Subscription identifier.
    pub subscription_id: String,
    /// Optional expiration in minutes.
    #[cfg_attr(feature = "serde", serde(default))]
    pub expiration_minutes: Option<u32>,
    /// Optional target expiration timestamp (ms since epoch).
    #[cfg_attr(feature = "serde", serde(default))]
    pub expiration_target_unix_ms: Option<u64>,
    /// Optional provider metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Option<Value>,
    /// Authenticated user reference.
    #[cfg_attr(feature = "serde", serde(default))]
    pub user: AuthUserRefV1,
}

/// Subscription renewal response (v1).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SubscriptionRenewOutV1 {
    /// Protocol version.
    pub v: u32,
    /// Subscription identifier.
    pub subscription_id: String,
    /// Expiration timestamp (ms since epoch).
    pub expiration_unix_ms: u64,
    /// Optional provider metadata.
    #[cfg_attr(feature = "serde", serde(default))]
    pub metadata: Option<Value>,
    /// Authenticated user reference.
    #[cfg_attr(feature = "serde", serde(default))]
    pub user: AuthUserRefV1,
}

/// Subscription deletion request (v1).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SubscriptionDeleteInV1 {
    /// Protocol version.
    pub v: u32,
    /// Provider identifier.
    pub provider: String,
    /// Subscription identifier.
    pub subscription_id: String,
    /// Authenticated user reference.
    #[cfg_attr(feature = "serde", serde(default))]
    pub user: AuthUserRefV1,
}

/// Subscription deletion response (v1).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct SubscriptionDeleteOutV1 {
    /// Protocol version.
    pub v: u32,
    /// Subscription identifier.
    pub subscription_id: String,
    /// Authenticated user reference.
    #[cfg_attr(feature = "serde", serde(default))]
    pub user: AuthUserRefV1,
}

/// Result alias for subscription ensure calls.
pub type SubscriptionEnsureResultV1 = SubscriptionEnsureOutV1;
/// Result alias for subscription delete calls.
pub type SubscriptionDeleteResultV1 = SubscriptionDeleteOutV1;
/// Alias for subscription renewal input.
pub type SubscriptionRenewalInV1 = SubscriptionRenewInV1;
/// Alias for subscription renewal output.
pub type SubscriptionRenewalOutV1 = SubscriptionRenewOutV1;

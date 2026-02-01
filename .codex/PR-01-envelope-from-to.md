# PR: greentic-types — Add `to[]` + `from` Actor, remove `user_id` (breaking)

## Summary
Update `ChannelMessageEnvelope` to:
- add `to: Vec<Destination>`
- replace `user_id: Option<String>` with `from: Option<Actor>`

This clarifies sender vs destination, and enables operators/providers to exchange a single envelope type.

## Files to change

### 1) `crates/greentic-types/src/messaging.rs`

#### A. Add new structs

Add near other messaging structs (above `ChannelMessageEnvelope`):

```rust
/// Message actor (sender/initiator).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Actor {
    /// Actor identifier in provider space (e.g., slack user id, webex person id).
    pub id: String,
    /// Optional actor kind (e.g. "user", "bot", "system").
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub kind: Option<String>,
}

/// Outbound destination for egress providers.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
pub struct Destination {
    /// Destination identifier (provider specific; may be composite e.g. "teamId:channelId").
    pub id: String,
    /// Optional destination kind (e.g. "chat", "room", "user", "channel", "email", "phone").
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub kind: Option<String>,
}
```

#### B. Update `ChannelMessageEnvelope`

Replace this field:

```rust
pub user_id: Option<String>,
```

with:

```rust
/// Optional actor (sender/initiator) associated with the message (primarily ingress).
#[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
)]
pub from: Option<Actor>,
```

Then add `to` near other core fields (recommended location: after `from` or after `reply_scope`):

```rust
/// Outbound destinations for egress. Empty means “unspecified” and may be satisfied by provider config defaults.
#[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Vec::is_empty")
)]
pub to: Vec<Destination>,
```

**Note:** Keep `#[serde(default)]` so older JSON can deserialize; this helps incremental repo upgrades even though it’s a “breaking” Rust change.

### 2) `crates/greentic-types/src/lib.rs`

Update the re-export list if you want `Actor` and `Destination` to be publicly available:

```rust
pub use messaging::{Actor, Destination, Attachment, ChannelMessageEnvelope, MessageMetadata};
```

### 3) `crates/greentic-types/src/schema.rs`
Ensure `Actor` and `Destination` are included in schema exports/index if required by your schema machinery.
If you already export `ChannelMessageEnvelope`, it will pull them in via references; but add them explicitly if your generator expects a type list.

### 4) Tests: `crates/greentic-types/tests/messaging_envelope_roundtrip.rs`

Update envelope constructors:
- remove `user_id: ...`
- add `from: ...` (or `None`)
- add `to: vec![]` (or a sample destination)

Example edit (pseudo):

```rust
let envelope = ChannelMessageEnvelope {
    // ...
    from: Some(Actor { id: "u123".into(), kind: Some("user".into()) }),
    to: vec![Destination { id: "room123".into(), kind: Some("room".into()) }],
    // ...
};
```

### 5) Docs
- `CHANGELOG.md`: add entry noting breaking rename: `user_id -> from` and new `to[]`.

## Commands / checks
Run:

```bash
cargo fmt
cargo test -p greentic-types
```

If you publish JSON Schemas as artifacts, regenerate and verify diffs.

## Acceptance criteria
- All greentic-types tests pass
- JSON schema for ChannelMessageEnvelope includes `from` and `to`, and no longer includes `user_id`

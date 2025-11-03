# greentic-types

Shared primitives for Greentic’s next-generation runner, deployer, connectors, packs, and state/session backends.  
Every repository in the `greentic-ng` stack depends on these types to exchange tenant identity, session cursors, state pointers, policy decisions, pack references, and canonical error/outcome envelopes.

## Crate features
- `serde` *(default)* – enables serde derives and helpers (pulls `serde_with` for stable semver + base64 encodings).
- `time` *(default)* – enables `time::OffsetDateTime` helpers for deadlines and telemetry spans.
- `uuid` – adds UUID-based constructors for session keys.
- `schemars` – emits JSON Schemas for all public types (used by docs and CI conformance tests).
- `std` *(default)* – allows attaching source errors to `NodeError`/`GreenticError`.

Disable defaults for fully `no_std` builds:
```toml
greentic-types = { version = "0.1.4", default-features = false, features = ["serde"] }
```

## Quickstart
```rust
use greentic_types::{
    AllowList, EnvId, ErrorCode, Outcome, SessionCursor, SessionKey, TenantCtx, TenantId,
};

let ctx = TenantCtx::new(EnvId::from("prod"), TenantId::from("tenant-42"))
    .with_team(Some("team-ops".into()))
    .with_user(Some("agent-7".into()));

let cursor = SessionCursor::new("node.entry")
    .with_wait_reason("awaiting human input")
    .with_outbox_marker("handoff");

let require_human: Outcome<()> = Outcome::Pending {
    reason: "Need operator approval".into(),
    expected_input: Some(vec!["approval".into()]),
};

let allow_policy = AllowList {
    domains: vec!["api.greentic.ai".into()],
    ports: vec![443],
    protocols: vec![greentic_types::Protocol::Https],
};

assert!(require_human.is_pending());
assert_eq!(cursor.node_pointer, "node.entry");
assert!(!allow_policy.is_empty());
```

### Sessions & telemetry
```rust
use greentic_types::{SessionKey, SpanContext};

let session = SessionKey::generate();
let span = SpanContext::new("tenant-42".into(), "flow-welcome", "runner-core")
    .with_session(session.clone())
    .with_node("node.entry");
```

### Outcomes & errors
```rust
use greentic_types::{ErrorCode, GreenticError, Outcome};

fn validate(input: &str) -> greentic_types::GResult<()> {
    if input.is_empty() {
        return Err(GreenticError::new(ErrorCode::InvalidInput, "missing payload"));
    }
    Ok(())
}

let outcome = match validate("payload") {
    Ok(_) => Outcome::Done("payload accepted"),
    Err(err) => Outcome::Error {
        code: err.code,
        message: err.message.clone(),
    },
};
```

### Schema generation
```rust
#[cfg(feature = "schemars")]
{
    let schema = schemars::schema_for!(greentic_types::TenantCtx);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
```

## Telemetry (auto-init)
- Enable with `features = ["telemetry-autoinit"]` to bundle the OTLP stack and entry-point macro.
- `#[greentic_types::telemetry::main(...)]` wraps `tokio::main`, installs OTLP once, and forwards to your async main.
- `install_telemetry("name")` is available if you need to wire custom runtimes or tests manually.
- Uses `OTEL_EXPORTER_OTLP_ENDPOINT` when set (defaults to `http://localhost:4317`).
- `set_current_tenant_ctx(&TenantCtx)` maps the current tenant into the task-local telemetry slot.

```rust
use greentic_types::{EnvId, TenantCtx, TenantId};
use greentic_types::telemetry::set_current_tenant_ctx;

#[greentic_types::telemetry::main(service_name = "greentic-runner")]
async fn main() -> anyhow::Result<()> {
    let ctx = TenantCtx::new(EnvId::from("prod"), TenantId::from("acme"))
        .with_session("s1")
        .with_flow("hello")
        .with_node("qa-1")
        .with_provider("runner");
    set_current_tenant_ctx(&ctx);
    tracing::info!("tenant-aware spans now have gt.* attrs");
    Ok(())
}
```

## Harmonised model
- **TenantCtx & TenantIdentity** – shared across runner, connectors, and state/session stores; keeps legacy (`tenant`, `team`, `user`) and next-gen (`tenant_id`, `team_id`, `user_id`, `impersonation`) fields aligned.
- **SessionKey/SessionCursor** – referenced by session routers and state stores.
- **StateKey/StatePath** – JSON pointer compatible navigation for persisted state.
- **Outcome<T> & GreenticError** – canonical execution envelope for nodes, adapters, and tools.
- **AllowList/NetworkPolicy & PolicyDecision** – security model used by deployer and connector sandboxes.
- **PackRef/Signature** – pack registry references with deterministic semver + base64 signatures.
- **SpanContext** – OTLP-aligned telemetry context (tenant, flow, node, provider, start/end).

## Working with other crates
- `greentic-runner`, `greentic-session-store`, `greentic-state-store`, `greentic-deployer`, `greentic-connectors`, and `greentic-packs` depend on this crate. Always add new shared types here first to avoid duplication.
- ABI/WIT contracts live in **greentic-interfaces**; never re-define those types here.

## Development workflow
```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all-features
```

CI (see `.github/workflows/publish.yml`) enforces the same gates on push/PR. Legacy `v*` tags still trigger the workflow alongside per-crate tags.

## Maintenance notes
- All public structs derive `schemars::JsonSchema` when the feature is enabled; integration tests assert schema registration and serde round-trips.
- `GResult<T>` aliases `Result<T, GreenticError>` for consistent error propagation.
- Prefer zero-copy APIs; the crate is `#![forbid(unsafe_code)]`.
- Use feature flags to keep downstream binaries lightweight (e.g., disable `uuid` in constrained connectors).

## Releases & Publishing
- Versions come directly from each crate’s `Cargo.toml`.
- Pushing to `master` tags any crate whose version changed in that commit using `<crate-name>-v<semver>`.
- After tagging (or even when no tags are created), the publish workflow attempts to publish all changed crates via `katyo/publish-crates@v2`.
- Publishing is idempotent; attempting to release the same version again succeeds without errors.

## Pack specifications
Legacy pack manifests remain available via `greentic_types::pack_spec::{PackSpec, ToolSpec}` for backwards compatibility during the MCP migration. New packs should still embed their contracts in `greentic-interfaces`.

## License
MIT License. See [LICENSE](LICENSE).

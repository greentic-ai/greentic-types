# greentic-types

Shared primitives for Greentic runtimes and surfaces to describe tenant-aware executions, normalized invocation envelopes, and structured node errors.

## Features
- Tenant, team, user, and environment identifiers with serde support
- `TenantCtx` with retry metadata and runtime-safe helpers
- `InvocationEnvelope` shared across messaging, cron, webhook, and runtime surfaces
- `NodeError` with retry/backoff hints and structured details
- Helpers for idempotency key generation and JSON serialization without panics

## Usage
```rust
use greentic_types::{
    make_idempotency_key, safe_json, EnvId, InvocationEnvelope, NodeResult, TenantCtx, TenantId,
};
use serde_json::json;

fn example() -> NodeResult<()> {
    let ctx = TenantCtx {
        env: EnvId::from("prod"),
        tenant: TenantId::from("tenant-123"),
        team: None,
        user: None,
        trace_id: Some("trace-1".into()),
        correlation_id: Some("corr-1".into()),
        deadline_unix_ms: None,
        attempt: 0,
        idempotency_key: None,
    };

    let envelope = InvocationEnvelope {
        ctx: ctx.clone(),
        flow_id: "welcome-flow".into(),
        node_id: Some("email-node".into()),
        op: "on_message".into(),
        payload: json!({ "subject": "Welcome!" }),
        metadata: json!({ "platform": "email" }),
    };

    let generated_key =
        make_idempotency_key(&ctx, &envelope.flow_id, envelope.node_id.as_deref(), None);
    let payload_value = safe_json(&envelope.payload)?;

    assert_eq!(generated_key.len(), 32);
    assert_eq!(payload_value["subject"], "Welcome!");
    Ok(())
}
```

## Development
```bash
cargo test
```

## License
MIT License. See [LICENSE](LICENSE).

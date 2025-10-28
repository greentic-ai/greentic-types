# greentic-types

Shared primitives for Greentic runtimes and surfaces to describe tenant-aware executions, normalized invocation envelopes, and structured node errors.

## Features
- Tenant, team, user, and environment identifiers with optional serde support
- `TenantCtx` with attempt counters and millisecond deadlines
- `InvocationEnvelope` shared across messaging, cron, webhook, and runtime surfaces
- `NodeError` with retry/backoff hints and structured text/binary details
- Pure-Rust idempotency key helper compatible with `no_std`

## Usage
```rust
use greentic_types::{
    make_idempotency_key, BinaryPayload, EnvId, InvocationDeadline, InvocationEnvelope, NodeResult,
    TenantCtx, TenantId,
};

fn example() -> NodeResult<()> {
    let ctx = TenantCtx {
        env: EnvId::from("prod"),
        tenant: TenantId::from("tenant-123"),
        team: None,
        user: None,
        trace_id: Some("trace-1".into()),
        correlation_id: Some("corr-1".into()),
        deadline: Some(InvocationDeadline::from_unix_millis(1_700_000_000_000)),
        attempt: 0,
        idempotency_key: None,
    };

    let payload: BinaryPayload = b"Welcome!".to_vec();
    let metadata: BinaryPayload = b"platform=email".to_vec();

    let envelope = InvocationEnvelope {
        ctx: ctx.clone(),
        flow_id: "welcome-flow".into(),
        node_id: Some("email-node".into()),
        op: "on_message".into(),
        payload,
        metadata,
    };

    let generated_key =
        make_idempotency_key(&ctx, &envelope.flow_id, envelope.node_id.as_deref(), None);

    assert_eq!(generated_key.len(), 32);
    Ok(())
}
```

## Development
```bash
cargo test
```

### no_std
Enable `default-features = false` and use only `time`-backed types that don't require alloc-heavy helpers.

## License
MIT License. See [LICENSE](LICENSE).

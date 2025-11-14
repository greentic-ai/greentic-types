# JSON Schema Publishing

Schemas for the shared Greentic types are published to GitHub Pages so IDEs, CLIs, and CI jobs can validate documents consistently. Every schema lives under the stable base URL:

```
https://greentic-ai.github.io/greentic-types/schemas/v1/<name>.schema.json
```

The `bin/export-schemas.rs` helper (or `greentic_types::write_all_schemas`) materialises the schemas into `dist/schemas/v1/`. The GitHub Pages workflow runs the helper on every push to `master` and republishes the `dist/` directory.

## Canonical URLs

| Type | Schema URL |
|------|------------|
| PackId | https://greentic-ai.github.io/greentic-types/schemas/v1/pack-id.schema.json |
| ComponentId | https://greentic-ai.github.io/greentic-types/schemas/v1/component-id.schema.json |
| FlowId | https://greentic-ai.github.io/greentic-types/schemas/v1/flow-id.schema.json |
| NodeId | https://greentic-ai.github.io/greentic-types/schemas/v1/node-id.schema.json |
| TenantContext | https://greentic-ai.github.io/greentic-types/schemas/v1/tenant-context.schema.json |
| HashDigest | https://greentic-ai.github.io/greentic-types/schemas/v1/hash-digest.schema.json |
| SemverReq | https://greentic-ai.github.io/greentic-types/schemas/v1/semver-req.schema.json |
| RedactionPath | https://greentic-ai.github.io/greentic-types/schemas/v1/redaction-path.schema.json |
| Capabilities | https://greentic-ai.github.io/greentic-types/schemas/v1/capabilities.schema.json |
| Flow | https://greentic-ai.github.io/greentic-types/schemas/v1/flow.schema.json |
| Node | https://greentic-ai.github.io/greentic-types/schemas/v1/node.schema.json |
| ComponentManifest | https://greentic-ai.github.io/greentic-types/schemas/v1/component-manifest.schema.json |
| PackManifest | https://greentic-ai.github.io/greentic-types/schemas/v1/pack-manifest.schema.json |
| Limits | https://greentic-ai.github.io/greentic-types/schemas/v1/limits.schema.json |
| TelemetrySpec | https://greentic-ai.github.io/greentic-types/schemas/v1/telemetry-spec.schema.json |
| NodeSummary | https://greentic-ai.github.io/greentic-types/schemas/v1/node-summary.schema.json |
| NodeFailure | https://greentic-ai.github.io/greentic-types/schemas/v1/node-failure.schema.json |
| NodeStatus | https://greentic-ai.github.io/greentic-types/schemas/v1/node-status.schema.json |
| RunStatus | https://greentic-ai.github.io/greentic-types/schemas/v1/run-status.schema.json |
| TranscriptOffset | https://greentic-ai.github.io/greentic-types/schemas/v1/transcript-offset.schema.json |
| ToolsCaps | https://greentic-ai.github.io/greentic-types/schemas/v1/tools-caps.schema.json |
| SecretsCaps | https://greentic-ai.github.io/greentic-types/schemas/v1/secrets-caps.schema.json |
| OtlpKeys | https://greentic-ai.github.io/greentic-types/schemas/v1/otlp-keys.schema.json |
| RunResult | https://greentic-ai.github.io/greentic-types/schemas/v1/run-result.schema.json |

> `OtlpKeys` and `RunResult` schemas are emitted when the `otel-keys` and `time` features are enabled respectively; both keep their canonical IDs.

## Generating locally

```bash
cargo run --bin export-schemas --all-features
ls dist/schemas/v1
```

Use these URLs in IDE validation rules, manifests, or CI assertions so other repos stay in sync with the shared types.

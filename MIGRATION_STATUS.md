# Migration status — greentic-types (Public-Launch Secrets)

- What changed: `SecretKey`, `SecretScope`, `SecretFormat`, and `SecretRequirement` are now defined here (not in `greentic-secrets-spec`) and exported via `greentic_types::secrets::*`. Use `SecretKey::parse` for validation. Secrets lists in capabilities (`SecretsCaps`), component host capabilities, deployment plans, and binding hints all use `Vec<SecretRequirement>`.
- What broke: serde/schema surface for secrets changed; any downstream expecting plain string keys or the old `SecretPlan` will fail to compile/deserialize. Schema consumers should refresh.
- New: Pack manifests/metadata now expose `secret_requirements: Vec<SecretRequirement>` so runners/deployers/distributors can read `.gtpack` requirements directly without custom parsing.
- Next repos to update: bump `greentic-types` to this version in pack tooling and hosts that read packs: `greentic-pack/packc` (write/aggregate), `greentic-runner/runner-host` (preflight/expose), `greentic-deployer` (preflight), `greentic-distributor` (API surface), `greentic-distributor-client` (DTO mapping).

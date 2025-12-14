# Migration status — greentic-types (Public-Launch Secrets)

- What changed: `SecretKey`, `SecretScope`, `SecretFormat`, and `SecretRequirement` are now defined here (not in `greentic-secrets-spec`) and exported via `greentic_types::secrets::*`. Use `SecretKey::parse` for validation. Secrets lists in capabilities (`SecretsCaps`), component host capabilities, deployment plans, and binding hints all use `Vec<SecretRequirement>`.
- What broke: serde/schema surface for secrets changed; any downstream expecting plain string keys or the old `SecretPlan` will fail to compile/deserialize. Schema consumers should refresh.
- Next repos to update: runners/deployer/dev/MCP/pack tooling must consume `SecretRequirement` from `greentic-types` and stop using string lists or spec-local definitions; pack builders should emit structured `secret_requirements` in `.gtpack`; CLI/secrets tooling already provides the spec types.

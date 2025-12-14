# Migration status — greentic-types (Public-Launch Secrets)

- What changed: now depends on `greentic-secrets-spec` and re-exports `SecretKey`, `SecretScope`, `SecretFormat`, `SecretRequirement`; secrets lists in capabilities (`SecretsCaps`), component host capabilities, deployment plans, and binding hints all use `Vec<SecretRequirement>` instead of `Vec<String>`. README documents the canonical source.
- What broke: serde/schema surface for secrets changed; any downstream expecting plain string keys or the old `SecretPlan` will fail to compile/deserialize. Schema consumers should refresh.
- Next repos to update: runners/deployer/dev/MCP/pack tooling must consume `SecretRequirement` from `greentic-secrets-spec` and stop using string lists; pack builders should emit structured `secret_requirements` in `.gtpack`; CLI/secrets tooling already provides the spec types.

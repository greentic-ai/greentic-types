# Provider extension ground truth

- Repo: `800264cf71ec31c42705fe599b2104a67e1749c9` on branch `master`
- Tools: `rg 15.1.0`

## Canonical extension identifier
- `greentic.provider-extension.v1` is the only canonical ID: exported as `PROVIDER_EXTENSION_ID` with the doc comment “Canonical provider extension identifier stored in pack manifests.” (`src/provider.rs:17-18`). All helper methods read/write this key and no fallback is implemented (`src/pack_manifest.rs:252-305`).
- Tests and fixtures exercise only this ID: the JSON fixture uses it as both map key and `kind` (`tests/fixtures/manifest_with_provider_ext.json:7-30`), and parsing helpers retrieve via `PROVIDER_EXTENSION_ID` (`tests/pack_manifest_extensions.rs:24-56`).
- Schema metadata references the inline schema but not an alternate extension key; helpers default version to `1.0.0` when synthesizing the entry (`src/pack_manifest.rs:279-289`).

## Non-canonical identifier in docs
- The only appearance of `greentic.ext.provider` is an example in `docs/pack_manifest_extensions.md:24-35` and as an “e.g.” comment on `ExtensionRef` (`src/pack_manifest.rs:227-228`). No code paths resolve or alias this string, so it would be treated as an unknown extension blob.

## Payload schema (inline)
- Type: `ProviderExtensionInline` (`src/provider.rs:103-115`), wrapped inside `ExtensionInline::Provider` (`src/pack_manifest.rs:188-218`).
- Fields:
  - `providers: Vec<ProviderDecl>` required (`src/provider.rs:107-110`).
  - `additional_fields: BTreeMap<String, Value>` `#[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]` for forward compatibility (`src/provider.rs:111-115`).
- `ProviderDecl` requires `provider_type`, `config_schema_ref`, and `runtime` plus optional lists/fields (`src/provider.rs:66-101`):
  - `capabilities`/`ops` default+skip when empty (`src/provider.rs:73-84`).
  - `state_schema_ref`/`docs_ref` optional (`src/provider.rs:87-100`).
  - `runtime: ProviderRuntimeRef` with required `component_ref`, `export`, `world` (`src/provider.rs:53-64`).
- Inline payload lives under `extensions[PROVIDER_EXTENSION_ID].inline` with serde-untyped passthrough for other extensions (`src/pack_manifest.rs:252-275`); CBOR roundtrips the same map without reinterpretation (`src/cbor.rs:93-136,260-287`).

## Versioning rules
- `ExtensionRef.version` is a free-form string with no semantic checks (`src/pack_manifest.rs:227-248`).
- `ensure_provider_extension_inline` synthesizes provider entries with `version: "1.0.0"` and `kind: PROVIDER_EXTENSION_ID`, establishing the default version to emit (`src/pack_manifest.rs:279-289`).

## Compatibility / aliases
- Lookup uses only `extensions.get(PROVIDER_EXTENSION_ID)`; there is no aliasing or dual-key lookup (`src/pack_manifest.rs:252-276`). A manifest keyed under `greentic.ext.provider` will parse but will be treated as an unknown extension and the provider helpers will return `None`.
- No deprecation markers or dual IDs exist; tests never mention `greentic.ext.provider`.

## Sample manifest check (user-supplied shape)
- Key `greentic.ext.provider` does **not** match the canonical `greentic.provider-extension.v1`, so helper accessors would miss it.
- Payload expectations: each provider entry must include `config_schema_ref`, `runtime.export`, and `runtime.world` in addition to `provider_type`/`runtime.component_ref` (`src/provider.rs:66-101,53-64`). If those fields are absent in the sample, deserialization into `ProviderDecl` would fail.
- Verdict: Does not match greentic-types; both the extension key and required provider fields must align with the definitions above.

## Guidance for consumers
- Use `greentic.provider-extension.v1` as both the `extensions` map key and the `kind` field when embedding provider metadata.
- Populate `ExtensionRef.version` (default `1.0.0` is used by helpers) and prefer `inline` for small payloads; unknown keys are ignored by design.
- Ensure each provider entry supplies `config_schema_ref` and full `runtime` (`component_ref`, `export`, `world`); other fields are optional and may be omitted to keep payloads compact.

## Evidence summary
- Canonical ID declaration and helpers: `src/provider.rs:17-18`, `src/pack_manifest.rs:252-305`.
- Inline schema definition: `src/provider.rs:103-115`, `src/provider.rs:66-101`, `src/provider.rs:53-64`.
- Default versioning: `src/pack_manifest.rs:279-289`.
- Test/fixture usage of canonical ID: `tests/pack_manifest_extensions.rs:24-56`, `tests/fixtures/manifest_with_provider_ext.json:7-30`.
- Conflicting doc example: `docs/pack_manifest_extensions.md:24-35`; “e.g.” comment: `src/pack_manifest.rs:227-228`.

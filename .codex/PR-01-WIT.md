Prompt 1 — greentic-types “Schema Registry + Envelope + Contract Tests” (PR-01)
Repo: greentic-types

Objective:
Implement the “locksmith” mechanism that keeps WIT interfaces and CBOR schemas in lockstep across legacy, 0.6.0, and future versions.

This PR must:
- Introduce a stable CBOR Envelope format (kind/schema/version/body).
- Introduce a schema registry listing every CBOR schema we support.
- Add v0.6.0 typed schemas for pack/component (descriptor + QA + validation stubs).
- Add contract tests + fixtures that prove encode/decode + canonical encoding stability.

Constraints:
- No WIT parsing in this PR; this is types + schema infra only.
- CBOR bytes must be canonical (deterministic ordering).
- All user-facing strings must use I18nText (key + fallback optional).
- Additive evolution must be supported (unknown fields ignored).
- Keep code modular under src/schemas/**.

Plan / tasks (do all):
1) Add `src/i18n_text.rs`:
   - `I18nText { key: String, fallback: Option<String> }`
   - helper `fn keys(&self) -> impl Iterator<Item=&str>` optional.

2) Add `src/envelope.rs`:
   - `Envelope { kind: String, schema: String, version: u32, body: CborBytes }`
   - encode/decode using existing CBOR canonical rules and `CborBytes`.
   - Ensure strict canonicalization on encode; ensure canonical on decode.
   - Add helpers: `Envelope::new(kind, schema, version, body_struct)` and `Envelope::decode_body<T>()`.

3) Create schema module layout:
   - `src/schemas/mod.rs` with `pub mod pack; pub mod component;`
   - `src/schemas/pack/v0_6_0/*`
   - `src/schemas/component/v0_6_0/*`

4) Implement minimal v0.6.0 schema structs (typed, i18n-aware):
   Pack:
   - `PackInfo` (id, version, role, display_name: Option<I18nText>, …)
   - `PackDescribe` (provided capabilities, required capabilities, units summary, optional metadata)
   - `CapabilityDescriptor` (capability_id: String, version_req: String, metadata: Option<CapabilityMetadata>)
   - `CapabilityMetadata` (tags, supports map, constraints map, quality hints, regions, compliance, hints)
   - `PackQaSpec` (mode, title:I18nText, description:Option<I18nText>, questions: Vec<Question>, defaults map)
   - `PackValidationResult` with `ok: bool` and `issues: Vec<Diagnostic>` (Diagnostic uses I18nText)
   Component:
   - `ComponentInfo`
   - `ComponentDescribe`
   - `ComponentQaSpec` same structure as PackQaSpec
   - `ComponentRunInput/Output` placeholder structs (keep generic maps for now)
   Common:
   - `QaMode` enum: default, setup, upgrade, remove
   - `Question` + `QuestionKind` + `ChoiceOption` where all labels/help/option labels/errors are I18nText.
   - Provide `fn i18n_keys(&self) -> BTreeSet<String>` on QA specs.

   Notes:
   - Use `BTreeMap` for deterministic map ordering on encode.
   - Use `serde` + existing CBOR machinery.
   - Keep “constraints”/“hints” values as CBOR-friendly `ciborium::value::Value` (or your existing CBOR Value type) NOT serde_json.

5) Add `src/schema_registry.rs`:
   - Define `SchemaDef { id: &'static str, version: u32, kind: &'static str }`
   - `pub const SCHEMAS: &[SchemaDef] = &[ ... ];`
   Register at least:
   - "greentic.pack.describe@0.6.0"
   - "greentic.pack.qa@0.6.0"
   - "greentic.pack.validation@0.6.0"
   - "greentic.component.describe@0.6.0"
   - "greentic.component.qa@0.6.0"
   Document how to add future versions.

6) Add fixtures + contract tests:
   - Create `fixtures/` folder (if not present):
     - fixtures/pack/describe_v0_6_0.cbor
     - fixtures/pack/qa_setup_v0_6_0.cbor
     - fixtures/component/qa_default_v0_6_0.cbor
   - Tests:
     - encode struct -> bytes canonical -> decode back equals
     - decode fixture -> struct -> encode -> bytes identical to fixture (byte-for-byte)
     - Envelope roundtrip
     - QA i18n key enumeration returns expected set

7) Update crate exports:
   - Re-export `I18nText`, `Envelope`, schema types, `SCHEMAS`.
   - Keep backwards compatibility where reasonable.

Acceptance criteria:
- `cargo test` passes.
- All fixtures roundtrip exactly.
- No serde_json is introduced in new schema types.
- All user-facing strings are I18nText.
- A clear README section or doc comment explains:
  “WIT ABI stays stable; CBOR schema evolves via schema ids and versions; registry + fixtures enforce lockstep.”

Do not ask me questions unless something is genuinely impossible; prefer a reasonable default and proceed.
Prompt 2 — “Legacy adapters” + one concrete migration (PR-02)
Repo: greentic-types (or greentic-adapters if you decide to create a small sibling crate inside the workspace)

Objective:
Add legacy-to-0.6.0 adapters so we can support old WIT/JSON-era payloads while standardizing on v0.6.0 typed CBOR schemas.

Scope:
- Implement one high-value adapter end-to-end to prove the pattern:
  Legacy component QA (0.5.0 JSON-like spec) -> v0.6.0 `ComponentQaSpec` (CBOR typed, i18n-aware).
- Add tests + fixtures.

Constraints:
- Do NOT modify legacy WIT packages.
- Adapters must be isolated: core v0.6.0 types remain clean.
- Use deterministic CBOR output.
- i18n: legacy raw strings become I18nText with generated keys + fallback set to original string.
  (Key generation rule below.)

Tasks:
1) Create module `src/adapters/mod.rs` and `src/adapters/component_v0_5_to_v0_6.rs`.

2) Define input model for legacy QA:
   - If legacy QA is currently “untyped JSON”, create a minimal struct that matches the legacy shape you see in the repo.
   - If it’s ambiguous, parse via `serde_json::Value` ONLY inside this adapter module (allowed here).
   - The adapter output must be `schemas::component::v0_6_0::ComponentQaSpec`.

3) Key generation for i18n:
   - For each legacy label/help/option label string:
     - generate key: `legacy.component.v0_5.<question_id>.<field>` (field = label/help/option.<value>)
     - set fallback = original string
   This keeps i18n deterministic and non-lossy.

4) Mapping rules:
   - Legacy “setup questions” -> QaMode::setup
   - Legacy “defaults/minimal” -> QaMode::default (can be derived by taking only required minimal questions or using defaults)
   - If legacy has no explicit “upgrade/remove”, generate empty specs for those modes (title+description only, no questions).
   - Convert choice lists to ChoiceOption with I18nText labels.
   - Preserve default values into `defaults`.

5) Add adapter entrypoint:
   - `fn adapt_qa_spec(mode: QaMode, legacy_bytes_or_json: ...) -> Result<CborBytes>`
   - returns canonical CBOR of ComponentQaSpec for that mode.

6) Add fixtures + tests:
   - fixtures/legacy/component_v0_5_qa.json (or .cbor if it exists)
   - tests that:
     - parse legacy fixture
     - adapt to v0.6.0
     - decode CBOR into ComponentQaSpec
     - verify:
       - i18n keys generated as per rule
       - fallbacks preserved
       - canonical encode stable (roundtrip equality)

Acceptance criteria:
- Adapter compiles and tests pass.
- v0.6.0 output is fully i18n-aware (no raw text in schema structs).
- Core schemas remain serde_json-free; serde_json usage is limited to adapters module only.
- The adapter pattern is documented (how to add more legacy adapters).

Proceed with sensible defaults; do not ask for permission for routine file edits/tests.
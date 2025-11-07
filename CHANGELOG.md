# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]
- Added the schema exporter binary, GitHub Pages workflow, and [`SCHEMAS.md`](SCHEMAS.md) so IDEs/CLIs can validate documents against canonical `$id`s.
- Documented feature flags + MSRV (Rust 1.85), introduced the `schema`/`otel-keys` flags, and exposed the crate `VERSION` constant.
- Hardened ID newtypes and `SemverReq` with `FromStr`/`TryFrom` implementations, serde guards, and property tests ensuring invalid identifiers cannot deserialize.
- Added CI checks for duplicate struct definitions and public schema `$id` sanity.
- Added `pack_spec` module with canonical `PackSpec` and `ToolSpec` structures for `pack.yaml` parsing.
- Introduced shared deployment context primitives (`Cloud`, `Platform`, `DeploymentCtx`) and made them available via `greentic_types`.

## [0.1.0] - 2025-10-23
- Initial release with tenant identifiers, context, and invocation envelope
- Added `NodeError`/`NodeResult` with retry and detail helpers
- Added idempotency key and safe JSON helpers with unit tests

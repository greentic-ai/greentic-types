# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]
- Added generic `EventEnvelope` + `EventId`, provider capability descriptors (`EventProviderDescriptor` with `EventProviderKind`, transports, reliability, ordering), and shared `ChannelMessageEnvelope` + `Attachment` for channel messaging; all derive serde/schemars and export JSON Schemas.
- Added supply-chain shared types and ID newtypes (RepoRef, ComponentRef, BuildRef, ScanRef, AttestationRef, PolicyRef, StoreRef, RegistryRef) plus BuildPlan/BuildStatus, ScanRequest/ScanResult, signing/verification refs, attestation statements, metadata records, and contexts, with schema exports and serde round-trip tests.
- Added `PackKind` to `PackManifest`, optional `host.iac` capabilities for components, and the generic `DeploymentPlan` family shared across runner/deployer repositories.
- Added the schema exporter binary, GitHub Pages workflow, and [`SCHEMAS.md`](SCHEMAS.md) so IDEs/CLIs can validate documents against canonical `$id`s.
- Documented feature flags + MSRV (Rust 1.85), introduced the `schema`/`otel-keys` flags, and exposed the crate `VERSION` constant.
- Hardened ID newtypes and `SemverReq` with `FromStr`/`TryFrom` implementations, serde guards, and property tests ensuring invalid identifiers cannot deserialize.
- Added CI checks for duplicate struct definitions and public schema `$id` sanity.
- Added `pack_spec` module with canonical `PackSpec` and `ToolSpec` structures for `pack.yaml` parsing.
- Introduced shared deployment context primitives (`Cloud`, `Platform`, `DeploymentCtx`) and made them available via `greentic_types`.
- Added generic `.ygtc` flow models, component manifests, and `.gtpack` manifest types (plus schema exports) under `flow`, `component`, and `pack_manifest`.
- Exposed helper APIs like `Flow::ingress`, `Flow::validate_components`, and `ComponentManifest::select_profile` to keep validation/profile logic consistent across repos.
- Added `MODELS.md` + README guidance describing the opaque-ID/capabilities-only design and marked `pack_spec` as legacy for migration planning.

## [0.1.0] - 2025-10-23
- Initial release with tenant identifiers, context, and invocation envelope
- Added `NodeError`/`NodeResult` with retry and detail helpers
- Added idempotency key and safe JSON helpers with unit tests

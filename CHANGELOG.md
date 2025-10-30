# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]
- Added `pack_spec` module with canonical `PackSpec` and `ToolSpec` structures for `pack.yaml` parsing.
- Introduced shared deployment context primitives (`Cloud`, `Platform`, `DeploymentCtx`) and made them available via `greentic_types`.

## [0.1.0] - 2025-10-23
- Initial release with tenant identifiers, context, and invocation envelope
- Added `NodeError`/`NodeResult` with retry and detail helpers
- Added idempotency key and safe JSON helpers with unit tests

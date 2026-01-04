# Pack manifest extensions

Pack manifests support forward-compatible extensions so provider-specific metadata can live outside the core types. Extensions are referenced from the `extensions` map on `PackManifest`, keyed by extension identifier.

Only `greentic.provider-extension.v1` is supported for provider metadata; other keys are treated as unknown extensions.

## Extension resolution
- If `inline` is present, use it as the authoritative payload.
- Otherwise, fetch the payload from `location` (file path, `https://`, etc.).
- When both `location` and `digest` are set, verify the fetched payload matches the digest before use.
- Parsers must ignore unknown extension keys; the core manifest remains valid even if extensions are missing or unrecognized.

## Pinning and integrity
- Set `digest` (e.g. `sha256:<hex>`) whenever the payload is remote to prevent tampering.
- In strict mode, require `digest` for any remote `location` and fail resolution if the digest is absent or mismatched.
- Include `version` as a string and keep it aligned with the referenced payload to avoid semver coupling in the core types.
- When shipping extensions alongside the pack, prefer `inline` for tiny payloads and `location` + `digest` for larger blobs.

## Best practices
- Ship a JSON Schema for each extension in the pack (e.g. under `schemas/`) so tooling can validate without network access.
- Keep inline payloads small and cacheable; store larger artifacts at a pinned location.
- Treat extensions as optional: runtimes should continue even if an unknown extension is present or cannot be resolved, unless the consuming tool explicitly requires it.

## Examples

Inline payload for a small provider hint:

```json
{
  "extensions": {
    "greentic.provider-extension.v1": {
      "kind": "greentic.provider-extension.v1",
      "version": "1.0.0",
      "inline": { "capabilities": ["messaging"] }
    }
  }
}
```

Remote payload pinned by digest:

```json
{
  "extensions": {
    "vendor.ext.audit": {
      "kind": "vendor.ext.audit",
      "version": "2.0.0",
      "location": "https://example.com/extensions/audit.json",
      "digest": "sha256:4c6f2ad3..."
    }
  }
}
```

#!/usr/bin/env bash
set -euo pipefail

SCHEMAS=(
  "https://greentic-ai.github.io/greentic-types/schemas/v1/pack-id.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/component-id.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/flow-id.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/node-id.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/tenant-context.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/hash-digest.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/semver-req.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/redaction-path.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/capabilities.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/limits.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/telemetry-spec.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/node-summary.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/node-failure.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/node-status.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/run-status.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/transcript-offset.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/tools-caps.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/secrets-caps.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/otlp-keys.schema.json"
  "https://greentic-ai.github.io/greentic-types/schemas/v1/run-result.schema.json"
)

for url in "${SCHEMAS[@]}"; do
  echo "Checking schema $url"
  body=$(curl -fsS "$url")
  schema_id=$(python3 - <<'PY'
import json, sys
print(json.loads(sys.stdin.read()).get("$id", ""))
PY
<<<"$body")
  if [[ "$schema_id" != "$url" ]]; then
    echo "Schema $url has mismatched $id ('$schema_id')" >&2
    exit 1
  fi
  echo "✔ $url"
done

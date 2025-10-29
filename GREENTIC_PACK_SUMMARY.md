# 🧩 Greentic Pack & Flow Model — Core Principles (2025-10)

## 1. Overall Concept
Greentic now uses **Packs** — self-contained, signed Wasm (or `.wpk`) artifacts that embed:
- Flows (`.ygtc` or compiled IR),
- Templates, tools (embedded Wasm or referenced by digest),
- A manifest (JSON/CBOR) describing flows, assets, imports, and capabilities,
- Optional embeddings (for A2A search),
- A digital signature (cosign/ed25519).

Each Pack exports a **standard WIT surface**:
```
list_flows() -> [FlowInfo]
get_flow_schema(flow_id)
run_flow(flow_id, input)
a2a_search(embedding, k)
```
and imports minimal host capabilities:
```
secrets.get, telemetry.emit, tool.invoke, http.fetch (optional)
```

The host verifies the signature, enforces quotas and secret policies, and provides adapters for **flow types** such as messaging, webhook, timer, websocket, and pubsub.

---

## 2. Flow-Type Architecture
- Flows no longer use explicit “channels”.  
- Each flow declares a **`type:`** defining its ingress/egress:
  - `messaging` → Telegram, Slack, Teams, etc.
  - `webhook` → HTTP ingress/response.
  - `timer` → cron/scheduler.
  - `websocket` → bidirectional connections.
  - `pubsub` → event topics.
- The **host** chooses the concrete adapter per tenant; the flow itself stays portable.

---

## 3. Flow Structure (`.ygtc`)
A flow is a small YAML graph with simple nodes:  
`qa`, `tool`, `template`, `emit`.

Example:
```yaml
id: weather_bot
title: Weather via Messaging
description: Ask for a city, call Weather API, reply with a multi-day text template, then end.
type: messaging

parameters:
  days_default: 3

nodes:
  in:
    qa:
      welcome: "Hi there! Let's get your weather forecast."
      questions:
        - id: q_location
          prompt: "👉 What location would you like a forecast for?"
          answer_type: text
          max_words: 3
      routing:
        - to: forecast_weather

  forecast_weather:
    tool:
      name: weather_api
      action: forecast_weather
    parameters:
      q: in.q_location
      days: parameters.days_default
    routing:
      - to: weather_text

  weather_text:
    template:
      text: |
        Weather for {{forecast_weather.payload.location.name}}, {{forecast_weather.payload.location.country}}:
        Now: {{forecast_weather.payload.current.condition.text}}, {{forecast_weather.payload.current.temp_c}}°C
        Forecast (next {{parameters.days_default}} day(s)):
        {{#forecast_weather.payload.forecast}}
        - {{date}}: ↑ {{day.maxtemp_c}}°C | ↓ {{day.mintemp_c}}°C — {{day.condition.text}}
        {{/forecast_weather.payload.forecast}}
        Thanks! Type /start to check another city.
    routing:
      - out
```

---

## 4. Node-to-Component Mapping
| Node type | Component (existing WIT) | Notes |
|------------|--------------------------|--------|
| `qa` | `qa.process` | e.g. Ollama LLM or RAG planner |
| `tool` | `tool.exec` | executes embedded or external tools (e.g. `weatherapi.wasm`) |
| `template` | `templating.handlebars` | renders inline or packaged templates |
| `emit` | handled by host | routes to adapter based on `flow.type` |

> ✅ No new interfaces — all nodes map to **existing component WITs**.

---

## 5. Pack Manifest Essentials
Inside the `.data` or custom section:

```json
{
  "pack_id": "greentic.weather.demo",
  "version": "0.1.0",
  "flows": [{ "id": "weather_bot", "type": "messaging" }],
  "tools": [{ "name": "weather_api", "digest": "sha256:...", "embedded": true }],
  "imports_required": ["secrets.get","telemetry.emit","tool.invoke"],
  "policies": { "memory": 128, "timeout_ms": 5000 },
  "signature": { "alg": "ed25519", "sig": "..." }
}
```

---

## 6. Host Runtime Responsibilities
- Verify signature & blob hashes before load.
- Provide `secrets.get`, `telemetry.emit`, and `tool.invoke` imports.
- Enforce per-tenant resource quotas, secret access, and network policy.
- Manage adapters for each **flow type** (Telegram, HTTP, cron, etc.).
- Track traces & metrics (OTLP spans labelled `{tenant, flow, node, provider}`).
- Retry, rate-limit, and deduplicate according to manifest policies.

---

## 7. Binding / Policy Example
```yaml
tenant: acme
flow_type_bindings:
  messaging:
    adapter: telegram
    config: { bot_name: "WeatherDemo" }
    secrets: [TELEGRAM_BOT_TOKEN]

components:
  qa.process:
    impl: ollama
    config:
      base_url: ${OLLAMA_BASE_URL}
      model: ${OLLAMA_MODEL:qwen2.5:14b}

  templating.handlebars: {}
  tool.exec:
    tool_map:
      weather_api:
        source: embedded
        action_map:
          forecast_weather: { entry: forecast_weather }

secrets:
  TELEGRAM_BOT_TOKEN: ${ENV.TELEGRAM_BOT_TOKEN}
  OLLAMA_BASE_URL: http://localhost:11434
```

---

## 8. Developer Flow
1. **Write flow(s)** (`flows/*.ygtc`).
2. **Add tools/templates** if needed.
3. **Define manifest (`pack.yaml`)**.
4. **Run `packc build`** → produces signed `pack.wasm`.
5. **Host loads** with bindings + secrets.
6. **Ingress event** → `run_flow(flow_id, input)` → components + emits → egress via flow type.

---

## 9. Example Runtime Chain
```
messaging ingress (Telegram) 
→ qa.process (Ollama) 
→ tool.exec (weatherapi.wasm)
→ templating.handlebars 
→ emit (messaging → Telegram send)
```

---

## 10. Security & Performance
- No secrets inside pack; all fetched via `secrets.get`.
- Host enforces capability whitelist and per-tenant policies.
- Packs are signed + content-addressed (immutable).
- Pool Wasm instances for low cold-start latency.
- Optional AOT compile/cache and per-flow pre-warm.

---

## 11. Distribution & Deployment
- Packs are OCI-publishable (`oci://greentic.ai/packs/weather-demo@v0.1.0`).
- Host can run in:
  - **Scratch container** (default),
  - **MicroVM / Unikernel** (isolation),
  - **Edge Worker** (restricted mode).
- Helm chart mounts packs, binds secrets, and sets webhook/timer adapters.

---

## 12. Minimal Requirements for a New Flow Pack
✅ Flow `.ygtc` file(s)  
✅ Optional templates and tools  
✅ `pack.yaml` manifest  
✅ Signature (via pack builder)  
✅ Tested under `greentic-pack-host`  

---

### TL;DR
> **Greentic Flows = tiny YAML graphs inside signed Wasm Packs.**  
> Each node maps to an existing WIT component (qa, tool, template).  
> Flows have types (`messaging`, `webhook`, `timer`, …); the host provides adapters for ingress/egress.  
> Packs are portable, self-contained, signed artifacts — they can run anywhere a verified host exists.

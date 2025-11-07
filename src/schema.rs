#![cfg(all(feature = "schemars", feature = "std"))]

//! Helper functions that expose JSON Schemas with stable `$id`s.

use alloc::vec::Vec;

#[cfg(feature = "time")]
use crate::run::RunResult;
#[cfg(feature = "otel-keys")]
use crate::telemetry::OtlpKeys;
use crate::{
    Capabilities, ComponentId, FlowId, HashDigest, Limits, NodeFailure, NodeId, NodeStatus,
    NodeSummary, PackId, RedactionPath, RunStatus, SecretsCaps, SemverReq, TelemetrySpec,
    TenantContext, ToolsCaps, TranscriptOffset, ids,
};
use schemars::{JsonSchema, Schema, schema_for};

fn schema_with_id<T: JsonSchema>(id: &str) -> Schema {
    let mut schema: Schema = schema_for!(T);
    if schema.get("$id").is_none() {
        schema.insert("$id".to_owned(), id.into());
    }
    schema
}

/// Internal descriptor describing a schema export.
#[derive(Clone, Copy)]
pub(crate) struct SchemaEntry {
    /// Output file name (including `.schema.json`).
    pub file_name: &'static str,
    /// Generator used to materialise the schema document.
    pub generator: fn() -> Schema,
}

macro_rules! define_schema_fn {
    ($(#[$meta:meta])* $fn_name:ident, $ty:ty, $id_const:expr) => {
        $(#[$meta])*
        #[doc = concat!(
            "Returns the JSON Schema for ",
            stringify!($ty),
            " with the canonical ID."
        )]
        pub fn $fn_name() -> Schema {
            schema_with_id::<$ty>($id_const)
        }
    };
}

define_schema_fn!(pack_id, PackId, ids::PACK_ID);
define_schema_fn!(component_id, ComponentId, ids::COMPONENT_ID);
define_schema_fn!(flow_id, FlowId, ids::FLOW_ID);
define_schema_fn!(node_id, NodeId, ids::NODE_ID);
define_schema_fn!(tenant_context, TenantContext, ids::TENANT_CONTEXT);
define_schema_fn!(hash_digest, HashDigest, ids::HASH_DIGEST);
define_schema_fn!(semver_req, SemverReq, ids::SEMVER_REQ);
define_schema_fn!(redaction_path, RedactionPath, ids::REDACTION_PATH);
define_schema_fn!(capabilities, Capabilities, ids::CAPABILITIES);
define_schema_fn!(limits, Limits, ids::LIMITS);
define_schema_fn!(telemetry_spec, TelemetrySpec, ids::TELEMETRY_SPEC);
define_schema_fn!(node_summary, NodeSummary, ids::NODE_SUMMARY);
define_schema_fn!(node_failure, NodeFailure, ids::NODE_FAILURE);
define_schema_fn!(node_status, NodeStatus, ids::NODE_STATUS);
define_schema_fn!(run_status, RunStatus, ids::RUN_STATUS);
define_schema_fn!(transcript_offset, TranscriptOffset, ids::TRANSCRIPT_OFFSET);
define_schema_fn!(tools_caps, ToolsCaps, ids::TOOLS_CAPS);
define_schema_fn!(secrets_caps, SecretsCaps, ids::SECRETS_CAPS);
#[cfg(feature = "otel-keys")]
define_schema_fn!(otlp_keys, OtlpKeys, ids::OTLP_KEYS);
#[cfg(feature = "time")]
define_schema_fn!(run_result, RunResult, ids::RUN_RESULT);

macro_rules! schema_entries_vec {
    ( $( $(#[$meta:meta])* { $fn_name:ident, $slug:literal, $id_const:expr } ),+ $(,)? ) => {
        pub(crate) fn entries() -> Vec<SchemaEntry> {
            let mut entries = Vec::new();
            $(
                $(#[$meta])*
                {
                    entries.push(SchemaEntry {
                        file_name: concat!($slug, ".schema.json"),
                        generator: $fn_name,
                    });
                }
            )+
            entries
        }
    };
}

schema_entries_vec! {
    { pack_id, "pack-id", ids::PACK_ID },
    { component_id, "component-id", ids::COMPONENT_ID },
    { flow_id, "flow-id", ids::FLOW_ID },
    { node_id, "node-id", ids::NODE_ID },
    { tenant_context, "tenant-context", ids::TENANT_CONTEXT },
    { hash_digest, "hash-digest", ids::HASH_DIGEST },
    { semver_req, "semver-req", ids::SEMVER_REQ },
    { redaction_path, "redaction-path", ids::REDACTION_PATH },
    { capabilities, "capabilities", ids::CAPABILITIES },
    { limits, "limits", ids::LIMITS },
    { telemetry_spec, "telemetry-spec", ids::TELEMETRY_SPEC },
    { node_summary, "node-summary", ids::NODE_SUMMARY },
    { node_failure, "node-failure", ids::NODE_FAILURE },
    { node_status, "node-status", ids::NODE_STATUS },
    { run_status, "run-status", ids::RUN_STATUS },
    { transcript_offset, "transcript-offset", ids::TRANSCRIPT_OFFSET },
    { tools_caps, "tools-caps", ids::TOOLS_CAPS },
    { secrets_caps, "secrets-caps", ids::SECRETS_CAPS },
    #[cfg(feature = "otel-keys")]
    { otlp_keys, "otlp-keys", ids::OTLP_KEYS },
    #[cfg(feature = "time")]
    { run_result, "run-result", ids::RUN_RESULT },
}

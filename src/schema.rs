#![cfg(all(feature = "schemars", feature = "std"))]

//! Helper functions that expose JSON Schemas with stable `$id`s.

use alloc::vec::Vec;

#[cfg(feature = "time")]
use crate::run::RunResult;
#[cfg(feature = "otel-keys")]
use crate::telemetry::OtlpKeys;
use crate::{
    ArtifactRef, Attachment, AttestationRef, AttestationStatement, BuildPlan, BuildRef,
    BuildStatus, Capabilities, ChannelMessageEnvelope, ComponentId, ComponentManifest,
    ComponentRef, EventEnvelope, EventProviderDescriptor, Flow, FlowId, HashDigest, Limits,
    MetadataRecord, Node, NodeFailure, NodeId, NodeStatus, NodeSummary, PackId, PackManifest,
    PolicyRef, RedactionPath, RegistryRef, RepoContext, RepoRef, RunStatus, SbomRef, ScanRef,
    ScanRequest, ScanResult, SecretsCaps, SemverReq, SignRequest, SignatureRef, SigningKeyRef,
    StatementRef, StoreContext, StoreRef, TelemetrySpec, TenantContext, ToolsCaps,
    TranscriptOffset, VerifyRequest, VerifyResult, ids,
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
define_schema_fn!(flow, Flow, ids::FLOW);
define_schema_fn!(node, Node, ids::NODE);
define_schema_fn!(
    component_manifest,
    ComponentManifest,
    ids::COMPONENT_MANIFEST
);
define_schema_fn!(pack_manifest, PackManifest, ids::PACK_MANIFEST);
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
define_schema_fn!(repo_ref, RepoRef, ids::REPO_REF);
define_schema_fn!(component_ref, ComponentRef, ids::COMPONENT_REF);
define_schema_fn!(build_ref, BuildRef, ids::BUILD_REF);
define_schema_fn!(scan_ref, ScanRef, ids::SCAN_REF);
define_schema_fn!(attestation_ref, AttestationRef, ids::ATTESTATION_REF);
define_schema_fn!(policy_ref, PolicyRef, ids::POLICY_REF);
define_schema_fn!(store_ref, StoreRef, ids::STORE_REF);
define_schema_fn!(registry_ref, RegistryRef, ids::REGISTRY_REF);
define_schema_fn!(artifact_ref, ArtifactRef, ids::ARTIFACT_REF);
define_schema_fn!(sbom_ref, SbomRef, ids::SBOM_REF);
define_schema_fn!(signing_key_ref, SigningKeyRef, ids::SIGNING_KEY_REF);
define_schema_fn!(signature_ref, SignatureRef, ids::SIGNATURE_REF);
define_schema_fn!(statement_ref, StatementRef, ids::STATEMENT_REF);
define_schema_fn!(build_plan, BuildPlan, ids::BUILD_PLAN);
define_schema_fn!(build_status, BuildStatus, ids::BUILD_STATUS);
define_schema_fn!(scan_request, ScanRequest, ids::SCAN_REQUEST);
define_schema_fn!(scan_result, ScanResult, ids::SCAN_RESULT);
define_schema_fn!(sign_request, SignRequest, ids::SIGN_REQUEST);
define_schema_fn!(verify_request, VerifyRequest, ids::VERIFY_REQUEST);
define_schema_fn!(verify_result, VerifyResult, ids::VERIFY_RESULT);
define_schema_fn!(
    attestation_statement,
    AttestationStatement,
    ids::ATTESTATION_STATEMENT
);
define_schema_fn!(metadata_record, MetadataRecord, ids::METADATA_RECORD);
define_schema_fn!(repo_context, RepoContext, ids::REPO_CONTEXT);
define_schema_fn!(store_context, StoreContext, ids::STORE_CONTEXT);
define_schema_fn!(event_envelope, EventEnvelope, ids::EVENT_ENVELOPE);
define_schema_fn!(
    event_provider_descriptor,
    EventProviderDescriptor,
    ids::EVENT_PROVIDER_DESCRIPTOR
);
define_schema_fn!(
    channel_message_envelope,
    ChannelMessageEnvelope,
    ids::CHANNEL_MESSAGE_ENVELOPE
);
define_schema_fn!(attachment, Attachment, ids::ATTACHMENT);
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
    { flow, "flow", ids::FLOW },
    { node, "node", ids::NODE },
    { component_manifest, "component-manifest", ids::COMPONENT_MANIFEST },
    { pack_manifest, "pack-manifest", ids::PACK_MANIFEST },
    { limits, "limits", ids::LIMITS },
    { telemetry_spec, "telemetry-spec", ids::TELEMETRY_SPEC },
    { node_summary, "node-summary", ids::NODE_SUMMARY },
    { node_failure, "node-failure", ids::NODE_FAILURE },
    { node_status, "node-status", ids::NODE_STATUS },
    { run_status, "run-status", ids::RUN_STATUS },
    { transcript_offset, "transcript-offset", ids::TRANSCRIPT_OFFSET },
    { tools_caps, "tools-caps", ids::TOOLS_CAPS },
    { secrets_caps, "secrets-caps", ids::SECRETS_CAPS },
    { repo_ref, "repo-ref", ids::REPO_REF },
    { component_ref, "component-ref", ids::COMPONENT_REF },
    { build_ref, "build-ref", ids::BUILD_REF },
    { scan_ref, "scan-ref", ids::SCAN_REF },
    { attestation_ref, "attestation-ref", ids::ATTESTATION_REF },
    { policy_ref, "policy-ref", ids::POLICY_REF },
    { store_ref, "store-ref", ids::STORE_REF },
    { registry_ref, "registry-ref", ids::REGISTRY_REF },
    { artifact_ref, "artifact-ref", ids::ARTIFACT_REF },
    { sbom_ref, "sbom-ref", ids::SBOM_REF },
    { signing_key_ref, "signing-key-ref", ids::SIGNING_KEY_REF },
    { signature_ref, "signature-ref", ids::SIGNATURE_REF },
    { statement_ref, "statement-ref", ids::STATEMENT_REF },
    { build_plan, "build-plan", ids::BUILD_PLAN },
    { build_status, "build-status", ids::BUILD_STATUS },
    { scan_request, "scan-request", ids::SCAN_REQUEST },
    { scan_result, "scan-result", ids::SCAN_RESULT },
    { sign_request, "sign-request", ids::SIGN_REQUEST },
    { verify_request, "verify-request", ids::VERIFY_REQUEST },
    { verify_result, "verify-result", ids::VERIFY_RESULT },
    { attestation_statement, "attestation-statement", ids::ATTESTATION_STATEMENT },
    { metadata_record, "metadata-record", ids::METADATA_RECORD },
    { repo_context, "repo-context", ids::REPO_CONTEXT },
    { store_context, "store-context", ids::STORE_CONTEXT },
    { event_envelope, "event-envelope", ids::EVENT_ENVELOPE },
    { event_provider_descriptor, "event-provider-descriptor", ids::EVENT_PROVIDER_DESCRIPTOR },
    { channel_message_envelope, "channel-message-envelope", ids::CHANNEL_MESSAGE_ENVELOPE },
    { attachment, "attachment", ids::ATTACHMENT },
    #[cfg(feature = "otel-keys")]
    { otlp_keys, "otlp-keys", ids::OTLP_KEYS },
    #[cfg(feature = "time")]
    { run_result, "run-result", ids::RUN_RESULT },
}

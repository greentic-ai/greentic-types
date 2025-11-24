#![cfg(all(feature = "schemars", feature = "std"))]

//! Helper functions that expose JSON Schemas with stable `$id`s.

use alloc::vec::Vec;

#[cfg(feature = "time")]
use crate::run::RunResult;
#[cfg(feature = "otel-keys")]
use crate::telemetry::OtlpKeys;
use crate::{
    ArtifactRef, Attachment, AttestationRef, AttestationStatement, BuildPlan, BuildRef,
    BuildStatus, BundleSpec, Capabilities, CapabilityMap, ChannelMessageEnvelope, Collection,
    ComponentId, ComponentManifest, ComponentRef, ConnectionKind, DesiredState,
    DesiredStateExportSpec, DesiredSubscriptionEntry, Environment, EnvironmentRef, EventEnvelope,
    EventProviderDescriptor, Flow, FlowId, HashDigest, LayoutSection, Limits, MetadataRecord,
    MetadataRecordRef, Node, NodeFailure, NodeId, NodeStatus, NodeSummary, PackId, PackManifest,
    PackOrComponentRef, PlanLimits, PolicyRef, PriceModel, ProductOverride, RedactionPath,
    RegistryRef, RepoContext, RepoRef, RunStatus, SbomRef, ScanRef, ScanRequest, ScanResult,
    SecretsCaps, SemverReq, SignRequest, SignatureRef, SigningKeyRef, StatementRef, StoreContext,
    StoreFront, StorePlan, StoreProduct, StoreProductKind, StoreRef, Subscription,
    SubscriptionStatus, TelemetrySpec, TenantContext, Theme, ToolsCaps, TranscriptOffset,
    VerifyRequest, VerifyResult, VersionStrategy, ids,
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
define_schema_fn!(
    metadata_record_ref,
    MetadataRecordRef,
    ids::METADATA_RECORD_REF
);
define_schema_fn!(environment_ref, EnvironmentRef, ids::ENVIRONMENT_REF);
define_schema_fn!(distributor_ref, crate::DistributorRef, ids::DISTRIBUTOR_REF);
define_schema_fn!(storefront_id, crate::StoreFrontId, ids::STOREFRONT_ID);
define_schema_fn!(
    store_product_id,
    crate::StoreProductId,
    ids::STORE_PRODUCT_ID
);
define_schema_fn!(store_plan_id, crate::StorePlanId, ids::STORE_PLAN_ID);
define_schema_fn!(subscription_id, crate::SubscriptionId, ids::SUBSCRIPTION_ID);
define_schema_fn!(bundle_id, crate::BundleId, ids::BUNDLE_ID);
define_schema_fn!(collection_id, crate::CollectionId, ids::COLLECTION_ID);
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
define_schema_fn!(bundle_spec, BundleSpec, ids::BUNDLE);
define_schema_fn!(
    desired_state_export_spec,
    DesiredStateExportSpec,
    ids::DESIRED_STATE_EXPORT
);
define_schema_fn!(desired_state, DesiredState, ids::DESIRED_STATE);
define_schema_fn!(
    desired_subscription_entry,
    DesiredSubscriptionEntry,
    ids::DESIRED_SUBSCRIPTION_ENTRY
);
define_schema_fn!(storefront, StoreFront, ids::STOREFRONT);
define_schema_fn!(store_product, StoreProduct, ids::STORE_PRODUCT);
define_schema_fn!(store_plan, StorePlan, ids::STORE_PLAN);
define_schema_fn!(capability_map, CapabilityMap, ids::CAPABILITY_MAP);
define_schema_fn!(subscription, Subscription, ids::SUBSCRIPTION);
define_schema_fn!(environment, Environment, ids::ENVIRONMENT);
define_schema_fn!(theme, Theme, ids::THEME);
define_schema_fn!(layout_section, LayoutSection, ids::LAYOUT_SECTION);
define_schema_fn!(collection, Collection, ids::COLLECTION);
define_schema_fn!(product_override, ProductOverride, ids::PRODUCT_OVERRIDE);
define_schema_fn!(
    store_product_kind,
    StoreProductKind,
    ids::STORE_PRODUCT_KIND
);
define_schema_fn!(version_strategy, VersionStrategy, ids::VERSION_STRATEGY);
define_schema_fn!(connection_kind, ConnectionKind, ids::CONNECTION_KIND);
define_schema_fn!(
    pack_or_component_ref,
    PackOrComponentRef,
    ids::PACK_OR_COMPONENT_REF
);
define_schema_fn!(plan_limits, PlanLimits, ids::PLAN_LIMITS);
define_schema_fn!(price_model, PriceModel, ids::PRICE_MODEL);
define_schema_fn!(
    subscription_status,
    SubscriptionStatus,
    ids::SUBSCRIPTION_STATUS
);
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
    { metadata_record_ref, "metadata-record-ref", ids::METADATA_RECORD_REF },
    { environment_ref, "environment-ref", ids::ENVIRONMENT_REF },
    { distributor_ref, "distributor-ref", ids::DISTRIBUTOR_REF },
    { storefront_id, "storefront-id", ids::STOREFRONT_ID },
    { store_product_id, "store-product-id", ids::STORE_PRODUCT_ID },
    { store_plan_id, "store-plan-id", ids::STORE_PLAN_ID },
    { subscription_id, "subscription-id", ids::SUBSCRIPTION_ID },
    { bundle_id, "bundle-id", ids::BUNDLE_ID },
    { collection_id, "collection-id", ids::COLLECTION_ID },
    { metadata_record_ref, "metadata-record-ref", ids::METADATA_RECORD_REF },
    { environment_ref, "environment-ref", ids::ENVIRONMENT_REF },
    { distributor_ref, "distributor-ref", ids::DISTRIBUTOR_REF },
    { storefront_id, "storefront-id", ids::STOREFRONT_ID },
    { store_product_id, "store-product-id", ids::STORE_PRODUCT_ID },
    { store_plan_id, "store-plan-id", ids::STORE_PLAN_ID },
    { subscription_id, "subscription-id", ids::SUBSCRIPTION_ID },
    { bundle_id, "bundle-id", ids::BUNDLE_ID },
    { collection_id, "collection-id", ids::COLLECTION_ID },
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
    { bundle_spec, "bundle", ids::BUNDLE },
    { desired_state_export_spec, "desired-state-export", ids::DESIRED_STATE_EXPORT },
    { desired_state, "desired-state", ids::DESIRED_STATE },
    { desired_subscription_entry, "desired-subscription-entry", ids::DESIRED_SUBSCRIPTION_ENTRY },
    { storefront, "storefront", ids::STOREFRONT },
    { store_product, "store-product", ids::STORE_PRODUCT },
    { store_plan, "store-plan", ids::STORE_PLAN },
    { capability_map, "capability-map", ids::CAPABILITY_MAP },
    { subscription, "subscription", ids::SUBSCRIPTION },
    { environment, "environment", ids::ENVIRONMENT },
    { theme, "theme", ids::THEME },
    { layout_section, "layout-section", ids::LAYOUT_SECTION },
    { collection, "collection", ids::COLLECTION },
    { product_override, "product-override", ids::PRODUCT_OVERRIDE },
    { store_product_kind, "store-product-kind", ids::STORE_PRODUCT_KIND },
    { version_strategy, "version-strategy", ids::VERSION_STRATEGY },
    { connection_kind, "connection-kind", ids::CONNECTION_KIND },
    { pack_or_component_ref, "pack-or-component-ref", ids::PACK_OR_COMPONENT_REF },
    { plan_limits, "plan-limits", ids::PLAN_LIMITS },
    { price_model, "price-model", ids::PRICE_MODEL },
    { subscription_status, "subscription-status", ids::SUBSCRIPTION_STATUS },
    #[cfg(feature = "otel-keys")]
    { otlp_keys, "otlp-keys", ids::OTLP_KEYS },
    #[cfg(feature = "time")]
    { run_result, "run-result", ids::RUN_RESULT },
}

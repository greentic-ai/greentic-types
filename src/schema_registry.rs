//! Registry of supported canonical CBOR schemas.

/// Schema definition entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaDef {
    /// Schema identifier.
    pub id: &'static str,
    /// Schema version.
    pub version: u32,
    /// Schema kind.
    pub kind: &'static str,
}

/// List of canonical schemas supported by this crate.
///
/// Add new schema entries in semver order and keep identifiers stable.
pub const SCHEMAS: &[SchemaDef] = &[
    SchemaDef {
        id: "greentic.pack.describe@0.6.0",
        version: 6,
        kind: "pack",
    },
    SchemaDef {
        id: "greentic.pack.qa@0.6.0",
        version: 6,
        kind: "pack",
    },
    SchemaDef {
        id: "greentic.pack.validation@0.6.0",
        version: 6,
        kind: "pack",
    },
    SchemaDef {
        id: "greentic.component.describe@0.6.0",
        version: 6,
        kind: "component",
    },
    SchemaDef {
        id: "greentic.component.qa@0.6.0",
        version: 6,
        kind: "component",
    },
];

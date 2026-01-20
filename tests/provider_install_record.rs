use greentic_types::{ProviderInstallId, ProviderInstallRecord, ProviderInstallRefs, TenantCtx};

#[test]
fn provider_install_id_rejects_invalid_values() {
    assert!(ProviderInstallId::new("install-1").is_ok());
    assert!(ProviderInstallId::new("bad id").is_err());
    assert!(ProviderInstallId::new("").is_err());
}

#[cfg(all(feature = "serde", feature = "time"))]
#[test]
fn provider_install_record_roundtrip() {
    use greentic_types::{EnvId, PackId, TenantId};
    use semver::Version;
    use serde_json::json;
    use time::OffsetDateTime;

    let tenant = TenantCtx::new(
        "prod".parse::<EnvId>().expect("env"),
        "tenant-1".parse::<TenantId>().expect("tenant"),
    );

    let mut config_refs = ProviderInstallRefs::new();
    config_refs.insert("config".to_owned(), "state:config".to_owned());

    let mut secret_refs = ProviderInstallRefs::new();
    secret_refs.insert("token".to_owned(), "secrets:token".to_owned());

    let record = ProviderInstallRecord {
        tenant,
        provider_id: "vendor.messaging".to_owned(),
        install_id: "install-1".parse().expect("install id"),
        pack_id: "greentic.demo.pack".parse::<PackId>().expect("pack id"),
        pack_version: Version::parse("1.2.3").expect("pack version"),
        created_at: OffsetDateTime::from_unix_timestamp(1_700_000_000).expect("created_at"),
        updated_at: OffsetDateTime::from_unix_timestamp(1_700_000_500).expect("updated_at"),
        config_refs,
        secret_refs,
        webhook_state: json!({"status": "ready"}),
        subscriptions_state: json!({"enabled": true}),
        metadata: json!({"region": "us-east-1"}),
    };

    let json = serde_json::to_string_pretty(&record).expect("serialize");
    let decoded: ProviderInstallRecord = serde_json::from_str(&json).expect("deserialize");

    assert_eq!(decoded, record);
}

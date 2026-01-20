use greentic_types::{Diagnostic, Severity, ValidationCounts, ValidationReport};

#[cfg(feature = "serde")]
use greentic_types::PackId;

#[cfg(feature = "serde")]
use semver::Version;

#[test]
fn report_has_errors() {
    let mut report = ValidationReport::default();
    assert!(!report.has_errors());
    assert_eq!(report.counts(), ValidationCounts::default());

    report.push(Diagnostic {
        severity: Severity::Warn,
        code: "PACK_WARNING".to_owned(),
        message: "warning".to_owned(),
        path: None,
        hint: None,
        data: serde_json::Value::Null,
    });
    assert!(!report.has_errors());
    assert_eq!(
        report.counts(),
        ValidationCounts {
            info: 0,
            warn: 1,
            error: 0,
        }
    );

    report.push(Diagnostic {
        severity: Severity::Error,
        code: "PACK_ERROR".to_owned(),
        message: "error".to_owned(),
        path: None,
        hint: None,
        data: serde_json::Value::Null,
    });
    assert!(report.has_errors());
    assert_eq!(
        report.counts(),
        ValidationCounts {
            info: 0,
            warn: 1,
            error: 1,
        }
    );
}

#[cfg(feature = "serde")]
#[test]
fn diagnostic_roundtrip() {
    let diagnostic = Diagnostic {
        severity: Severity::Warn,
        code: "PACK_TEST".to_owned(),
        message: "check this".to_owned(),
        path: Some("flows.demo".to_owned()),
        hint: Some("update the flow".to_owned()),
        data: serde_json::json!({"detail": "value"}),
    };

    let json = serde_json::to_string_pretty(&diagnostic).expect("serialize diagnostic");
    let decoded: Diagnostic = serde_json::from_str(&json).expect("deserialize diagnostic");

    assert_eq!(decoded, diagnostic);
}

#[cfg(feature = "serde")]
#[test]
fn report_roundtrip() {
    let diagnostic = Diagnostic {
        severity: Severity::Info,
        code: "PACK_INFO".to_owned(),
        message: "ok".to_owned(),
        path: Some("pack_id".to_owned()),
        hint: None,
        data: serde_json::Value::Null,
    };

    let report = ValidationReport {
        pack_id: Some("greentic.demo.pack".parse::<PackId>().expect("pack id")),
        pack_version: Some(Version::parse("1.2.3").expect("version")),
        diagnostics: vec![diagnostic],
    };

    let json = serde_json::to_string_pretty(&report).expect("serialize report");
    let decoded: ValidationReport = serde_json::from_str(&json).expect("deserialize report");

    assert_eq!(decoded, report);
}

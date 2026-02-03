#![cfg(feature = "serde")]

use serde::Serialize;

use greentic_types::{
    AdaptiveCardVersion, CapabilityProfile, RenderDiagnostics, RenderPlanHints, RendererMode, Tier,
};

fn pretty_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).expect("serialize")
}

#[test]
fn renderer_mode_shape_and_strict_flag() {
    let strict_mode = RendererMode::AdaptiveCardDowngrade {
        target_version: AdaptiveCardVersion::from("1.4"),
        strict: true,
    };

    let expected_strict = r#"{
  "mode": "adaptive_card_downgrade",
  "target_version": "1.4",
  "strict": true
}"#;
    assert_eq!(pretty_json(&strict_mode), expected_strict);

    let relaxed_mode = RendererMode::AdaptiveCardDowngrade {
        target_version: AdaptiveCardVersion::from("1.4"),
        strict: false,
    };

    let relaxed_json = pretty_json(&relaxed_mode);
    assert!(relaxed_json.contains(r#""mode": "adaptive_card_downgrade""#));
    assert!(!relaxed_json.contains("strict"));
    let roundtrip: RendererMode = serde_json::from_str(&relaxed_json).expect("deserialize");
    assert_eq!(roundtrip, relaxed_mode);
}

#[test]
fn capability_profile_serialization() {
    let profile = CapabilityProfile {
        max_adaptive_card_version: Some(AdaptiveCardVersion::from("1.4")),
        supports_adaptive_cards: Some(true),
        supports_actions: Some(false),
        supports_media: None,
        supports_input_controls: Some(false),
        supports_markdown: Some(true),
    };

    let expected = r#"{
  "max_adaptive_card_version": "1.4",
  "supports_adaptive_cards": true,
  "supports_actions": false,
  "supports_input_controls": false,
  "supports_markdown": true
}"#;

    assert_eq!(pretty_json(&profile), expected);
    let roundtrip: CapabilityProfile =
        serde_json::from_str(expected).expect("deserialize capability profile");
    assert_eq!(roundtrip, profile);
}

#[test]
fn render_diagnostics_and_hints_roundtrip() {
    let diagnostics = RenderDiagnostics {
        tier: Some(Tier::TierB),
        warnings: vec!["fallback".into()],
        errors: vec!["missing-media".into()],
    };

    let expected_diagnostics = r#"{
  "tier": "tier_b",
  "warnings": [
    "fallback"
  ],
  "errors": [
    "missing-media"
  ]
}"#;
    assert_eq!(pretty_json(&diagnostics), expected_diagnostics);

    let hints = RenderPlanHints {
        renderer_mode: Some(RendererMode::TextOnly),
        capability_profile: Some(CapabilityProfile {
            max_adaptive_card_version: None,
            supports_adaptive_cards: Some(false),
            supports_actions: None,
            supports_media: Some(true),
            supports_input_controls: None,
            supports_markdown: Some(false),
        }),
        diagnostics: Some(diagnostics.clone()),
        tier: Some(Tier::TierD),
    };

    let hints_json = pretty_json(&hints);
    let parsed: RenderPlanHints =
        serde_json::from_str(&hints_json).expect("deserialize hints roundtrip");
    assert_eq!(parsed, hints);
    assert_eq!(parsed.tier, Some(Tier::TierD));
    let relaxed: RenderPlanHints =
        serde_json::from_str(r#"{"tier":"tier_c","unknown_hint":null}"#).expect("deserialize");
    assert_eq!(relaxed.tier, Some(Tier::TierC));
}

#[test]
fn tier_from_json() {
    let tier: Tier = serde_json::from_str(r#""tier_b""#).expect("tier decode");
    assert_eq!(tier, Tier::TierB);

    let mode: RendererMode =
        serde_json::from_str(r#"{"mode":"text_only","ignored":"field"}"#).expect("ignored");
    assert_eq!(mode, RendererMode::TextOnly);
}

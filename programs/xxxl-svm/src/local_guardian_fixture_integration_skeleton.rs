use crate::local_fixture_generator_skeleton::{
    build_local_fixture_set_skeleton, contains_unsafe_fixture_text,
    XxxlLocalFixtureGeneratorSkeletonError, XxxlLocalFixtureSetSkeleton,
};
use crate::local_guardian_descriptor_skeleton::{
    build_local_guardian_descriptor_skeleton, descriptor_threshold_would_pass_skeleton,
    validate_local_guardian_descriptor_safety_skeleton, XxxlLocalGuardianDescriptorSkeleton,
    XxxlLocalGuardianDescriptorSkeletonError,
};

pub const XXXL_LOCAL_GUARDIAN_FIXTURE_INTEGRATION_SKELETON_STATUS: &str =
    "LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_SKELETON_NOT_EXECUTING";

pub const LOCAL_ONLY: bool = true;
pub const TESTNET_ALLOWED: bool = false;
pub const LIVE_ROUTE_ALLOWED: bool = false;
pub const SIGNING_ENABLED: bool = false;
pub const GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED: bool = false;
pub const PRODUCTION_KEYS_ALLOWED: bool = false;
pub const WRITES_TO_DISK: bool = false;
pub const FIXTURE_FILE_EMISSION_ENABLED: bool = false;
pub const LOCAL_VALIDATOR_EXECUTION_APPROVED: bool = false;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalGuardianFixtureIntegrationSkeletonError {
    Generator(XxxlLocalFixtureGeneratorSkeletonError),
    Descriptor(XxxlLocalGuardianDescriptorSkeletonError),
    EmptyIntegrationId,
    UnsafeTextPattern,
    FixtureSafetyFailed,
    DescriptorSafetyFailed,
    ThresholdModelMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalGuardianFixtureIntegrationSkeleton {
    pub status: &'static str,
    pub integration_id: String,
    pub local_only: bool,
    pub testnet_allowed: bool,
    pub live_route_allowed: bool,
    pub signing_enabled: bool,
    pub guardian_package_construction_enabled: bool,
    pub production_keys_allowed: bool,
    pub writes_to_disk: bool,
    pub fixture_file_emission_enabled: bool,
    pub local_validator_execution_approved: bool,
    pub fixture_set_id: String,
    pub fixture_safety_result: String,
    pub gateway_config_fixture: String,
    pub guardian_set_account_fixture: String,
    pub mint_state_fixture: String,
    pub processed_event_fixture: String,
    pub descriptor_id: String,
    pub descriptor_status: String,
    pub descriptor_integrity_hash_value: String,
    pub descriptor_guardian_set_id: String,
    pub descriptor_threshold: u8,
    pub descriptor_guardian_count: u8,
    pub descriptor_route_id: String,
    pub descriptor_source_chain_id: String,
    pub descriptor_mint_token: String,
    pub guardian_fixture_ids: Vec<String>,
    pub success_threshold_model_passes: bool,
    pub failure_threshold_model_fails: bool,
    pub expected_failure_case_ids: Vec<String>,
    pub mutation_invariance_policy: String,
}

pub fn validate_local_guardian_integration_text_skeleton(
    value: &str,
) -> Result<(), XxxlLocalGuardianFixtureIntegrationSkeletonError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::EmptyIntegrationId);
    }

    if contains_unsafe_fixture_text(trimmed) {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::UnsafeTextPattern);
    }

    Ok(())
}

pub fn validate_fixture_set_for_guardian_integration_skeleton(
    fixture_set: &XxxlLocalFixtureSetSkeleton,
) -> Result<(), XxxlLocalGuardianFixtureIntegrationSkeletonError> {
    if !fixture_set.manifest.local_only
        || fixture_set.manifest.testnet_allowed
        || fixture_set.manifest.live_rpc_allowed
        || fixture_set.manifest.production_keys_allowed
        || fixture_set.safety_report.result != "PASS"
    {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::FixtureSafetyFailed);
    }

    Ok(())
}

pub fn expected_guardian_descriptor_failure_case_ids_skeleton() -> Vec<String> {
    vec![
        "guardian_descriptor_id_mismatch".to_string(),
        "guardian_descriptor_integrity_mismatch".to_string(),
        "guardian_set_id_mismatch".to_string(),
        "guardian_threshold_zero".to_string(),
        "guardian_threshold_exceeds_count".to_string(),
        "guardian_count_zero".to_string(),
        "guardian_duplicate_fixture".to_string(),
        "guardian_unknown_fixture".to_string(),
        "guardian_malformed_fixture".to_string(),
        "guardian_wrong_route_id".to_string(),
        "guardian_wrong_source_chain_id".to_string(),
        "guardian_wrong_mint_token".to_string(),
        "guardian_wrong_message_hash_label".to_string(),
        "guardian_signing_material_marker_detected".to_string(),
        "guardian_package_construction_marker_detected".to_string(),
        "guardian_production_key_marker_detected".to_string(),
        "guardian_live_route_marker_detected".to_string(),
    ]
}

fn guardian_fixture_ids_from_descriptor_skeleton(
    descriptor: &XxxlLocalGuardianDescriptorSkeleton,
) -> Vec<String> {
    descriptor
        .guardian_public_key_fixtures
        .iter()
        .map(|fixture| fixture.fixture_id.clone())
        .collect()
}

pub fn build_local_guardian_fixture_integration_skeleton(
    integration_id: &str,
    fixture_set_id: &str,
    fixture_set_name: &str,
    deterministic_seed_label: &str,
    seed_byte: u8,
    descriptor_id: &str,
    route_id: &str,
    source_chain_id: &str,
    mint_token: &str,
    guardian_set_id: &str,
    threshold: u8,
    guardian_count: u8,
    guardian_seed_label: &str,
) -> Result<XxxlLocalGuardianFixtureIntegrationSkeleton, XxxlLocalGuardianFixtureIntegrationSkeletonError>
{
    validate_local_guardian_integration_text_skeleton(integration_id)?;

    let fixture_set = build_local_fixture_set_skeleton(
        fixture_set_id,
        fixture_set_name,
        deterministic_seed_label,
        seed_byte,
    )
    .map_err(XxxlLocalGuardianFixtureIntegrationSkeletonError::Generator)?;

    validate_fixture_set_for_guardian_integration_skeleton(&fixture_set)?;

    let descriptor = build_local_guardian_descriptor_skeleton(
        descriptor_id,
        route_id,
        source_chain_id,
        mint_token,
        guardian_set_id,
        threshold,
        guardian_count,
        guardian_seed_label,
    )
    .map_err(XxxlLocalGuardianFixtureIntegrationSkeletonError::Descriptor)?;

    validate_local_guardian_descriptor_safety_skeleton(&descriptor)
        .map_err(|_| XxxlLocalGuardianFixtureIntegrationSkeletonError::DescriptorSafetyFailed)?;

    let success_threshold_model_passes =
        descriptor_threshold_would_pass_skeleton(&descriptor, threshold);

    let failure_threshold_model_fails = if threshold > 0 {
        !descriptor_threshold_would_pass_skeleton(&descriptor, threshold - 1)
    } else {
        false
    };

    if !success_threshold_model_passes || !failure_threshold_model_fails {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::ThresholdModelMismatch);
    }

    Ok(XxxlLocalGuardianFixtureIntegrationSkeleton {
        status: XXXL_LOCAL_GUARDIAN_FIXTURE_INTEGRATION_SKELETON_STATUS,
        integration_id: integration_id.to_string(),
        local_only: LOCAL_ONLY,
        testnet_allowed: TESTNET_ALLOWED,
        live_route_allowed: LIVE_ROUTE_ALLOWED,
        signing_enabled: SIGNING_ENABLED,
        guardian_package_construction_enabled: GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED,
        production_keys_allowed: PRODUCTION_KEYS_ALLOWED,
        writes_to_disk: WRITES_TO_DISK,
        fixture_file_emission_enabled: FIXTURE_FILE_EMISSION_ENABLED,
        local_validator_execution_approved: LOCAL_VALIDATOR_EXECUTION_APPROVED,
        fixture_set_id: fixture_set.manifest.fixture_set_id,
        fixture_safety_result: fixture_set.safety_report.result.to_string(),
        gateway_config_fixture: fixture_set.pubkeys.gateway_config.to_string(),
        guardian_set_account_fixture: fixture_set.pubkeys.guardian_set.to_string(),
        mint_state_fixture: fixture_set.pubkeys.mint_state.to_string(),
        processed_event_fixture: fixture_set.pubkeys.processed_event.to_string(),
        descriptor_id: descriptor.descriptor_id.clone(),
        descriptor_status: descriptor.descriptor_status.to_string(),
        descriptor_integrity_hash_value: descriptor.descriptor_integrity_hash_value.clone(),
        descriptor_guardian_set_id: descriptor.guardian_set_id.clone(),
        descriptor_threshold: descriptor.threshold,
        descriptor_guardian_count: descriptor.guardian_count,
        descriptor_route_id: descriptor.route_id.clone(),
        descriptor_source_chain_id: descriptor.source_chain_id.clone(),
        descriptor_mint_token: descriptor.mint_token.clone(),
        guardian_fixture_ids: guardian_fixture_ids_from_descriptor_skeleton(&descriptor),
        success_threshold_model_passes,
        failure_threshold_model_fails,
        expected_failure_case_ids: expected_guardian_descriptor_failure_case_ids_skeleton(),
        mutation_invariance_policy: "byte_identical_on_descriptor_failure".to_string(),
    })
}

pub fn validate_local_guardian_fixture_integration_safety_skeleton(
    integration: &XxxlLocalGuardianFixtureIntegrationSkeleton,
) -> Result<(), XxxlLocalGuardianFixtureIntegrationSkeletonError> {
    if !integration.local_only
        || integration.testnet_allowed
        || integration.live_route_allowed
        || integration.signing_enabled
        || integration.guardian_package_construction_enabled
        || integration.production_keys_allowed
        || integration.writes_to_disk
        || integration.fixture_file_emission_enabled
        || integration.local_validator_execution_approved
    {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::UnsafeTextPattern);
    }

    if integration.fixture_safety_result != "PASS" {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::FixtureSafetyFailed);
    }

    if integration.guardian_fixture_ids.len() != integration.descriptor_guardian_count as usize {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::DescriptorSafetyFailed);
    }

    if !integration.success_threshold_model_passes || !integration.failure_threshold_model_fails {
        return Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::ThresholdModelMismatch);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_status_and_flags_are_local_only_non_executing() {
        assert_eq!(
            XXXL_LOCAL_GUARDIAN_FIXTURE_INTEGRATION_SKELETON_STATUS,
            "LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_SKELETON_NOT_EXECUTING"
        );
        assert!(LOCAL_ONLY);
        assert!(!TESTNET_ALLOWED);
        assert!(!LIVE_ROUTE_ALLOWED);
        assert!(!SIGNING_ENABLED);
        assert!(!GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED);
        assert!(!PRODUCTION_KEYS_ALLOWED);
        assert!(!WRITES_TO_DISK);
        assert!(!FIXTURE_FILE_EMISSION_ENABLED);
        assert!(!LOCAL_VALIDATOR_EXECUTION_APPROVED);
    }

    #[test]
    fn integration_text_rejects_empty_and_unsafe_patterns() {
        assert_eq!(
            validate_local_guardian_integration_text_skeleton(""),
            Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::EmptyIntegrationId)
        );

        assert_eq!(
            validate_local_guardian_integration_text_skeleton("production integration"),
            Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::UnsafeTextPattern)
        );

        assert_eq!(
            validate_local_guardian_integration_text_skeleton("guardian_integration_alpha"),
            Ok(())
        );
    }

    #[test]
    fn expected_failure_case_ids_are_stable() {
        let cases = expected_guardian_descriptor_failure_case_ids_skeleton();

        assert!(cases.len() >= 10);
        assert!(cases.contains(&"guardian_descriptor_id_mismatch".to_string()));
        assert!(cases.contains(&"guardian_descriptor_integrity_mismatch".to_string()));
        assert!(cases.contains(&"guardian_threshold_zero".to_string()));
        assert!(cases.contains(&"guardian_signing_material_marker_detected".to_string()));
        assert!(cases.contains(&"guardian_package_construction_marker_detected".to_string()));
        assert!(cases.contains(&"guardian_production_key_marker_detected".to_string()));
    }

    #[test]
    fn integration_builds_local_descriptor_fixture_link() {
        let integration = build_local_guardian_fixture_integration_skeleton(
            "guardian_integration_alpha",
            "fixture_alpha",
            "fixture alpha",
            "fixture seed alpha",
            0x42,
            "descriptor_alpha",
            "route_local",
            "source_chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("integration");

        assert_eq!(
            integration.status,
            "LOCAL_GUARDIAN_DESCRIPTOR_FIXTURE_INTEGRATION_SKELETON_NOT_EXECUTING"
        );
        assert_eq!(integration.integration_id, "guardian_integration_alpha");
        assert_eq!(integration.fixture_set_id, "fixture_alpha");
        assert_eq!(integration.descriptor_id, "descriptor_alpha");
        assert_eq!(integration.descriptor_guardian_set_id, "guardian_set_1");
        assert_eq!(integration.descriptor_threshold, 2);
        assert_eq!(integration.descriptor_guardian_count, 3);
        assert_eq!(integration.guardian_fixture_ids.len(), 3);
        assert!(integration.success_threshold_model_passes);
        assert!(integration.failure_threshold_model_fails);
        assert!(integration.local_only);
        assert!(!integration.signing_enabled);
        assert!(!integration.guardian_package_construction_enabled);
        assert!(!integration.fixture_file_emission_enabled);
        assert!(!integration.local_validator_execution_approved);
    }

    #[test]
    fn integration_safety_validation_accepts_safe_integration() {
        let integration = build_local_guardian_fixture_integration_skeleton(
            "guardian_integration_alpha",
            "fixture_alpha",
            "fixture alpha",
            "fixture seed alpha",
            0x42,
            "descriptor_alpha",
            "route_local",
            "source_chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("integration");

        validate_local_guardian_fixture_integration_safety_skeleton(&integration)
            .expect("safe integration");
    }

    #[test]
    fn integration_rejects_unsafe_fixture_identity() {
        let err = build_local_guardian_fixture_integration_skeleton(
            "guardian_integration_alpha",
            "fixture_alpha",
            "production fixture",
            "fixture seed alpha",
            0x42,
            "descriptor_alpha",
            "route_local",
            "source_chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect_err("unsafe fixture identity should fail");

        assert_eq!(
            err,
            XxxlLocalGuardianFixtureIntegrationSkeletonError::Generator(
                XxxlLocalFixtureGeneratorSkeletonError::UnsafeTextPattern
            )
        );
    }

    #[test]
    fn integration_rejects_unsafe_descriptor_identity() {
        let err = build_local_guardian_fixture_integration_skeleton(
            "guardian_integration_alpha",
            "fixture_alpha",
            "fixture alpha",
            "fixture seed alpha",
            0x42,
            "descriptor_alpha",
            "route_local",
            "source_chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "private_key seed",
        )
        .expect_err("unsafe descriptor should fail");

        assert_eq!(
            err,
            XxxlLocalGuardianFixtureIntegrationSkeletonError::Descriptor(
                XxxlLocalGuardianDescriptorSkeletonError::UnsafeTextPattern
            )
        );
    }

    #[test]
    fn integration_rejects_invalid_threshold() {
        let err = build_local_guardian_fixture_integration_skeleton(
            "guardian_integration_alpha",
            "fixture_alpha",
            "fixture alpha",
            "fixture seed alpha",
            0x42,
            "descriptor_alpha",
            "route_local",
            "source_chain_local",
            "mint_local",
            "guardian_set_1",
            4,
            3,
            "guardian seed alpha",
        )
        .expect_err("invalid threshold should fail");

        assert_eq!(
            err,
            XxxlLocalGuardianFixtureIntegrationSkeletonError::Descriptor(
                XxxlLocalGuardianDescriptorSkeletonError::ThresholdExceedsGuardianCount
            )
        );
    }

    #[test]
    fn integration_safety_rejects_mutated_execution_flag() {
        let mut integration = build_local_guardian_fixture_integration_skeleton(
            "guardian_integration_alpha",
            "fixture_alpha",
            "fixture alpha",
            "fixture seed alpha",
            0x42,
            "descriptor_alpha",
            "route_local",
            "source_chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("integration");

        integration.local_validator_execution_approved = true;

        assert_eq!(
            validate_local_guardian_fixture_integration_safety_skeleton(&integration),
            Err(XxxlLocalGuardianFixtureIntegrationSkeletonError::UnsafeTextPattern)
        );
    }
}

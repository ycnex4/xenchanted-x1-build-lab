use crate::local_guardian_fixture_integration_skeleton::{
    build_local_guardian_fixture_integration_skeleton,
    validate_local_guardian_fixture_integration_safety_skeleton,
    XxxlLocalGuardianFixtureIntegrationSkeleton,
    XxxlLocalGuardianFixtureIntegrationSkeletonError,
};

pub const XXXL_LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_STATUS: &str =
    "LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_NOT_EXECUTING";

pub const LOCAL_ONLY: bool = true;
pub const TESTNET_ALLOWED: bool = false;
pub const LIVE_ROUTE_ALLOWED: bool = false;
pub const SIGNING_ENABLED: bool = false;
pub const GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED: bool = false;
pub const PRODUCTION_KEYS_ALLOWED: bool = false;
pub const WRITES_TO_DISK: bool = false;
pub const FIXTURE_FILE_EMISSION_ENABLED: bool = false;
pub const FAILURE_MATRIX_EXECUTION_ENABLED: bool = false;
pub const LOCAL_VALIDATOR_EXECUTION_APPROVED: bool = false;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalGuardianFailureMatrixSkeletonError {
    Integration(XxxlLocalGuardianFixtureIntegrationSkeletonError),
    EmptyMatrixId,
    EmptyFailureCaseId,
    UnknownFailureGroup,
    RawSecretMarkerDetected,
    MatrixSafetyFailed,
    MissingFailureCases,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalGuardianFailureCaseSkeleton {
    pub failure_case_id: String,
    pub failure_group: &'static str,
    pub local_only: bool,
    pub expected_mutation: bool,
    pub expected_signing: bool,
    pub expected_package_construction: bool,
    pub expected_submit: bool,
    pub expected_testnet_action: bool,
    pub expected_local_validator_execution: bool,
    pub expected_error_label: String,
    pub no_mutation_policy: String,
    pub log_expectation_id: String,
    pub safety_report_expectation_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalGuardianFailureMatrixSkeleton {
    pub matrix_status: &'static str,
    pub matrix_id: String,
    pub local_only: bool,
    pub testnet_allowed: bool,
    pub live_route_allowed: bool,
    pub signing_enabled: bool,
    pub guardian_package_construction_enabled: bool,
    pub production_keys_allowed: bool,
    pub writes_to_disk: bool,
    pub fixture_file_emission_enabled: bool,
    pub failure_matrix_execution_enabled: bool,
    pub local_validator_execution_approved: bool,
    pub integration_id: String,
    pub fixture_set_id: String,
    pub descriptor_id: String,
    pub descriptor_guardian_set_id: String,
    pub descriptor_threshold: u8,
    pub descriptor_guardian_count: u8,
    pub descriptor_route_id: String,
    pub descriptor_source_chain_id: String,
    pub descriptor_mint_token: String,
    pub failure_cases: Vec<XxxlLocalGuardianFailureCaseSkeleton>,
    pub no_mutation_accounts: Vec<String>,
    pub mutation_invariance_policy: String,
}

pub fn expected_guardian_failure_groups_skeleton() -> [&'static str; 10] {
    [
        "descriptor_identity",
        "descriptor_integrity",
        "guardian_set_mapping",
        "threshold",
        "guardian_fixture_list",
        "route_scope",
        "message_boundary",
        "signing_material_safety",
        "package_construction_safety",
        "live_testnet_production_safety",
    ]
}

pub fn contains_raw_secret_marker_skeleton(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();

    lowered.contains("private_key=")
        || lowered.contains("secret_key=")
        || lowered.contains("keypair_path=")
        || lowered.contains("mnemonic=")
        || lowered.contains("seed_phrase=")
        || lowered.contains("raw_signing_material=")
}

pub fn classify_guardian_failure_case_skeleton(
    failure_case_id: &str,
) -> &'static str {
    if failure_case_id.contains("descriptor_id")
        || failure_case_id.contains("missing_guardian_descriptor")
        || failure_case_id.contains("empty_guardian_descriptor")
        || failure_case_id.contains("malformed_guardian_descriptor")
        || failure_case_id.contains("descriptor_scope")
    {
        "descriptor_identity"
    } else if failure_case_id.contains("integrity")
        || failure_case_id.contains("source_commit")
        || failure_case_id.contains("file_path")
    {
        "descriptor_integrity"
    } else if failure_case_id.contains("guardian_set")
    {
        "guardian_set_mapping"
    } else if failure_case_id.contains("threshold")
        || failure_case_id.contains("approval_count")
    {
        "threshold"
    } else if failure_case_id.contains("fixture")
        || failure_case_id.contains("production_key")
        || failure_case_id.contains("real_guardian")
        || failure_case_id.contains("keypair")
        || failure_case_id.contains("private_material")
    {
        "guardian_fixture_list"
    } else if failure_case_id.contains("route")
        || failure_case_id.contains("source_chain")
        || failure_case_id.contains("mint_token")
        || failure_case_id.contains("network_scope")
        || failure_case_id.contains("testnet_activation")
        || failure_case_id.contains("live_route")
    {
        "route_scope"
    } else if failure_case_id.contains("message")
        || failure_case_id.contains("schema")
        || failure_case_id.contains("payload")
    {
        "message_boundary"
    } else if failure_case_id.contains("signing")
        || failure_case_id.contains("private_key")
        || failure_case_id.contains("secret_key")
        || failure_case_id.contains("mnemonic")
        || failure_case_id.contains("seed_phrase")
    {
        "signing_material_safety"
    } else if failure_case_id.contains("package")
    {
        "package_construction_safety"
    } else {
        "live_testnet_production_safety"
    }
}

pub fn validate_guardian_failure_group_skeleton(
    failure_group: &'static str,
) -> Result<(), XxxlLocalGuardianFailureMatrixSkeletonError> {
    if expected_guardian_failure_groups_skeleton().contains(&failure_group) {
        Ok(())
    } else {
        Err(XxxlLocalGuardianFailureMatrixSkeletonError::UnknownFailureGroup)
    }
}

pub fn build_local_guardian_failure_case_skeleton(
    failure_case_id: &str,
) -> Result<XxxlLocalGuardianFailureCaseSkeleton, XxxlLocalGuardianFailureMatrixSkeletonError> {
    let trimmed = failure_case_id.trim();

    if trimmed.is_empty() {
        return Err(XxxlLocalGuardianFailureMatrixSkeletonError::EmptyFailureCaseId);
    }

    if contains_raw_secret_marker_skeleton(trimmed) {
        return Err(XxxlLocalGuardianFailureMatrixSkeletonError::RawSecretMarkerDetected);
    }

    let failure_group = classify_guardian_failure_case_skeleton(trimmed);
    validate_guardian_failure_group_skeleton(failure_group)?;

    Ok(XxxlLocalGuardianFailureCaseSkeleton {
        failure_case_id: trimmed.to_string(),
        failure_group,
        local_only: true,
        expected_mutation: false,
        expected_signing: false,
        expected_package_construction: false,
        expected_submit: false,
        expected_testnet_action: false,
        expected_local_validator_execution: false,
        expected_error_label: format!("{}_rejected", trimmed),
        no_mutation_policy: "byte_identical".to_string(),
        log_expectation_id: format!("{}_log_expectation", trimmed),
        safety_report_expectation_id: format!("{}_safety_report_expectation", trimmed),
    })
}

fn build_failure_cases_from_integration_skeleton(
    integration: &XxxlLocalGuardianFixtureIntegrationSkeleton,
) -> Result<Vec<XxxlLocalGuardianFailureCaseSkeleton>, XxxlLocalGuardianFailureMatrixSkeletonError>
{
    let mut cases = Vec::with_capacity(integration.expected_failure_case_ids.len());

    for failure_case_id in &integration.expected_failure_case_ids {
        cases.push(build_local_guardian_failure_case_skeleton(failure_case_id)?);
    }

    if cases.is_empty() {
        return Err(XxxlLocalGuardianFailureMatrixSkeletonError::MissingFailureCases);
    }

    Ok(cases)
}

fn no_mutation_accounts_from_integration_skeleton(
    integration: &XxxlLocalGuardianFixtureIntegrationSkeleton,
) -> Vec<String> {
    vec![
        integration.gateway_config_fixture.clone(),
        integration.guardian_set_account_fixture.clone(),
        integration.mint_state_fixture.clone(),
        integration.processed_event_fixture.clone(),
        "spl_mint_future_fixture_if_present".to_string(),
        "recipient_token_account_future_fixture_if_present".to_string(),
    ]
}

pub fn build_local_guardian_failure_matrix_from_integration_skeleton(
    matrix_id: &str,
    integration: &XxxlLocalGuardianFixtureIntegrationSkeleton,
) -> Result<XxxlLocalGuardianFailureMatrixSkeleton, XxxlLocalGuardianFailureMatrixSkeletonError> {
    let trimmed = matrix_id.trim();

    if trimmed.is_empty() {
        return Err(XxxlLocalGuardianFailureMatrixSkeletonError::EmptyMatrixId);
    }

    if contains_raw_secret_marker_skeleton(trimmed) {
        return Err(XxxlLocalGuardianFailureMatrixSkeletonError::RawSecretMarkerDetected);
    }

    validate_local_guardian_fixture_integration_safety_skeleton(integration)
        .map_err(|_| XxxlLocalGuardianFailureMatrixSkeletonError::MatrixSafetyFailed)?;

    let failure_cases = build_failure_cases_from_integration_skeleton(integration)?;

    Ok(XxxlLocalGuardianFailureMatrixSkeleton {
        matrix_status: XXXL_LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_STATUS,
        matrix_id: trimmed.to_string(),
        local_only: LOCAL_ONLY,
        testnet_allowed: TESTNET_ALLOWED,
        live_route_allowed: LIVE_ROUTE_ALLOWED,
        signing_enabled: SIGNING_ENABLED,
        guardian_package_construction_enabled: GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED,
        production_keys_allowed: PRODUCTION_KEYS_ALLOWED,
        writes_to_disk: WRITES_TO_DISK,
        fixture_file_emission_enabled: FIXTURE_FILE_EMISSION_ENABLED,
        failure_matrix_execution_enabled: FAILURE_MATRIX_EXECUTION_ENABLED,
        local_validator_execution_approved: LOCAL_VALIDATOR_EXECUTION_APPROVED,
        integration_id: integration.integration_id.clone(),
        fixture_set_id: integration.fixture_set_id.clone(),
        descriptor_id: integration.descriptor_id.clone(),
        descriptor_guardian_set_id: integration.descriptor_guardian_set_id.clone(),
        descriptor_threshold: integration.descriptor_threshold,
        descriptor_guardian_count: integration.descriptor_guardian_count,
        descriptor_route_id: integration.descriptor_route_id.clone(),
        descriptor_source_chain_id: integration.descriptor_source_chain_id.clone(),
        descriptor_mint_token: integration.descriptor_mint_token.clone(),
        failure_cases,
        no_mutation_accounts: no_mutation_accounts_from_integration_skeleton(integration),
        mutation_invariance_policy: integration.mutation_invariance_policy.clone(),
    })
}

pub fn build_local_guardian_failure_matrix_skeleton(
    matrix_id: &str,
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
) -> Result<XxxlLocalGuardianFailureMatrixSkeleton, XxxlLocalGuardianFailureMatrixSkeletonError> {
    let integration = build_local_guardian_fixture_integration_skeleton(
        integration_id,
        fixture_set_id,
        fixture_set_name,
        deterministic_seed_label,
        seed_byte,
        descriptor_id,
        route_id,
        source_chain_id,
        mint_token,
        guardian_set_id,
        threshold,
        guardian_count,
        guardian_seed_label,
    )
    .map_err(XxxlLocalGuardianFailureMatrixSkeletonError::Integration)?;

    build_local_guardian_failure_matrix_from_integration_skeleton(matrix_id, &integration)
}

pub fn validate_local_guardian_failure_matrix_safety_skeleton(
    matrix: &XxxlLocalGuardianFailureMatrixSkeleton,
) -> Result<(), XxxlLocalGuardianFailureMatrixSkeletonError> {
    if !matrix.local_only
        || matrix.testnet_allowed
        || matrix.live_route_allowed
        || matrix.signing_enabled
        || matrix.guardian_package_construction_enabled
        || matrix.production_keys_allowed
        || matrix.writes_to_disk
        || matrix.fixture_file_emission_enabled
        || matrix.failure_matrix_execution_enabled
        || matrix.local_validator_execution_approved
    {
        return Err(XxxlLocalGuardianFailureMatrixSkeletonError::MatrixSafetyFailed);
    }

    if matrix.failure_cases.is_empty() {
        return Err(XxxlLocalGuardianFailureMatrixSkeletonError::MissingFailureCases);
    }

    for case in &matrix.failure_cases {
        if !case.local_only
            || case.expected_mutation
            || case.expected_signing
            || case.expected_package_construction
            || case.expected_submit
            || case.expected_testnet_action
            || case.expected_local_validator_execution
            || case.no_mutation_policy != "byte_identical"
        {
            return Err(XxxlLocalGuardianFailureMatrixSkeletonError::MatrixSafetyFailed);
        }

        validate_guardian_failure_group_skeleton(case.failure_group)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_status_and_flags_are_local_only_non_executing() {
        assert_eq!(
            XXXL_LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_STATUS,
            "LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_NOT_EXECUTING"
        );
        assert!(LOCAL_ONLY);
        assert!(!TESTNET_ALLOWED);
        assert!(!LIVE_ROUTE_ALLOWED);
        assert!(!SIGNING_ENABLED);
        assert!(!GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED);
        assert!(!PRODUCTION_KEYS_ALLOWED);
        assert!(!WRITES_TO_DISK);
        assert!(!FIXTURE_FILE_EMISSION_ENABLED);
        assert!(!FAILURE_MATRIX_EXECUTION_ENABLED);
        assert!(!LOCAL_VALIDATOR_EXECUTION_APPROVED);
    }

    #[test]
    fn failure_groups_are_stable() {
        let groups = expected_guardian_failure_groups_skeleton();

        assert_eq!(groups.len(), 10);
        assert!(groups.contains(&"descriptor_identity"));
        assert!(groups.contains(&"descriptor_integrity"));
        assert!(groups.contains(&"guardian_set_mapping"));
        assert!(groups.contains(&"threshold"));
        assert!(groups.contains(&"guardian_fixture_list"));
        assert!(groups.contains(&"route_scope"));
        assert!(groups.contains(&"message_boundary"));
        assert!(groups.contains(&"signing_material_safety"));
        assert!(groups.contains(&"package_construction_safety"));
        assert!(groups.contains(&"live_testnet_production_safety"));
    }

    #[test]
    fn failure_case_builder_defaults_to_no_mutation_no_execution() {
        let case = build_local_guardian_failure_case_skeleton("guardian_threshold_zero")
            .expect("case");

        assert_eq!(case.failure_group, "threshold");
        assert!(case.local_only);
        assert!(!case.expected_mutation);
        assert!(!case.expected_signing);
        assert!(!case.expected_package_construction);
        assert!(!case.expected_submit);
        assert!(!case.expected_testnet_action);
        assert!(!case.expected_local_validator_execution);
        assert_eq!(case.no_mutation_policy, "byte_identical");
    }

    #[test]
    fn failure_case_builder_rejects_empty_and_raw_secret_assignment_markers() {
        assert_eq!(
            build_local_guardian_failure_case_skeleton(""),
            Err(XxxlLocalGuardianFailureMatrixSkeletonError::EmptyFailureCaseId)
        );

        assert_eq!(
            build_local_guardian_failure_case_skeleton("private_key=value"),
            Err(XxxlLocalGuardianFailureMatrixSkeletonError::RawSecretMarkerDetected)
        );
    }

    #[test]
    fn matrix_builds_from_local_integration_inputs() {
        let matrix = build_local_guardian_failure_matrix_skeleton(
            "guardian_matrix_alpha",
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
        .expect("matrix");

        assert_eq!(
            matrix.matrix_status,
            "LOCAL_GUARDIAN_FAILURE_MATRIX_SKELETON_NOT_EXECUTING"
        );
        assert_eq!(matrix.matrix_id, "guardian_matrix_alpha");
        assert_eq!(matrix.integration_id, "guardian_integration_alpha");
        assert_eq!(matrix.fixture_set_id, "fixture_alpha");
        assert_eq!(matrix.descriptor_id, "descriptor_alpha");
        assert_eq!(matrix.descriptor_threshold, 2);
        assert_eq!(matrix.descriptor_guardian_count, 3);
        assert!(!matrix.failure_cases.is_empty());
        assert!(matrix.no_mutation_accounts.len() >= 4);
        assert!(matrix.local_only);
        assert!(!matrix.failure_matrix_execution_enabled);
        assert!(!matrix.local_validator_execution_approved);
    }

    #[test]
    fn matrix_contains_expected_guardian_failure_cases() {
        let matrix = build_local_guardian_failure_matrix_skeleton(
            "guardian_matrix_alpha",
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
        .expect("matrix");

        let ids: Vec<String> = matrix
            .failure_cases
            .iter()
            .map(|case| case.failure_case_id.clone())
            .collect();

        assert!(ids.contains(&"guardian_descriptor_id_mismatch".to_string()));
        assert!(ids.contains(&"guardian_descriptor_integrity_mismatch".to_string()));
        assert!(ids.contains(&"guardian_threshold_zero".to_string()));
        assert!(ids.contains(&"guardian_signing_material_marker_detected".to_string()));
        assert!(ids.contains(&"guardian_package_construction_marker_detected".to_string()));
    }

    #[test]
    fn matrix_safety_validation_accepts_safe_matrix() {
        let matrix = build_local_guardian_failure_matrix_skeleton(
            "guardian_matrix_alpha",
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
        .expect("matrix");

        validate_local_guardian_failure_matrix_safety_skeleton(&matrix).expect("safe matrix");
    }

    #[test]
    fn matrix_safety_rejects_mutated_execution_flag() {
        let mut matrix = build_local_guardian_failure_matrix_skeleton(
            "guardian_matrix_alpha",
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
        .expect("matrix");

        matrix.failure_matrix_execution_enabled = true;

        assert_eq!(
            validate_local_guardian_failure_matrix_safety_skeleton(&matrix),
            Err(XxxlLocalGuardianFailureMatrixSkeletonError::MatrixSafetyFailed)
        );
    }

    #[test]
    fn matrix_rejects_unsafe_integration_inputs() {
        let err = build_local_guardian_failure_matrix_skeleton(
            "guardian_matrix_alpha",
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
        .expect_err("unsafe fixture should fail");

        match err {
            XxxlLocalGuardianFailureMatrixSkeletonError::Integration(_) => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn matrix_rejects_empty_matrix_id() {
        let err = build_local_guardian_failure_matrix_skeleton(
            "",
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
        .expect_err("empty matrix id should fail");

        assert_eq!(err, XxxlLocalGuardianFailureMatrixSkeletonError::EmptyMatrixId);
    }
}

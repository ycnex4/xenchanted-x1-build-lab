use crate::local_fixture_generator_skeleton::contains_unsafe_fixture_text;

pub const XXXL_LOCAL_GUARDIAN_DESCRIPTOR_SKELETON_STATUS: &str =
    "LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING";

pub const LOCAL_ONLY: bool = true;
pub const TESTNET_ALLOWED: bool = false;
pub const LIVE_ROUTE_ALLOWED: bool = false;
pub const SIGNING_ENABLED: bool = false;
pub const GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED: bool = false;
pub const PRODUCTION_KEYS_ALLOWED: bool = false;
pub const WRITES_TO_DISK: bool = false;
pub const LOCAL_VALIDATOR_EXECUTION_APPROVED: bool = false;

pub const DESCRIPTOR_VERSION: &str = "local-guardian-descriptor-skeleton-v1";
pub const PUBLIC_KEY_ENCODING: &str = "local-fixture-public-key";
pub const SIGNATURE_ALGORITHM_LABEL: &str = "ed25519-label-only-not-signing";
pub const MESSAGE_HASH_ALGORITHM_LABEL: &str = "keccak256-label-only";
pub const INTEGRITY_HASH_ALGORITHM_LABEL: &str = "local-deterministic-fnv64-label-only";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalGuardianDescriptorSkeletonError {
    EmptyDescriptorId,
    EmptyRouteId,
    EmptyMintToken,
    EmptySourceChainId,
    EmptyGuardianSetId,
    EmptyGuardianSeedLabel,
    ZeroGuardianCount,
    ZeroThreshold,
    ThresholdExceedsGuardianCount,
    UnsafeTextPattern,
    DuplicateGuardianFixture,
    SigningMaterialMarkerDetected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalGuardianPublicKeyFixtureSkeleton {
    pub guardian_index: u8,
    pub fixture_id: String,
    pub public_key_fixture: String,
    pub local_only: bool,
    pub can_sign: bool,
    pub production_key: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalGuardianDescriptorSkeleton {
    pub descriptor_version: &'static str,
    pub descriptor_status: &'static str,
    pub descriptor_id: String,
    pub descriptor_scope: String,
    pub network_scope: String,
    pub route_id: String,
    pub source_chain_id: String,
    pub mint_token: String,
    pub guardian_set_id: String,
    pub threshold: u8,
    pub guardian_count: u8,
    pub guardian_public_key_fixtures: Vec<XxxlLocalGuardianPublicKeyFixtureSkeleton>,
    pub public_key_encoding: &'static str,
    pub signature_algorithm: &'static str,
    pub message_hash_algorithm: &'static str,
    pub canonical_message_schema_version: String,
    pub descriptor_integrity_hash_algorithm: &'static str,
    pub descriptor_integrity_hash_value: String,
    pub descriptor_source: String,
    pub activation_status: String,
    pub rotation_policy: String,
    pub emergency_disable_policy: String,
    pub no_signing_material_statement: String,
    pub local_only: bool,
    pub testnet_allowed: bool,
    pub live_route_allowed: bool,
    pub signing_enabled: bool,
    pub guardian_package_construction_enabled: bool,
    pub production_keys_allowed: bool,
    pub writes_to_disk: bool,
    pub local_validator_execution_approved: bool,
}

pub fn contains_signing_material_marker_skeleton(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();

    lowered.contains("private_key")
        || lowered.contains("secret_key")
        || lowered.contains("keypair")
        || lowered.contains("mnemonic")
        || lowered.contains("seed_phrase")
        || lowered.contains("begin private key")
        || lowered.contains("production guardian")
        || lowered.contains("real guardian")
        || lowered.contains("signing key")
}

pub fn validate_local_guardian_descriptor_text_skeleton(
    value: &str,
) -> Result<(), XxxlLocalGuardianDescriptorSkeletonError> {
    let trimmed = value.trim();

    if contains_unsafe_fixture_text(trimmed) {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::UnsafeTextPattern);
    }

    if contains_signing_material_marker_skeleton(trimmed) {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::SigningMaterialMarkerDetected);
    }

    Ok(())
}

pub fn deterministic_local_guardian_public_key_fixture_skeleton(
    descriptor_id: &str,
    guardian_seed_label: &str,
    guardian_index: u8,
) -> Result<XxxlLocalGuardianPublicKeyFixtureSkeleton, XxxlLocalGuardianDescriptorSkeletonError> {
    if descriptor_id.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyDescriptorId);
    }

    if guardian_seed_label.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyGuardianSeedLabel);
    }

    validate_local_guardian_descriptor_text_skeleton(descriptor_id)?;
    validate_local_guardian_descriptor_text_skeleton(guardian_seed_label)?;

    let fixture_id = format!("{}_guardian_{:02}", descriptor_id, guardian_index);
    let public_key_fixture = format!(
        "local_guardian_public_key_fixture_{}_{}_not_real_key",
        guardian_seed_label.replace(' ', "_"),
        guardian_index
    );

    validate_local_guardian_descriptor_text_skeleton(&fixture_id)?;
    validate_local_guardian_descriptor_text_skeleton(&public_key_fixture)?;

    Ok(XxxlLocalGuardianPublicKeyFixtureSkeleton {
        guardian_index,
        fixture_id,
        public_key_fixture,
        local_only: true,
        can_sign: false,
        production_key: false,
    })
}

pub fn build_local_guardian_public_key_fixtures_skeleton(
    descriptor_id: &str,
    guardian_seed_label: &str,
    guardian_count: u8,
) -> Result<Vec<XxxlLocalGuardianPublicKeyFixtureSkeleton>, XxxlLocalGuardianDescriptorSkeletonError>
{
    if guardian_count == 0 {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::ZeroGuardianCount);
    }

    let mut fixtures = Vec::with_capacity(guardian_count as usize);

    for index in 0..guardian_count {
        let fixture = deterministic_local_guardian_public_key_fixture_skeleton(
            descriptor_id,
            guardian_seed_label,
            index,
        )?;

        if fixtures
            .iter()
            .any(|existing: &XxxlLocalGuardianPublicKeyFixtureSkeleton| {
                existing.public_key_fixture == fixture.public_key_fixture
            })
        {
            return Err(XxxlLocalGuardianDescriptorSkeletonError::DuplicateGuardianFixture);
        }

        fixtures.push(fixture);
    }

    Ok(fixtures)
}

fn local_integrity_hash_skeleton(parts: &[&str]) -> String {
    let mut value: u64 = 0xcbf29ce484222325;

    for part in parts {
        for byte in part.as_bytes() {
            value ^= u64::from(*byte);
            value = value.wrapping_mul(0x100000001b3);
        }
    }

    format!("local_fnv64_{value:016x}")
}

pub fn build_local_guardian_descriptor_skeleton(
    descriptor_id: &str,
    route_id: &str,
    source_chain_id: &str,
    mint_token: &str,
    guardian_set_id: &str,
    threshold: u8,
    guardian_count: u8,
    guardian_seed_label: &str,
) -> Result<XxxlLocalGuardianDescriptorSkeleton, XxxlLocalGuardianDescriptorSkeletonError> {
    if descriptor_id.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyDescriptorId);
    }

    if route_id.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyRouteId);
    }

    if source_chain_id.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptySourceChainId);
    }

    if mint_token.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyMintToken);
    }

    if guardian_set_id.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyGuardianSetId);
    }

    if guardian_seed_label.trim().is_empty() {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyGuardianSeedLabel);
    }

    if guardian_count == 0 {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::ZeroGuardianCount);
    }

    if threshold == 0 {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::ZeroThreshold);
    }

    if threshold > guardian_count {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::ThresholdExceedsGuardianCount);
    }

    for value in [
        descriptor_id,
        route_id,
        source_chain_id,
        mint_token,
        guardian_set_id,
        guardian_seed_label,
    ] {
        validate_local_guardian_descriptor_text_skeleton(value)?;
    }

    let guardian_public_key_fixtures = build_local_guardian_public_key_fixtures_skeleton(
        descriptor_id,
        guardian_seed_label,
        guardian_count,
    )?;

    let descriptor_integrity_hash_value = local_integrity_hash_skeleton(&[
        descriptor_id,
        route_id,
        source_chain_id,
        mint_token,
        guardian_set_id,
        &threshold.to_string(),
        &guardian_count.to_string(),
        guardian_seed_label,
    ]);

    Ok(XxxlLocalGuardianDescriptorSkeleton {
        descriptor_version: DESCRIPTOR_VERSION,
        descriptor_status: XXXL_LOCAL_GUARDIAN_DESCRIPTOR_SKELETON_STATUS,
        descriptor_id: descriptor_id.to_string(),
        descriptor_scope: "local_only".to_string(),
        network_scope: "local_validator_only".to_string(),
        route_id: route_id.to_string(),
        source_chain_id: source_chain_id.to_string(),
        mint_token: mint_token.to_string(),
        guardian_set_id: guardian_set_id.to_string(),
        threshold,
        guardian_count,
        guardian_public_key_fixtures,
        public_key_encoding: PUBLIC_KEY_ENCODING,
        signature_algorithm: SIGNATURE_ALGORITHM_LABEL,
        message_hash_algorithm: MESSAGE_HASH_ALGORITHM_LABEL,
        canonical_message_schema_version: "stage-1-gateway-message-v1".to_string(),
        descriptor_integrity_hash_algorithm: INTEGRITY_HASH_ALGORITHM_LABEL,
        descriptor_integrity_hash_value,
        descriptor_source: "local_guardian_descriptor_skeleton".to_string(),
        activation_status: "local_fixture_only_not_active_on_testnet".to_string(),
        rotation_policy: "local_fixture_rotation_only_not_runtime_policy".to_string(),
        emergency_disable_policy: "local_fixture_disable_only_not_runtime_policy".to_string(),
        no_signing_material_statement:
            "This descriptor contains local public key fixtures only and no signing material."
                .to_string(),
        local_only: LOCAL_ONLY,
        testnet_allowed: TESTNET_ALLOWED,
        live_route_allowed: LIVE_ROUTE_ALLOWED,
        signing_enabled: SIGNING_ENABLED,
        guardian_package_construction_enabled: GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED,
        production_keys_allowed: PRODUCTION_KEYS_ALLOWED,
        writes_to_disk: WRITES_TO_DISK,
        local_validator_execution_approved: LOCAL_VALIDATOR_EXECUTION_APPROVED,
    })
}

pub fn descriptor_threshold_would_pass_skeleton(
    descriptor: &XxxlLocalGuardianDescriptorSkeleton,
    unique_valid_approval_count: u8,
) -> bool {
    unique_valid_approval_count >= descriptor.threshold
        && unique_valid_approval_count <= descriptor.guardian_count
}

pub fn validate_local_guardian_descriptor_safety_skeleton(
    descriptor: &XxxlLocalGuardianDescriptorSkeleton,
) -> Result<(), XxxlLocalGuardianDescriptorSkeletonError> {
    if !descriptor.local_only
        || descriptor.testnet_allowed
        || descriptor.live_route_allowed
        || descriptor.signing_enabled
        || descriptor.guardian_package_construction_enabled
        || descriptor.production_keys_allowed
        || descriptor.writes_to_disk
        || descriptor.local_validator_execution_approved
    {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::UnsafeTextPattern);
    }

    if descriptor.threshold == 0 {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::ZeroThreshold);
    }

    if descriptor.guardian_count == 0 {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::ZeroGuardianCount);
    }

    if descriptor.threshold > descriptor.guardian_count {
        return Err(XxxlLocalGuardianDescriptorSkeletonError::ThresholdExceedsGuardianCount);
    }

    for guardian in &descriptor.guardian_public_key_fixtures {
        validate_local_guardian_descriptor_text_skeleton(&guardian.fixture_id)?;
        validate_local_guardian_descriptor_text_skeleton(&guardian.public_key_fixture)?;

        if !guardian.local_only || guardian.can_sign || guardian.production_key {
            return Err(XxxlLocalGuardianDescriptorSkeletonError::UnsafeTextPattern);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_status_and_flags_are_local_only_not_signing() {
        assert_eq!(
            XXXL_LOCAL_GUARDIAN_DESCRIPTOR_SKELETON_STATUS,
            "LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING"
        );
        assert!(LOCAL_ONLY);
        assert!(!TESTNET_ALLOWED);
        assert!(!LIVE_ROUTE_ALLOWED);
        assert!(!SIGNING_ENABLED);
        assert!(!GUARDIAN_PACKAGE_CONSTRUCTION_ENABLED);
        assert!(!PRODUCTION_KEYS_ALLOWED);
        assert!(!WRITES_TO_DISK);
        assert!(!LOCAL_VALIDATOR_EXECUTION_APPROVED);
    }

    #[test]
    fn signing_material_markers_are_rejected() {
        assert!(contains_signing_material_marker_skeleton("private_key"));
        assert!(contains_signing_material_marker_skeleton("secret_key"));
        assert!(contains_signing_material_marker_skeleton("keypair path"));
        assert!(contains_signing_material_marker_skeleton("mnemonic words"));
        assert!(contains_signing_material_marker_skeleton("seed_phrase"));
        assert!(contains_signing_material_marker_skeleton("BEGIN PRIVATE KEY"));
        assert!(!contains_signing_material_marker_skeleton("local descriptor fixture"));
    }

    #[test]
    fn local_public_key_fixture_is_deterministic_and_non_signing() {
        let first = deterministic_local_guardian_public_key_fixture_skeleton(
            "descriptor_alpha",
            "guardian seed alpha",
            0,
        )
        .expect("first fixture");

        let second = deterministic_local_guardian_public_key_fixture_skeleton(
            "descriptor_alpha",
            "guardian seed alpha",
            0,
        )
        .expect("second fixture");

        assert_eq!(first, second);
        assert!(first.local_only);
        assert!(!first.can_sign);
        assert!(!first.production_key);
        assert!(first.public_key_fixture.contains("not_real_key"));
    }

    #[test]
    fn public_key_fixture_changes_by_index() {
        let first = deterministic_local_guardian_public_key_fixture_skeleton(
            "descriptor_alpha",
            "guardian seed alpha",
            0,
        )
        .expect("first fixture");

        let second = deterministic_local_guardian_public_key_fixture_skeleton(
            "descriptor_alpha",
            "guardian seed alpha",
            1,
        )
        .expect("second fixture");

        assert_ne!(first.public_key_fixture, second.public_key_fixture);
        assert_ne!(first.fixture_id, second.fixture_id);
    }

    #[test]
    fn public_key_fixture_list_rejects_zero_count() {
        assert_eq!(
            build_local_guardian_public_key_fixtures_skeleton(
                "descriptor_alpha",
                "guardian seed alpha",
                0
            ),
            Err(XxxlLocalGuardianDescriptorSkeletonError::ZeroGuardianCount)
        );
    }

    #[test]
    fn descriptor_rejects_empty_required_fields() {
        assert_eq!(
            build_local_guardian_descriptor_skeleton(
                "",
                "route_local",
                "chain_local",
                "mint_local",
                "guardian_set_1",
                2,
                3,
                "guardian seed alpha",
            ),
            Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyDescriptorId)
        );

        assert_eq!(
            build_local_guardian_descriptor_skeleton(
                "descriptor_alpha",
                "",
                "chain_local",
                "mint_local",
                "guardian_set_1",
                2,
                3,
                "guardian seed alpha",
            ),
            Err(XxxlLocalGuardianDescriptorSkeletonError::EmptyRouteId)
        );
    }

    #[test]
    fn descriptor_rejects_invalid_thresholds() {
        assert_eq!(
            build_local_guardian_descriptor_skeleton(
                "descriptor_alpha",
                "route_local",
                "chain_local",
                "mint_local",
                "guardian_set_1",
                0,
                3,
                "guardian seed alpha",
            ),
            Err(XxxlLocalGuardianDescriptorSkeletonError::ZeroThreshold)
        );

        assert_eq!(
            build_local_guardian_descriptor_skeleton(
                "descriptor_alpha",
                "route_local",
                "chain_local",
                "mint_local",
                "guardian_set_1",
                4,
                3,
                "guardian seed alpha",
            ),
            Err(XxxlLocalGuardianDescriptorSkeletonError::ThresholdExceedsGuardianCount)
        );
    }

    #[test]
    fn descriptor_rejects_unsafe_text() {
        assert_eq!(
            build_local_guardian_descriptor_skeleton(
                "descriptor_alpha",
                "route_local",
                "chain_local",
                "mint_local",
                "guardian_set_1",
                2,
                3,
                "production seed",
            ),
            Err(XxxlLocalGuardianDescriptorSkeletonError::UnsafeTextPattern)
        );

        assert_eq!(
            build_local_guardian_descriptor_skeleton(
                "descriptor_alpha",
                "route_local",
                "chain_local",
                "mint_local",
                "guardian_set_1",
                2,
                3,
                "private_key seed",
            ),
            Err(XxxlLocalGuardianDescriptorSkeletonError::UnsafeTextPattern)
        );
    }

    #[test]
    fn descriptor_builds_local_only_non_signing_model() {
        let descriptor = build_local_guardian_descriptor_skeleton(
            "descriptor_alpha",
            "route_local",
            "chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("descriptor");

        assert_eq!(
            descriptor.descriptor_status,
            "LOCAL_ONLY_GUARDIAN_DESCRIPTOR_SKELETON_NOT_SIGNING"
        );
        assert_eq!(descriptor.descriptor_version, DESCRIPTOR_VERSION);
        assert_eq!(descriptor.network_scope, "local_validator_only");
        assert_eq!(descriptor.threshold, 2);
        assert_eq!(descriptor.guardian_count, 3);
        assert_eq!(descriptor.guardian_public_key_fixtures.len(), 3);
        assert!(descriptor.local_only);
        assert!(!descriptor.testnet_allowed);
        assert!(!descriptor.live_route_allowed);
        assert!(!descriptor.signing_enabled);
        assert!(!descriptor.guardian_package_construction_enabled);
        assert!(!descriptor.production_keys_allowed);
        assert!(!descriptor.writes_to_disk);
        assert!(!descriptor.local_validator_execution_approved);
        assert!(descriptor
            .descriptor_integrity_hash_value
            .starts_with("local_fnv64_"));
    }

    #[test]
    fn descriptor_safety_validation_accepts_safe_descriptor() {
        let descriptor = build_local_guardian_descriptor_skeleton(
            "descriptor_alpha",
            "route_local",
            "chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("descriptor");

        validate_local_guardian_descriptor_safety_skeleton(&descriptor)
            .expect("safe descriptor");
    }

    #[test]
    fn descriptor_threshold_pass_model_is_stable() {
        let descriptor = build_local_guardian_descriptor_skeleton(
            "descriptor_alpha",
            "route_local",
            "chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("descriptor");

        assert!(!descriptor_threshold_would_pass_skeleton(&descriptor, 0));
        assert!(!descriptor_threshold_would_pass_skeleton(&descriptor, 1));
        assert!(descriptor_threshold_would_pass_skeleton(&descriptor, 2));
        assert!(descriptor_threshold_would_pass_skeleton(&descriptor, 3));
        assert!(!descriptor_threshold_would_pass_skeleton(&descriptor, 4));
    }

    #[test]
    fn integrity_hash_changes_when_descriptor_changes() {
        let first = build_local_guardian_descriptor_skeleton(
            "descriptor_alpha",
            "route_local",
            "chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("first");

        let second = build_local_guardian_descriptor_skeleton(
            "descriptor_beta",
            "route_local",
            "chain_local",
            "mint_local",
            "guardian_set_1",
            2,
            3,
            "guardian seed alpha",
        )
        .expect("second");

        assert_ne!(
            first.descriptor_integrity_hash_value,
            second.descriptor_integrity_hash_value
        );
    }
}

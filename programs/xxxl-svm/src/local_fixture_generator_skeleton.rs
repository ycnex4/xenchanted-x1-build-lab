use solana_program::pubkey::Pubkey;

pub const XXXL_LOCAL_FIXTURE_GENERATOR_SKELETON_STATUS: &str =
    "LOCAL_ONLY_FIXTURE_GENERATOR_SKELETON_NOT_EXECUTABLE_DRY_RUN";

pub const LOCAL_ONLY: bool = true;
pub const TESTNET_ALLOWED: bool = false;
pub const LIVE_RPC_ALLOWED: bool = false;
pub const PRODUCTION_KEYS_ALLOWED: bool = false;
pub const SUBMIT_COMMANDS_ALLOWED: bool = false;
pub const DEPLOY_COMMANDS_ALLOWED: bool = false;
pub const UPGRADE_COMMANDS_ALLOWED: bool = false;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalFixtureGeneratorSkeletonError {
    EmptyFixtureSetId,
    EmptyDeterministicSeedLabel,
    UnsafeTextPattern,
    NonLocalBoundary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalFixtureManifestSkeleton {
    pub manifest_version: &'static str,
    pub status: &'static str,
    pub local_only: bool,
    pub testnet_allowed: bool,
    pub live_rpc_allowed: bool,
    pub production_keys_allowed: bool,
    pub fixture_set_id: String,
    pub fixture_set_name: String,
    pub deterministic_seed_label: String,
    pub program_fixture_id: String,
    pub account_fixture_ids: Vec<String>,
    pub instruction_fixture_ids: Vec<String>,
    pub success_scenario_ids: Vec<String>,
    pub failure_scenario_ids: Vec<String>,
    pub mutation_invariance_ids: Vec<String>,
    pub safety_report_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalFixtureSafetyReportSkeleton {
    pub schema_version: &'static str,
    pub safety_report_id: String,
    pub local_only: bool,
    pub testnet_allowed: bool,
    pub live_rpc_detected: bool,
    pub production_keys_detected: bool,
    pub key_material_paths_detected: bool,
    pub private_material_detected: bool,
    pub submit_commands_detected: bool,
    pub deploy_commands_detected: bool,
    pub upgrade_commands_detected: bool,
    pub result: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlLocalProgramFixtureSkeleton {
    pub local_program_id: Pubkey,
    pub live_route_enabled: bool,
    pub b1c7_guard_intact: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlLocalPubkeyFixtureSkeleton {
    pub program_id: Pubkey,
    pub spl_mint: Pubkey,
    pub gateway_config: Pubkey,
    pub guardian_set: Pubkey,
    pub mint_state: Pubkey,
    pub processed_event: Pubkey,
    pub recipient_owner: Pubkey,
    pub recipient_token_account: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalFixtureSetSkeleton {
    pub manifest: XxxlLocalFixtureManifestSkeleton,
    pub safety_report: XxxlLocalFixtureSafetyReportSkeleton,
    pub program_fixture: XxxlLocalProgramFixtureSkeleton,
    pub pubkeys: XxxlLocalPubkeyFixtureSkeleton,
}

fn deterministic_pubkey(seed_byte: u8, domain_byte: u8) -> Pubkey {
    let mut bytes = [0u8; 32];
    bytes[0] = 0x58;
    bytes[1] = 0x58;
    bytes[2] = 0x58;
    bytes[3] = 0x4c;
    bytes[4] = seed_byte;
    bytes[31] = domain_byte;
    Pubkey::new_from_array(bytes)
}

pub fn contains_unsafe_fixture_text(input: &str) -> bool {
    let lowered = input.to_ascii_lowercase();

    lowered.contains("testnet")
        || lowered.contains("mainnet")
        || lowered.contains("live rpc")
        || lowered.contains("fee payer")
        || lowered.contains("production")
        || lowered.contains("submit")
        || lowered.contains("deploy")
        || lowered.contains("upgrade")
        || lowered.contains("mnemonic")
        || lowered.contains("secret")
        || lowered.contains("private")
        || lowered.contains("keypair")
        || lowered.contains("url scheme")
}

pub fn validate_local_fixture_generator_boundary_skeleton(
) -> Result<(), XxxlLocalFixtureGeneratorSkeletonError> {
    if LOCAL_ONLY
        && !TESTNET_ALLOWED
        && !LIVE_RPC_ALLOWED
        && !PRODUCTION_KEYS_ALLOWED
        && !SUBMIT_COMMANDS_ALLOWED
        && !DEPLOY_COMMANDS_ALLOWED
        && !UPGRADE_COMMANDS_ALLOWED
    {
        Ok(())
    } else {
        Err(XxxlLocalFixtureGeneratorSkeletonError::NonLocalBoundary)
    }
}

pub fn build_local_fixture_manifest_skeleton(
    fixture_set_id: &str,
    fixture_set_name: &str,
    deterministic_seed_label: &str,
) -> Result<XxxlLocalFixtureManifestSkeleton, XxxlLocalFixtureGeneratorSkeletonError> {
    validate_local_fixture_generator_boundary_skeleton()?;

    if fixture_set_id.trim().is_empty() {
        return Err(XxxlLocalFixtureGeneratorSkeletonError::EmptyFixtureSetId);
    }

    if deterministic_seed_label.trim().is_empty() {
        return Err(XxxlLocalFixtureGeneratorSkeletonError::EmptyDeterministicSeedLabel);
    }

    let combined = format!(
        "{} {} {}",
        fixture_set_id, fixture_set_name, deterministic_seed_label
    );

    if contains_unsafe_fixture_text(&combined) {
        return Err(XxxlLocalFixtureGeneratorSkeletonError::UnsafeTextPattern);
    }

    Ok(XxxlLocalFixtureManifestSkeleton {
        manifest_version: "1",
        status: "LOCAL_VALIDATOR_ONLY_FIXTURE_SET",
        local_only: true,
        testnet_allowed: false,
        live_rpc_allowed: false,
        production_keys_allowed: false,
        fixture_set_id: fixture_set_id.to_string(),
        fixture_set_name: fixture_set_name.to_string(),
        deterministic_seed_label: deterministic_seed_label.to_string(),
        program_fixture_id: format!("{}_program", fixture_set_id),
        account_fixture_ids: vec![
            format!("{}_gateway_config", fixture_set_id),
            format!("{}_guardian_set", fixture_set_id),
            format!("{}_mint_state", fixture_set_id),
            format!("{}_processed_event", fixture_set_id),
            format!("{}_spl_mint", fixture_set_id),
            format!("{}_recipient_token_account", fixture_set_id),
        ],
        instruction_fixture_ids: vec![
            format!("{}_initialize_gateway_config", fixture_set_id),
            format!("{}_initialize_guardian_set", fixture_set_id),
            format!("{}_initialize_mint_state", fixture_set_id),
            format!("{}_consume_gateway_mint", fixture_set_id),
        ],
        success_scenario_ids: vec![format!("{}_success_consume_gateway_mint", fixture_set_id)],
        failure_scenario_ids: vec![
            format!("{}_failure_wrong_account_order", fixture_set_id),
            format!("{}_failure_replayed_processed_event", fixture_set_id),
            format!("{}_failure_zero_amount", fixture_set_id),
            format!("{}_failure_inactive_mint_state", fixture_set_id),
        ],
        mutation_invariance_ids: vec![
            format!("{}_mutation_invariance_wrong_account_order", fixture_set_id),
            format!("{}_mutation_invariance_replayed_processed_event", fixture_set_id),
            format!("{}_mutation_invariance_zero_amount", fixture_set_id),
            format!("{}_mutation_invariance_inactive_mint_state", fixture_set_id),
        ],
        safety_report_id: format!("{}_safety_report", fixture_set_id),
    })
}

pub fn build_local_fixture_safety_report_skeleton(
    fixture_set_id: &str,
) -> Result<XxxlLocalFixtureSafetyReportSkeleton, XxxlLocalFixtureGeneratorSkeletonError> {
    validate_local_fixture_generator_boundary_skeleton()?;

    if fixture_set_id.trim().is_empty() {
        return Err(XxxlLocalFixtureGeneratorSkeletonError::EmptyFixtureSetId);
    }

    if contains_unsafe_fixture_text(fixture_set_id) {
        return Err(XxxlLocalFixtureGeneratorSkeletonError::UnsafeTextPattern);
    }

    Ok(XxxlLocalFixtureSafetyReportSkeleton {
        schema_version: "1",
        safety_report_id: format!("{}_safety_report", fixture_set_id),
        local_only: true,
        testnet_allowed: false,
        live_rpc_detected: false,
        production_keys_detected: false,
        key_material_paths_detected: false,
        private_material_detected: false,
        submit_commands_detected: false,
        deploy_commands_detected: false,
        upgrade_commands_detected: false,
        result: "PASS",
    })
}

pub fn build_local_pubkey_fixture_skeleton(seed_byte: u8) -> XxxlLocalPubkeyFixtureSkeleton {
    XxxlLocalPubkeyFixtureSkeleton {
        program_id: deterministic_pubkey(seed_byte, 0x01),
        spl_mint: deterministic_pubkey(seed_byte, 0x02),
        gateway_config: deterministic_pubkey(seed_byte, 0x03),
        guardian_set: deterministic_pubkey(seed_byte, 0x04),
        mint_state: deterministic_pubkey(seed_byte, 0x05),
        processed_event: deterministic_pubkey(seed_byte, 0x06),
        recipient_owner: deterministic_pubkey(seed_byte, 0x07),
        recipient_token_account: deterministic_pubkey(seed_byte, 0x08),
        token_program: deterministic_pubkey(seed_byte, 0x09),
        system_program: deterministic_pubkey(seed_byte, 0x0a),
    }
}

pub fn build_local_program_fixture_skeleton(seed_byte: u8) -> XxxlLocalProgramFixtureSkeleton {
    XxxlLocalProgramFixtureSkeleton {
        local_program_id: deterministic_pubkey(seed_byte, 0x01),
        live_route_enabled: false,
        b1c7_guard_intact: true,
    }
}

pub fn build_local_fixture_set_skeleton(
    fixture_set_id: &str,
    fixture_set_name: &str,
    deterministic_seed_label: &str,
    seed_byte: u8,
) -> Result<XxxlLocalFixtureSetSkeleton, XxxlLocalFixtureGeneratorSkeletonError> {
    let manifest = build_local_fixture_manifest_skeleton(
        fixture_set_id,
        fixture_set_name,
        deterministic_seed_label,
    )?;
    let safety_report = build_local_fixture_safety_report_skeleton(fixture_set_id)?;
    let program_fixture = build_local_program_fixture_skeleton(seed_byte);
    let pubkeys = build_local_pubkey_fixture_skeleton(seed_byte);

    Ok(XxxlLocalFixtureSetSkeleton {
        manifest,
        safety_report,
        program_fixture,
        pubkeys,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_generator_skeleton_status_is_local_only() {
        assert_eq!(
            XXXL_LOCAL_FIXTURE_GENERATOR_SKELETON_STATUS,
            "LOCAL_ONLY_FIXTURE_GENERATOR_SKELETON_NOT_EXECUTABLE_DRY_RUN"
        );
    }

    #[test]
    fn fixture_generator_boundary_rejects_non_local_flags_by_construction() {
        assert_eq!(validate_local_fixture_generator_boundary_skeleton(), Ok(()));
        assert!(LOCAL_ONLY);
        assert!(!TESTNET_ALLOWED);
        assert!(!LIVE_RPC_ALLOWED);
        assert!(!PRODUCTION_KEYS_ALLOWED);
        assert!(!SUBMIT_COMMANDS_ALLOWED);
        assert!(!DEPLOY_COMMANDS_ALLOWED);
        assert!(!UPGRADE_COMMANDS_ALLOWED);
    }

    #[test]
    fn fixture_manifest_uses_safe_local_defaults() {
        let manifest = build_local_fixture_manifest_skeleton(
            "fixture_alpha",
            "fixture alpha",
            "seed alpha",
        )
        .expect("manifest ok");

        assert_eq!(manifest.manifest_version, "1");
        assert_eq!(manifest.status, "LOCAL_VALIDATOR_ONLY_FIXTURE_SET");
        assert!(manifest.local_only);
        assert!(!manifest.testnet_allowed);
        assert!(!manifest.live_rpc_allowed);
        assert!(!manifest.production_keys_allowed);
        assert_eq!(manifest.fixture_set_id, "fixture_alpha");
        assert_eq!(manifest.account_fixture_ids.len(), 6);
        assert_eq!(manifest.instruction_fixture_ids.len(), 4);
        assert_eq!(manifest.success_scenario_ids.len(), 1);
        assert_eq!(manifest.failure_scenario_ids.len(), 4);
        assert_eq!(manifest.mutation_invariance_ids.len(), 4);
    }

    #[test]
    fn fixture_manifest_rejects_empty_fixture_set_id() {
        assert_eq!(
            build_local_fixture_manifest_skeleton("", "fixture alpha", "seed alpha"),
            Err(XxxlLocalFixtureGeneratorSkeletonError::EmptyFixtureSetId)
        );
    }

    #[test]
    fn fixture_manifest_rejects_empty_seed_label() {
        assert_eq!(
            build_local_fixture_manifest_skeleton("fixture_alpha", "fixture alpha", ""),
            Err(XxxlLocalFixtureGeneratorSkeletonError::EmptyDeterministicSeedLabel)
        );
    }

    #[test]
    fn fixture_manifest_rejects_unsafe_text_patterns() {
        assert_eq!(
            build_local_fixture_manifest_skeleton(
                "fixture_alpha",
                "production fixture",
                "seed alpha"
            ),
            Err(XxxlLocalFixtureGeneratorSkeletonError::UnsafeTextPattern)
        );

        assert_eq!(
            build_local_fixture_manifest_skeleton(
                "fixture_alpha",
                "fixture alpha",
                "testnet seed"
            ),
            Err(XxxlLocalFixtureGeneratorSkeletonError::UnsafeTextPattern)
        );
    }

    #[test]
    fn safety_report_is_pass_and_local_only() {
        let report =
            build_local_fixture_safety_report_skeleton("fixture_alpha").expect("report ok");

        assert_eq!(report.schema_version, "1");
        assert_eq!(report.safety_report_id, "fixture_alpha_safety_report");
        assert!(report.local_only);
        assert!(!report.testnet_allowed);
        assert!(!report.live_rpc_detected);
        assert!(!report.production_keys_detected);
        assert!(!report.key_material_paths_detected);
        assert!(!report.private_material_detected);
        assert!(!report.submit_commands_detected);
        assert!(!report.deploy_commands_detected);
        assert!(!report.upgrade_commands_detected);
        assert_eq!(report.result, "PASS");
    }

    #[test]
    fn deterministic_pubkey_fixture_is_stable_for_same_seed() {
        let a = build_local_pubkey_fixture_skeleton(0x42);
        let b = build_local_pubkey_fixture_skeleton(0x42);

        assert_eq!(a, b);
        assert_ne!(a.program_id, a.spl_mint);
        assert_ne!(a.gateway_config, a.guardian_set);
        assert_ne!(a.mint_state, a.processed_event);
    }

    #[test]
    fn deterministic_pubkey_fixture_changes_for_different_seed() {
        let a = build_local_pubkey_fixture_skeleton(0x42);
        let b = build_local_pubkey_fixture_skeleton(0x43);

        assert_ne!(a.program_id, b.program_id);
        assert_ne!(a.spl_mint, b.spl_mint);
        assert_ne!(a.recipient_token_account, b.recipient_token_account);
    }

    #[test]
    fn full_fixture_set_builds_with_guard_intact_and_live_route_disabled() {
        let fixture_set = build_local_fixture_set_skeleton(
            "fixture_alpha",
            "fixture alpha",
            "seed alpha",
            0x42,
        )
        .expect("fixture set ok");

        assert_eq!(fixture_set.manifest.fixture_set_id, "fixture_alpha");
        assert_eq!(fixture_set.safety_report.result, "PASS");
        assert!(!fixture_set.program_fixture.live_route_enabled);
        assert!(fixture_set.program_fixture.b1c7_guard_intact);
        assert_eq!(
            fixture_set.program_fixture.local_program_id,
            fixture_set.pubkeys.program_id
        );
    }
}

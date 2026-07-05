use crate::local_fixture_generator_skeleton::{
    build_local_fixture_set_skeleton, contains_unsafe_fixture_text,
    XxxlLocalFixtureGeneratorSkeletonError, XxxlLocalFixtureSetSkeleton,
};

pub const XXXL_LOCAL_FIXTURE_FILE_EMITTER_SKELETON_STATUS: &str =
    "LOCAL_FIXTURE_FILE_EMITTER_SKELETON_NOT_WRITING_FILES";

pub const FILE_EMISSION_ENABLED: bool = false;
pub const WRITES_TO_DISK: bool = false;
pub const LOCAL_VALIDATOR_EXECUTION_APPROVED: bool = false;
pub const TESTNET_SUBMIT_ENABLED: bool = false;
pub const LIVE_RPC_ENABLED: bool = false;
pub const UPGRADE_ENABLED: bool = false;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalFixtureFileEmitterSkeletonError {
    Generator(XxxlLocalFixtureGeneratorSkeletonError),
    EmptyOutputDirectory,
    UnsafeOutputDirectory,
    MissingExpectedFile,
    UnsafeRenderedText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalFixtureFileSkeleton {
    pub file_name: &'static str,
    pub file_kind: &'static str,
    pub local_only: bool,
    pub would_write_to_disk: bool,
    pub rendered_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalFixtureEmissionPlanSkeleton {
    pub status: &'static str,
    pub output_directory: String,
    pub local_only: bool,
    pub writes_to_disk: bool,
    pub file_emission_enabled: bool,
    pub local_validator_execution_approved: bool,
    pub testnet_submit_enabled: bool,
    pub live_rpc_enabled: bool,
    pub upgrade_enabled: bool,
    pub files: Vec<XxxlLocalFixtureFileSkeleton>,
}

pub fn expected_local_fixture_file_names_skeleton() -> [&'static str; 10] {
    [
        "manifest.json",
        "accounts.json",
        "instructions.json",
        "scenarios.json",
        "expected-snapshots.json",
        "failure-matrix.json",
        "mutation-invariance.json",
        "logs.json",
        "safety-report.json",
        "README.local-only.txt",
    ]
}

pub fn validate_local_fixture_output_directory_skeleton(
    output_directory: &str,
) -> Result<(), XxxlLocalFixtureFileEmitterSkeletonError> {
    let trimmed = output_directory.trim();

    if trimmed.is_empty() {
        return Err(XxxlLocalFixtureFileEmitterSkeletonError::EmptyOutputDirectory);
    }

    if contains_unsafe_fixture_text(trimmed)
        || trimmed.starts_with('/')
        || trimmed.contains("..")
        || trimmed.contains('\\')
    {
        return Err(XxxlLocalFixtureFileEmitterSkeletonError::UnsafeOutputDirectory);
    }

    Ok(())
}

fn render_manifest_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"manifest_version\":\"{}\",\"status\":\"{}\",\"local_only\":{},\"testnet_allowed\":{},\"live_rpc_allowed\":{},\"production_keys_allowed\":{},\"fixture_set_id\":\"{}\",\"safety_report_id\":\"{}\"}}",
        fixture_set.manifest.manifest_version,
        fixture_set.manifest.status,
        fixture_set.manifest.local_only,
        fixture_set.manifest.testnet_allowed,
        fixture_set.manifest.live_rpc_allowed,
        fixture_set.manifest.production_keys_allowed,
        fixture_set.manifest.fixture_set_id,
        fixture_set.manifest.safety_report_id
    )
}

fn render_accounts_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"schema_version\":\"1\",\"local_only\":true,\"accounts\":[\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"]}}",
        fixture_set.pubkeys.gateway_config,
        fixture_set.pubkeys.guardian_set,
        fixture_set.pubkeys.mint_state,
        fixture_set.pubkeys.processed_event,
        fixture_set.pubkeys.spl_mint,
        fixture_set.pubkeys.recipient_token_account
    )
}

fn render_instructions_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"schema_version\":\"1\",\"local_only\":true,\"instruction_fixture_ids\":[\"{}\",\"{}\",\"{}\",\"{}\"]}}",
        fixture_set.manifest.instruction_fixture_ids[0],
        fixture_set.manifest.instruction_fixture_ids[1],
        fixture_set.manifest.instruction_fixture_ids[2],
        fixture_set.manifest.instruction_fixture_ids[3]
    )
}

fn render_scenarios_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"schema_version\":\"1\",\"local_only\":true,\"success_scenarios\":[\"{}\"],\"failure_scenarios\":{}}}",
        fixture_set.manifest.success_scenario_ids[0],
        fixture_set.manifest.failure_scenario_ids.len()
    )
}

fn render_snapshots_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"schema_version\":\"1\",\"local_only\":true,\"fixture_set_id\":\"{}\",\"snapshot_policy\":\"before_and_after\"}}",
        fixture_set.manifest.fixture_set_id
    )
}

fn render_failure_matrix_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"schema_version\":\"1\",\"local_only\":true,\"failure_case_count\":{},\"expected_no_mutation\":true}}",
        fixture_set.manifest.failure_scenario_ids.len()
    )
}

fn render_mutation_invariance_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"schema_version\":\"1\",\"local_only\":true,\"mutation_invariance_count\":{},\"comparison\":\"byte_identical\"}}",
        fixture_set.manifest.mutation_invariance_ids.len()
    )
}

fn render_logs_skeleton() -> String {
    "{\"schema_version\":\"1\",\"local_only\":true,\"logs_are_sanitized\":true,\"forbidden_material_allowed\":false}".to_string()
}

fn render_safety_report_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "{{\"schema_version\":\"{}\",\"safety_report_id\":\"{}\",\"local_only\":{},\"testnet_allowed\":{},\"live_rpc_detected\":{},\"production_keys_detected\":{},\"result\":\"{}\"}}",
        fixture_set.safety_report.schema_version,
        fixture_set.safety_report.safety_report_id,
        fixture_set.safety_report.local_only,
        fixture_set.safety_report.testnet_allowed,
        fixture_set.safety_report.live_rpc_detected,
        fixture_set.safety_report.production_keys_detected,
        fixture_set.safety_report.result
    )
}

fn render_readme_skeleton(fixture_set: &XxxlLocalFixtureSetSkeleton) -> String {
    format!(
        "LOCAL ONLY FIXTURE SET\nfixture_set_id={}\nnot_for_external_network=true\nnot_for_production=true\nwrites_to_disk=false\nexecution_approved=false\n",
        fixture_set.manifest.fixture_set_id
    )
}

fn ensure_rendered_text_is_safe_skeleton(
    text: &str,
) -> Result<(), XxxlLocalFixtureFileEmitterSkeletonError> {
    if text.contains("BEGIN ")
        || text.contains("rpc_url")
        || text.contains("keypair_path")
        || text.contains("mnemonic")
        || text.contains("seed_phrase")
        || text.contains("secret_key")
        || text.contains("private_key")
    {
        return Err(XxxlLocalFixtureFileEmitterSkeletonError::UnsafeRenderedText);
    }

    Ok(())
}

pub fn build_local_fixture_files_skeleton(
    fixture_set: &XxxlLocalFixtureSetSkeleton,
) -> Result<Vec<XxxlLocalFixtureFileSkeleton>, XxxlLocalFixtureFileEmitterSkeletonError> {
    let rendered = vec![
        (
            "manifest.json",
            "manifest",
            render_manifest_skeleton(fixture_set),
        ),
        (
            "accounts.json",
            "accounts",
            render_accounts_skeleton(fixture_set),
        ),
        (
            "instructions.json",
            "instructions",
            render_instructions_skeleton(fixture_set),
        ),
        (
            "scenarios.json",
            "scenarios",
            render_scenarios_skeleton(fixture_set),
        ),
        (
            "expected-snapshots.json",
            "snapshots",
            render_snapshots_skeleton(fixture_set),
        ),
        (
            "failure-matrix.json",
            "failure_matrix",
            render_failure_matrix_skeleton(fixture_set),
        ),
        (
            "mutation-invariance.json",
            "mutation_invariance",
            render_mutation_invariance_skeleton(fixture_set),
        ),
        ("logs.json", "logs", render_logs_skeleton()),
        (
            "safety-report.json",
            "safety_report",
            render_safety_report_skeleton(fixture_set),
        ),
        (
            "README.local-only.txt",
            "readme",
            render_readme_skeleton(fixture_set),
        ),
    ];

    let mut files = Vec::with_capacity(rendered.len());

    for (file_name, file_kind, rendered_text) in rendered {
        ensure_rendered_text_is_safe_skeleton(&rendered_text)?;
        files.push(XxxlLocalFixtureFileSkeleton {
            file_name,
            file_kind,
            local_only: true,
            would_write_to_disk: false,
            rendered_text,
        });
    }

    Ok(files)
}

pub fn validate_expected_local_fixture_files_present_skeleton(
    files: &[XxxlLocalFixtureFileSkeleton],
) -> Result<(), XxxlLocalFixtureFileEmitterSkeletonError> {
    for expected in expected_local_fixture_file_names_skeleton() {
        if !files.iter().any(|file| file.file_name == expected) {
            return Err(XxxlLocalFixtureFileEmitterSkeletonError::MissingExpectedFile);
        }
    }

    Ok(())
}

pub fn build_local_fixture_emission_plan_skeleton(
    output_directory: &str,
    fixture_set_id: &str,
    fixture_set_name: &str,
    deterministic_seed_label: &str,
    seed_byte: u8,
) -> Result<XxxlLocalFixtureEmissionPlanSkeleton, XxxlLocalFixtureFileEmitterSkeletonError> {
    validate_local_fixture_output_directory_skeleton(output_directory)?;

    let fixture_set = build_local_fixture_set_skeleton(
        fixture_set_id,
        fixture_set_name,
        deterministic_seed_label,
        seed_byte,
    )
    .map_err(XxxlLocalFixtureFileEmitterSkeletonError::Generator)?;

    let files = build_local_fixture_files_skeleton(&fixture_set)?;

    validate_expected_local_fixture_files_present_skeleton(&files)?;

    Ok(XxxlLocalFixtureEmissionPlanSkeleton {
        status: XXXL_LOCAL_FIXTURE_FILE_EMITTER_SKELETON_STATUS,
        output_directory: output_directory.to_string(),
        local_only: true,
        writes_to_disk: WRITES_TO_DISK,
        file_emission_enabled: FILE_EMISSION_ENABLED,
        local_validator_execution_approved: LOCAL_VALIDATOR_EXECUTION_APPROVED,
        testnet_submit_enabled: TESTNET_SUBMIT_ENABLED,
        live_rpc_enabled: LIVE_RPC_ENABLED,
        upgrade_enabled: UPGRADE_ENABLED,
        files,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_emitter_status_is_non_writing_skeleton() {
        assert_eq!(
            XXXL_LOCAL_FIXTURE_FILE_EMITTER_SKELETON_STATUS,
            "LOCAL_FIXTURE_FILE_EMITTER_SKELETON_NOT_WRITING_FILES"
        );
        assert!(!FILE_EMISSION_ENABLED);
        assert!(!WRITES_TO_DISK);
        assert!(!LOCAL_VALIDATOR_EXECUTION_APPROVED);
        assert!(!TESTNET_SUBMIT_ENABLED);
        assert!(!LIVE_RPC_ENABLED);
        assert!(!UPGRADE_ENABLED);
    }

    #[test]
    fn expected_file_names_are_complete_and_stable() {
        let names = expected_local_fixture_file_names_skeleton();

        assert_eq!(names.len(), 10);
        assert_eq!(names[0], "manifest.json");
        assert_eq!(names[1], "accounts.json");
        assert_eq!(names[2], "instructions.json");
        assert_eq!(names[3], "scenarios.json");
        assert_eq!(names[4], "expected-snapshots.json");
        assert_eq!(names[5], "failure-matrix.json");
        assert_eq!(names[6], "mutation-invariance.json");
        assert_eq!(names[7], "logs.json");
        assert_eq!(names[8], "safety-report.json");
        assert_eq!(names[9], "README.local-only.txt");
    }

    #[test]
    fn output_directory_rejects_empty_and_unsafe_paths() {
        assert_eq!(
            validate_local_fixture_output_directory_skeleton(""),
            Err(XxxlLocalFixtureFileEmitterSkeletonError::EmptyOutputDirectory)
        );

        assert_eq!(
            validate_local_fixture_output_directory_skeleton("/tmp/fixtures"),
            Err(XxxlLocalFixtureFileEmitterSkeletonError::UnsafeOutputDirectory)
        );

        assert_eq!(
            validate_local_fixture_output_directory_skeleton("../fixtures"),
            Err(XxxlLocalFixtureFileEmitterSkeletonError::UnsafeOutputDirectory)
        );

        assert_eq!(
            validate_local_fixture_output_directory_skeleton("tmp/testnet-fixtures"),
            Err(XxxlLocalFixtureFileEmitterSkeletonError::UnsafeOutputDirectory)
        );
    }

    #[test]
    fn output_directory_accepts_local_disposable_relative_path() {
        assert_eq!(
            validate_local_fixture_output_directory_skeleton(
                "tmp/local-validator-fixtures/phase-41k6-b6-local-only"
            ),
            Ok(())
        );
    }

    #[test]
    fn local_fixture_files_are_built_without_disk_writes() {
        let fixture_set = build_local_fixture_set_skeleton(
            "fixture_alpha",
            "fixture alpha",
            "seed alpha",
            0x42,
        )
        .expect("fixture set ok");

        let files = build_local_fixture_files_skeleton(&fixture_set).expect("files ok");

        assert_eq!(files.len(), 10);
        assert!(files.iter().all(|file| file.local_only));
        assert!(files.iter().all(|file| !file.would_write_to_disk));
        validate_expected_local_fixture_files_present_skeleton(&files).expect("all files present");
    }

    #[test]
    fn rendered_files_include_manifest_and_safety_report() {
        let fixture_set = build_local_fixture_set_skeleton(
            "fixture_alpha",
            "fixture alpha",
            "seed alpha",
            0x42,
        )
        .expect("fixture set ok");

        let files = build_local_fixture_files_skeleton(&fixture_set).expect("files ok");

        let manifest = files
            .iter()
            .find(|file| file.file_name == "manifest.json")
            .expect("manifest");
        assert!(manifest.rendered_text.contains("LOCAL_VALIDATOR_ONLY_FIXTURE_SET"));
        assert!(manifest.rendered_text.contains("fixture_alpha"));

        let safety = files
            .iter()
            .find(|file| file.file_name == "safety-report.json")
            .expect("safety");
        assert!(safety.rendered_text.contains("PASS"));
        assert!(safety.rendered_text.contains("fixture_alpha_safety_report"));
    }

    #[test]
    fn missing_expected_file_is_rejected() {
        let files = vec![XxxlLocalFixtureFileSkeleton {
            file_name: "manifest.json",
            file_kind: "manifest",
            local_only: true,
            would_write_to_disk: false,
            rendered_text: "{}".to_string(),
        }];

        assert_eq!(
            validate_expected_local_fixture_files_present_skeleton(&files),
            Err(XxxlLocalFixtureFileEmitterSkeletonError::MissingExpectedFile)
        );
    }

    #[test]
    fn full_emission_plan_is_local_only_and_non_writing() {
        let plan = build_local_fixture_emission_plan_skeleton(
            "tmp/local-validator-fixtures/phase-41k6-b6-local-only",
            "fixture_alpha",
            "fixture alpha",
            "seed alpha",
            0x42,
        )
        .expect("plan ok");

        assert_eq!(
            plan.status,
            "LOCAL_FIXTURE_FILE_EMITTER_SKELETON_NOT_WRITING_FILES"
        );
        assert_eq!(
            plan.output_directory,
            "tmp/local-validator-fixtures/phase-41k6-b6-local-only"
        );
        assert!(plan.local_only);
        assert!(!plan.writes_to_disk);
        assert!(!plan.file_emission_enabled);
        assert!(!plan.local_validator_execution_approved);
        assert!(!plan.testnet_submit_enabled);
        assert!(!plan.live_rpc_enabled);
        assert!(!plan.upgrade_enabled);
        assert_eq!(plan.files.len(), 10);
    }

    #[test]
    fn full_emission_plan_rejects_unsafe_fixture_identity() {
        let err = build_local_fixture_emission_plan_skeleton(
            "tmp/local-validator-fixtures/phase-41k6-b6-local-only",
            "fixture_alpha",
            "production fixture",
            "seed alpha",
            0x42,
        )
        .expect_err("unsafe fixture should fail");

        assert_eq!(
            err,
            XxxlLocalFixtureFileEmitterSkeletonError::Generator(
                XxxlLocalFixtureGeneratorSkeletonError::UnsafeTextPattern
            )
        );
    }

    #[test]
    fn rendered_text_safety_rejects_forbidden_markers() {
        assert_eq!(
            ensure_rendered_text_is_safe_skeleton("contains private_key marker"),
            Err(XxxlLocalFixtureFileEmitterSkeletonError::UnsafeRenderedText)
        );

        assert_eq!(
            ensure_rendered_text_is_safe_skeleton("safe local text"),
            Ok(())
        );
    }
}

use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::io;
use std::path::{Component, Path};

use xxxl_svm::local_fixture_file_emitter_skeleton::{
    build_local_fixture_emission_plan_skeleton, expected_local_fixture_file_names_skeleton,
};

const OUTPUT_DIRECTORY: &str = "tmp/local-validator-fixtures/phase-41k6-b6-local-only";
const FIXTURE_SET_ID: &str = "phase_41k6_b6_local_only_fixture_set_001";
const FIXTURE_SET_NAME: &str = "phase 41k6 b6 local only fixture set 001";
const DETERMINISTIC_SEED_LABEL: &str = "phase 41k6 b6 local only deterministic seed 001";
const SEED_BYTE: u8 = 0x42;

fn boxed_error(message: impl Into<String>) -> Box<dyn Error> {
    Box::new(io::Error::new(io::ErrorKind::Other, message.into()))
}

fn validate_output_directory(output_directory: &str) -> Result<(), Box<dyn Error>> {
    if output_directory != OUTPUT_DIRECTORY {
        return Err(boxed_error("output directory does not match the B6.58 approved path"));
    }

    let path = Path::new(output_directory);

    if path.is_absolute() {
        return Err(boxed_error("absolute output directory is forbidden"));
    }

    for component in path.components() {
        if matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        ) {
            return Err(boxed_error("unsafe output directory component detected"));
        }
    }

    Ok(())
}

fn ensure_file_name_is_safe(file_name: &str) -> Result<(), Box<dyn Error>> {
    if file_name.trim().is_empty()
        || file_name.contains('/')
        || file_name.contains('\\')
        || file_name.contains("..")
    {
        return Err(boxed_error(format!("unsafe fixture file name: {file_name}")));
    }

    Ok(())
}

fn ensure_rendered_text_is_b6_58_safe(file_name: &str, text: &str) -> Result<(), Box<dyn Error>> {
    let lowered = text.to_ascii_lowercase();

    let forbidden_fragments = [
        "begin ",
        concat!("rpc", "_url"),
        concat!("keypair", "_path"),
        concat!("mnemonic", ":"),
        concat!("mnemonic", "="),
        concat!("seed", "_phrase"),
        concat!("secret", "_key"),
        concat!("private", "_key"),
        "http://",
        "https://",
        "ws://",
        "wss://",
    ];

    for forbidden in forbidden_fragments {
        if lowered.contains(forbidden) {
            return Err(boxed_error(format!(
                "forbidden material marker detected in {file_name}: {forbidden}"
            )));
        }
    }

    Ok(())
}

fn assert_expected_file_set(actual: &BTreeSet<String>) -> Result<(), Box<dyn Error>> {
    let expected: BTreeSet<String> = expected_local_fixture_file_names_skeleton()
        .iter()
        .map(|name| (*name).to_string())
        .collect();

    if actual != &expected {
        return Err(boxed_error(format!(
            "fixture file set mismatch; expected {:?}, got {:?}",
            expected, actual
        )));
    }

    Ok(())
}

fn assert_directory_is_empty(path: &Path) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        return Ok(());
    }

    if !path.is_dir() {
        return Err(boxed_error("approved output path exists but is not a directory"));
    }

    let mut entries = fs::read_dir(path)?;
    if entries.next().is_some() {
        return Err(boxed_error(
            "approved output directory already exists and is not empty",
        ));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    validate_output_directory(OUTPUT_DIRECTORY)?;

    let plan = build_local_fixture_emission_plan_skeleton(
        OUTPUT_DIRECTORY,
        FIXTURE_SET_ID,
        FIXTURE_SET_NAME,
        DETERMINISTIC_SEED_LABEL,
        SEED_BYTE,
    )
    .map_err(|err| boxed_error(format!("failed to build local fixture plan: {err:?}")))?;

    if !plan.local_only {
        return Err(boxed_error("fixture plan is not local-only"));
    }

    if plan.local_validator_execution_approved
        || plan.testnet_submit_enabled
        || plan.live_rpc_enabled
        || plan.upgrade_enabled
    {
        return Err(boxed_error("execution/testnet/live/upgrade flag unexpectedly enabled"));
    }

    if plan.output_directory != OUTPUT_DIRECTORY {
        return Err(boxed_error("plan output directory mismatch"));
    }

    let planned_names: BTreeSet<String> = plan
        .files
        .iter()
        .map(|file| file.file_name.to_string())
        .collect();

    assert_expected_file_set(&planned_names)?;

    for file in &plan.files {
        ensure_file_name_is_safe(file.file_name)?;
        ensure_rendered_text_is_b6_58_safe(file.file_name, &file.rendered_text)?;

        if !file.local_only {
            return Err(boxed_error(format!(
                "fixture file is not marked local-only: {}",
                file.file_name
            )));
        }
    }

    let output_path = Path::new(OUTPUT_DIRECTORY);
    assert_directory_is_empty(output_path)?;
    fs::create_dir_all(output_path)?;

    for file in &plan.files {
        let target = output_path.join(file.file_name);
        fs::write(target, format!("{}\n", file.rendered_text))?;
    }

    let mut emitted_names = BTreeSet::new();

    for entry in fs::read_dir(output_path)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            return Err(boxed_error("unexpected non-file entry in fixture output directory"));
        }

        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| boxed_error("non-utf8 fixture file name"))?;

        emitted_names.insert(name);
    }

    assert_expected_file_set(&emitted_names)?;

    println!("RESULT: OK");
    println!("OUTPUT_DIRECTORY: {OUTPUT_DIRECTORY}");
    println!("FIXTURE_SET_ID: {FIXTURE_SET_ID}");
    println!("GENERATED_FILE_COUNT: {}", emitted_names.len());
    println!(
        "GENERATED_FILES: {}",
        emitted_names.into_iter().collect::<Vec<_>>().join(",")
    );
    println!("LOCAL_VALIDATOR_EXECUTION: NOT_EXECUTED");
    println!("TESTNET_ACTION: NOT_EXECUTED");
    println!("SIGNING: NOT_EXECUTED");
    println!("GUARDIAN_PACKAGE_CONSTRUCTION: NOT_EXECUTED");
    println!("SPL_SETUP: NOT_EXECUTED");
    println!("UPGRADE_INIT_SUBMIT: NOT_EXECUTED");

    Ok(())
}

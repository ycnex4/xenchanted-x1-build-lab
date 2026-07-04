use std::{path::PathBuf, process::Command, str::FromStr, sync::Once};

use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use xxxl_svm::{
    error::XxxlError,
    processed_event_marking_boundary::build_final_consumed_processed_event_account_image,
    state::PROCESSED_EVENT_ACCOUNT_LEN,
};

const HARNESS_MAGIC: &[u8] = b"PHASE_41K4_MARK_ATOMIC_V1";
const CANONICAL_EVENT_KEY: [u8; 32] = [0x44; 32];
const ROUTE_ID: [u8; 32] = [0x11; 32];
const RECIPIENT: [u8; 32] = [0x55; 32];
const CONSUMED_AMOUNT: u128 = 1_000;
const CONSUMED_SLOT: u64 = 77;

static SBF_BUILD: Once = Once::new();

#[test]
fn phase_41k4_zero_lamports_pda_allocates_assigns_and_marks_consumed() {
    with_mollusk(|mollusk, program_id| {
        let rent_min = mollusk
            .sysvars
            .rent
            .minimum_balance(PROCESSED_EVENT_ACCOUNT_LEN);

        assert_marking_success_case(mollusk, program_id, 0, 10_000_000, rent_min);
    });
}

#[test]
fn phase_41k4_rent_exempt_dusted_pda_marks_consumed_without_top_up() {
    with_mollusk(|mollusk, program_id| {
        let rent_min = mollusk
            .sysvars
            .rent
            .minimum_balance(PROCESSED_EVENT_ACCOUNT_LEN);

        assert_marking_success_case(mollusk, program_id, rent_min + 7, 10_000_000, 0);
    });
}

#[test]
fn phase_41k4_insufficient_lamports_pda_tops_up_then_marks_consumed() {
    with_mollusk(|mollusk, program_id| {
        let rent_min = mollusk
            .sysvars
            .rent
            .minimum_balance(PROCESSED_EVENT_ACCOUNT_LEN);
        let starting_lamports = rent_min - 123;

        assert_marking_success_case(mollusk, program_id, starting_lamports, 10_000_000, 123);
    });
}

#[test]
fn phase_41k4_top_up_failure_leaves_no_partial_state() {
    with_mollusk(|mollusk, program_id| {
        let processed_event_pda = processed_event_pda(program_id);

        let rent_min = mollusk
            .sysvars
            .rent
            .minimum_balance(PROCESSED_EVENT_ACCOUNT_LEN);
        let starting_processed_lamports = rent_min - 100;
        let starting_payer_lamports = 99;

        let instruction = marking_instruction(program_id, processed_event_pda);
        let accounts = marking_accounts(
            processed_event_pda,
            starting_processed_lamports,
            starting_payer_lamports,
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &accounts,
            &[
                Check::err(ProgramError::Custom(XxxlError::InvalidRentExemption as u32)),
                Check::account(&processed_event_pda)
                    .lamports(starting_processed_lamports)
                    .owner(&system_program_id())
                    .space(0)
                    .data(&[])
                    .build(),
                Check::account(&rent_payer_pubkey())
                    .lamports(starting_payer_lamports)
                    .owner(&system_program_id())
                    .space(0)
                    .data(&[])
                    .build(),
            ],
        );
    });
}

fn with_mollusk(f: impl FnOnce(&Mollusk, Pubkey)) {
    ensure_sbf_harness_built();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sbf_out_dir = manifest_dir.join("target/sbpf-solana-solana/release");
    std::env::set_var("SBF_OUT_DIR", sbf_out_dir);

    let program_id = Pubkey::new_unique();
    let mollusk = Mollusk::new(&program_id, "xxxl_svm");

    f(&mollusk, program_id);
}

fn ensure_sbf_harness_built() {
    SBF_BUILD.call_once(|| {
        let status = Command::new("cargo")
            .args(["build-sbf", "--features", "phase-41k4-svm-test-harness"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("spawn cargo build-sbf for phase 41K.4 SVM harness");

        assert!(status.success(), "cargo build-sbf harness build failed");
    });
}

fn assert_marking_success_case(
    mollusk: &Mollusk,
    program_id: Pubkey,
    starting_processed_lamports: u64,
    starting_payer_lamports: u64,
    expected_top_up: u64,
) {
    let processed_event_pda = processed_event_pda(program_id);

    let rent_min = mollusk
        .sysvars
        .rent
        .minimum_balance(PROCESSED_EVENT_ACCOUNT_LEN);
    let expected_processed_lamports = starting_processed_lamports + expected_top_up;
    let expected_payer_lamports = starting_payer_lamports - expected_top_up;

    assert!(
        expected_processed_lamports >= rent_min,
        "success case must leave processed-event account rent exempt"
    );

    let expected_image = build_final_consumed_processed_event_account_image(
        &CANONICAL_EVENT_KEY,
        &ROUTE_ID,
        &RECIPIENT,
        CONSUMED_AMOUNT,
        CONSUMED_SLOT,
    )
    .expect("final consumed processed-event image");

    let instruction = marking_instruction(program_id, processed_event_pda);
    let accounts = marking_accounts(
        processed_event_pda,
        starting_processed_lamports,
        starting_payer_lamports,
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::success(),
            Check::account(&processed_event_pda)
                .lamports(expected_processed_lamports)
                .owner(&program_id)
                .space(PROCESSED_EVENT_ACCOUNT_LEN)
                .data(expected_image.as_ref())
                .build(),
            Check::account(&rent_payer_pubkey())
                .lamports(expected_payer_lamports)
                .owner(&system_program_id())
                .space(0)
                .data(&[])
                .build(),
        ],
    );
}

fn marking_instruction(program_id: Pubkey, processed_event_pda: Pubkey) -> Instruction {
    Instruction::new_with_bytes(
        program_id,
        &marking_instruction_data(),
        vec![
            AccountMeta::new(processed_event_pda, false),
            AccountMeta::new(rent_payer_pubkey(), true),
            AccountMeta::new_readonly(system_program_id(), false),
        ],
    )
}

fn marking_instruction_data() -> Vec<u8> {
    let mut data = Vec::with_capacity(HARNESS_MAGIC.len() + 32 + 32 + 32 + 16 + 8);
    data.extend_from_slice(HARNESS_MAGIC);
    data.extend_from_slice(&CANONICAL_EVENT_KEY);
    data.extend_from_slice(&ROUTE_ID);
    data.extend_from_slice(&RECIPIENT);
    data.extend_from_slice(&CONSUMED_AMOUNT.to_le_bytes());
    data.extend_from_slice(&CONSUMED_SLOT.to_le_bytes());
    data
}

fn marking_accounts(
    processed_event_pda: Pubkey,
    processed_event_lamports: u64,
    rent_payer_lamports: u64,
) -> Vec<(Pubkey, Account)> {
    vec![
        (
            processed_event_pda,
            Account::new(processed_event_lamports, 0, &system_program_id()),
        ),
        (
            rent_payer_pubkey(),
            Account::new(rent_payer_lamports, 0, &system_program_id()),
        ),
        (system_program_id(), keyed_system_program_account()),
    ]
}

fn processed_event_pda(program_id: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"xxxl", b"processed-event", CANONICAL_EVENT_KEY.as_ref()],
        &program_id,
    )
    .0
}

fn rent_payer_pubkey() -> Pubkey {
    Pubkey::new_from_array([0x99; 32])
}

fn system_program_id() -> Pubkey {
    Pubkey::from_str("11111111111111111111111111111111").expect("system program id")
}

fn native_loader_id() -> Pubkey {
    Pubkey::from_str("NativeLoader1111111111111111111111111111111").expect("native loader id")
}

fn keyed_system_program_account() -> Account {
    let mut account = Account::new(1_000_000, 0, &native_loader_id());
    account.executable = true;
    account
}

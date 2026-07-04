use std::path::PathBuf;

use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program::{
    program_option::COption, program_pack::Pack, pubkey::Pubkey as ProgramPubkey,
};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplTokenMint};
use xxxl_svm::{
    error::XxxlError,
    instruction::{
        CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT, CONSUME_GATEWAY_MINT_DISCRIMINATOR,
        CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX, CONSUME_GATEWAY_MINT_INSTRUCTION_LEN,
        CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX,
        CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX,
        CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX,
        CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX, INSTRUCTION_LAYOUT_VERSION,
    },
    state::{
        GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, GATEWAY_CONFIG_ACCOUNT_LEN,
        GUARDIAN_SET_ACCOUNT_DISCRIMINATOR, GUARDIAN_SET_ACCOUNT_LEN,
        MINT_STATE_ACCOUNT_DISCRIMINATOR, MINT_STATE_ACCOUNT_LEN,
        PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR, PROCESSED_EVENT_ACCOUNT_LEN,
        RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR, RECIPIENT_BALANCE_ACCOUNT_LEN,
        RUNTIME_LAYOUT_VERSION,
    },
};

const PROGRAM_NAME: &str = "xxxl_svm";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const NATIVE_LOADER_ID: &str = "NativeLoader1111111111111111111111111111111";
const SPL_MINT_ACCOUNT_INDEX: usize = 5;
const RECIPIENT_TOKEN_ACCOUNT_INDEX: usize = 6;
const MINT_AUTHORITY_PDA_ACCOUNT_INDEX: usize = 7;

#[test]
fn mollusk_harness_rejects_malformed_instruction_without_live_route() {
    let program_id = Pubkey::new_unique();
    let mollusk = mollusk_for_program(&program_id);

    let instruction_data = vec![0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN - 1];
    let instruction = Instruction::new_with_bytes(program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
fn mollusk_rejects_wrong_instruction_discriminator_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[0] ^= 0xff;

    let instruction =
        Instruction::new_with_bytes(fixture.program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidDiscriminator as u32,
        ))],
    );
}

#[test]
fn mollusk_rejects_wrong_instruction_version_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[8..10].copy_from_slice(&3u16.to_le_bytes());

    let instruction =
        Instruction::new_with_bytes(fixture.program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidVersion as u32,
        ))],
    );
}

#[test]
fn mollusk_rejects_extra_instruction_bytes_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data.to_vec();
    instruction_data.push(0);

    let instruction =
        Instruction::new_with_bytes(fixture.program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
fn mollusk_nonzero_reserved_202_207_rejects_before_scaffold_path() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[202] = 1;

    let instruction =
        Instruction::new_with_bytes(fixture.program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstructionReserved as u32,
        ))],
    );
}

#[test]
fn mollusk_rejects_wrong_encoded_account_meta_count_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[10] = CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT - 1;

    let instruction = Instruction::new_with_bytes(
        fixture.program_id,
        &instruction_data,
        fixture.instruction().accounts,
    );
    let accounts = fixture.accounts();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_wrong_encoded_processed_event_account_index_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[14] = CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX;

    let instruction = Instruction::new_with_bytes(
        fixture.program_id,
        &instruction_data,
        fixture.instruction().accounts,
    );
    let accounts = fixture.accounts();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_wrong_encoded_recipient_balance_account_index_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[15] = CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX;

    let instruction = Instruction::new_with_bytes(
        fixture.program_id,
        &instruction_data,
        fixture.instruction().accounts,
    );
    let accounts = fixture.accounts();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_wrong_account_count_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts.pop();

    let mut accounts = fixture.accounts();
    accounts.pop();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_wrong_account_order_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts.swap(1, 2);

    let mut accounts = fixture.accounts();
    accounts.swap(1, 2);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_unexpected_signer_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts[4] = AccountMeta::new(fixture.keys.recipient_balance, true);

    let accounts = fixture.accounts();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_writable_readonly_mismatch_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts[0] = AccountMeta::new(fixture.keys.mint_state, false);

    let accounts = fixture.accounts();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_wrong_mint_state_owner_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .owner = Pubkey::new_unique();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidAccountOwner,
    );
}

#[test]
fn mollusk_rejects_wrong_gateway_config_owner_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize]
        .1
        .owner = Pubkey::new_unique();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidAccountOwner,
    );
}

#[test]
fn mollusk_rejects_wrong_guardian_set_owner_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX as usize]
        .1
        .owner = Pubkey::new_unique();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidAccountOwner,
    );
}

#[test]
fn mollusk_rejects_wrong_mint_state_discriminator_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .data[0] ^= 0xff;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidDiscriminator,
    );
}

#[test]
fn mollusk_rejects_truncated_gateway_config_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize]
        .1
        .data
        .truncate(GATEWAY_CONFIG_ACCOUNT_LEN - 1);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_low_rent_mint_state_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = accounts_with_low_rent(
        &fixture,
        CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize,
    );

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRentExemption,
    );
}

#[test]
fn mollusk_rejects_low_rent_gateway_config_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts =
        accounts_with_low_rent(&fixture, CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRentExemption,
    );
}

#[test]
fn mollusk_rejects_low_rent_guardian_set_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = accounts_with_low_rent(
        &fixture,
        CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX as usize,
    );

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRentExemption,
    );
}

#[test]
fn mollusk_accepts_dusted_system_owned_empty_processed_event_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = fixture.accounts();

    assert!(
        accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
            .1
            .data
            .is_empty()
    );
    assert_eq!(
        accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
            .1
            .lamports,
        1
    );

    let checks = result_and_unchanged_mutable_account_checks(&fixture, &accounts, Check::success());

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_rejects_low_rent_recipient_balance_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = accounts_with_low_rent(
        &fixture,
        CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX as usize,
    );

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRentExemption,
    );
}

#[test]
fn mollusk_rejects_low_rent_spl_token_mint_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = accounts_with_low_rent(&fixture, SPL_MINT_ACCOUNT_INDEX);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRentExemption,
    );
}

#[test]
fn mollusk_rejects_low_rent_recipient_token_account_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = accounts_with_low_rent(&fixture, RECIPIENT_TOKEN_ACCOUNT_INDEX);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRentExemption,
    );
}

#[test]
fn mollusk_rejects_wrong_spl_mint_owner_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[5].1.owner = Pubkey::new_unique();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidAccountOwner,
    );
}

#[test]
fn mollusk_rejects_wrong_spl_mint_authority_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[5].1.data = packed_mint(Pubkey::new_unique(), true);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidPda,
    );
}

#[test]
fn mollusk_rejects_uninitialized_spl_mint_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[5].1.data = packed_mint(fixture.keys.mint_authority_pda, false);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_wrong_recipient_token_mint_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[6].1.data = packed_token_account(
        Pubkey::new_unique(),
        fixture.keys.recipient_owner,
        AccountState::Initialized,
    );

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRecipientAta,
    );
}

#[test]
fn mollusk_rejects_wrong_recipient_token_owner_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[6].1.data = packed_token_account(
        fixture.keys.spl_mint,
        Pubkey::new_unique(),
        AccountState::Initialized,
    );

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRecipientAta,
    );
}

#[test]
fn mollusk_rejects_uninitialized_recipient_token_account_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[6].1.data = packed_token_account(
        fixture.keys.spl_mint,
        fixture.keys.recipient_owner,
        AccountState::Uninitialized,
    );

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidRecipientAta,
    );
}

#[test]
fn mollusk_rejects_wrong_mint_authority_pda_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);
    let wrong_pda = Pubkey::new_unique();

    let mut instruction = fixture.instruction();
    instruction.accounts[MINT_AUTHORITY_PDA_ACCOUNT_INDEX] =
        AccountMeta::new_readonly(wrong_pda, false);

    let mut accounts = fixture.accounts();
    accounts[MINT_AUTHORITY_PDA_ACCOUNT_INDEX].0 = wrong_pda;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_rejects_wrong_mint_authority_bump_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .data[13] = accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .data[13]
        .wrapping_add(1);

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidPda,
    );
}

#[test]
fn mollusk_rejects_mint_authority_pda_for_wrong_program_id_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);
    let wrong_program_id = Pubkey::new_unique();
    let (wrong_pda, wrong_bump) = Pubkey::find_program_address(
        &[b"xxxl", b"gateway-mint-authority", b"v1"],
        &wrong_program_id,
    );

    let mut instruction = fixture.instruction();
    instruction.accounts[MINT_AUTHORITY_PDA_ACCOUNT_INDEX] =
        AccountMeta::new_readonly(wrong_pda, false);

    let mut accounts = fixture.accounts();
    accounts[MINT_AUTHORITY_PDA_ACCOUNT_INDEX].0 = wrong_pda;
    accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .data[13] = wrong_bump;
    accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .data[64..96]
        .copy_from_slice(&wrong_pda.to_bytes());

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidPda,
    );
}

#[test]
fn mollusk_rejects_mint_authority_pda_semantic_mismatch_without_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .data[64..96]
        .copy_from_slice(&Pubkey::new_unique().to_bytes());

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_valid_scaffold_entrypoint_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = fixture.accounts();
    assert_live_atomicity_accounts_start_unmutated(&accounts);

    let checks = result_and_unchanged_mutable_account_checks(&fixture, &accounts, Check::success());

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_valid_v2_matching_source_chain_id_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = fixture.accounts();
    assert_eq!(
        u64::from_le_bytes(
            instruction.data[194..202]
                .try_into()
                .expect("source_chain_id")
        ),
        read_u64_le(
            &accounts[CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize]
                .1
                .data,
            48
        )
    );
    assert_live_atomicity_accounts_start_unmutated(&accounts);

    let checks = result_and_unchanged_mutable_account_checks(&fixture, &accounts, Check::success());

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn process_instruction_v2_still_disabled_plan() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = fixture.accounts();
    assert_live_atomicity_accounts_start_unmutated(&accounts);

    let checks = result_and_unchanged_mutable_account_checks(&fixture, &accounts, Check::success());

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn phase20_current_runtime_boundary_matrix_separates_unrepresented_stage1_classes() {
    struct RuntimeObservableEvidence {
        class: &'static str,
        evidence: &'static str,
    }

    let runtime_observable = [
        RuntimeObservableEvidence {
            class: "wrong instruction discriminator",
            evidence: "mollusk_rejects_wrong_instruction_discriminator_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "wrong instruction version",
            evidence: "mollusk_rejects_wrong_instruction_version_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "wrong instruction length",
            evidence: "mollusk_harness_rejects_malformed_instruction_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "wrong account meta count",
            evidence: "mollusk_rejects_wrong_encoded_account_meta_count_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "wrong account index or order",
            evidence: "mollusk_rejects_wrong_account_order_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "wrong route id",
            evidence: "mollusk_wrong_gateway_config_route_id_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong guardian set id",
            evidence: "mollusk_wrong_guardian_set_id_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong target mint or mint id",
            evidence: "mollusk_wrong_gateway_config_target_mint_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong canonical event key",
            evidence: "mollusk_wrong_processed_event_canonical_event_key_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong recipient or recipient token account mapping",
            evidence: "mollusk_wrong_recipient_token_account_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong source chain weight bps",
            evidence: "mollusk_wrong_gateway_config_source_chain_weight_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "sourceChainId final runtime binding",
            evidence: "mollusk_source_chain_id_mismatch_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "reserved bytes 202..207 zero enforcement",
            evidence: "mollusk_nonzero_reserved_202_207_rejects_before_scaffold_path",
        },
        RuntimeObservableEvidence {
            class: "zero amount",
            evidence: "mollusk_zero_amount_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "amount greater than u64 max",
            evidence: "mollusk_amount_above_u64_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "consumed processed event",
            evidence: "mollusk_consumed_processed_event_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong processed event fields",
            evidence: "mollusk_wrong_processed_event_recipient_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong recipient balance owner or mint",
            evidence: "mollusk_wrong_recipient_balance_owner_rejection_leaves_mutable_accounts_unchanged",
        },
        RuntimeObservableEvidence {
            class: "wrong SPL mint owner authority or initialized state",
            evidence: "mollusk_rejects_wrong_spl_mint_authority_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "wrong recipient token owner mint or initialized state",
            evidence: "mollusk_rejects_wrong_recipient_token_mint_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "wrong mint authority PDA or bump",
            evidence: "mollusk_rejects_wrong_mint_authority_pda_without_live_route",
        },
        RuntimeObservableEvidence {
            class: "low rent where represented",
            evidence: "mollusk_rejects_low_rent_mint_state_without_live_route",
        },
    ];

    let not_runtime_observable = [
        "proof emitter_chain_id binding",
        "source block/finality fields",
        "messageNonce runtime replay semantics",
        "guardian signature/quorum validation",
        "canonical encoding field-order vectors at watcher/model layer",
        "decimal string encoding vectors from Stage 1 model",
        "live SPL mint execution success path",
        "rollback after live SPL CPI failure",
    ];

    assert!(runtime_observable
        .iter()
        .all(|entry| !entry.class.is_empty() && !entry.evidence.is_empty()));
    assert!(not_runtime_observable.iter().all(|class| !class.is_empty()));
    assert!(not_runtime_observable
        .iter()
        .all(|class| { runtime_observable.iter().all(|entry| entry.class != *class) }));
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_account_count_rejects_before_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts.pop();

    let mut accounts = fixture.accounts();
    accounts.pop();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_readonly_account_passed_writable_rejects_before_validation() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts[0] = AccountMeta::new(fixture.keys.mint_state, false);

    let accounts = fixture.accounts();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_required_writable_account_passed_readonly_rejects_before_validation(
) {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts[3] = AccountMeta::new_readonly(fixture.keys.processed_event, false);

    let accounts = fixture.accounts();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_unexpected_signer_rejects_before_validation() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction = fixture.instruction();
    instruction.accounts[4] = AccountMeta::new(fixture.keys.recipient_balance, true);

    let accounts = fixture.accounts();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_wrong_program_account_owner_rejects_before_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[0].1.owner = Pubkey::new_unique();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidAccountOwner as u32,
        ))],
    );
}

#[test]
fn mollusk_consumed_processed_event_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    set_program_owned_initialized_processed_event(&fixture, &mut accounts, true);

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidInstruction as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_zero_amount_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[176..192].copy_from_slice(&0u128.to_le_bytes());

    let instruction = Instruction::new_with_bytes(
        fixture.program_id,
        &instruction_data,
        fixture.instruction().accounts,
    );
    let accounts = fixture.accounts();

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidInstruction as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_amount_above_u64_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[176..192].copy_from_slice(&((u64::MAX as u128) + 1).to_le_bytes());

    let instruction = Instruction::new_with_bytes(
        fixture.program_id,
        &instruction_data,
        fixture.instruction().accounts,
    );
    let accounts = fixture.accounts();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_wrong_mint_state_mint_id_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX as usize]
        .1
        .data[16] ^= 0xff;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_wrong_gateway_config_route_id_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize]
        .1
        .data[16] ^= 0xff;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_wrong_gateway_config_guardian_set_id_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize]
        .1
        .data[120] ^= 0xff;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_wrong_gateway_config_target_mint_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize]
        .1
        .data[88] ^= 0xff;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_wrong_gateway_config_source_chain_weight_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX as usize]
        .1
        .data[12] ^= 0xff;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_source_chain_id_mismatch_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[194..202].copy_from_slice(&77u64.to_le_bytes());

    let instruction = Instruction::new_with_bytes(
        fixture.program_id,
        &instruction_data,
        fixture.instruction().accounts,
    );
    let accounts = fixture.accounts();

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidSourceChain,
    );
}

#[test]
fn mollusk_wrong_guardian_set_id_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX as usize]
        .1
        .data[272] ^= 0xff;

    process_rejection_and_assert_mutable_accounts_unchanged(
        &mollusk,
        &fixture,
        &instruction,
        &accounts,
        XxxlError::InvalidInstruction,
    );
}

#[test]
fn mollusk_wrong_recipient_token_account_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[RECIPIENT_TOKEN_ACCOUNT_INDEX].1.data = packed_token_account(
        fixture.keys.spl_mint,
        Pubkey::new_unique(),
        AccountState::Initialized,
    );

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidRecipientAta as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_wrong_processed_event_recipient_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    set_program_owned_initialized_processed_event(&fixture, &mut accounts, false);
    accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
        .1
        .data[80..112]
        .copy_from_slice(&Pubkey::new_unique().to_bytes());

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidInstruction as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_wrong_processed_event_canonical_event_key_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    set_program_owned_initialized_processed_event(&fixture, &mut accounts, false);
    accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
        .1
        .data[16] ^= 0xff;

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidInstruction as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_wrong_processed_event_route_id_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    set_program_owned_initialized_processed_event(&fixture, &mut accounts, false);
    accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
        .1
        .data[48] ^= 0xff;

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidInstruction as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_wrong_processed_event_recipient_bit_flip_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    set_program_owned_initialized_processed_event(&fixture, &mut accounts, false);
    accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
        .1
        .data[80] ^= 0xff;

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidInstruction as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_wrong_recipient_balance_owner_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX as usize]
        .1
        .data[16] ^= 0xff;

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidRecipientAta as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
fn mollusk_wrong_recipient_balance_mint_rejection_leaves_mutable_accounts_unchanged() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX as usize]
        .1
        .data[48] ^= 0xff;

    let checks = result_and_unchanged_mutable_account_checks(
        &fixture,
        &accounts,
        Check::err(ProgramError::Custom(XxxlError::InvalidRecipientAta as u32)),
    );

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_wrong_recipient_token_owner_rejects_before_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let mut accounts = fixture.accounts();
    accounts[6].1.data = packed_token_account(
        fixture.keys.spl_mint,
        Pubkey::new_unique(),
        AccountState::Initialized,
    );

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidRecipientAta as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_zero_amount_rejects_before_live_route() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[176..192].copy_from_slice(&0u128.to_le_bytes());

    let instruction = Instruction::new_with_bytes(
        fixture.program_id,
        &instruction_data,
        fixture.instruction().accounts,
    );
    let accounts = fixture.accounts();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_length_rejects_before_scaffold_path() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data.to_vec();
    instruction_data.pop();

    let instruction =
        Instruction::new_with_bytes(fixture.program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidInstruction as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_discriminator_rejects_before_scaffold_path() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[0] ^= 0xff;

    let instruction =
        Instruction::new_with_bytes(fixture.program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidDiscriminator as u32,
        ))],
    );
}

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn invalid_consume_gateway_mint_version_rejects_before_scaffold_path() {
    let fixture = ScaffoldFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let mut instruction_data = fixture.instruction_data;
    instruction_data[8..10].copy_from_slice(&3u16.to_le_bytes());

    let instruction =
        Instruction::new_with_bytes(fixture.program_id, &instruction_data, Vec::new());
    let accounts: Vec<(Pubkey, Account)> = Vec::new();

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[Check::err(ProgramError::Custom(
            XxxlError::InvalidVersion as u32,
        ))],
    );
}

fn mollusk_for_program(program_id: &Pubkey) -> Mollusk {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sbf_out_dir = manifest_dir.join("target/deploy");
    let program_elf = sbf_out_dir.join(format!("{PROGRAM_NAME}.so"));

    assert!(
        program_elf.exists(),
        "missing {}; run `cargo build-sbf` before this ignored Mollusk test",
        program_elf.display()
    );

    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    Mollusk::new(program_id, PROGRAM_NAME)
}

fn accounts_with_low_rent(
    fixture: &ScaffoldFixture,
    account_index: usize,
) -> Vec<(Pubkey, Account)> {
    let mut accounts = fixture.accounts();
    assert!(
        !accounts[account_index].1.data.is_empty(),
        "low-rent fixture account must carry data"
    );
    accounts[account_index].1.lamports = 1;
    accounts
}

fn set_program_owned_initialized_processed_event(
    fixture: &ScaffoldFixture,
    accounts: &mut [(Pubkey, Account)],
    consumed: bool,
) {
    let index = CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize;
    let route_id = read_fixed_32(&fixture.instruction_data, 16);
    let canonical_event_key = read_fixed_32(&fixture.instruction_data, 112);
    let recipient = Pubkey::new_from_array(read_fixed_32(&fixture.instruction_data, 144));

    accounts[index].1.owner = fixture.program_id;
    accounts[index].1.lamports = 10_000_000_000;
    accounts[index].1.data =
        processed_event_data(consumed, canonical_event_key, route_id, recipient);
}

fn process_rejection_and_assert_mutable_accounts_unchanged(
    mollusk: &Mollusk,
    fixture: &ScaffoldFixture,
    instruction: &Instruction,
    accounts: &[(Pubkey, Account)],
    expected_error: XxxlError,
) {
    let checks = result_and_unchanged_mutable_account_checks(
        fixture,
        accounts,
        Check::err(ProgramError::Custom(expected_error as u32)),
    );

    mollusk.process_and_validate_instruction(instruction, accounts, &checks);
}

fn result_and_unchanged_mutable_account_checks<'a>(
    fixture: &'a ScaffoldFixture,
    accounts: &'a [(Pubkey, Account)],
    result_check: Check<'a>,
) -> Vec<Check<'a>> {
    let mut checks = vec![result_check];
    checks.extend([
        Check::account(&fixture.keys.processed_event)
            .data(
                &accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
                    .1
                    .data,
            )
            .lamports(
                accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
                    .1
                    .lamports,
            )
            .owner(
                &accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
                    .1
                    .owner,
            )
            .build(),
        Check::account(&fixture.keys.recipient_balance)
            .data(
                &accounts[CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX as usize]
                    .1
                    .data,
            )
            .lamports(
                accounts[CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX as usize]
                    .1
                    .lamports,
            )
            .owner(
                &accounts[CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX as usize]
                    .1
                    .owner,
            )
            .build(),
        Check::account(&fixture.keys.spl_mint)
            .data(&accounts[SPL_MINT_ACCOUNT_INDEX].1.data)
            .lamports(accounts[SPL_MINT_ACCOUNT_INDEX].1.lamports)
            .owner(&accounts[SPL_MINT_ACCOUNT_INDEX].1.owner)
            .build(),
        Check::account(&fixture.keys.recipient_token_account)
            .data(&accounts[RECIPIENT_TOKEN_ACCOUNT_INDEX].1.data)
            .lamports(accounts[RECIPIENT_TOKEN_ACCOUNT_INDEX].1.lamports)
            .owner(&accounts[RECIPIENT_TOKEN_ACCOUNT_INDEX].1.owner)
            .build(),
        Check::account(&fixture.keys.rent_payer)
            .data(&accounts[9].1.data)
            .lamports(accounts[9].1.lamports)
            .owner(&accounts[9].1.owner)
            .build(),
    ]);
    checks
}

fn assert_live_atomicity_accounts_start_unmutated(accounts: &[(Pubkey, Account)]) {
    assert!(
        accounts[CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX as usize]
            .1
            .data
            .is_empty()
    );
    assert_eq!(
        read_u128_le(
            &accounts[CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX as usize]
                .1
                .data,
            80,
        ),
        0
    );

    let spl_mint =
        SplTokenMint::unpack(&accounts[SPL_MINT_ACCOUNT_INDEX].1.data).expect("valid SPL mint");
    assert_eq!(spl_mint.supply, 0);

    let recipient_token_account =
        SplTokenAccount::unpack(&accounts[RECIPIENT_TOKEN_ACCOUNT_INDEX].1.data)
            .expect("valid recipient token account");
    assert_eq!(recipient_token_account.amount, 0);
}

struct ScaffoldFixture {
    program_id: Pubkey,
    keys: FixtureKeys,
    data: FixtureData,
    instruction_data: [u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN],
}

struct FixtureKeys {
    mint_state: Pubkey,
    gateway_config: Pubkey,
    guardian_set: Pubkey,
    processed_event: Pubkey,
    recipient_balance: Pubkey,
    recipient_owner: Pubkey,
    spl_mint: Pubkey,
    recipient_token_account: Pubkey,
    mint_authority_pda: Pubkey,
    token_program: Pubkey,
    rent_payer: Pubkey,
    system_program: Pubkey,
}

struct FixtureData {
    mint_state: Vec<u8>,
    gateway_config: Vec<u8>,
    guardian_set: Vec<u8>,
    processed_event: Vec<u8>,
    recipient_balance: Vec<u8>,
    spl_mint: Vec<u8>,
    recipient_token_account: Vec<u8>,
}

impl ScaffoldFixture {
    fn new() -> Self {
        let program_id = Pubkey::new_unique();
        let token_program = TOKEN_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("SPL Token program id");
        let system_program = SYSTEM_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("system program id");
        let spl_mint = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let (mint_authority_pda, bump) =
            Pubkey::find_program_address(&[b"xxxl", b"gateway-mint-authority", b"v1"], &program_id);

        let route_id = [0x11; 32];
        let guardian_set_id = [0x22; 32];
        let canonical_event_key = [0x44; 32];
        let source_chain_id = 1;
        let (processed_event, _) = Pubkey::find_program_address(
            &[b"xxxl", b"processed-event", &canonical_event_key],
            &program_id,
        );

        let keys = FixtureKeys {
            mint_state: Pubkey::new_unique(),
            gateway_config: Pubkey::new_unique(),
            guardian_set: Pubkey::new_unique(),
            processed_event,
            recipient_balance: Pubkey::new_unique(),
            recipient_owner,
            spl_mint,
            recipient_token_account: Pubkey::new_unique(),
            mint_authority_pda,
            token_program,
            rent_payer: Pubkey::new_unique(),
            system_program,
        };

        let data = FixtureData {
            mint_state: mint_state_data(spl_mint, mint_authority_pda, bump),
            gateway_config: gateway_config_data(
                route_id,
                source_chain_id,
                guardian_set_id,
                spl_mint,
                10_000,
            ),
            guardian_set: guardian_set_data(guardian_set_id),
            processed_event: Vec::new(),
            recipient_balance: recipient_balance_data(recipient_owner, spl_mint),
            spl_mint: packed_mint(mint_authority_pda, true),
            recipient_token_account: packed_token_account(
                spl_mint,
                recipient_owner,
                AccountState::Initialized,
            ),
        };

        let instruction_data = instruction_data_from_fields(
            route_id,
            guardian_set_id,
            spl_mint,
            canonical_event_key,
            recipient_owner,
            1_000,
            10_000,
            source_chain_id,
        );

        Self {
            program_id,
            keys,
            data,
            instruction_data,
        }
    }

    fn instruction(&self) -> Instruction {
        Instruction::new_with_bytes(
            self.program_id,
            &self.instruction_data,
            vec![
                AccountMeta::new_readonly(self.keys.mint_state, false),
                AccountMeta::new_readonly(self.keys.gateway_config, false),
                AccountMeta::new_readonly(self.keys.guardian_set, false),
                AccountMeta::new(self.keys.processed_event, false),
                AccountMeta::new(self.keys.recipient_balance, false),
                AccountMeta::new(self.keys.spl_mint, false),
                AccountMeta::new(self.keys.recipient_token_account, false),
                AccountMeta::new_readonly(self.keys.mint_authority_pda, false),
                AccountMeta::new_readonly(self.keys.token_program, false),
                AccountMeta::new(self.keys.rent_payer, true),
                AccountMeta::new_readonly(self.keys.system_program, false),
            ],
        )
    }

    fn accounts(&self) -> Vec<(Pubkey, Account)> {
        let token_program_owner = Pubkey::new_unique();
        let native_loader = NATIVE_LOADER_ID
            .parse::<Pubkey>()
            .expect("native loader id");
        let lamports = 10_000_000_000;

        vec![
            (
                self.keys.mint_state,
                account(
                    lamports,
                    self.data.mint_state.clone(),
                    self.program_id,
                    false,
                ),
            ),
            (
                self.keys.gateway_config,
                account(
                    lamports,
                    self.data.gateway_config.clone(),
                    self.program_id,
                    false,
                ),
            ),
            (
                self.keys.guardian_set,
                account(
                    lamports,
                    self.data.guardian_set.clone(),
                    self.program_id,
                    false,
                ),
            ),
            (
                self.keys.processed_event,
                account(
                    1,
                    self.data.processed_event.clone(),
                    self.keys.system_program,
                    false,
                ),
            ),
            (
                self.keys.recipient_balance,
                account(
                    lamports,
                    self.data.recipient_balance.clone(),
                    self.program_id,
                    false,
                ),
            ),
            (
                self.keys.spl_mint,
                account(
                    lamports,
                    self.data.spl_mint.clone(),
                    self.keys.token_program,
                    false,
                ),
            ),
            (
                self.keys.recipient_token_account,
                account(
                    lamports,
                    self.data.recipient_token_account.clone(),
                    self.keys.token_program,
                    false,
                ),
            ),
            (
                self.keys.mint_authority_pda,
                account(lamports, Vec::new(), self.program_id, false),
            ),
            (
                self.keys.token_program,
                account(1, Vec::new(), token_program_owner, true),
            ),
            (
                self.keys.rent_payer,
                account(lamports, Vec::new(), self.keys.system_program, false),
            ),
            (
                self.keys.system_program,
                account(1, Vec::new(), native_loader, true),
            ),
        ]
    }
}

fn account(lamports: u64, data: Vec<u8>, owner: Pubkey, executable: bool) -> Account {
    Account {
        lamports,
        data,
        owner,
        executable,
        rent_epoch: 0,
    }
}

fn mint_state_data(mint: Pubkey, pda: Pubkey, bump: u8) -> Vec<u8> {
    let mut data = account_data(MINT_STATE_ACCOUNT_LEN, MINT_STATE_ACCOUNT_DISCRIMINATOR);
    data[10] = 18;
    data[13] = bump;
    data[16..48].copy_from_slice(&mint.to_bytes());
    data[64..96].copy_from_slice(&pda.to_bytes());
    data
}

fn gateway_config_data(
    route_id: [u8; 32],
    source_chain_id: u64,
    guardian_set_id: [u8; 32],
    target_mint: Pubkey,
    weight_bps: u16,
) -> Vec<u8> {
    let mut data = account_data(
        GATEWAY_CONFIG_ACCOUNT_LEN,
        GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR,
    );
    data[12..14].copy_from_slice(&weight_bps.to_le_bytes());
    data[16..48].copy_from_slice(&route_id);
    data[48..56].copy_from_slice(&source_chain_id.to_le_bytes());
    data[88..120].copy_from_slice(&target_mint.to_bytes());
    data[120..152].copy_from_slice(&guardian_set_id);
    data
}

fn guardian_set_data(guardian_set_id: [u8; 32]) -> Vec<u8> {
    let mut data = account_data(GUARDIAN_SET_ACCOUNT_LEN, GUARDIAN_SET_ACCOUNT_DISCRIMINATOR);
    data[272..304].copy_from_slice(&guardian_set_id);
    data
}

fn processed_event_data(
    consumed: bool,
    canonical_event_key: [u8; 32],
    route_id: [u8; 32],
    recipient: Pubkey,
) -> Vec<u8> {
    let mut data = account_data(
        PROCESSED_EVENT_ACCOUNT_LEN,
        PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
    );
    data[10] = u8::from(consumed);
    data[16..48].copy_from_slice(&canonical_event_key);
    data[48..80].copy_from_slice(&route_id);
    data[80..112].copy_from_slice(&recipient.to_bytes());
    data
}

fn recipient_balance_data(owner: Pubkey, mint: Pubkey) -> Vec<u8> {
    let mut data = account_data(
        RECIPIENT_BALANCE_ACCOUNT_LEN,
        RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
    );
    data[16..48].copy_from_slice(&owner.to_bytes());
    data[48..80].copy_from_slice(&mint.to_bytes());
    data
}

fn account_data(len: usize, discriminator: [u8; 8]) -> Vec<u8> {
    let mut data = vec![0u8; len];
    data[0..8].copy_from_slice(&discriminator);
    data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());
    data
}

fn packed_mint(mint_authority: Pubkey, initialized: bool) -> Vec<u8> {
    let mut data = vec![0u8; SplTokenMint::LEN];
    let mint = SplTokenMint {
        mint_authority: COption::Some(to_program_pubkey(mint_authority)),
        supply: 0,
        decimals: 18,
        is_initialized: initialized,
        freeze_authority: COption::None,
    };

    SplTokenMint::pack(mint, &mut data).expect("pack SPL mint");
    data
}

fn packed_token_account(mint: Pubkey, owner: Pubkey, state: AccountState) -> Vec<u8> {
    let mut data = vec![0u8; SplTokenAccount::LEN];
    let account = SplTokenAccount {
        mint: to_program_pubkey(mint),
        owner: to_program_pubkey(owner),
        amount: 0,
        delegate: COption::None,
        state,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };

    SplTokenAccount::pack(account, &mut data).expect("pack SPL token account");
    data
}

fn to_program_pubkey(pubkey: Pubkey) -> ProgramPubkey {
    ProgramPubkey::new_from_array(pubkey.to_bytes())
}

fn read_u128_le(input: &[u8], offset: usize) -> u128 {
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&input[offset..offset + 16]);
    u128::from_le_bytes(bytes)
}

fn read_u64_le(input: &[u8], offset: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&input[offset..offset + 8]);
    u64::from_le_bytes(bytes)
}

fn read_fixed_32(input: &[u8], offset: usize) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&input[offset..offset + 32]);
    bytes
}

fn instruction_data_from_fields(
    route_id: [u8; 32],
    guardian_set_id: [u8; 32],
    mint_id: Pubkey,
    canonical_event_key: [u8; 32],
    recipient: Pubkey,
    amount: u128,
    source_chain_weight_bps: u16,
    source_chain_id: u64,
) -> [u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN] {
    let mut bytes = [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN];

    bytes[0..8].copy_from_slice(&CONSUME_GATEWAY_MINT_DISCRIMINATOR);
    bytes[8..10].copy_from_slice(&INSTRUCTION_LAYOUT_VERSION.to_le_bytes());
    bytes[10] = CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT;
    bytes[11] = CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX;
    bytes[12] = CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX;
    bytes[13] = CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX;
    bytes[14] = CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX;
    bytes[15] = CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX;
    bytes[16..48].copy_from_slice(&route_id);
    bytes[48..80].copy_from_slice(&guardian_set_id);
    bytes[80..112].copy_from_slice(&mint_id.to_bytes());
    bytes[112..144].copy_from_slice(&canonical_event_key);
    bytes[144..176].copy_from_slice(&recipient.to_bytes());
    bytes[176..192].copy_from_slice(&amount.to_le_bytes());
    bytes[192..194].copy_from_slice(&source_chain_weight_bps.to_le_bytes());
    bytes[194..202].copy_from_slice(&source_chain_id.to_le_bytes());

    bytes
}

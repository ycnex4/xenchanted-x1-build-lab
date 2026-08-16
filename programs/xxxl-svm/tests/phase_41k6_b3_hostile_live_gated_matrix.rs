#![cfg(all(
    feature = "phase-41k5-d2-production-path-test-gate",
    feature = "dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build",
    feature = "phase-41k6-b1c7-handler-integration-test-gate",
    feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build"
))]

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    sync::Once,
};

use mollusk_svm::{program::loader_keys, result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program::sysvar::instructions::{BorrowedAccountMeta, BorrowedInstruction};
use solana_program::{
    ed25519_program, program_option::COption, program_pack::Pack, pubkey::Pubkey as ProgramPubkey,
    sysvar,
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
    processed_event_marking_boundary::build_final_consumed_processed_event_account_image,
    state::{
        GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, GATEWAY_CONFIG_ACCOUNT_LEN,
        GUARDIAN_SET_ACCOUNT_DISCRIMINATOR, GUARDIAN_SET_ACCOUNT_LEN,
        MINT_STATE_ACCOUNT_DISCRIMINATOR, MINT_STATE_ACCOUNT_LEN, PROCESSED_EVENT_ACCOUNT_LEN,
        RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR, RECIPIENT_BALANCE_ACCOUNT_LEN,
        RUNTIME_LAYOUT_VERSION,
    },
    verifier::{compute_b1c_expected_authorization_payload_hash, B1CAuthorizationPayloadContext},
};

const PROGRAM_NAME: &str = "xxxl_svm";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const NATIVE_LOADER_ID: &str = "NativeLoader1111111111111111111111111111111";

const ROUTE_ID: [u8; 32] = [0x11; 32];
const GUARDIAN_SET_ID: [u8; 32] = [0x22; 32];
const CANONICAL_EVENT_KEY: [u8; 32] = [0x44; 32];
const SOURCE_CHAIN_ID: u64 = 1;
const SOURCE_CHAIN_WEIGHT_BPS: u16 = 10_000;
const AMOUNT: u64 = 1_000;

const GUARDIAN_ONE: [u8; 32] = [0xA1; 32];
const GUARDIAN_TWO: [u8; 32] = [0xA2; 32];
const GUARDIAN_THREE: [u8; 32] = [0xA3; 32];
const UNKNOWN_GUARDIAN: [u8; 32] = [0xEE; 32];
const GUARDIAN_THRESHOLD: u16 = 2;
const GUARDIAN_COUNT: u8 = 3;

const ACCOUNT_INDEX_PROCESSED_EVENT: usize = 3;
const ACCOUNT_INDEX_RECIPIENT_BALANCE: usize = 4;
const ACCOUNT_INDEX_SPL_MINT: usize = 5;
const ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT: usize = 6;
const ACCOUNT_INDEX_RENT_PAYER: usize = 9;

const ED25519_SIGNATURE_LEN: usize = 64;
const ED25519_PUBLIC_KEY_LEN: usize = 32;
const ED25519_SINGLE_SIGNATURE_HEADER_LEN: usize = 16;
const ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL: u16 = u16::MAX;

static SBF_BUILD: Once = Once::new();

#[test]
fn phase_41k6_b3_wrong_payload_hash_evidence_rejects_before_mark_and_mint() {
    let fixture = B2LiveGatedSuccessFixture::new();
    let mut wrong_payload_hash = fixture.payload_hash();
    wrong_payload_hash[0] ^= 0xff;

    run_b3_hostile_case_before_mutation(
        fixture,
        vec![
            ed25519_precompile_instruction(wrong_payload_hash, GUARDIAN_ONE),
            ed25519_precompile_instruction(wrong_payload_hash, GUARDIAN_TWO),
        ],
    );
}

#[test]
fn phase_41k6_b3_unknown_guardian_evidence_rejects_before_mark_and_mint() {
    let fixture = B2LiveGatedSuccessFixture::new();
    let payload_hash = fixture.payload_hash();

    run_b3_hostile_case_before_mutation(
        fixture,
        vec![
            ed25519_precompile_instruction(payload_hash, GUARDIAN_ONE),
            ed25519_precompile_instruction(payload_hash, UNKNOWN_GUARDIAN),
        ],
    );
}

#[test]
fn phase_41k6_b3_guardian_set_id_binding_mismatch_rejects_before_mark_and_mint() {
    let mut fixture = B2LiveGatedSuccessFixture::new();
    let payload_hash_bound_to_original_guardian_set_id = fixture.payload_hash();
    let new_guardian_set_id = [0x99; 32];

    let (new_guardian_set, _) = Pubkey::find_program_address(
        &[b"xxxl", b"guardian-set", &new_guardian_set_id],
        &fixture.program_id,
    );

    fixture.keys.guardian_set = new_guardian_set;
    fixture.data.guardian_set = guardian_set_data(new_guardian_set_id);
    fixture.data.gateway_config = gateway_config_data(
        ROUTE_ID,
        SOURCE_CHAIN_ID,
        new_guardian_set_id,
        fixture.keys.spl_mint,
        SOURCE_CHAIN_WEIGHT_BPS,
    );
    fixture.instruction_data = instruction_data_from_fields(
        ROUTE_ID,
        new_guardian_set_id,
        fixture.keys.spl_mint,
        CANONICAL_EVENT_KEY,
        fixture.keys.recipient_owner,
        AMOUNT as u128,
        SOURCE_CHAIN_WEIGHT_BPS,
        SOURCE_CHAIN_ID,
    );

    run_b3_hostile_case_before_mutation(
        fixture,
        vec![
            ed25519_precompile_instruction(
                payload_hash_bound_to_original_guardian_set_id,
                GUARDIAN_ONE,
            ),
            ed25519_precompile_instruction(
                payload_hash_bound_to_original_guardian_set_id,
                GUARDIAN_TWO,
            ),
        ],
    );
}

#[test]
fn phase_41k6_b3_recipient_binding_mismatch_rejects_before_mark_and_mint() {
    let mut fixture = B2LiveGatedSuccessFixture::new();
    let payload_hash_bound_to_original_recipient = fixture.payload_hash();

    fixture.keys.recipient_token_account = Pubkey::new_unique();
    fixture.data.recipient_token_account = packed_token_account_with_amount(
        fixture.keys.spl_mint,
        fixture.keys.recipient_owner,
        AccountState::Initialized,
        0,
    );

    run_b3_hostile_case_before_mutation(
        fixture,
        vec![
            ed25519_precompile_instruction(payload_hash_bound_to_original_recipient, GUARDIAN_ONE),
            ed25519_precompile_instruction(payload_hash_bound_to_original_recipient, GUARDIAN_TWO),
        ],
    );
}

#[test]
fn phase_41k6_b3_mint_binding_mismatch_rejects_before_mark_and_mint() {
    let mut fixture = B2LiveGatedSuccessFixture::new();
    let payload_hash_bound_to_original_mint = fixture.payload_hash();

    let new_spl_mint = Pubkey::new_unique();
    fixture.keys.spl_mint = new_spl_mint;
    fixture.data.gateway_config = gateway_config_data(
        ROUTE_ID,
        SOURCE_CHAIN_ID,
        GUARDIAN_SET_ID,
        new_spl_mint,
        SOURCE_CHAIN_WEIGHT_BPS,
    );
    fixture.data.recipient_balance =
        recipient_balance_data(fixture.keys.recipient_owner, new_spl_mint);
    fixture.data.spl_mint = packed_mint_with_supply(fixture.keys.mint_authority_pda, true, 0);
    fixture.data.recipient_token_account = packed_token_account_with_amount(
        new_spl_mint,
        fixture.keys.recipient_owner,
        AccountState::Initialized,
        0,
    );
    fixture.instruction_data = instruction_data_from_fields(
        ROUTE_ID,
        GUARDIAN_SET_ID,
        new_spl_mint,
        CANONICAL_EVENT_KEY,
        fixture.keys.recipient_owner,
        AMOUNT as u128,
        SOURCE_CHAIN_WEIGHT_BPS,
        SOURCE_CHAIN_ID,
    );

    run_b3_hostile_case_before_mutation_with_expected_error(
        fixture,
        vec![
            ed25519_precompile_instruction(payload_hash_bound_to_original_mint, GUARDIAN_ONE),
            ed25519_precompile_instruction(payload_hash_bound_to_original_mint, GUARDIAN_TWO),
        ],
        ProgramError::Custom(XxxlError::InvalidPda as u32),
    );
}

#[test]
fn phase_41k6_b3_processed_event_replay_rejects_before_second_mark_and_mint() {
    let fixture = B2LiveGatedSuccessFixture::new();
    let payload_hash = fixture.payload_hash();

    let mut accounts = fixture.accounts_b1_v3();
    let consumed_processed_event_data = build_final_consumed_processed_event_account_image(
        &CANONICAL_EVENT_KEY,
        &ROUTE_ID,
        &fixture.keys.recipient_owner.to_bytes(),
        AMOUNT as u128,
        0,
    )
    .expect("consumed processed_event replay fixture");

    accounts[ACCOUNT_INDEX_PROCESSED_EVENT].1 = account(
        10_000_000_000,
        consumed_processed_event_data.to_vec(),
        fixture.program_id,
        false,
    );

    run_b3_hostile_case_with_accounts_before_mutation(
        fixture,
        vec![
            ed25519_precompile_instruction(payload_hash, GUARDIAN_ONE),
            ed25519_precompile_instruction(payload_hash, GUARDIAN_TWO),
        ],
        accounts,
        ProgramError::Custom(XxxlError::InvalidInstruction as u32),
    );
}

#[test]
fn phase_41k6_b3_duplicate_guardian_evidence_rejects_before_mark_and_mint() {
    let fixture = B2LiveGatedSuccessFixture::new();
    let payload_hash = fixture.payload_hash();

    run_b3_hostile_case_before_mutation(
        fixture,
        vec![
            ed25519_precompile_instruction(payload_hash, GUARDIAN_ONE),
            ed25519_precompile_instruction(payload_hash, GUARDIAN_ONE),
        ],
    );
}

#[test]
fn phase_41k6_b3_insufficient_quorum_rejects_before_mark_and_mint() {
    let fixture = B2LiveGatedSuccessFixture::new();
    let payload_hash = fixture.payload_hash();

    run_b3_hostile_case_before_mutation(
        fixture,
        vec![ed25519_precompile_instruction(payload_hash, GUARDIAN_ONE)],
    );
}

fn run_b3_hostile_case_before_mutation(
    fixture: B2LiveGatedSuccessFixture,
    prior_ed25519_instructions: Vec<Instruction>,
) {
    run_b3_hostile_case_before_mutation_with_expected_error(
        fixture,
        prior_ed25519_instructions,
        ProgramError::Custom(XxxlError::InvalidInstruction as u32),
    );
}

fn run_b3_hostile_case_before_mutation_with_expected_error(
    fixture: B2LiveGatedSuccessFixture,
    prior_ed25519_instructions: Vec<Instruction>,
    expected_error: ProgramError,
) {
    let accounts = fixture.accounts_b1_v3();

    run_b3_hostile_case_with_accounts_before_mutation(
        fixture,
        prior_ed25519_instructions,
        accounts,
        expected_error,
    );
}

fn run_b3_hostile_case_with_accounts_before_mutation(
    fixture: B2LiveGatedSuccessFixture,
    prior_ed25519_instructions: Vec<Instruction>,
    accounts: Vec<(Pubkey, Account)>,
    expected_error: ProgramError,
) {
    let mollusk = mollusk_for_program(&fixture.program_id);
    let current_instruction = fixture.instruction_b1_v3();

    let processed_event_before = &accounts[ACCOUNT_INDEX_PROCESSED_EVENT].1;
    let spl_mint_before = &accounts[ACCOUNT_INDEX_SPL_MINT].1;
    let recipient_token_account_before = &accounts[ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT].1;
    let recipient_balance_before = &accounts[ACCOUNT_INDEX_RECIPIENT_BALANCE].1;
    let rent_payer_before = &accounts[ACCOUNT_INDEX_RENT_PAYER].1;

    let mut transaction_instructions = prior_ed25519_instructions;
    transaction_instructions.push(current_instruction);

    mollusk.process_and_validate_transaction_instructions(
        &transaction_instructions,
        &accounts,
        &[
            Check::err(expected_error),
            Check::account(&fixture.keys.processed_event)
                .owner(&processed_event_before.owner)
                .space(processed_event_before.data.len())
                .data(&processed_event_before.data)
                .build(),
            Check::account(&fixture.keys.spl_mint)
                .owner(&spl_mint_before.owner)
                .space(spl_mint_before.data.len())
                .data(&spl_mint_before.data)
                .build(),
            Check::account(&fixture.keys.recipient_token_account)
                .owner(&recipient_token_account_before.owner)
                .space(recipient_token_account_before.data.len())
                .data(&recipient_token_account_before.data)
                .build(),
            Check::account(&fixture.keys.recipient_balance)
                .owner(&recipient_balance_before.owner)
                .space(recipient_balance_before.data.len())
                .data(&recipient_balance_before.data)
                .build(),
            Check::account(&fixture.keys.rent_payer)
                .owner(&rent_payer_before.owner)
                .space(rent_payer_before.data.len())
                .data(&rent_payer_before.data)
                .build(),
        ],
    );
}

struct B2LiveGatedSuccessFixture {
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
    instructions_sysvar: Pubkey,
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

impl B2LiveGatedSuccessFixture {
    fn new() -> Self {
        let program_id = Pubkey::new_unique();
        let token_program = TOKEN_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("SPL Token program id");
        let system_program = SYSTEM_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("system program id");
        let instructions_sysvar = Pubkey::new_from_array(sysvar::instructions::id().to_bytes());
        let spl_mint = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();

        let (mint_authority_pda, mint_authority_bump) =
            Pubkey::find_program_address(&[b"xxxl", b"gateway-mint-authority", b"v1"], &program_id);

        let (guardian_set, _) = Pubkey::find_program_address(
            &[b"xxxl", b"guardian-set", &GUARDIAN_SET_ID],
            &program_id,
        );

        let (processed_event, _) = Pubkey::find_program_address(
            &[b"xxxl", b"processed-event", &CANONICAL_EVENT_KEY],
            &program_id,
        );

        let canonical_asset_id = spl_mint.to_bytes();
        let (mint_state, _) = Pubkey::find_program_address(
            &[b"xxxl", b"mint-state", &canonical_asset_id],
            &program_id,
        );
        let (gateway_config, _) =
            Pubkey::find_program_address(&[b"xxxl", b"gateway-config", &ROUTE_ID], &program_id);
        let (recipient_balance, _) = Pubkey::find_program_address(
            &[
                b"xxxl",
                b"recipient-balance",
                &recipient_owner.to_bytes(),
                &spl_mint.to_bytes(),
            ],
            &program_id,
        );

        let keys = FixtureKeys {
            mint_state,
            gateway_config,
            guardian_set,
            processed_event,
            recipient_balance,
            recipient_owner,
            spl_mint,
            recipient_token_account: Pubkey::new_unique(),
            mint_authority_pda,
            token_program,
            rent_payer: Pubkey::new_unique(),
            system_program,
            instructions_sysvar,
        };

        let data = FixtureData {
            mint_state: mint_state_data(spl_mint, mint_authority_pda, mint_authority_bump),
            gateway_config: gateway_config_data(
                ROUTE_ID,
                SOURCE_CHAIN_ID,
                GUARDIAN_SET_ID,
                spl_mint,
                SOURCE_CHAIN_WEIGHT_BPS,
            ),
            guardian_set: guardian_set_data(GUARDIAN_SET_ID),
            processed_event: Vec::new(),
            recipient_balance: recipient_balance_data(recipient_owner, spl_mint),
            spl_mint: packed_mint_with_supply(mint_authority_pda, true, 0),
            recipient_token_account: packed_token_account_with_amount(
                spl_mint,
                recipient_owner,
                AccountState::Initialized,
                0,
            ),
        };

        let instruction_data = instruction_data_from_fields(
            ROUTE_ID,
            GUARDIAN_SET_ID,
            spl_mint,
            CANONICAL_EVENT_KEY,
            recipient_owner,
            AMOUNT as u128,
            SOURCE_CHAIN_WEIGHT_BPS,
            SOURCE_CHAIN_ID,
        );

        Self {
            program_id,
            keys,
            data,
            instruction_data,
        }
    }

    fn instruction_b1_v3(&self) -> Instruction {
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
                AccountMeta::new_readonly(self.keys.instructions_sysvar, false),
            ],
        )
    }

    fn accounts_b1_v3(&self) -> Vec<(Pubkey, Account)> {
        let native_loader = native_loader_id();
        let lamports = 10_000_000_000;

        let mut accounts = vec![
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
                account(lamports, Vec::new(), loader_keys::LOADER_V3, true),
            ),
            (
                self.keys.rent_payer,
                account(lamports, Vec::new(), self.keys.system_program, false),
            ),
            (
                self.keys.system_program,
                account(1, Vec::new(), native_loader, true),
            ),
        ];

        accounts
    }

    fn payload_hash(&self) -> [u8; 32] {
        compute_b1c_expected_authorization_payload_hash(&B1CAuthorizationPayloadContext {
            processed_event: ProgramPubkey::new_from_array(self.keys.processed_event.to_bytes()),
            route_id: ROUTE_ID,
            mint: ProgramPubkey::new_from_array(self.keys.spl_mint.to_bytes()),
            recipient: ProgramPubkey::new_from_array(self.keys.recipient_token_account.to_bytes()),
            amount: AMOUNT,
            guardian_set_id: GUARDIAN_SET_ID,
        })
    }
}

fn instructions_sysvar_data(current_instruction: &Instruction, payload_hash: [u8; 32]) -> Vec<u8> {
    let guardian_one_ix = ed25519_precompile_instruction(payload_hash, GUARDIAN_ONE);
    let guardian_two_ix = ed25519_precompile_instruction(payload_hash, GUARDIAN_TWO);

    let guardian_one_program_id = to_program_pubkey(guardian_one_ix.program_id);
    let guardian_two_program_id = to_program_pubkey(guardian_two_ix.program_id);
    let current_program_id = to_program_pubkey(current_instruction.program_id);

    let current_account_pubkeys: Vec<ProgramPubkey> = current_instruction
        .accounts
        .iter()
        .map(|account| to_program_pubkey(account.pubkey))
        .collect();

    let current_accounts: Vec<BorrowedAccountMeta<'_>> = current_instruction
        .accounts
        .iter()
        .zip(current_account_pubkeys.iter())
        .map(|(account, pubkey)| BorrowedAccountMeta {
            pubkey,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();

    let borrowed_instructions = [
        BorrowedInstruction {
            program_id: &guardian_one_program_id,
            accounts: Vec::new(),
            data: &guardian_one_ix.data,
        },
        BorrowedInstruction {
            program_id: &guardian_two_program_id,
            accounts: Vec::new(),
            data: &guardian_two_ix.data,
        },
        BorrowedInstruction {
            program_id: &current_program_id,
            accounts: current_accounts,
            data: &current_instruction.data,
        },
    ];

    let mut data = sysvar::instructions::construct_instructions_data(&borrowed_instructions);
    sysvar::instructions::store_current_index(&mut data, 2);

    data
}

fn ed25519_precompile_instruction(message: [u8; 32], signer: [u8; 32]) -> Instruction {
    Instruction {
        program_id: Pubkey::new_from_array(ed25519_program::id().to_bytes()),
        accounts: Vec::new(),
        data: ed25519_instruction_data(&message, signer),
    }
}

fn ed25519_instruction_data(message: &[u8; 32], signer: [u8; 32]) -> Vec<u8> {
    let signature_offset = ED25519_SINGLE_SIGNATURE_HEADER_LEN as u16;
    let public_key_offset = signature_offset + ED25519_SIGNATURE_LEN as u16;
    let message_data_offset = public_key_offset + ED25519_PUBLIC_KEY_LEN as u16;
    let message_data_size = message.len() as u16;

    let mut data = vec![
        1, 0, 0, 0, 0xff, 0xff, 0, 0, 0xff, 0xff, 0, 0, 0, 0, 0xff, 0xff,
    ];

    data[2..4].copy_from_slice(&signature_offset.to_le_bytes());
    data[4..6].copy_from_slice(&ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL.to_le_bytes());
    data[6..8].copy_from_slice(&public_key_offset.to_le_bytes());
    data[8..10].copy_from_slice(&ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL.to_le_bytes());
    data[10..12].copy_from_slice(&message_data_offset.to_le_bytes());
    data[12..14].copy_from_slice(&message_data_size.to_le_bytes());
    data[14..16].copy_from_slice(&ED25519_CURRENT_INSTRUCTION_INDEX_SENTINEL.to_le_bytes());

    data.extend_from_slice(&[0x55; ED25519_SIGNATURE_LEN]);
    data.extend_from_slice(&signer);
    data.extend_from_slice(message);

    data
}

fn mollusk_for_program(program_id: &Pubkey) -> Mollusk {
    ensure_sbf_b2_built();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sbf_out_dir = manifest_dir.join("target/sbpf-solana-solana/release");
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let mut mollusk = Mollusk::new(program_id, PROGRAM_NAME);
    let token_program = TOKEN_PROGRAM_ID
        .parse::<Pubkey>()
        .expect("SPL Token program id");
    let spl_token_elf =
        find_litesvm_spl_token_elf().expect("LiteSVM SPL Token SBF ELF for B3 Mollusk CPI test");
    eprintln!("B3 using SPL Token ELF: {}", spl_token_elf.display());
    let elf = fs::read(&spl_token_elf).expect("read SPL Token SBF ELF");

    mollusk.add_program_with_loader_and_elf(&token_program, &loader_keys::LOADER_V3, &elf);

    let ed25519_program_id = Pubkey::new_from_array(ed25519_program::id().to_bytes());
    let ed25519_noop_elf =
        find_noop_aligned_sbf_elf().expect("noop SBF ELF for B3 prior Ed25519 harness stub");
    eprintln!(
        "B3 using Ed25519 noop stub ELF: {}",
        ed25519_noop_elf.display()
    );
    let ed25519_noop = fs::read(&ed25519_noop_elf).expect("read noop SBF ELF");
    mollusk.add_program_with_loader_and_elf(
        &ed25519_program_id,
        &loader_keys::LOADER_V3,
        &ed25519_noop,
    );

    mollusk
}

fn ensure_sbf_b2_built() {
    SBF_BUILD.call_once(|| {
        let status = Command::new("cargo")
            .args([
                "build-sbf",
                "--features",
                "phase-41k5-d2-production-path-test-gate,dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build,phase-41k6-b1c7-handler-integration-test-gate,dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("spawn cargo build-sbf for phase 41K.6 B2 valid quorum live-gated success");

        assert!(
            status.success(),
            "cargo build-sbf B3 hostile live-gated matrix build failed"
        );
    });
}

fn find_litesvm_spl_token_elf() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let registry_src = PathBuf::from(home.clone()).join(".cargo/registry/src");

    fs::read_dir(registry_src)
        .ok()?
        .filter_map(Result::ok)
        .find_map(|entry| {
            let candidate = entry
                .path()
                .join("litesvm-0.10.0/src/programs/elf/spl_token-3.5.0.so");
            candidate.exists().then_some(candidate)
        })
}

fn find_noop_aligned_sbf_elf() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let registry_src = PathBuf::from(home.clone()).join(".cargo/registry/src");

    fs::read_dir(registry_src)
        .ok()?
        .filter_map(Result::ok)
        .find_map(|entry| {
            let candidate = entry
                .path()
                .join("solana-bpf-loader-program-4.0.0/test_elfs/out/noop_aligned.so");
            candidate.exists().then_some(candidate)
        })
        .or_else(|| {
            fs::read_dir(PathBuf::from(home.clone()).join(".cargo/registry/src"))
                .ok()?
                .filter_map(Result::ok)
                .find_map(|entry| {
                    let candidate = entry
                        .path()
                        .join("solana-bpf-loader-program-3.1.14/test_elfs/out/noop_aligned.so");
                    candidate.exists().then_some(candidate)
                })
        })
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
    data[10] = 1;
    data[12..14].copy_from_slice(&GUARDIAN_THRESHOLD.to_le_bytes());
    data[14] = GUARDIAN_COUNT;
    data[16..48].copy_from_slice(&GUARDIAN_ONE);
    data[48..80].copy_from_slice(&GUARDIAN_TWO);
    data[80..112].copy_from_slice(&GUARDIAN_THREE);
    data[272..304].copy_from_slice(&guardian_set_id);
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

fn packed_mint_with_supply(mint_authority: Pubkey, initialized: bool, supply: u64) -> Vec<u8> {
    let mut data = vec![0u8; SplTokenMint::LEN];
    let mint = SplTokenMint {
        mint_authority: COption::Some(to_program_pubkey(mint_authority)),
        supply,
        decimals: 18,
        is_initialized: initialized,
        freeze_authority: COption::None,
    };

    SplTokenMint::pack(mint, &mut data).expect("pack SPL mint");
    data
}

fn packed_token_account_with_amount(
    mint: Pubkey,
    owner: Pubkey,
    state: AccountState,
    amount: u64,
) -> Vec<u8> {
    let mut data = vec![0u8; SplTokenAccount::LEN];
    let account = SplTokenAccount {
        mint: to_program_pubkey(mint),
        owner: to_program_pubkey(owner),
        amount,
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

fn instruction_data_from_fields(
    route_id: [u8; 32],
    guardian_set_id: [u8; 32],
    canonical_asset_id: Pubkey,
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
    bytes[80..112].copy_from_slice(&canonical_asset_id.to_bytes());
    bytes[112..144].copy_from_slice(&canonical_event_key);
    bytes[144..176].copy_from_slice(&recipient.to_bytes());
    bytes[176..192].copy_from_slice(&amount.to_le_bytes());
    bytes[192..194].copy_from_slice(&source_chain_weight_bps.to_le_bytes());
    bytes[194..202].copy_from_slice(&source_chain_id.to_le_bytes());
    bytes
}

fn native_loader_id() -> Pubkey {
    Pubkey::from_str(NATIVE_LOADER_ID).expect("native loader id")
}

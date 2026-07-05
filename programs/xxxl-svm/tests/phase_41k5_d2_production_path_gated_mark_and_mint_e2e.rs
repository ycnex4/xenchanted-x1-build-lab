#![cfg(all(
    feature = "phase-41k5-d2-production-path-test-gate",
    feature = "dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build"
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
    processed_event_marking_boundary::build_final_consumed_processed_event_account_image,
    state::{
        GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, GATEWAY_CONFIG_ACCOUNT_LEN,
        GUARDIAN_SET_ACCOUNT_DISCRIMINATOR, GUARDIAN_SET_ACCOUNT_LEN,
        MINT_STATE_ACCOUNT_DISCRIMINATOR, MINT_STATE_ACCOUNT_LEN, PROCESSED_EVENT_ACCOUNT_LEN,
        RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR, RECIPIENT_BALANCE_ACCOUNT_LEN,
        RUNTIME_LAYOUT_VERSION,
    },
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

const ACCOUNT_INDEX_MINT_STATE: usize = 0;
const ACCOUNT_INDEX_GATEWAY_CONFIG: usize = 1;
const ACCOUNT_INDEX_GUARDIAN_SET: usize = 2;
const ACCOUNT_INDEX_PROCESSED_EVENT: usize = 3;
const ACCOUNT_INDEX_RECIPIENT_BALANCE: usize = 4;
const ACCOUNT_INDEX_SPL_MINT: usize = 5;
const ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT: usize = 6;
const ACCOUNT_INDEX_MINT_AUTHORITY_PDA: usize = 7;
const ACCOUNT_INDEX_TOKEN_PROGRAM: usize = 8;
const ACCOUNT_INDEX_RENT_PAYER: usize = 9;
const ACCOUNT_INDEX_SYSTEM_PROGRAM: usize = 10;

static SBF_BUILD: Once = Once::new();

#[test]
fn phase_41k5_d2_production_path_without_b1c7_evidence_rejects_before_mutation() {
    let fixture = ProductionPathFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = fixture.accounts();

    let processed_event_before = &accounts[ACCOUNT_INDEX_PROCESSED_EVENT].1;
    let rent_payer_before = &accounts[ACCOUNT_INDEX_RENT_PAYER].1;
    let spl_mint_before = &accounts[ACCOUNT_INDEX_SPL_MINT].1;
    let recipient_token_account_before = &accounts[ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT].1;
    let recipient_balance_before = &accounts[ACCOUNT_INDEX_RECIPIENT_BALANCE].1;

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::err(ProgramError::Custom(XxxlError::InvalidInstruction as u32)),
            Check::account(&fixture.keys.processed_event)
                .lamports(processed_event_before.lamports)
                .owner(&processed_event_before.owner)
                .space(processed_event_before.data.len())
                .data(&processed_event_before.data)
                .build(),
            Check::account(&fixture.keys.rent_payer)
                .lamports(rent_payer_before.lamports)
                .owner(&rent_payer_before.owner)
                .space(rent_payer_before.data.len())
                .data(&rent_payer_before.data)
                .build(),
            Check::account(&fixture.keys.spl_mint)
                .lamports(spl_mint_before.lamports)
                .owner(&spl_mint_before.owner)
                .space(spl_mint_before.data.len())
                .data(&spl_mint_before.data)
                .build(),
            Check::account(&fixture.keys.recipient_token_account)
                .lamports(recipient_token_account_before.lamports)
                .owner(&recipient_token_account_before.owner)
                .space(recipient_token_account_before.data.len())
                .data(&recipient_token_account_before.data)
                .build(),
            Check::account(&fixture.keys.recipient_balance)
                .lamports(recipient_balance_before.lamports)
                .owner(&recipient_balance_before.owner)
                .space(recipient_balance_before.data.len())
                .data(&recipient_balance_before.data)
                .build(),
        ],
    );
}

struct ProductionPathFixture {
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
    mint_authority_bump: u8,
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

impl ProductionPathFixture {
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
        let (mint_authority_pda, mint_authority_bump) =
            Pubkey::find_program_address(&[b"xxxl", b"gateway-mint-authority", b"v1"], &program_id);
        let (processed_event, _) = Pubkey::find_program_address(
            &[b"xxxl", b"processed-event", &CANONICAL_EVENT_KEY],
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
            mint_authority_bump,
            token_program,
            rent_payer: Pubkey::new_unique(),
            system_program,
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
            spl_mint: packed_mint(mint_authority_pda, true),
            recipient_token_account: packed_token_account(
                spl_mint,
                recipient_owner,
                AccountState::Initialized,
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
        let native_loader = native_loader_id();
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
        ]
    }
}

fn mollusk_for_program(program_id: &Pubkey) -> Mollusk {
    ensure_sbf_d2_built();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sbf_out_dir = manifest_dir.join("target/sbpf-solana-solana/release");
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let mut mollusk = Mollusk::new(program_id, PROGRAM_NAME);
    let token_program = TOKEN_PROGRAM_ID
        .parse::<Pubkey>()
        .expect("SPL Token program id");
    let spl_token_elf =
        find_litesvm_spl_token_elf().expect("LiteSVM SPL Token SBF ELF for D2 Mollusk CPI test");
    eprintln!("D2 using SPL Token ELF: {}", spl_token_elf.display());
    let elf = fs::read(&spl_token_elf).expect("read SPL Token SBF ELF");

    mollusk.add_program_with_loader_and_elf(&token_program, &loader_keys::LOADER_V3, &elf);

    mollusk
}

fn ensure_sbf_d2_built() {
    SBF_BUILD.call_once(|| {
        let status = Command::new("cargo")
            .args([
                "build-sbf",
                "--features",
                "phase-41k5-d2-production-path-test-gate,dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("spawn cargo build-sbf for phase 41K.5 D2 production-path e2e");

        assert!(status.success(), "cargo build-sbf D2 production-path build failed");
    });
}

fn find_litesvm_spl_token_elf() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let registry_src = PathBuf::from(home).join(".cargo/registry/src");

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

fn native_loader_id() -> Pubkey {
    Pubkey::from_str(NATIVE_LOADER_ID).expect("native loader id")
}

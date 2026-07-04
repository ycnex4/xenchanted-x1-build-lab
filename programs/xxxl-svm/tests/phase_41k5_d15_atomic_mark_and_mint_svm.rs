#![cfg(all(
    feature = "phase-41k5-spl-mint-to-cpi-test-gate",
    feature = "dangerously-allow-phase-41k5-spl-mint-to-cpi-test-gate-sbf-build"
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
use solana_pubkey::Pubkey;
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplTokenMint};
use xxxl_svm::{
    phase_41k5_d15_atomic_mark_and_mint_svm_harness::PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_MAGIC,
    processed_event_marking_boundary::build_final_consumed_processed_event_account_image,
    state::PROCESSED_EVENT_ACCOUNT_LEN,
};

const PROGRAM_NAME: &str = "xxxl_svm";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";
const NATIVE_LOADER_ID: &str = "NativeLoader1111111111111111111111111111111";

const CANONICAL_EVENT_KEY: [u8; 32] = [0x44; 32];
const ROUTE_ID: [u8; 32] = [0x11; 32];
const AMOUNT: u64 = 1_000;

const ACCOUNT_INDEX_PROCESSED_EVENT: usize = 0;
const ACCOUNT_INDEX_SPL_MINT: usize = 1;
const ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT: usize = 2;
const ACCOUNT_INDEX_RENT_PAYER: usize = 5;

static SBF_BUILD: Once = Once::new();

#[test]
fn phase_41k5_d15_harness_marks_processed_event_and_mints_spl_tokens() {
    let fixture = HarnessFixture::new();
    let mollusk = mollusk_for_program(&fixture.program_id);

    let instruction = fixture.instruction();
    let accounts = fixture.accounts();

    let starting_processed_lamports = accounts[ACCOUNT_INDEX_PROCESSED_EVENT].1.lamports;
    let starting_rent_payer_lamports = accounts[ACCOUNT_INDEX_RENT_PAYER].1.lamports;
    let rent_min = mollusk
        .sysvars
        .rent
        .minimum_balance(PROCESSED_EVENT_ACCOUNT_LEN);
    let expected_top_up = rent_min.saturating_sub(starting_processed_lamports);
    let consumed_slot = mollusk.sysvars.clock.slot;

    let expected_processed_event = build_final_consumed_processed_event_account_image(
        &CANONICAL_EVENT_KEY,
        &ROUTE_ID,
        &fixture.recipient_owner.to_bytes(),
        AMOUNT as u128,
        consumed_slot,
    )
    .expect("expected processed event final image");

    let mut expected_mint =
        SplTokenMint::unpack(&accounts[ACCOUNT_INDEX_SPL_MINT].1.data).expect("valid SPL mint");
    expected_mint.supply = expected_mint
        .supply
        .checked_add(AMOUNT)
        .expect("mint supply add");
    let mut expected_mint_data = vec![0u8; SplTokenMint::LEN];
    SplTokenMint::pack(expected_mint, &mut expected_mint_data).expect("pack expected SPL mint");

    let mut expected_recipient_token_account =
        SplTokenAccount::unpack(&accounts[ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT].1.data)
            .expect("valid recipient token account");
    expected_recipient_token_account.amount = expected_recipient_token_account
        .amount
        .checked_add(AMOUNT)
        .expect("recipient token amount add");
    let mut expected_recipient_token_data = vec![0u8; SplTokenAccount::LEN];
    SplTokenAccount::pack(
        expected_recipient_token_account,
        &mut expected_recipient_token_data,
    )
    .expect("pack expected recipient token account");

    mollusk.process_and_validate_instruction(
        &instruction,
        &accounts,
        &[
            Check::success(),
            Check::account(&fixture.processed_event)
                .lamports(starting_processed_lamports + expected_top_up)
                .owner(&fixture.program_id)
                .space(PROCESSED_EVENT_ACCOUNT_LEN)
                .data(expected_processed_event.as_ref())
                .build(),
            Check::account(&fixture.rent_payer)
                .lamports(starting_rent_payer_lamports - expected_top_up)
                .owner(&fixture.system_program)
                .space(0)
                .data(&[])
                .build(),
            Check::account(&fixture.spl_mint)
                .data(&expected_mint_data)
                .lamports(accounts[ACCOUNT_INDEX_SPL_MINT].1.lamports)
                .owner(&fixture.token_program)
                .build(),
            Check::account(&fixture.recipient_token_account)
                .data(&expected_recipient_token_data)
                .lamports(accounts[ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT].1.lamports)
                .owner(&fixture.token_program)
                .build(),
        ],
    );
}

struct HarnessFixture {
    program_id: Pubkey,
    processed_event: Pubkey,
    spl_mint: Pubkey,
    recipient_owner: Pubkey,
    recipient_token_account: Pubkey,
    mint_authority_pda: Pubkey,
    mint_authority_bump: u8,
    token_program: Pubkey,
    rent_payer: Pubkey,
    system_program: Pubkey,
}

impl HarnessFixture {
    fn new() -> Self {
        let program_id = Pubkey::new_unique();
        let (processed_event, _) = Pubkey::find_program_address(
            &[b"xxxl", b"processed-event", &CANONICAL_EVENT_KEY],
            &program_id,
        );
        let spl_mint = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let recipient_token_account = Pubkey::new_unique();
        let (mint_authority_pda, mint_authority_bump) =
            Pubkey::find_program_address(&[b"xxxl", b"gateway-mint-authority", b"v1"], &program_id);
        let token_program = TOKEN_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("SPL Token program id");
        let system_program = SYSTEM_PROGRAM_ID
            .parse::<Pubkey>()
            .expect("system program id");

        Self {
            program_id,
            processed_event,
            spl_mint,
            recipient_owner,
            recipient_token_account,
            mint_authority_pda,
            mint_authority_bump,
            token_program,
            rent_payer: Pubkey::new_unique(),
            system_program,
        }
    }

    fn instruction(&self) -> Instruction {
        Instruction::new_with_bytes(
            self.program_id,
            &harness_instruction_data(self.recipient_owner, AMOUNT, self.mint_authority_bump),
            vec![
                AccountMeta::new(self.processed_event, false),
                AccountMeta::new(self.spl_mint, false),
                AccountMeta::new(self.recipient_token_account, false),
                AccountMeta::new_readonly(self.mint_authority_pda, false),
                AccountMeta::new_readonly(self.token_program, false),
                AccountMeta::new(self.rent_payer, true),
                AccountMeta::new_readonly(self.system_program, false),
            ],
        )
    }

    fn accounts(&self) -> Vec<(Pubkey, Account)> {
        let lamports = 10_000_000_000;

        vec![
            (
                self.processed_event,
                account(1, Vec::new(), self.system_program, false),
            ),
            (
                self.spl_mint,
                account(
                    lamports,
                    packed_mint(self.mint_authority_pda, true),
                    self.token_program,
                    false,
                ),
            ),
            (
                self.recipient_token_account,
                account(
                    lamports,
                    packed_token_account(
                        self.spl_mint,
                        self.recipient_owner,
                        AccountState::Initialized,
                    ),
                    self.token_program,
                    false,
                ),
            ),
            (
                self.mint_authority_pda,
                account(lamports, Vec::new(), self.program_id, false),
            ),
            (
                self.token_program,
                account(lamports, Vec::new(), loader_keys::LOADER_V3, true),
            ),
            (
                self.rent_payer,
                account(lamports, Vec::new(), self.system_program, false),
            ),
            (
                self.system_program,
                account(1, Vec::new(), native_loader_id(), true),
            ),
        ]
    }
}

fn mollusk_for_program(program_id: &Pubkey) -> Mollusk {
    ensure_sbf_harness_built();

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sbf_out_dir = manifest_dir.join("target/sbpf-solana-solana/release");
    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let mut mollusk = Mollusk::new(program_id, PROGRAM_NAME);
    let token_program = TOKEN_PROGRAM_ID
        .parse::<Pubkey>()
        .expect("SPL Token program id");
    let spl_token_elf = find_spl_token_elf_for_mollusk(&manifest_dir)
        .expect("SPL Token SBF ELF for Mollusk CPI test");
    eprintln!("D1.5 using SPL Token ELF: {}", spl_token_elf.display());
    let elf = fs::read(&spl_token_elf).expect("read SPL Token SBF ELF");

    mollusk.add_program_with_loader_and_elf(&token_program, &loader_keys::LOADER_V3, &elf);

    mollusk
}

fn ensure_sbf_harness_built() {
    SBF_BUILD.call_once(|| {
        let status = Command::new("cargo")
            .args([
                "build-sbf",
                "--features",
                "phase-41k5-spl-mint-to-cpi-test-gate,dangerously-allow-phase-41k5-spl-mint-to-cpi-test-gate-sbf-build",
            ])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("spawn cargo build-sbf for phase 41K.5 D1.5 harness");

        assert!(status.success(), "cargo build-sbf D1.5 harness build failed");
    });
}

fn find_spl_token_elf_for_mollusk(_manifest_dir: &Path) -> Option<PathBuf> {
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

fn harness_instruction_data(recipient: Pubkey, amount: u64, mint_authority_bump: u8) -> Vec<u8> {
    let mut data = Vec::with_capacity(
        PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_MAGIC.len() + 32 + 32 + 32 + 8 + 1,
    );
    data.extend_from_slice(PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_MAGIC);
    data.extend_from_slice(&CANONICAL_EVENT_KEY);
    data.extend_from_slice(&ROUTE_ID);
    data.extend_from_slice(&recipient.to_bytes());
    data.extend_from_slice(&amount.to_le_bytes());
    data.push(mint_authority_bump);
    data
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

fn native_loader_id() -> Pubkey {
    Pubkey::from_str(NATIVE_LOADER_ID).expect("native loader id")
}

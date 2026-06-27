use std::path::PathBuf;

use mollusk_svm::{result::Check, Mollusk};
use solana_account::Account;
use solana_instruction::{AccountMeta, Instruction};
use solana_program::{
    program_option::COption, program_pack::Pack, pubkey::Pubkey as ProgramPubkey,
};
use solana_pubkey::Pubkey;
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplTokenMint};
use xxxl_svm::{
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

#[test]
#[ignore = "requires cargo build-sbf and target/deploy/xxxl_svm.so"]
fn valid_consume_gateway_mint_scaffold_succeeds_without_state_mutation() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sbf_out_dir = manifest_dir.join("target/deploy");
    let program_elf = sbf_out_dir.join(format!("{PROGRAM_NAME}.so"));

    assert!(
        program_elf.exists(),
        "missing {}; run `cargo build-sbf` before this ignored Mollusk test",
        program_elf.display()
    );

    std::env::set_var("SBF_OUT_DIR", &sbf_out_dir);

    let fixture = ScaffoldFixture::new();
    let mollusk = Mollusk::new(&fixture.program_id, PROGRAM_NAME);

    let instruction = fixture.instruction();
    let accounts = fixture.accounts();

    let checks = vec![
        Check::success(),
        Check::account(&fixture.keys.processed_event)
            .data(&fixture.data.processed_event)
            .build(),
        Check::account(&fixture.keys.recipient_balance)
            .data(&fixture.data.recipient_balance)
            .build(),
        Check::account(&fixture.keys.spl_mint)
            .data(&fixture.data.spl_mint)
            .build(),
        Check::account(&fixture.keys.recipient_token_account)
            .data(&fixture.data.recipient_token_account)
            .build(),
    ];

    mollusk.process_and_validate_instruction(&instruction, &accounts, &checks);
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
    spl_mint: Pubkey,
    recipient_token_account: Pubkey,
    mint_authority_pda: Pubkey,
    token_program: Pubkey,
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
        let spl_mint = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let (mint_authority_pda, bump) =
            Pubkey::find_program_address(&[b"xxxl", b"gateway-mint-authority", b"v1"], &program_id);

        let route_id = [0x11; 32];
        let guardian_set_id = [0x22; 32];
        let canonical_event_key = [0x44; 32];

        let keys = FixtureKeys {
            mint_state: Pubkey::new_unique(),
            gateway_config: Pubkey::new_unique(),
            guardian_set: Pubkey::new_unique(),
            processed_event: Pubkey::new_unique(),
            recipient_balance: Pubkey::new_unique(),
            spl_mint,
            recipient_token_account: Pubkey::new_unique(),
            mint_authority_pda,
            token_program,
        };

        let data = FixtureData {
            mint_state: mint_state_data(spl_mint, mint_authority_pda, bump),
            gateway_config: gateway_config_data(route_id, guardian_set_id, spl_mint, 10_000),
            guardian_set: guardian_set_data(guardian_set_id),
            processed_event: processed_event_data(
                false,
                canonical_event_key,
                route_id,
                recipient_owner,
            ),
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
                AccountMeta::new(self.keys.mint_state, false),
                AccountMeta::new(self.keys.gateway_config, false),
                AccountMeta::new_readonly(self.keys.guardian_set, false),
                AccountMeta::new(self.keys.processed_event, false),
                AccountMeta::new(self.keys.recipient_balance, false),
                AccountMeta::new(self.keys.spl_mint, false),
                AccountMeta::new(self.keys.recipient_token_account, false),
                AccountMeta::new_readonly(self.keys.mint_authority_pda, false),
                AccountMeta::new_readonly(self.keys.token_program, false),
            ],
        )
    }

    fn accounts(&self) -> Vec<(Pubkey, Account)> {
        let token_program_owner = Pubkey::new_unique();
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
                    lamports,
                    self.data.processed_event.clone(),
                    self.program_id,
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

fn instruction_data_from_fields(
    route_id: [u8; 32],
    guardian_set_id: [u8; 32],
    mint_id: Pubkey,
    canonical_event_key: [u8; 32],
    recipient: Pubkey,
    amount: u128,
    source_chain_weight_bps: u16,
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

    bytes
}

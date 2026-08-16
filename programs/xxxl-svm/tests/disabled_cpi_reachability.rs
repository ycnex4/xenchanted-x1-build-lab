use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, program_option::COption,
    program_pack::Pack, pubkey::Pubkey, rent::Rent, system_program,
};
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplTokenMint};

use xxxl_svm::{
    cpi::{
        guarded_mint_to_cpi_execution_gate_boundary, spl_mint_to_cpi_execution_enabled,
        MintToCpiAccounts, MintToCpiBoundary, MintToCpiPlanningBoundary,
    },
    error::XxxlError,
    execution_plan::{
        AtomicConsumeGatewayMintExecutionPlan, ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER,
    },
    instruction::{ConsumeGatewayMintArgs, CONSUME_GATEWAY_MINT_INSTRUCTION_LEN},
    pda::{find_gateway_mint_authority, find_mint_state},
    processor::build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary,
    state::{
        GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, GATEWAY_CONFIG_ACCOUNT_LEN,
        GUARDIAN_SET_ACCOUNT_DISCRIMINATOR, GUARDIAN_SET_ACCOUNT_LEN,
        MINT_STATE_ACCOUNT_DISCRIMINATOR, MINT_STATE_ACCOUNT_LEN,
        PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR, PROCESSED_EVENT_ACCOUNT_LEN,
        RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR, RECIPIENT_BALANCE_ACCOUNT_LEN,
        RUNTIME_LAYOUT_VERSION,
    },
};

#[test]
fn spl_mint_to_cpi_execution_gate_is_disabled() {
    assert!(!spl_mint_to_cpi_execution_enabled());
}

#[test]
fn guarded_mint_to_cpi_boundary_rejects_before_invoke_signed_when_global_gate_disabled() {
    with_valid_disabled_cpi_fixture(|program_id, execution_plan, planning_boundary, boundary| {
        let result = guarded_mint_to_cpi_execution_gate_boundary(
            program_id,
            execution_plan,
            planning_boundary,
            boundary,
        );

        assert_eq!(
            result,
            Err(ProgramError::Custom(XxxlError::CpiBoundaryNotReady as u32))
        );
    });
}

#[test]
fn guarded_mint_to_cpi_boundary_rejects_when_execution_plan_live_route_flag_set() {
    with_valid_disabled_cpi_fixture(|program_id, execution_plan, planning_boundary, boundary| {
        let mut live_execution_plan = *execution_plan;
        live_execution_plan.live_route_activation_enabled = true;

        let result = guarded_mint_to_cpi_execution_gate_boundary(
            program_id,
            &live_execution_plan,
            planning_boundary,
            boundary,
        );

        assert_eq!(
            result,
            Err(ProgramError::Custom(XxxlError::CpiBoundaryNotReady as u32))
        );
    });
}

#[test]
fn guarded_mint_to_cpi_boundary_rejects_invoke_signed_planning_flag() {
    with_valid_disabled_cpi_fixture(|program_id, execution_plan, planning_boundary, boundary| {
        let mut invoke_signed_planning_boundary = *planning_boundary;
        invoke_signed_planning_boundary.invoke_signed_from_process_instruction_enabled = true;

        let result = guarded_mint_to_cpi_execution_gate_boundary(
            program_id,
            execution_plan,
            &invoke_signed_planning_boundary,
            boundary,
        );

        assert_eq!(
            result,
            Err(ProgramError::Custom(XxxlError::CpiBoundaryNotReady as u32))
        );
    });
}

#[test]
fn guarded_mint_to_cpi_boundary_rejects_planning_boundary_mismatch_after_expected_plan() {
    with_valid_disabled_cpi_fixture(|program_id, execution_plan, planning_boundary, boundary| {
        let mut mismatched_planning_boundary = *planning_boundary;
        mismatched_planning_boundary.amount += 1;

        let result = guarded_mint_to_cpi_execution_gate_boundary(
            program_id,
            execution_plan,
            &mismatched_planning_boundary,
            boundary,
        );

        assert_eq!(
            result,
            Err(ProgramError::Custom(XxxlError::InvalidInstruction as u32))
        );
    });
}

// The pre-41K.4 direct local mutation boundary is now test-only inside the
// library crate. Integration tests must not import it as public API; this keeps
// the legacy ProgramOwned + consumed=false mutation path out of non-test
// surfaces and forces future live wiring to use the audited 41K.4 marking
// boundary instead.
#[test]
fn disabled_spl_cpi_gate_rejects_before_live_atomicity_mutations() {
    let mut fixture = RuntimeFixture::new();
    let processed_event_before = fixture.data.processed_event.clone();
    let recipient_balance_before = fixture.data.recipient_balance.clone();
    let spl_mint_before = fixture.data.spl_mint.clone();
    let recipient_token_account_before = fixture.data.recipient_token_account.clone();

    let program_id = fixture.program_id;
    let args = fixture.args;
    let rent = Rent::default();
    let accounts = fixture.accounts();

    let result = build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary(
        &program_id,
        &accounts,
        &args,
        &rent,
        1920,
    );

    assert_eq!(
        result,
        Err(ProgramError::Custom(XxxlError::CpiBoundaryNotReady as u32))
    );

    drop(accounts);

    assert_eq!(fixture.data.processed_event, processed_event_before);
    assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    assert_eq!(fixture.data.spl_mint, spl_mint_before);
    assert_eq!(
        fixture.data.recipient_token_account,
        recipient_token_account_before
    );
    assert_spl_supply_and_recipient_amount(&fixture, 0, 0);
}

fn with_valid_disabled_cpi_fixture<T>(
    f: impl FnOnce(
        &Pubkey,
        &AtomicConsumeGatewayMintExecutionPlan,
        &MintToCpiPlanningBoundary,
        &MintToCpiBoundary<'_, '_>,
    ) -> T,
) -> T {
    let program_id = Pubkey::new_unique();
    let token_program_key = spl_token::id();
    let mint_key = Pubkey::new_unique();
    let recipient_token_account_key = Pubkey::new_unique();
    let (mint_authority_pda_key, mint_authority_bump) = find_gateway_mint_authority(&program_id);

    let owner = Pubkey::new_unique();

    let mut token_program_lamports = 0;
    let mut mint_lamports = 0;
    let mut recipient_token_account_lamports = 0;
    let mut mint_authority_pda_lamports = 0;

    let mut token_program_data = [];
    let mut mint_data = [];
    let mut recipient_token_account_data = [];
    let mut mint_authority_pda_data = [];

    let token_program = AccountInfo::new(
        &token_program_key,
        false,
        false,
        &mut token_program_lamports,
        &mut token_program_data,
        &owner,
        true,
        0,
    );

    let mint = AccountInfo::new(
        &mint_key,
        false,
        true,
        &mut mint_lamports,
        &mut mint_data,
        &owner,
        false,
        0,
    );

    let recipient_token_account = AccountInfo::new(
        &recipient_token_account_key,
        false,
        true,
        &mut recipient_token_account_lamports,
        &mut recipient_token_account_data,
        &owner,
        false,
        0,
    );

    let mint_authority_pda = AccountInfo::new(
        &mint_authority_pda_key,
        false,
        false,
        &mut mint_authority_pda_lamports,
        &mut mint_authority_pda_data,
        &program_id,
        false,
        0,
    );

    let execution_plan = AtomicConsumeGatewayMintExecutionPlan {
        steps: ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER,
        canonical_event_key: [1u8; 32],
        route_id: [2u8; 32],
        recipient: [3u8; 32],
        target_mint_pubkey: mint_key.to_bytes(),
        amount: 1_000,
        consumed_slot: 123,
        source_chain_weight_bps: 10_000,
        live_route_activation_enabled: false,
        mint_to_invocation_from_process_instruction_enabled: false,
    };

    let planning_boundary = MintToCpiPlanningBoundary {
        token_program: token_program_key,
        mint: mint_key,
        recipient_token_account: recipient_token_account_key,
        mint_authority_pda: mint_authority_pda_key,
        mint_authority_bump,
        amount: execution_plan.amount,
        live_route_activation_enabled: false,
        invoke_signed_from_process_instruction_enabled: false,
    };

    let boundary = MintToCpiBoundary {
        accounts: MintToCpiAccounts {
            token_program: &token_program,
            mint: &mint,
            recipient_token_account: &recipient_token_account,
            mint_authority_pda: &mint_authority_pda,
        },
        mint_authority_bump,
        amount: execution_plan.amount,
    };

    f(&program_id, &execution_plan, &planning_boundary, &boundary)
}

struct RuntimeFixture {
    program_id: Pubkey,
    owners: RuntimeFixtureOwners,
    keys: RuntimeFixtureKeys,
    lamports: RuntimeFixtureLamports,
    data: RuntimeFixtureData,
    args: ConsumeGatewayMintArgs,
}

struct RuntimeFixtureOwners {
    program: Pubkey,
    spl_token: Pubkey,
    token_program_owner: Pubkey,
}

struct RuntimeFixtureKeys {
    mint_state: Pubkey,
    gateway_config: Pubkey,
    guardian_set: Pubkey,
    processed_event: Pubkey,
    recipient_balance: Pubkey,
    spl_mint: Pubkey,
    recipient_token_account: Pubkey,
    mint_authority_pda: Pubkey,
    token_program: Pubkey,
    rent_payer: Pubkey,
    system_program: Pubkey,
}

struct RuntimeFixtureLamports {
    mint_state: u64,
    gateway_config: u64,
    guardian_set: u64,
    processed_event: u64,
    recipient_balance: u64,
    spl_mint: u64,
    recipient_token_account: u64,
    mint_authority_pda: u64,
    token_program: u64,
    rent_payer: u64,
    system_program: u64,
}

struct RuntimeFixtureData {
    mint_state: Vec<u8>,
    gateway_config: Vec<u8>,
    guardian_set: Vec<u8>,
    processed_event: Vec<u8>,
    recipient_balance: Vec<u8>,
    spl_mint: Vec<u8>,
    recipient_token_account: Vec<u8>,
    mint_authority_pda: Vec<u8>,
    token_program: Vec<u8>,
    rent_payer: Vec<u8>,
    system_program: Vec<u8>,
}

impl RuntimeFixture {
    fn new() -> Self {
        let program_id = Pubkey::new_unique();
        let (mint_authority_pda, bump) = find_gateway_mint_authority(&program_id);
        let spl_mint = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let route_id = [0x11; 32];
        let guardian_set_id = [0x22; 32];
        let canonical_event_key = [0x44; 32];
        let source_chain_id = 1;
        let (processed_event, _) = Pubkey::find_program_address(
            &[b"xxxl", b"processed-event", &canonical_event_key],
            &program_id,
        );

        let owners = RuntimeFixtureOwners {
            program: program_id,
            spl_token: spl_token::id(),
            token_program_owner: Pubkey::new_unique(),
        };

        let keys = RuntimeFixtureKeys {
            mint_state: find_mint_state(&program_id, &spl_mint.to_bytes()).0,
            gateway_config: Pubkey::new_unique(),
            guardian_set: Pubkey::new_unique(),
            processed_event,
            recipient_balance: Pubkey::new_unique(),
            spl_mint,
            recipient_token_account: Pubkey::new_unique(),
            mint_authority_pda,
            token_program: spl_token::id(),
            rent_payer: Pubkey::new_unique(),
            system_program: system_program::id(),
        };

        let data = RuntimeFixtureData {
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
            mint_authority_pda: Vec::new(),
            token_program: Vec::new(),
            rent_payer: Vec::new(),
            system_program: Vec::new(),
        };

        let rent = Rent::default();
        let lamports = RuntimeFixtureLamports {
            mint_state: rent.minimum_balance(data.mint_state.len()),
            gateway_config: rent.minimum_balance(data.gateway_config.len()),
            guardian_set: rent.minimum_balance(data.guardian_set.len()),
            processed_event: 1,
            recipient_balance: rent.minimum_balance(data.recipient_balance.len()),
            spl_mint: rent.minimum_balance(data.spl_mint.len()),
            recipient_token_account: rent.minimum_balance(data.recipient_token_account.len()),
            mint_authority_pda: 0,
            token_program: 0,
            rent_payer: 10_000_000,
            system_program: 0,
        };

        let args = ConsumeGatewayMintArgs {
            raw: [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN],
            account_meta_count: 11,
            route_account_index: 1,
            guardian_set_account_index: 2,
            mint_state_account_index: 0,
            processed_event_account_index: 3,
            recipient_balance_account_index: 4,
            route_id,
            guardian_set_id,
            canonical_asset_id: spl_mint.to_bytes(),
            canonical_event_key,
            recipient: recipient_owner.to_bytes(),
            amount: 1_000,
            source_chain_id,
            source_chain_weight_bps: 10_000,
        };

        Self {
            program_id,
            owners,
            keys,
            lamports,
            data,
            args,
        }
    }

    fn accounts(&mut self) -> Vec<AccountInfo<'_>> {
        vec![
            AccountInfo::new(
                &self.keys.mint_state,
                false,
                false,
                &mut self.lamports.mint_state,
                &mut self.data.mint_state,
                &self.owners.program,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.gateway_config,
                false,
                false,
                &mut self.lamports.gateway_config,
                &mut self.data.gateway_config,
                &self.owners.program,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.guardian_set,
                false,
                false,
                &mut self.lamports.guardian_set,
                &mut self.data.guardian_set,
                &self.owners.program,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.processed_event,
                false,
                true,
                &mut self.lamports.processed_event,
                &mut self.data.processed_event,
                &self.keys.system_program,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.recipient_balance,
                false,
                true,
                &mut self.lamports.recipient_balance,
                &mut self.data.recipient_balance,
                &self.owners.program,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.spl_mint,
                false,
                true,
                &mut self.lamports.spl_mint,
                &mut self.data.spl_mint,
                &self.owners.spl_token,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.recipient_token_account,
                false,
                true,
                &mut self.lamports.recipient_token_account,
                &mut self.data.recipient_token_account,
                &self.owners.spl_token,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.mint_authority_pda,
                false,
                false,
                &mut self.lamports.mint_authority_pda,
                &mut self.data.mint_authority_pda,
                &self.owners.program,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.token_program,
                false,
                false,
                &mut self.lamports.token_program,
                &mut self.data.token_program,
                &self.owners.token_program_owner,
                true,
                0,
            ),
            AccountInfo::new(
                &self.keys.rent_payer,
                true,
                true,
                &mut self.lamports.rent_payer,
                &mut self.data.rent_payer,
                &self.owners.token_program_owner,
                false,
                0,
            ),
            AccountInfo::new(
                &self.keys.system_program,
                false,
                false,
                &mut self.lamports.system_program,
                &mut self.data.system_program,
                &self.owners.token_program_owner,
                true,
                0,
            ),
        ]
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
        mint_authority: COption::Some(mint_authority),
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
        mint,
        owner,
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

fn assert_spl_supply_and_recipient_amount(
    fixture: &RuntimeFixture,
    expected_supply: u64,
    expected_recipient_amount: u64,
) {
    let spl_mint = SplTokenMint::unpack(&fixture.data.spl_mint).expect("valid SPL mint");
    assert_eq!(spl_mint.supply, expected_supply);

    let recipient_token_account = SplTokenAccount::unpack(&fixture.data.recipient_token_account)
        .expect("valid recipient token account");
    assert_eq!(recipient_token_account.amount, expected_recipient_amount);
}

fn read_u128_le(input: &[u8], offset: usize) -> u128 {
    let mut output = [0u8; 16];
    output.copy_from_slice(&input[offset..offset + 16]);
    u128::from_le_bytes(output)
}

fn read_u64_le(input: &[u8], offset: usize) -> u64 {
    let mut output = [0u8; 8];
    output.copy_from_slice(&input[offset..offset + 8]);
    u64::from_le_bytes(output)
}

fn read_fixed_32(input: &[u8], offset: usize) -> [u8; 32] {
    let mut output = [0u8; 32];
    output.copy_from_slice(&input[offset..offset + 32]);
    output
}

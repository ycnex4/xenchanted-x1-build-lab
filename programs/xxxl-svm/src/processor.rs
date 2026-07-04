// LEGACY / PRE-41K.4:
// This processor scaffold validates and plans around an already-initialized
// program-owned processed_event account. It is intentionally not a Phase
// 41K.4 atomic marking route and must not be used as live replay protection.
//
// Phase 41K.4 must remain isolated until a later mark+mint atomic integration
// proves quorum + decode + eligibility + mark + SPL mint in one execution path.
#[cfg(test)]
use crate::execution_plan::apply_atomic_state_mutation_composition_boundary;

use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_program, sysvar::Sysvar,
};

use crate::{
    account_contract::assert_consume_gateway_mint_account_contract,
    cpi::{
        assert_gateway_mint_authority_pda, guarded_mint_to_cpi_execution_gate_boundary,
        plan_mint_to_cpi_boundary, MintToCpiAccounts, MintToCpiBoundary, MintToCpiPlanningBoundary,
    },
    error::XxxlError,
    execution_plan::{
        build_atomic_consume_gateway_mint_execution_plan, AtomicConsumeGatewayMintExecutionPlan,
    },
    instruction::{
        ConsumeGatewayMintArgs, XxxlInstruction, CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT,
    },
    state::{
        GatewayConfigAccountView, GuardianSetAccountView, MintStateAccountView,
        RecipientBalanceAccountView,
    },
    validation::{
        assert_account_owner, assert_initialized_mint_account, assert_recipient_ata_boundary,
        assert_rent_exempt,
    },
    verifier::{
        load_phase_41k_3_processed_registry_account_info,
        Phase41K3ProcessedRegistryAccountLoadingStatus,
    },
};

pub const CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS: usize = 11;

pub const ACCOUNT_INDEX_MINT_STATE: usize = 0;
pub const ACCOUNT_INDEX_GATEWAY_CONFIG: usize = 1;
pub const ACCOUNT_INDEX_GUARDIAN_SET: usize = 2;
pub const ACCOUNT_INDEX_PROCESSED_EVENT: usize = 3;
pub const ACCOUNT_INDEX_RECIPIENT_BALANCE: usize = 4;
pub const ACCOUNT_INDEX_SPL_TOKEN_MINT: usize = 5;
pub const ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT: usize = 6;
pub const ACCOUNT_INDEX_MINT_AUTHORITY_PDA: usize = 7;
pub const ACCOUNT_INDEX_TOKEN_PROGRAM: usize = 8;
pub const ACCOUNT_INDEX_RENT_PAYER: usize = 9;
pub const ACCOUNT_INDEX_SYSTEM_PROGRAM: usize = 10;

pub const LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED: bool = false;

pub struct PreparedConsumeGatewayMintCpi<'a, 'b> {
    pub boundary: MintToCpiBoundary<'a, 'b>,
    pub mint_decimals: u8,
    pub source_chain_weight_bps: u16,
}

pub struct RuntimeConsumeGatewayMintPlanningComposition {
    pub execution_plan: AtomicConsumeGatewayMintExecutionPlan,
    pub mint_to_cpi_plan: MintToCpiPlanningBoundary,
    pub live_route_activation_enabled: bool,
    pub invoke_signed_from_process_instruction_enabled: bool,
}

#[cfg(test)]
pub struct RuntimeConsumeGatewayMintLocalStateMutationComposition {
    pub planning_composition: RuntimeConsumeGatewayMintPlanningComposition,
    pub recipient_balance_after: u128,
    pub live_route_activation_enabled: bool,
    pub invoke_signed_from_process_instruction_enabled: bool,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    #[cfg(feature = "phase-41k4-svm-test-harness")]
    {
        if crate::processed_event_marking_svm_harness::is_phase_41k4_marking_svm_harness_instruction(
            instruction_data,
        ) {
            return crate::processed_event_marking_svm_harness::process_phase_41k4_marking_svm_harness_instruction(
                program_id,
                accounts,
                instruction_data,
            );
        }
    }

    let instruction = XxxlInstruction::unpack(instruction_data)?;

    match instruction {
        XxxlInstruction::ConsumeGatewayMint(args) => {
            process_consume_gateway_mint(program_id, accounts, &args)
        }
    }
}

fn process_consume_gateway_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: &ConsumeGatewayMintArgs,
) -> ProgramResult {
    let rent = Rent::get()?;
    let clock = Clock::get()?;

    let _execution_plan = build_runtime_consume_gateway_mint_execution_plan_boundary(
        program_id, accounts, args, &rent, clock.slot,
    )?;

    msg!("XXXL consume_gateway_mint execution plan built; live route execution is not activated");
    Ok(())
}

fn build_runtime_consume_gateway_mint_execution_plan_boundary(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: &ConsumeGatewayMintArgs,
    rent: &Rent,
    consumed_slot: u64,
) -> Result<AtomicConsumeGatewayMintExecutionPlan, ProgramError> {
    let prepared = prepare_consume_gateway_mint_cpi_boundary(program_id, accounts, args, rent)?;
    let execution_plan =
        build_atomic_consume_gateway_mint_execution_plan(args, &prepared, consumed_slot)?;

    if execution_plan.live_route_activation_enabled
        || execution_plan.mint_to_invocation_from_process_instruction_enabled
    {
        return Err(XxxlError::CpiBoundaryNotReady.into());
    }

    Ok(execution_plan)
}

pub fn build_runtime_consume_gateway_mint_planning_composition_boundary(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: &ConsumeGatewayMintArgs,
    rent: &Rent,
    consumed_slot: u64,
) -> Result<RuntimeConsumeGatewayMintPlanningComposition, ProgramError> {
    let prepared = prepare_consume_gateway_mint_cpi_boundary(program_id, accounts, args, rent)?;
    let execution_plan =
        build_atomic_consume_gateway_mint_execution_plan(args, &prepared, consumed_slot)?;

    if execution_plan.live_route_activation_enabled
        || execution_plan.mint_to_invocation_from_process_instruction_enabled
    {
        return Err(XxxlError::CpiBoundaryNotReady.into());
    }

    let mint_to_cpi_plan =
        plan_mint_to_cpi_boundary(program_id, &execution_plan, &prepared.boundary)?;

    if mint_to_cpi_plan.live_route_activation_enabled
        || mint_to_cpi_plan.invoke_signed_from_process_instruction_enabled
    {
        return Err(XxxlError::CpiBoundaryNotReady.into());
    }

    Ok(RuntimeConsumeGatewayMintPlanningComposition {
        execution_plan,
        mint_to_cpi_plan,
        live_route_activation_enabled: false,
        invoke_signed_from_process_instruction_enabled: false,
    })
}

#[cfg(test)]
pub fn build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: &ConsumeGatewayMintArgs,
    rent: &Rent,
    consumed_slot: u64,
) -> Result<RuntimeConsumeGatewayMintLocalStateMutationComposition, ProgramError> {
    let planning_composition = build_runtime_consume_gateway_mint_planning_composition_boundary(
        program_id,
        accounts,
        args,
        rent,
        consumed_slot,
    )?;

    if planning_composition.live_route_activation_enabled
        || planning_composition.invoke_signed_from_process_instruction_enabled
        || planning_composition
            .execution_plan
            .live_route_activation_enabled
        || planning_composition
            .execution_plan
            .mint_to_invocation_from_process_instruction_enabled
        || planning_composition
            .mint_to_cpi_plan
            .live_route_activation_enabled
        || planning_composition
            .mint_to_cpi_plan
            .invoke_signed_from_process_instruction_enabled
    {
        return Err(XxxlError::CpiBoundaryNotReady.into());
    }

    let processed_event_account =
        account_at(accounts, args.processed_event_account_index as usize)?;
    let recipient_balance_account =
        account_at(accounts, args.recipient_balance_account_index as usize)?;

    let recipient_balance_after = {
        let mut processed_event_data = processed_event_account.try_borrow_mut_data()?;
        let mut recipient_balance_data = recipient_balance_account.try_borrow_mut_data()?;

        apply_atomic_state_mutation_composition_boundary(
            &mut processed_event_data,
            &mut recipient_balance_data,
            &planning_composition.execution_plan,
        )?
    };

    Ok(RuntimeConsumeGatewayMintLocalStateMutationComposition {
        planning_composition,
        recipient_balance_after,
        live_route_activation_enabled: false,
        invoke_signed_from_process_instruction_enabled: false,
    })
}

pub fn build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: &ConsumeGatewayMintArgs,
    rent: &Rent,
    consumed_slot: u64,
) -> ProgramResult {
    let prepared = prepare_consume_gateway_mint_cpi_boundary(program_id, accounts, args, rent)?;

    let planning_composition = build_runtime_consume_gateway_mint_planning_composition_boundary(
        program_id,
        accounts,
        args,
        rent,
        consumed_slot,
    )?;

    guarded_mint_to_cpi_execution_gate_boundary(
        program_id,
        &planning_composition.execution_plan,
        &planning_composition.mint_to_cpi_plan,
        &prepared.boundary,
    )
}

pub fn build_guarded_consume_gateway_mint_live_handler_fixture(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: &ConsumeGatewayMintArgs,
    rent: &Rent,
    consumed_slot: u64,
) -> Result<AtomicConsumeGatewayMintExecutionPlan, ProgramError> {
    build_runtime_consume_gateway_mint_execution_plan_boundary(
        program_id,
        accounts,
        args,
        rent,
        consumed_slot,
    )
}

pub fn prepare_consume_gateway_mint_cpi_boundary<'a, 'b>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
    args: &ConsumeGatewayMintArgs,
    rent: &Rent,
) -> Result<PreparedConsumeGatewayMintCpi<'a, 'b>, ProgramError> {
    if accounts.len() != CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS
        || args.account_meta_count != CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    assert_consume_gateway_mint_account_contract(accounts)?;

    let mint_state_account = account_at(accounts, args.mint_state_account_index as usize)?;
    let gateway_config_account = account_at(accounts, args.route_account_index as usize)?;
    let guardian_set_account = account_at(accounts, args.guardian_set_account_index as usize)?;
    let processed_event_account =
        account_at(accounts, args.processed_event_account_index as usize)?;
    let recipient_balance_account =
        account_at(accounts, args.recipient_balance_account_index as usize)?;
    let spl_token_mint_account = account_at(accounts, ACCOUNT_INDEX_SPL_TOKEN_MINT)?;
    let recipient_token_account = account_at(accounts, ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT)?;
    let mint_authority_pda = account_at(accounts, ACCOUNT_INDEX_MINT_AUTHORITY_PDA)?;
    let token_program = account_at(accounts, ACCOUNT_INDEX_TOKEN_PROGRAM)?;
    let _rent_payer = account_at(accounts, ACCOUNT_INDEX_RENT_PAYER)?;
    let system_program_account = account_at(accounts, ACCOUNT_INDEX_SYSTEM_PROGRAM)?;

    if token_program.key != &spl_token::id() {
        return Err(XxxlError::InvalidAccountOwner.into());
    }

    if system_program_account.key != &system_program::id() {
        return Err(XxxlError::InvalidAccountOwner.into());
    }

    for program_owned_account in [
        mint_state_account,
        gateway_config_account,
        guardian_set_account,
        recipient_balance_account,
    ] {
        assert_account_owner(program_owned_account, program_id)?;
        assert_rent_exempt(program_owned_account, rent)?;
    }

    assert_rent_exempt(spl_token_mint_account, rent)?;
    assert_rent_exempt(recipient_token_account, rent)?;

    let mint_state_data = mint_state_account.try_borrow_data()?;
    let gateway_config_data = gateway_config_account.try_borrow_data()?;
    let guardian_set_data = guardian_set_account.try_borrow_data()?;
    let recipient_balance_data = recipient_balance_account.try_borrow_data()?;

    let mint_state = MintStateAccountView::new(&mint_state_data)?;
    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;
    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;
    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;

    if mint_state.mint_pubkey() != args.mint_id
        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    assert_gateway_mint_authority_pda(
        program_id,
        mint_authority_pda.key,
        mint_state.gateway_mint_authority_bump(),
    )?;

    if gateway_config.route_id() != args.route_id
        || gateway_config.guardian_set_id() != args.guardian_set_id
        || gateway_config.target_mint() != args.mint_id
        || gateway_config.source_chain_weight_bps() != args.source_chain_weight_bps
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if args.source_chain_id != gateway_config.source_chain_id() {
        return Err(XxxlError::InvalidSourceChain.into());
    }

    if guardian_set.guardian_set_id() != args.guardian_set_id {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let processed_event_load = load_phase_41k_3_processed_registry_account_info(
        Some(processed_event_account),
        program_id,
        &args.canonical_event_key,
        &args.route_id,
        &args.recipient,
    );

    if processed_event_load.status
        != Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountUnprocessed
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != args.mint_id {
        return Err(XxxlError::InvalidRecipientAta.into());
    }

    let mint_pubkey = Pubkey::new_from_array(args.mint_id);
    let recipient_owner = Pubkey::new_from_array(args.recipient);

    let mint_decimals =
        assert_initialized_mint_account(spl_token_mint_account, mint_authority_pda.key)?;

    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &mint_pubkey)?;

    if args.amount == 0 || args.amount > u64::MAX as u128 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    Ok(PreparedConsumeGatewayMintCpi {
        boundary: MintToCpiBoundary {
            accounts: MintToCpiAccounts {
                token_program,
                mint: spl_token_mint_account,
                recipient_token_account,
                mint_authority_pda,
            },
            mint_authority_bump: mint_state.gateway_mint_authority_bump(),
            amount: args.amount as u64,
        },
        mint_decimals,
        source_chain_weight_bps: args.source_chain_weight_bps,
    })
}

fn account_at<'a, 'b>(
    accounts: &'a [AccountInfo<'b>],
    index: usize,
) -> Result<&'a AccountInfo<'b>, ProgramError> {
    accounts
        .get(index)
        .ok_or_else(|| XxxlError::InvalidInstruction.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        instruction::CONSUME_GATEWAY_MINT_INSTRUCTION_LEN,
        pda::find_gateway_mint_authority,
        state::{
            GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, GATEWAY_CONFIG_ACCOUNT_LEN,
            GUARDIAN_SET_ACCOUNT_DISCRIMINATOR, GUARDIAN_SET_ACCOUNT_LEN,
            MINT_STATE_ACCOUNT_DISCRIMINATOR, MINT_STATE_ACCOUNT_LEN,
            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR, PROCESSED_EVENT_ACCOUNT_LEN,
            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR, RECIPIENT_BALANCE_ACCOUNT_LEN,
            RUNTIME_LAYOUT_VERSION,
        },
        verifier::find_phase_41k_3_processed_event_pda,
    };
    use solana_program::{
        account_info::AccountInfo, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    };
    use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplTokenMint};
    use std::str::FromStr;

    const FIXTURE_PROGRAM_ID: &str = "11111111111111111111111111111111";

    #[test]
    fn handler_integration_prepares_cpi_boundary_after_decode_and_validation() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let expected_source_chain_id = read_u64_le(&fixture.data.gateway_config, 48);

        let accounts = fixture.accounts();

        let prepared =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent)
                .expect("prepared CPI boundary");

        assert_eq!(prepared.boundary.amount, 1_000);
        assert_eq!(prepared.boundary.mint_authority_bump, fixture_bump());
        assert_eq!(prepared.mint_decimals, 18);
        assert_eq!(prepared.source_chain_weight_bps, 10_000);
        assert_eq!(args.source_chain_id, expected_source_chain_id);
        assert_eq!(
            prepared.boundary.accounts.token_program.key,
            &spl_token::id()
        );
    }

    #[test]
    fn consume_gateway_mint_v2_happy_path_matches_gateway_config() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent)
            .expect("v2 source_chain_id matches GatewayConfig");

        assert_eq!(args.source_chain_id, 1);
    }

    #[test]
    fn handler_integration_rejects_wrong_account_count() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();

        let accounts = fixture.accounts();
        let result = prepare_consume_gateway_mint_cpi_boundary(
            &program_id,
            &accounts[..CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS - 1],
            &args,
            &rent,
        );

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_gateway_route_mismatch() {
        let mut fixture = HandlerFixture::new();
        fixture.data.gateway_config[16] ^= 0xff;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_consumed_processed_event() {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[10] = 1;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_mint_authority_pda() {
        let mut fixture = HandlerFixture::new();
        fixture.keys.mint_authority_pda = Pubkey::new_unique();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_spl_mint_owner() {
        let mut fixture = HandlerFixture::new();
        fixture.owners.spl_token = Pubkey::new_unique();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidAccountOwner);
    }

    #[test]
    fn handler_integration_rejects_wrong_recipient_token_mint() {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_token_account = packed_token_account(
            Pubkey::new_unique(),
            fixture.keys.recipient_owner,
            AccountState::Initialized,
        );

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidRecipientAta);
    }

    #[test]
    fn handler_integration_rejects_zero_amount() {
        let mut fixture = HandlerFixture::new();
        fixture.args.amount = 0;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_account_order() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let mut accounts = fixture.accounts();

        accounts.swap(ACCOUNT_INDEX_GATEWAY_CONFIG, ACCOUNT_INDEX_GUARDIAN_SET);

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn handler_integration_rejects_wrong_program_owner_for_program_owned_account() {
        let mut fixture = HandlerFixture::new();
        fixture.owners.program = Pubkey::new_unique();

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidAccountOwner);
    }

    #[test]
    fn handler_integration_rejects_wrong_spl_token_program_id() {
        let mut fixture = HandlerFixture::new();
        fixture.keys.token_program = Pubkey::new_unique();

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidAccountOwner);
    }

    #[test]
    fn handler_integration_rejects_wrong_spl_mint_authority() {
        let mut fixture = HandlerFixture::new();
        fixture.data.spl_mint = packed_mint(Pubkey::new_unique(), true);

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidPda);
    }

    #[test]
    fn handler_integration_rejects_wrong_mint_authority_bump() {
        let mut fixture = HandlerFixture::new();
        fixture.data.mint_state[13] = fixture.data.mint_state[13].wrapping_add(1);

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidPda);
    }

    #[test]
    fn handler_integration_rejects_gateway_config_guardian_set_id_mismatch() {
        let mut fixture = HandlerFixture::new();
        fixture.data.gateway_config[120] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_gateway_config_target_mint_mismatch() {
        let mut fixture = HandlerFixture::new();
        fixture.data.gateway_config[88] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_gateway_config_source_chain_weight_mismatch() {
        let mut fixture = HandlerFixture::new();
        fixture.data.gateway_config[12] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_rejects_source_chain_id_mismatch() {
        let mut fixture = HandlerFixture::new();
        fixture.data.gateway_config[48..56].copy_from_slice(&2u64.to_le_bytes());

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidSourceChain);
    }

    #[test]
    fn handler_rejects_source_chain_id_zero() {
        let mut fixture = HandlerFixture::new();
        fixture.args.source_chain_id = 0;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidSourceChain);
    }

    #[test]
    fn handler_rejects_source_chain_id_unexpected() {
        let mut fixture = HandlerFixture::new();
        fixture.args.source_chain_id = 77;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidSourceChain);
    }

    #[test]
    fn handler_integration_rejects_wrong_guardian_set_id() {
        let mut fixture = HandlerFixture::new();
        fixture.data.guardian_set[272] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_processed_event_canonical_event_key() {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[16] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_processed_event_route_id() {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[48] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_processed_event_recipient() {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[80] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_recipient_balance_owner() {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_balance[16] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidRecipientAta);
    }

    #[test]
    fn handler_integration_rejects_wrong_recipient_balance_mint() {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_balance[48] ^= 0xff;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidRecipientAta);
    }

    #[test]
    fn handler_integration_rejects_amount_larger_than_spl_token_u64_range() {
        let mut fixture = HandlerFixture::new();
        fixture.args.amount = u64::MAX as u128 + 1;

        assert_prepare_boundary_rejects(&mut fixture, XxxlError::InvalidInstruction);
    }

    #[test]
    fn guarded_live_handler_fixture_builds_disabled_execution_plan_after_validation() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let plan = build_guarded_consume_gateway_mint_live_handler_fixture(
            &program_id,
            &accounts,
            &args,
            &rent,
            77,
        )
        .expect("guarded live handler fixture plan");

        assert_eq!(plan.canonical_event_key, args.canonical_event_key);
        assert_eq!(plan.route_id, args.route_id);
        assert_eq!(plan.recipient, args.recipient);
        assert_eq!(plan.mint, args.mint_id);
        assert_eq!(plan.amount, 1_000);
        assert_eq!(plan.consumed_slot, 77);
        assert_eq!(plan.source_chain_weight_bps, 10_000);
        assert!(!plan.live_route_activation_enabled);
        assert!(!plan.mint_to_invocation_from_process_instruction_enabled);
    }

    #[test]
    fn guarded_live_handler_fixture_rejects_invalid_boundary_before_plan() {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[10] = 1;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result = build_guarded_consume_gateway_mint_live_handler_fixture(
            &program_id,
            &accounts,
            &args,
            &rent,
            77,
        );

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn process_instruction_execution_plan_helper_builds_plan_without_live_route_activation() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let execution_plan = build_runtime_consume_gateway_mint_execution_plan_boundary(
            &program_id,
            &accounts,
            &args,
            &rent,
            88,
        )
        .expect("execution plan boundary remains non-live");

        assert_eq!(execution_plan.amount, 1_000);
        assert_eq!(execution_plan.consumed_slot, 88);
        assert_eq!(execution_plan.canonical_event_key, args.canonical_event_key);
        assert_eq!(execution_plan.route_id, args.route_id);
        assert_eq!(execution_plan.recipient, args.recipient);
        assert_eq!(execution_plan.mint, args.mint_id);
        assert_eq!(execution_plan.source_chain_weight_bps, 10_000);
        assert!(!execution_plan.live_route_activation_enabled);
        assert!(!execution_plan.mint_to_invocation_from_process_instruction_enabled);
    }

    #[test]
    fn runtime_planning_composition_boundary_builds_execution_and_cpi_plans_without_mutation() {
        let mut fixture = HandlerFixture::new();
        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();
        let expected_mint_authority_pda = fixture.keys.mint_authority_pda;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let composition = build_runtime_consume_gateway_mint_planning_composition_boundary(
            &program_id,
            &accounts,
            &args,
            &rent,
            99,
        )
        .expect("runtime planning composition boundary");

        assert_eq!(composition.execution_plan.amount, 1_000);
        assert_eq!(composition.execution_plan.consumed_slot, 99);
        assert_eq!(
            composition.execution_plan.canonical_event_key,
            args.canonical_event_key
        );
        assert_eq!(composition.execution_plan.route_id, args.route_id);
        assert_eq!(composition.execution_plan.recipient, args.recipient);
        assert_eq!(composition.execution_plan.mint, args.mint_id);
        assert_eq!(composition.execution_plan.source_chain_weight_bps, 10_000);
        assert!(!composition.execution_plan.live_route_activation_enabled);
        assert!(
            !composition
                .execution_plan
                .mint_to_invocation_from_process_instruction_enabled
        );

        assert_eq!(composition.mint_to_cpi_plan.token_program, spl_token::id());
        assert_eq!(composition.mint_to_cpi_plan.mint.to_bytes(), args.mint_id);
        assert_eq!(composition.mint_to_cpi_plan.amount, 1_000);
        assert_eq!(
            composition.mint_to_cpi_plan.mint_authority_pda,
            expected_mint_authority_pda
        );
        assert!(!composition.mint_to_cpi_plan.live_route_activation_enabled);
        assert!(
            !composition
                .mint_to_cpi_plan
                .invoke_signed_from_process_instruction_enabled
        );

        assert!(!composition.live_route_activation_enabled);
        assert!(!composition.invoke_signed_from_process_instruction_enabled);

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    }

    #[test]
    fn runtime_planning_composition_boundary_rejects_consumed_event_without_mutation() {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[10] = 1;

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_planning_composition_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                99,
            ),
            XxxlError::InvalidInstruction,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    }

    #[test]
    fn runtime_planning_composition_boundary_rejects_zero_amount_without_mutation() {
        let mut fixture = HandlerFixture::new();
        fixture.args.amount = 0;

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_planning_composition_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                99,
            ),
            XxxlError::InvalidInstruction,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    }

    #[test]
    fn runtime_planning_composition_boundary_rejects_wrong_recipient_token_account_without_mutation(
    ) {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_token_account = packed_token_account(
            fixture.keys.spl_mint,
            Pubkey::new_unique(),
            AccountState::Initialized,
        );

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_planning_composition_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                99,
            ),
            XxxlError::InvalidRecipientAta,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    }

    #[test]
    fn runtime_local_state_mutation_composition_boundary_rejects_live_style_system_owned_empty_processed_event(
    ) {
        let mut fixture = HandlerFixture::new();

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();
        let spl_mint_before = fixture.data.spl_mint.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                123,
            ),
            XxxlError::InvalidInstruction,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
        assert_eq!(fixture.data.spl_mint, spl_mint_before);
    }

    #[test]
    fn runtime_local_state_mutation_composition_boundary_rejects_recipient_overflow_before_event_mark(
    ) {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_balance[80..96].copy_from_slice(&u128::MAX.to_le_bytes());

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                123,
            ),
            XxxlError::InvalidInstruction,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    }

    #[test]
    fn runtime_local_state_mutation_composition_boundary_rejects_consumed_event_without_credit() {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[10] = 1;

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                123,
            ),
            XxxlError::InvalidInstruction,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    }

    #[test]
    fn runtime_local_state_mutation_composition_boundary_rejects_wrong_recipient_token_account_without_mutation(
    ) {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_token_account = packed_token_account(
            fixture.keys.spl_mint,
            Pubkey::new_unique(),
            AccountState::Initialized,
        );

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                123,
            ),
            XxxlError::InvalidRecipientAta,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
    }

    #[test]
    fn runtime_disabled_spl_cpi_gate_boundary_rejects_at_gate_without_mutation() {
        let mut fixture = HandlerFixture::new();

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();
        let spl_mint_before = fixture.data.spl_mint.clone();
        let recipient_token_account_before = fixture.data.recipient_token_account.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                144,
            ),
            XxxlError::CpiBoundaryNotReady,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
        assert_eq!(fixture.data.spl_mint, spl_mint_before);
        assert_eq!(
            fixture.data.recipient_token_account,
            recipient_token_account_before
        );
    }

    #[test]
    fn runtime_disabled_spl_cpi_gate_boundary_rejects_consumed_event_before_gate_without_mutation()
    {
        let mut fixture = HandlerFixture::new();
        fixture.use_program_owned_initialized_unconsumed_processed_event();
        fixture.data.processed_event[10] = 1;

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();
        let spl_mint_before = fixture.data.spl_mint.clone();
        let recipient_token_account_before = fixture.data.recipient_token_account.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                144,
            ),
            XxxlError::InvalidInstruction,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
        assert_eq!(fixture.data.spl_mint, spl_mint_before);
        assert_eq!(
            fixture.data.recipient_token_account,
            recipient_token_account_before
        );
    }

    #[test]
    fn runtime_disabled_spl_cpi_gate_boundary_rejects_wrong_recipient_token_account_without_mutation(
    ) {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_token_account = packed_token_account(
            fixture.keys.spl_mint,
            Pubkey::new_unique(),
            AccountState::Initialized,
        );

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();
        let spl_mint_before = fixture.data.spl_mint.clone();
        let recipient_token_account_before = fixture.data.recipient_token_account.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                144,
            ),
            XxxlError::InvalidRecipientAta,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
        assert_eq!(fixture.data.spl_mint, spl_mint_before);
        assert_eq!(
            fixture.data.recipient_token_account,
            recipient_token_account_before
        );
    }

    #[test]
    fn runtime_disabled_spl_cpi_gate_boundary_rejects_zero_amount_without_mutation() {
        let mut fixture = HandlerFixture::new();
        fixture.args.amount = 0;

        let processed_before = fixture.data.processed_event.clone();
        let recipient_balance_before = fixture.data.recipient_balance.clone();
        let spl_mint_before = fixture.data.spl_mint.clone();
        let recipient_token_account_before = fixture.data.recipient_token_account.clone();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary(
                &program_id,
                &accounts,
                &args,
                &rent,
                144,
            ),
            XxxlError::InvalidInstruction,
        );

        drop(accounts);

        assert_eq!(fixture.data.processed_event, processed_before);
        assert_eq!(fixture.data.recipient_balance, recipient_balance_before);
        assert_eq!(fixture.data.spl_mint, spl_mint_before);
        assert_eq!(
            fixture.data.recipient_token_account,
            recipient_token_account_before
        );
    }

    #[test]
    fn runtime_account_contract_rejects_unnecessary_writable_readonly_account() {
        let mut fixture = HandlerFixture::new();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let mut accounts = fixture.accounts();

        accounts[ACCOUNT_INDEX_MINT_STATE].is_writable = true;

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn runtime_account_contract_rejects_missing_required_writable_account() {
        let mut fixture = HandlerFixture::new();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let mut accounts = fixture.accounts();

        accounts[ACCOUNT_INDEX_PROCESSED_EVENT].is_writable = false;

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn runtime_account_contract_rejects_unexpected_external_signer() {
        let mut fixture = HandlerFixture::new();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let mut accounts = fixture.accounts();

        accounts[ACCOUNT_INDEX_RECIPIENT_BALANCE].is_signer = true;

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn runtime_account_contract_rejects_missing_rent_payer_signature() {
        let mut fixture = HandlerFixture::new();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let mut accounts = fixture.accounts();

        accounts[ACCOUNT_INDEX_RENT_PAYER].is_signer = false;

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn runtime_account_contract_rejects_readonly_rent_payer() {
        let mut fixture = HandlerFixture::new();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let mut accounts = fixture.accounts();

        accounts[ACCOUNT_INDEX_RENT_PAYER].is_writable = false;

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn runtime_account_contract_rejects_wrong_system_program_id() {
        let mut fixture = HandlerFixture::new();
        fixture.keys.system_program = Pubkey::new_unique();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            XxxlError::InvalidAccountOwner,
        );
    }

    struct HandlerFixture {
        program_id: Pubkey,
        owners: FixtureOwners,
        keys: FixtureKeys,
        lamports: FixtureLamports,
        data: FixtureData,
        args: ConsumeGatewayMintArgs,
    }

    struct FixtureOwners {
        program: Pubkey,
        processed_event: Pubkey,
        spl_token: Pubkey,
        token_program_owner: Pubkey,
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
        rent_payer: Pubkey,
        system_program: Pubkey,
        recipient_owner: Pubkey,
    }

    struct FixtureLamports {
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

    struct FixtureData {
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

    impl HandlerFixture {
        fn new() -> Self {
            let program_id =
                Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
            let (mint_authority_pda, bump) = find_gateway_mint_authority(&program_id);

            let spl_mint = Pubkey::new_unique();
            let recipient_owner = Pubkey::new_unique();
            let route_id = [0x11; 32];
            let guardian_set_id = [0x22; 32];
            let canonical_event_key = [0x44; 32];
            let source_chain_id = 1;
            let (processed_event, _) =
                find_phase_41k_3_processed_event_pda(&program_id, &canonical_event_key);

            let owners = FixtureOwners {
                program: program_id,
                processed_event: system_program::id(),
                spl_token: spl_token::id(),
                token_program_owner: Pubkey::new_unique(),
            };

            let keys = FixtureKeys {
                mint_state: Pubkey::new_unique(),
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
                recipient_owner,
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
                mint_authority_pda: Vec::new(),
                token_program: Vec::new(),
                rent_payer: Vec::new(),
                system_program: Vec::new(),
            };

            let rent = Rent::default();

            let lamports = FixtureLamports {
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
                mint_id: spl_mint.to_bytes(),
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

        fn use_program_owned_initialized_unconsumed_processed_event(&mut self) {
            self.owners.processed_event = self.program_id;
            self.data.processed_event = processed_event_data(
                false,
                self.args.canonical_event_key,
                self.args.route_id,
                Pubkey::new_from_array(self.args.recipient),
            );
            self.lamports.processed_event =
                Rent::default().minimum_balance(self.data.processed_event.len());
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
                    &self.owners.processed_event,
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
        data[10] = if consumed { 1 } else { 0 };
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

        SplTokenMint::pack(mint, &mut data).expect("pack mint");
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

        SplTokenAccount::pack(account, &mut data).expect("pack token account");
        data
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

    fn fixture_bump() -> u8 {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);
        bump
    }

    fn assert_prepare_boundary_rejects(fixture: &mut HandlerFixture, error: XxxlError) {
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        assert_custom_error(
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent),
            error,
        );
    }

    fn assert_custom_error<T>(result: Result<T, ProgramError>, error: XxxlError) {
        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
    }
}

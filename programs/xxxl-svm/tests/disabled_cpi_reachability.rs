use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

use xxxl_svm::{
    cpi::{
        guarded_mint_to_cpi_execution_gate_boundary, spl_mint_to_cpi_execution_enabled,
        MintToCpiAccounts, MintToCpiBoundary, MintToCpiPlanningBoundary,
    },
    error::XxxlError,
    execution_plan::{AtomicConsumeGatewayMintExecutionPlan, ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER},
    pda::find_gateway_mint_authority,
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
            Err(ProgramError::Custom(
                XxxlError::CpiBoundaryNotReady as u32
            ))
        );
    });
}

#[test]
fn guarded_mint_to_cpi_boundary_rejects_live_execution_plan_flag() {
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
            Err(ProgramError::Custom(
                XxxlError::CpiBoundaryNotReady as u32
            ))
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
            Err(ProgramError::Custom(
                XxxlError::CpiBoundaryNotReady as u32
            ))
        );
    });
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
        mint: mint_key.to_bytes(),
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

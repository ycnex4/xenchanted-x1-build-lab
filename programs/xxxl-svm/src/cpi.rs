use solana_program::{
    account_info::AccountInfo, instruction::Instruction, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey,
};

#[cfg(all(
    feature = "phase-41k5-d2-production-path-test-gate",
    not(feature = "dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build")
))]
compile_error!(
    "phase-41k5-d2-production-path-test-gate opens the production ConsumeGatewayMint SPL CPI path. \
     It is a non-production integration test gate and must never be included in deploy artifacts. \
     For D2 Mollusk SBF tests only, explicitly add feature \
     dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build."
);

#[cfg(all(
    feature = "phase-41k5-d2-production-path-test-gate",
    feature = "dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build",
    not(feature = "phase-41k6-b1c7-handler-integration-test-gate")
))]
compile_error!(
    "phase-41k5-d2-production-path-test-gate cannot open SPL mint CPI without      phase-41k6-b1c7-handler-integration-test-gate. B1 closure requires guardian      authorization before any live mark+mint path."
);

use crate::{
    error::XxxlError,
    pda::{
        find_gateway_mint_authority, GATEWAY_MINT_AUTHORITY_SEED_0, GATEWAY_MINT_AUTHORITY_SEED_1,
        GATEWAY_MINT_AUTHORITY_SEED_2,
    },
};

pub struct MintToCpiAccounts<'a, 'b> {
    pub token_program: &'a AccountInfo<'b>,
    pub mint: &'a AccountInfo<'b>,
    pub recipient_token_account: &'a AccountInfo<'b>,
    pub mint_authority_pda: &'a AccountInfo<'b>,
}

pub struct MintToCpiBoundary<'a, 'b> {
    pub accounts: MintToCpiAccounts<'a, 'b>,
    pub mint_authority_bump: u8,
    pub amount: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MintToCpiPlanningBoundary {
    pub token_program: Pubkey,
    pub mint: Pubkey,
    pub recipient_token_account: Pubkey,
    pub mint_authority_pda: Pubkey,
    pub mint_authority_bump: u8,
    pub amount: u64,
    pub live_route_activation_enabled: bool,
    pub invoke_signed_from_process_instruction_enabled: bool,
}

pub fn plan_mint_to_cpi_boundary(
    program_id: &Pubkey,
    execution_plan: &crate::execution_plan::AtomicConsumeGatewayMintExecutionPlan,
    boundary: &MintToCpiBoundary<'_, '_>,
) -> Result<MintToCpiPlanningBoundary, ProgramError> {
    if execution_plan.live_route_activation_enabled
        || execution_plan.mint_to_invocation_from_process_instruction_enabled
        || execution_plan.amount == 0
        || boundary.amount == 0
        || boundary.amount != execution_plan.amount
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if boundary.accounts.token_program.key != &spl_token::id() {
        return Err(XxxlError::InvalidAccountOwner.into());
    }

    if boundary.accounts.mint.key.to_bytes() != execution_plan.mint {
        return Err(XxxlError::InvalidInstruction.into());
    }

    assert_gateway_mint_authority_pda(
        program_id,
        boundary.accounts.mint_authority_pda.key,
        boundary.mint_authority_bump,
    )?;

    build_mint_to_instruction(
        boundary.accounts.token_program.key,
        boundary.accounts.mint.key,
        boundary.accounts.recipient_token_account.key,
        boundary.accounts.mint_authority_pda.key,
        boundary.amount,
    )?;

    let signer_seeds = gateway_mint_authority_signer_seeds(&boundary.mint_authority_bump);

    if signer_seeds[0] != GATEWAY_MINT_AUTHORITY_SEED_0
        || signer_seeds[1] != GATEWAY_MINT_AUTHORITY_SEED_1
        || signer_seeds[2] != GATEWAY_MINT_AUTHORITY_SEED_2
        || signer_seeds[3] != core::slice::from_ref(&boundary.mint_authority_bump)
    {
        return Err(XxxlError::InvalidPda.into());
    }

    Ok(MintToCpiPlanningBoundary {
        token_program: *boundary.accounts.token_program.key,
        mint: *boundary.accounts.mint.key,
        recipient_token_account: *boundary.accounts.recipient_token_account.key,
        mint_authority_pda: *boundary.accounts.mint_authority_pda.key,
        mint_authority_bump: boundary.mint_authority_bump,
        amount: boundary.amount,
        live_route_activation_enabled: false,
        invoke_signed_from_process_instruction_enabled: false,
    })
}

#[cfg(all(
    feature = "phase-41k5-d2-production-path-test-gate",
    feature = "dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build",
    feature = "phase-41k6-b1c7-handler-integration-test-gate",
    feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build"
))]
pub fn spl_mint_to_cpi_execution_enabled() -> bool {
    let _phase_41k5_d2_gate_marker = "PHASE_41K5_D2_SPL_CPI_GATE_OPEN_AFTER_B1C7";
    true
}

#[cfg(not(all(
    feature = "phase-41k5-d2-production-path-test-gate",
    feature = "dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build",
    feature = "phase-41k6-b1c7-handler-integration-test-gate",
    feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build"
)))]
pub fn spl_mint_to_cpi_execution_enabled() -> bool {
    let _phase_41k5_d2_gate_marker = "PHASE_41K5_D2_SPL_CPI_GATE_CLOSED";
    false
}

pub fn guarded_mint_to_cpi_execution_gate_boundary(
    program_id: &Pubkey,
    execution_plan: &crate::execution_plan::AtomicConsumeGatewayMintExecutionPlan,
    planning_boundary: &MintToCpiPlanningBoundary,
    boundary: &MintToCpiBoundary<'_, '_>,
) -> Result<(), ProgramError> {
    if execution_plan.live_route_activation_enabled
        || execution_plan.mint_to_invocation_from_process_instruction_enabled
        || planning_boundary.live_route_activation_enabled
        || planning_boundary.invoke_signed_from_process_instruction_enabled
    {
        return Err(XxxlError::CpiBoundaryNotReady.into());
    }

    let expected_planning_boundary =
        plan_mint_to_cpi_boundary(program_id, execution_plan, boundary)?;

    if expected_planning_boundary != *planning_boundary {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if !spl_mint_to_cpi_execution_enabled() {
        return Err(XxxlError::CpiBoundaryNotReady.into());
    }

    mint_to_cpi_boundary(
        program_id,
        MintToCpiBoundary {
            accounts: MintToCpiAccounts {
                token_program: boundary.accounts.token_program,
                mint: boundary.accounts.mint,
                recipient_token_account: boundary.accounts.recipient_token_account,
                mint_authority_pda: boundary.accounts.mint_authority_pda,
            },
            mint_authority_bump: boundary.mint_authority_bump,
            amount: boundary.amount,
        },
    )
}

pub fn build_mint_to_instruction(
    token_program_id: &Pubkey,
    mint: &Pubkey,
    recipient_token_account: &Pubkey,
    mint_authority_pda: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    if amount == 0 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    spl_token::instruction::mint_to(
        token_program_id,
        mint,
        recipient_token_account,
        mint_authority_pda,
        &[],
        amount,
    )
}

pub fn gateway_mint_authority_signer_seeds(bump: &u8) -> [&[u8]; 4] {
    [
        GATEWAY_MINT_AUTHORITY_SEED_0,
        GATEWAY_MINT_AUTHORITY_SEED_1,
        GATEWAY_MINT_AUTHORITY_SEED_2,
        core::slice::from_ref(bump),
    ]
}

pub fn assert_gateway_mint_authority_pda(
    program_id: &Pubkey,
    mint_authority_pda: &Pubkey,
    mint_authority_bump: u8,
) -> Result<(), ProgramError> {
    let (expected_pda, expected_bump) = find_gateway_mint_authority(program_id);

    if mint_authority_pda != &expected_pda || mint_authority_bump != expected_bump {
        return Err(XxxlError::InvalidPda.into());
    }

    Ok(())
}

pub fn mint_to_cpi_boundary(
    program_id: &Pubkey,
    boundary: MintToCpiBoundary<'_, '_>,
) -> Result<(), ProgramError> {
    assert_gateway_mint_authority_pda(
        program_id,
        boundary.accounts.mint_authority_pda.key,
        boundary.mint_authority_bump,
    )?;

    let instruction = build_mint_to_instruction(
        boundary.accounts.token_program.key,
        boundary.accounts.mint.key,
        boundary.accounts.recipient_token_account.key,
        boundary.accounts.mint_authority_pda.key,
        boundary.amount,
    )?;

    let signer_seeds = gateway_mint_authority_signer_seeds(&boundary.mint_authority_bump);

    invoke_signed(
        &instruction,
        &[
            boundary.accounts.token_program.clone(),
            boundary.accounts.mint.clone(),
            boundary.accounts.recipient_token_account.clone(),
            boundary.accounts.mint_authority_pda.clone(),
        ],
        &[&signer_seeds],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
    use std::str::FromStr;

    const FIXTURE_PROGRAM_ID: &str = "11111111111111111111111111111111";

    struct CpiFixtureParams {
        program_id: Pubkey,
        token_program_key: Pubkey,
        mint_key: Pubkey,
        recipient_token_account_key: Pubkey,
        mint_authority_pda_key: Pubkey,
        mint_authority_bump: u8,
        boundary_amount: u64,
    }

    fn with_mint_to_cpi_boundary_fixture<T>(
        params: CpiFixtureParams,
        f: impl FnOnce(&MintToCpiBoundary<'_, '_>) -> T,
    ) -> T {
        let program_id = params.program_id;
        let token_program_key = params.token_program_key;
        let mint_key = params.mint_key;
        let recipient_token_account_key = params.recipient_token_account_key;
        let mint_authority_pda_key = params.mint_authority_pda_key;
        let mint_authority_bump = params.mint_authority_bump;
        let boundary_amount = params.boundary_amount;
        let mut token_program_lamports = 0;
        let mut mint_lamports = 0;
        let mut recipient_lamports = 0;
        let mut pda_lamports = 0;

        let mut token_program_data = [];
        let mut mint_data = [];
        let mut recipient_data = [];
        let mut pda_data = [];

        let token_program_owner = Pubkey::new_unique();

        let token_program = AccountInfo::new(
            &token_program_key,
            false,
            false,
            &mut token_program_lamports,
            &mut token_program_data,
            &token_program_owner,
            true,
            0,
        );
        let mint = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut mint_lamports,
            &mut mint_data,
            &token_program_key,
            false,
            0,
        );
        let recipient_token_account = AccountInfo::new(
            &recipient_token_account_key,
            false,
            true,
            &mut recipient_lamports,
            &mut recipient_data,
            &token_program_key,
            false,
            0,
        );
        let mint_authority_pda = AccountInfo::new(
            &mint_authority_pda_key,
            false,
            false,
            &mut pda_lamports,
            &mut pda_data,
            &program_id,
            false,
            0,
        );

        let boundary = MintToCpiBoundary {
            accounts: MintToCpiAccounts {
                token_program: &token_program,
                mint: &mint,
                recipient_token_account: &recipient_token_account,
                mint_authority_pda: &mint_authority_pda,
            },
            mint_authority_bump,
            amount: boundary_amount,
        };

        f(&boundary)
    }

    fn execution_plan_for_mint(
        mint: Pubkey,
    ) -> crate::execution_plan::AtomicConsumeGatewayMintExecutionPlan {
        crate::execution_plan::AtomicConsumeGatewayMintExecutionPlan {
            steps: crate::execution_plan::ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER,
            canonical_event_key: [0x44; 32],
            route_id: [0x11; 32],
            recipient: [0x55; 32],
            mint: mint.to_bytes(),
            amount: 1_000,
            consumed_slot: 77,
            source_chain_weight_bps: 10_000,
            live_route_activation_enabled: false,
            mint_to_invocation_from_process_instruction_enabled: false,
        }
    }

    fn assert_custom_error<T>(result: Result<T, ProgramError>, error: XxxlError) {
        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
    }

    #[test]
    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_when_gate_disabled() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let recipient_token_account_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        assert!(!spl_mint_to_cpi_execution_enabled());

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key,
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                let planning_boundary =
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
                        .expect("mint_to CPI planning boundary");

                assert_custom_error(
                    guarded_mint_to_cpi_execution_gate_boundary(
                        &program_id,
                        &execution_plan,
                        &planning_boundary,
                        boundary,
                    ),
                    XxxlError::CpiBoundaryNotReady,
                );
            },
        );
    }

    #[test]
    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_planning_boundary_mismatch() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let recipient_token_account_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key,
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                let mut planning_boundary =
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
                        .expect("mint_to CPI planning boundary");
                planning_boundary.amount = 999;

                assert_custom_error(
                    guarded_mint_to_cpi_execution_gate_boundary(
                        &program_id,
                        &execution_plan,
                        &planning_boundary,
                        boundary,
                    ),
                    XxxlError::InvalidInstruction,
                );
            },
        );
    }

    #[test]
    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_live_route_flag_before_cpi() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let recipient_token_account_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key,
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                let mut planning_boundary =
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
                        .expect("mint_to CPI planning boundary");
                planning_boundary.live_route_activation_enabled = true;

                assert_custom_error(
                    guarded_mint_to_cpi_execution_gate_boundary(
                        &program_id,
                        &execution_plan,
                        &planning_boundary,
                        boundary,
                    ),
                    XxxlError::CpiBoundaryNotReady,
                );
            },
        );
    }

    #[test]
    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_wrong_pda_before_gate() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);
        let fake_planning_boundary = MintToCpiPlanningBoundary {
            token_program: spl_token::id(),
            mint: mint_key,
            recipient_token_account: Pubkey::new_unique(),
            mint_authority_pda: Pubkey::new_unique(),
            mint_authority_bump: bump,
            amount: 1_000,
            live_route_activation_enabled: false,
            invoke_signed_from_process_instruction_enabled: false,
        };

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key: fake_planning_boundary.recipient_token_account,
                mint_authority_pda_key: Pubkey::new_unique(),
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                assert_custom_error(
                    guarded_mint_to_cpi_execution_gate_boundary(
                        &program_id,
                        &execution_plan,
                        &fake_planning_boundary,
                        boundary,
                    ),
                    XxxlError::InvalidPda,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_builds_plan_without_invoke_signed() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let recipient_token_account_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key,
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                let plan = plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
                    .expect("mint_to CPI planning boundary");

                assert_eq!(plan.token_program, spl_token::id());
                assert_eq!(plan.mint, mint_key);
                assert_eq!(plan.recipient_token_account, recipient_token_account_key);
                assert_eq!(plan.mint_authority_pda, pda);
                assert_eq!(plan.mint_authority_bump, bump);
                assert_eq!(plan.amount, 1_000);
                assert!(!plan.live_route_activation_enabled);
                assert!(!plan.invoke_signed_from_process_instruction_enabled);
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_amount_mismatch() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 999,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidInstruction,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_zero_boundary_amount() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 0,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidInstruction,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_live_route_flag() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let mut execution_plan = execution_plan_for_mint(mint_key);
        execution_plan.live_route_activation_enabled = true;

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidInstruction,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_mint_to_flag() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let mut execution_plan = execution_plan_for_mint(mint_key);
        execution_plan.mint_to_invocation_from_process_instruction_enabled = true;

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidInstruction,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_wrong_token_program() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: Pubkey::new_unique(),
                mint_key,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidAccountOwner,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_wrong_mint_mapping() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let boundary_mint = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(Pubkey::new_unique());

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key: boundary_mint,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: pda,
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidInstruction,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_wrong_pda() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: Pubkey::new_unique(),
                mint_authority_bump: bump,
                boundary_amount: 1_000,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidPda,
                );
            },
        );
    }

    #[test]
    fn mint_to_cpi_planning_boundary_rejects_wrong_bump() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let mint_key = Pubkey::new_unique();
        let execution_plan = execution_plan_for_mint(mint_key);

        with_mint_to_cpi_boundary_fixture(
            CpiFixtureParams {
                program_id,
                token_program_key: spl_token::id(),
                mint_key,
                recipient_token_account_key: Pubkey::new_unique(),
                mint_authority_pda_key: pda,
                mint_authority_bump: bump.wrapping_add(1),
                boundary_amount: 1_000,
            },
            |boundary| {
                assert_custom_error(
                    plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
                    XxxlError::InvalidPda,
                );
            },
        );
    }

    #[test]
    fn mint_to_instruction_uses_spl_token_program_and_expected_accounts() {
        let mint = Pubkey::new_unique();
        let recipient_token_account = Pubkey::new_unique();
        let mint_authority_pda = Pubkey::new_unique();

        let instruction = build_mint_to_instruction(
            &spl_token::id(),
            &mint,
            &recipient_token_account,
            &mint_authority_pda,
            1_000,
        )
        .expect("valid mint_to instruction");

        assert_eq!(instruction.program_id, spl_token::id());
        assert_eq!(instruction.accounts.len(), 3);
        assert_eq!(instruction.accounts[0].pubkey, mint);
        assert_eq!(instruction.accounts[1].pubkey, recipient_token_account);
        assert_eq!(instruction.accounts[2].pubkey, mint_authority_pda);
        assert!(instruction.accounts[0].is_writable);
        assert!(instruction.accounts[1].is_writable);
        assert!(instruction.accounts[2].is_signer);
    }

    #[test]
    fn mint_to_instruction_rejects_zero_amount() {
        let result = build_mint_to_instruction(
            &spl_token::id(),
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            0,
        );

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidInstruction as u32)
        );
    }

    #[test]
    fn signer_seeds_are_exact_and_include_bump() {
        let bump = 201;
        let seeds = gateway_mint_authority_signer_seeds(&bump);

        assert_eq!(seeds[0], b"xxxl");
        assert_eq!(seeds[1], b"gateway-mint-authority");
        assert_eq!(seeds[2], b"v1");
        assert_eq!(seeds[3], &[201]);
    }

    #[test]
    fn gateway_mint_authority_pda_accepts_real_derived_fixture() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);

        assert_gateway_mint_authority_pda(&program_id, &pda, bump)
            .expect("valid gateway mint authority PDA");
    }

    #[test]
    fn gateway_mint_authority_pda_rejects_wrong_pda() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);
        let wrong_pda = Pubkey::new_unique();

        let result = assert_gateway_mint_authority_pda(&program_id, &wrong_pda, bump);

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidPda as u32)
        );
    }

    #[test]
    fn gateway_mint_authority_pda_rejects_wrong_bump() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);

        let result = assert_gateway_mint_authority_pda(&program_id, &pda, bump.wrapping_add(1));

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidPda as u32)
        );
    }

    #[test]
    fn mint_to_boundary_rejects_wrong_pda_before_invoke_signed() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);

        let token_program_key = spl_token::id();
        let mint_key = Pubkey::new_unique();
        let recipient_key = Pubkey::new_unique();
        let wrong_pda_key = Pubkey::new_unique();

        let mut token_program_lamports = 0;
        let mut mint_lamports = 0;
        let mut recipient_lamports = 0;
        let mut pda_lamports = 0;

        let mut token_program_data = [];
        let mut mint_data = [];
        let mut recipient_data = [];
        let mut pda_data = [];

        let token_program_owner = Pubkey::new_unique();
        let token_program = AccountInfo::new(
            &token_program_key,
            false,
            false,
            &mut token_program_lamports,
            &mut token_program_data,
            &token_program_owner,
            true,
            0,
        );
        let mint = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut mint_lamports,
            &mut mint_data,
            &token_program_key,
            false,
            0,
        );
        let recipient_token_account = AccountInfo::new(
            &recipient_key,
            false,
            true,
            &mut recipient_lamports,
            &mut recipient_data,
            &token_program_key,
            false,
            0,
        );
        let mint_authority_pda = AccountInfo::new(
            &wrong_pda_key,
            false,
            false,
            &mut pda_lamports,
            &mut pda_data,
            &program_id,
            false,
            0,
        );

        let boundary = MintToCpiBoundary {
            accounts: MintToCpiAccounts {
                token_program: &token_program,
                mint: &mint,
                recipient_token_account: &recipient_token_account,
                mint_authority_pda: &mint_authority_pda,
            },
            mint_authority_bump: bump,
            amount: 1,
        };

        let result = mint_to_cpi_boundary(&program_id, boundary);

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidPda as u32)
        );
    }
}

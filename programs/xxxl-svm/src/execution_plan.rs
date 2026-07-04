use solana_program::program_error::ProgramError;

// LEGACY / PRE-41K.4:
// This module still represents an older planning scaffold that assumes an
// already-initialized program-owned processed-event account.
// It is not valid Phase 41K.4 atomic replay marking semantics.
// 41K.4 must use a separate SystemOwnedEmpty -> InitializedConsumed boundary.

#[cfg(test)]
use crate::state::mark_processed_event_consumed_legacy_planning_only;

use crate::{
    error::XxxlError,
    instruction::ConsumeGatewayMintArgs,
    processor::PreparedConsumeGatewayMintCpi,
    state::{credit_recipient_balance, ProcessedEventAccountView, RecipientBalanceAccountView},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtomicExecutionStep {
    ValidateAndPrepareCpi,
    MarkProcessedEventConsumed,
    CreditRecipientBalance,
    KeepLiveRouteDisabled,
}

pub const ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER: [AtomicExecutionStep; 4] = [
    AtomicExecutionStep::ValidateAndPrepareCpi,
    AtomicExecutionStep::MarkProcessedEventConsumed,
    AtomicExecutionStep::CreditRecipientBalance,
    AtomicExecutionStep::KeepLiveRouteDisabled,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtomicConsumeGatewayMintExecutionPlan {
    pub steps: [AtomicExecutionStep; 4],
    pub canonical_event_key: [u8; 32],
    pub route_id: [u8; 32],
    pub recipient: [u8; 32],
    pub mint: [u8; 32],
    pub amount: u64,
    pub consumed_slot: u64,
    pub source_chain_weight_bps: u16,
    pub live_route_activation_enabled: bool,
    pub mint_to_invocation_from_process_instruction_enabled: bool,
}

pub fn assert_atomic_consume_gateway_mint_step_order(
    steps: &[AtomicExecutionStep],
) -> Result<(), ProgramError> {
    if steps != ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER.as_slice() {
        return Err(XxxlError::InvalidInstruction.into());
    }

    Ok(())
}

pub fn build_atomic_consume_gateway_mint_execution_plan(
    args: &ConsumeGatewayMintArgs,
    prepared: &PreparedConsumeGatewayMintCpi<'_, '_>,
    consumed_slot: u64,
) -> Result<AtomicConsumeGatewayMintExecutionPlan, ProgramError> {
    if args.amount == 0
        || args.amount > u64::MAX as u128
        || prepared.boundary.amount != args.amount as u64
        || prepared.source_chain_weight_bps != args.source_chain_weight_bps
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    Ok(AtomicConsumeGatewayMintExecutionPlan {
        steps: ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER,
        canonical_event_key: args.canonical_event_key,
        route_id: args.route_id,
        recipient: args.recipient,
        mint: args.mint_id,
        amount: args.amount as u64,
        consumed_slot,
        source_chain_weight_bps: args.source_chain_weight_bps,
        live_route_activation_enabled: false,
        mint_to_invocation_from_process_instruction_enabled: false,
    })
}

#[cfg(test)]
#[allow(deprecated)]
pub fn apply_processed_event_mutation_boundary(
    processed_event_data: &mut [u8],
    execution_plan: &AtomicConsumeGatewayMintExecutionPlan,
) -> Result<(), ProgramError> {
    assert_atomic_consume_gateway_mint_step_order(&execution_plan.steps)?;

    if execution_plan.live_route_activation_enabled
        || execution_plan.mint_to_invocation_from_process_instruction_enabled
        || execution_plan.amount == 0
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    mark_processed_event_consumed_legacy_planning_only(
        processed_event_data,
        execution_plan.canonical_event_key,
        execution_plan.route_id,
        execution_plan.recipient,
        execution_plan.amount as u128,
        execution_plan.consumed_slot,
    )
}

pub fn apply_recipient_balance_mutation_boundary(
    recipient_balance_data: &mut [u8],
    execution_plan: &AtomicConsumeGatewayMintExecutionPlan,
) -> Result<u128, ProgramError> {
    assert_atomic_consume_gateway_mint_step_order(&execution_plan.steps)?;

    if execution_plan.live_route_activation_enabled
        || execution_plan.mint_to_invocation_from_process_instruction_enabled
        || execution_plan.amount == 0
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    credit_recipient_balance(
        recipient_balance_data,
        execution_plan.recipient,
        execution_plan.mint,
        execution_plan.amount as u128,
        execution_plan.canonical_event_key,
    )
}

#[cfg(test)]
pub fn apply_atomic_state_mutation_composition_boundary(
    processed_event_data: &mut [u8],
    recipient_balance_data: &mut [u8],
    execution_plan: &AtomicConsumeGatewayMintExecutionPlan,
) -> Result<u128, ProgramError> {
    assert_atomic_consume_gateway_mint_step_order(&execution_plan.steps)?;

    if execution_plan.live_route_activation_enabled
        || execution_plan.mint_to_invocation_from_process_instruction_enabled
        || execution_plan.amount == 0
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    {
        let processed_event = ProcessedEventAccountView::new(processed_event_data)?;

        if processed_event.consumed()
            || processed_event.canonical_event_key() != execution_plan.canonical_event_key
            || processed_event.route_id() != execution_plan.route_id
            || processed_event.recipient() != execution_plan.recipient
        {
            return Err(XxxlError::InvalidInstruction.into());
        }
    }

    {
        let recipient_balance = RecipientBalanceAccountView::new(recipient_balance_data)?;

        if recipient_balance.owner() != execution_plan.recipient
            || recipient_balance.mint() != execution_plan.mint
        {
            return Err(XxxlError::InvalidRecipientAta.into());
        }

        recipient_balance
            .balance()
            .checked_add(execution_plan.amount as u128)
            .ok_or(XxxlError::InvalidInstruction)?;
    }

    apply_processed_event_mutation_boundary(processed_event_data, execution_plan)?;
    apply_recipient_balance_mutation_boundary(recipient_balance_data, execution_plan)
}

#[cfg(test)]
#[allow(deprecated)]
pub fn apply_atomic_state_mutations_fixture(
    processed_event_data: &mut [u8],
    recipient_balance_data: &mut [u8],
    args: &ConsumeGatewayMintArgs,
    consumed_slot: u64,
) -> Result<u128, ProgramError> {
    if args.amount == 0 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    {
        let processed_event = ProcessedEventAccountView::new(processed_event_data)?;

        if processed_event.consumed()
            || processed_event.canonical_event_key() != args.canonical_event_key
            || processed_event.route_id() != args.route_id
            || processed_event.recipient() != args.recipient
        {
            return Err(XxxlError::InvalidInstruction.into());
        }
    }

    {
        let recipient_balance = RecipientBalanceAccountView::new(recipient_balance_data)?;

        if recipient_balance.owner() != args.recipient || recipient_balance.mint() != args.mint_id {
            return Err(XxxlError::InvalidRecipientAta.into());
        }

        recipient_balance
            .balance()
            .checked_add(args.amount)
            .ok_or(XxxlError::InvalidInstruction)?;
    }

    mark_processed_event_consumed_legacy_planning_only(
        processed_event_data,
        args.canonical_event_key,
        args.route_id,
        args.recipient,
        args.amount,
        consumed_slot,
    )?;

    credit_recipient_balance(
        recipient_balance_data,
        args.recipient,
        args.mint_id,
        args.amount,
        args.canonical_event_key,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cpi::{MintToCpiAccounts, MintToCpiBoundary},
        instruction::CONSUME_GATEWAY_MINT_INSTRUCTION_LEN,
        processor::PreparedConsumeGatewayMintCpi,
        state::{
            GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
            PROCESSED_EVENT_ACCOUNT_LEN, RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
            RECIPIENT_BALANCE_ACCOUNT_LEN, RUNTIME_LAYOUT_VERSION,
        },
    };
    use solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

    #[test]
    fn atomic_step_order_is_fixed() {
        assert_atomic_consume_gateway_mint_step_order(
            ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER.as_slice(),
        )
        .expect("valid atomic order");
    }

    #[test]
    fn atomic_step_order_rejects_reordered_steps() {
        let reordered = [
            AtomicExecutionStep::ValidateAndPrepareCpi,
            AtomicExecutionStep::CreditRecipientBalance,
            AtomicExecutionStep::MarkProcessedEventConsumed,
            AtomicExecutionStep::KeepLiveRouteDisabled,
        ];

        assert_custom_error(
            assert_atomic_consume_gateway_mint_step_order(&reordered),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn atomic_execution_plan_is_built_from_prepared_cpi_boundary() {
        let args = valid_args();

        let token_program_key = spl_token::id();
        let mint_key = Pubkey::new_from_array(args.mint_id);
        let recipient_token_account_key = Pubkey::new_unique();
        let mint_authority_pda_key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();

        let mut token_program_lamports = 0;
        let mut mint_lamports = 0;
        let mut recipient_lamports = 0;
        let mut pda_lamports = 0;

        let mut token_program_data = [];
        let mut mint_data = [];
        let mut recipient_data = [];
        let mut pda_data = [];

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
            &owner,
            false,
            0,
        );

        let prepared = PreparedConsumeGatewayMintCpi {
            boundary: MintToCpiBoundary {
                accounts: MintToCpiAccounts {
                    token_program: &token_program,
                    mint: &mint,
                    recipient_token_account: &recipient_token_account,
                    mint_authority_pda: &mint_authority_pda,
                },
                mint_authority_bump: 201,
                amount: 1_000,
            },
            mint_decimals: 18,
            source_chain_weight_bps: 10_000,
        };

        let plan =
            build_atomic_consume_gateway_mint_execution_plan(&args, &prepared, 77).expect("plan");

        assert_eq!(plan.steps, ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER);
        assert_eq!(plan.amount, 1_000);
        assert_eq!(plan.consumed_slot, 77);
        assert_eq!(plan.canonical_event_key, args.canonical_event_key);
        assert_eq!(plan.recipient, args.recipient);
        assert_eq!(plan.mint, args.mint_id);
        assert!(!plan.live_route_activation_enabled);
        assert!(!plan.mint_to_invocation_from_process_instruction_enabled);
    }

    #[test]
    fn atomic_execution_plan_rejects_amount_mismatch() {
        let args = valid_args();

        let token_program_key = spl_token::id();
        let mint_key = Pubkey::new_from_array(args.mint_id);
        let recipient_token_account_key = Pubkey::new_unique();
        let mint_authority_pda_key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();

        let mut token_program_lamports = 0;
        let mut mint_lamports = 0;
        let mut recipient_lamports = 0;
        let mut pda_lamports = 0;

        let mut token_program_data = [];
        let mut mint_data = [];
        let mut recipient_data = [];
        let mut pda_data = [];

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
            &owner,
            false,
            0,
        );

        let prepared = PreparedConsumeGatewayMintCpi {
            boundary: MintToCpiBoundary {
                accounts: MintToCpiAccounts {
                    token_program: &token_program,
                    mint: &mint,
                    recipient_token_account: &recipient_token_account,
                    mint_authority_pda: &mint_authority_pda,
                },
                mint_authority_bump: 201,
                amount: 999,
            },
            mint_decimals: 18,
            source_chain_weight_bps: 10_000,
        };

        assert_custom_error(
            build_atomic_consume_gateway_mint_execution_plan(&args, &prepared, 77),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn processed_event_mutation_boundary_marks_event_from_execution_plan() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);

        apply_processed_event_mutation_boundary(&mut processed_event_data, &plan)
            .expect("processed event mutation boundary");

        let processed_event =
            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");

        assert!(processed_event.consumed());
        assert_eq!(processed_event.consumed_amount(), 1_000);
        assert_eq!(read_u64_le(&processed_event_data, 128), 77);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_replay_without_changes() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, true);
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_wrong_event_key_without_changes() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        processed_event_data[16] ^= 0xff;
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_wrong_route_without_changes() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        processed_event_data[48] ^= 0xff;
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_wrong_recipient_without_changes() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        processed_event_data[80] ^= 0xff;
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_zero_amount_plan_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.amount = 0;

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_live_route_flag_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.live_route_activation_enabled = true;

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_mint_to_flag_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.mint_to_invocation_from_process_instruction_enabled = true;

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn processed_event_mutation_boundary_rejects_reordered_steps_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.steps = [
            AtomicExecutionStep::ValidateAndPrepareCpi,
            AtomicExecutionStep::CreditRecipientBalance,
            AtomicExecutionStep::MarkProcessedEventConsumed,
            AtomicExecutionStep::KeepLiveRouteDisabled,
        ];

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let before = processed_event_data.clone();

        assert_custom_error(
            apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, before);
    }

    #[test]
    fn recipient_balance_mutation_boundary_credits_balance_from_execution_plan() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);

        let next_balance =
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan)
                .expect("recipient balance mutation boundary");

        let recipient_balance =
            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");

        assert_eq!(next_balance, 1_200);
        assert_eq!(recipient_balance.balance(), 1_200);
        assert_eq!(
            read_fixed_32(&recipient_balance_data, 96),
            args.canonical_event_key
        );
    }

    #[test]
    fn recipient_balance_mutation_boundary_rejects_wrong_owner_without_changes() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        recipient_balance_data[16] ^= 0xff;
        let before = recipient_balance_data.clone();

        assert_custom_error(
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan),
            XxxlError::InvalidRecipientAta,
        );

        assert_eq!(recipient_balance_data, before);
    }

    #[test]
    fn recipient_balance_mutation_boundary_rejects_wrong_mint_without_changes() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        recipient_balance_data[48] ^= 0xff;
        let before = recipient_balance_data.clone();

        assert_custom_error(
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan),
            XxxlError::InvalidRecipientAta,
        );

        assert_eq!(recipient_balance_data, before);
    }

    #[test]
    fn recipient_balance_mutation_boundary_rejects_zero_amount_plan_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.amount = 0;

        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let before = recipient_balance_data.clone();

        assert_custom_error(
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(recipient_balance_data, before);
    }

    #[test]
    fn recipient_balance_mutation_boundary_rejects_balance_overflow_without_changes() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut recipient_balance_data = valid_recipient_balance_data(&args, u128::MAX);
        let before = recipient_balance_data.clone();

        assert_custom_error(
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(recipient_balance_data, before);
    }

    #[test]
    fn recipient_balance_mutation_boundary_rejects_live_route_flag_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.live_route_activation_enabled = true;

        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let before = recipient_balance_data.clone();

        assert_custom_error(
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(recipient_balance_data, before);
    }

    #[test]
    fn recipient_balance_mutation_boundary_rejects_mint_to_flag_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.mint_to_invocation_from_process_instruction_enabled = true;

        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let before = recipient_balance_data.clone();

        assert_custom_error(
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(recipient_balance_data, before);
    }

    #[test]
    fn recipient_balance_mutation_boundary_rejects_reordered_steps_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.steps = [
            AtomicExecutionStep::ValidateAndPrepareCpi,
            AtomicExecutionStep::CreditRecipientBalance,
            AtomicExecutionStep::MarkProcessedEventConsumed,
            AtomicExecutionStep::KeepLiveRouteDisabled,
        ];

        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let before = recipient_balance_data.clone();

        assert_custom_error(
            apply_recipient_balance_mutation_boundary(&mut recipient_balance_data, &plan),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(recipient_balance_data, before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_marks_event_and_credits_balance() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);

        let next_balance = apply_atomic_state_mutation_composition_boundary(
            &mut processed_event_data,
            &mut recipient_balance_data,
            &plan,
        )
        .expect("atomic composition boundary");

        let processed_event =
            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");
        let recipient_balance =
            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");

        assert_eq!(next_balance, 1_200);
        assert!(processed_event.consumed());
        assert_eq!(processed_event.consumed_amount(), 1_000);
        assert_eq!(read_u64_le(&processed_event_data, 128), 77);
        assert_eq!(recipient_balance.balance(), 1_200);
        assert_eq!(
            read_fixed_32(&recipient_balance_data, 96),
            args.canonical_event_key
        );
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_recipient_overflow_before_event_mark() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, u128::MAX);
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_wrong_recipient_owner_before_event_mark()
    {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        recipient_balance_data[16] ^= 0xff;
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidRecipientAta,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_wrong_mint_before_event_mark() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        recipient_balance_data[48] ^= 0xff;
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidRecipientAta,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_replay_before_balance_credit() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, true);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_wrong_event_key_before_balance_credit() {
        let args = valid_args();
        let plan = valid_execution_plan();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        processed_event_data[16] ^= 0xff;
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_zero_amount_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.amount = 0;

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_live_route_flag_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.live_route_activation_enabled = true;

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_mint_to_flag_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.mint_to_invocation_from_process_instruction_enabled = true;

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_composition_boundary_rejects_reordered_steps_without_changes() {
        let args = valid_args();
        let mut plan = valid_execution_plan();
        plan.steps = [
            AtomicExecutionStep::ValidateAndPrepareCpi,
            AtomicExecutionStep::CreditRecipientBalance,
            AtomicExecutionStep::MarkProcessedEventConsumed,
            AtomicExecutionStep::KeepLiveRouteDisabled,
        ];

        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        let processed_before = processed_event_data.clone();
        let balance_before = recipient_balance_data.clone();

        assert_custom_error(
            apply_atomic_state_mutation_composition_boundary(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &plan,
            ),
            XxxlError::InvalidInstruction,
        );

        assert_eq!(processed_event_data, processed_before);
        assert_eq!(recipient_balance_data, balance_before);
    }

    #[test]
    fn atomic_state_mutation_fixture_marks_processed_and_credits_balance() {
        let args = valid_args();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);

        let next_balance = apply_atomic_state_mutations_fixture(
            &mut processed_event_data,
            &mut recipient_balance_data,
            &args,
            77,
        )
        .expect("atomic state mutations");

        let processed_event =
            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");
        let recipient_balance =
            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");

        assert_eq!(next_balance, 1_200);
        assert!(processed_event.consumed());
        assert_eq!(processed_event.consumed_amount(), 1_000);
        assert_eq!(recipient_balance.balance(), 1_200);
        assert_eq!(
            read_fixed_32(&recipient_balance_data, 96),
            args.canonical_event_key
        );
    }

    #[test]
    fn atomic_state_mutation_fixture_rejects_replay_before_credit() {
        let args = valid_args();
        let mut processed_event_data = valid_processed_event_data(&args, true);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);

        assert_custom_error(
            apply_atomic_state_mutations_fixture(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &args,
                77,
            ),
            XxxlError::InvalidInstruction,
        );

        let recipient_balance =
            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");

        assert_eq!(recipient_balance.balance(), 200);
    }

    #[test]
    fn atomic_state_mutation_fixture_rejects_balance_overflow_before_marking_processed() {
        let args = valid_args();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, u128::MAX);

        assert_custom_error(
            apply_atomic_state_mutations_fixture(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &args,
                77,
            ),
            XxxlError::InvalidInstruction,
        );

        let processed_event =
            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");

        assert!(!processed_event.consumed());
    }

    #[test]
    fn atomic_state_mutation_fixture_rejects_wrong_recipient_balance_owner() {
        let args = valid_args();
        let mut processed_event_data = valid_processed_event_data(&args, false);
        let mut recipient_balance_data = valid_recipient_balance_data(&args, 200);
        recipient_balance_data[16] ^= 0xff;

        assert_custom_error(
            apply_atomic_state_mutations_fixture(
                &mut processed_event_data,
                &mut recipient_balance_data,
                &args,
                77,
            ),
            XxxlError::InvalidRecipientAta,
        );
    }

    fn valid_args() -> ConsumeGatewayMintArgs {
        ConsumeGatewayMintArgs {
            raw: [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN],
            account_meta_count: 9,
            route_account_index: 1,
            guardian_set_account_index: 2,
            mint_state_account_index: 0,
            processed_event_account_index: 3,
            recipient_balance_account_index: 4,
            route_id: [0x11; 32],
            guardian_set_id: [0x22; 32],
            mint_id: [0x33; 32],
            canonical_event_key: [0x44; 32],
            recipient: [0x55; 32],
            amount: 1_000,
            source_chain_id: 1,
            source_chain_weight_bps: 10_000,
        }
    }

    fn valid_execution_plan() -> AtomicConsumeGatewayMintExecutionPlan {
        let args = valid_args();

        AtomicConsumeGatewayMintExecutionPlan {
            steps: ATOMIC_CONSUME_GATEWAY_MINT_STEP_ORDER,
            canonical_event_key: args.canonical_event_key,
            route_id: args.route_id,
            recipient: args.recipient,
            mint: args.mint_id,
            amount: args.amount as u64,
            consumed_slot: 77,
            source_chain_weight_bps: args.source_chain_weight_bps,
            live_route_activation_enabled: false,
            mint_to_invocation_from_process_instruction_enabled: false,
        }
    }

    fn valid_processed_event_data(args: &ConsumeGatewayMintArgs, consumed: bool) -> Vec<u8> {
        let mut data = account_data(
            PROCESSED_EVENT_ACCOUNT_LEN,
            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
        );

        data[10] = if consumed { 1 } else { 0 };
        data[16..48].copy_from_slice(&args.canonical_event_key);
        data[48..80].copy_from_slice(&args.route_id);
        data[80..112].copy_from_slice(&args.recipient);

        data
    }

    fn valid_recipient_balance_data(args: &ConsumeGatewayMintArgs, balance: u128) -> Vec<u8> {
        let mut data = account_data(
            RECIPIENT_BALANCE_ACCOUNT_LEN,
            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
        );

        data[16..48].copy_from_slice(&args.recipient);
        data[48..80].copy_from_slice(&args.mint_id);
        data[80..96].copy_from_slice(&balance.to_le_bytes());

        data
    }

    fn account_data(len: usize, discriminator: [u8; 8]) -> Vec<u8> {
        let mut data = vec![0u8; len];

        data[0..8].copy_from_slice(&discriminator);
        data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());

        data
    }

    fn read_u64_le(input: &[u8], offset: usize) -> u64 {
        u64::from_le_bytes([
            input[offset],
            input[offset + 1],
            input[offset + 2],
            input[offset + 3],
            input[offset + 4],
            input[offset + 5],
            input[offset + 6],
            input[offset + 7],
        ])
    }

    fn read_fixed_32(input: &[u8], offset: usize) -> [u8; 32] {
        let mut output = [0u8; 32];
        output.copy_from_slice(&input[offset..offset + 32]);
        output
    }

    fn assert_custom_error<T>(result: Result<T, ProgramError>, error: XxxlError) {
        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
    }

    #[test]
    fn fixture_does_not_depend_on_gateway_config_account_layout() {
        assert_eq!(GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR.len(), 8);
    }
}

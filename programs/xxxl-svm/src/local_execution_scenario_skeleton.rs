use crate::{
    account_validation_skeleton::expected_account_metas_for_tag,
    instruction_payload_skeleton::{
        ConsumeGatewayMintPayloadBoundarySkeleton, InitializeGatewayConfigPayloadSkeleton,
        InitializeGuardianSetHeaderPayloadSkeleton, InitializeMintStatePayloadSkeleton,
    },
    local_execution_plan_skeleton::{
        plan_local_execution_skeleton, XxxlLocalExecutionContextSkeleton,
        XxxlLocalExecutionPlanSkeleton, XxxlLocalExecutionPlanSkeletonError,
        XxxlLocalExecutionPlanSkeletonInput,
    },
    state_account_layout_skeleton::{
        GatewayConfigAccountLayoutSkeleton, GuardianSetHeaderAccountLayoutSkeleton,
        MintStateAccountLayoutSkeleton, ProcessedEventAccountLayoutSkeleton,
    },
    state_instruction_skeleton::XxxlGatewayInstructionTag,
    typed_instruction_skeleton::{
        encode_typed_instruction_skeleton, XxxlTypedInstructionSkeleton,
    },
};
use solana_program::pubkey::Pubkey;

pub const XXXL_LOCAL_EXECUTION_SCENARIO_SKELETON_STATUS: &str =
    "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalExecutionScenarioSkeletonError {
    Plan(XxxlLocalExecutionPlanSkeletonError),
    UnexpectedPlanVariant,
}

impl From<XxxlLocalExecutionPlanSkeletonError> for XxxlLocalExecutionScenarioSkeletonError {
    fn from(value: XxxlLocalExecutionPlanSkeletonError) -> Self {
        Self::Plan(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlLocalExecutionScenarioSkeletonInput {
    pub route_id: [u8; 32],
    pub source_chain_id: u64,
    pub guardian_set_id: u64,
    pub gateway_config_bump: u8,
    pub guardian_set_bump: u8,
    pub mint_state_bump: u8,
    pub processed_event_bump: u8,
    pub gateway_mint_authority_pda: Pubkey,
    pub mint: Pubkey,
    pub mint_authority_pda: Pubkey,
    pub mint_authority_bump: u8,
    pub token_program: Pubkey,
    pub decimals: u8,
    pub guardian_threshold: u8,
    pub guardian_count: u8,
    pub guardian_status: u8,
    pub initial_total_minted: u128,
    pub mint_state_is_active: bool,
    pub message_hash: [u8; 32],
    pub canonical_event_key: [u8; 32],
    pub mint_amount: u128,
    pub source_burn_tx_hash: [u8; 32],
    pub source_burn_event_index: u32,
    pub recipient_hash: [u8; 32],
    pub processed_at_slot: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalExecutionScenarioSkeletonOutput {
    pub gateway_config: GatewayConfigAccountLayoutSkeleton,
    pub guardian_set: GuardianSetHeaderAccountLayoutSkeleton,
    pub initial_mint_state: MintStateAccountLayoutSkeleton,
    pub next_mint_state: MintStateAccountLayoutSkeleton,
    pub processed_event: ProcessedEventAccountLayoutSkeleton,
}

pub fn run_local_execution_scenario_skeleton(
    input: XxxlLocalExecutionScenarioSkeletonInput,
) -> Result<XxxlLocalExecutionScenarioSkeletonOutput, XxxlLocalExecutionScenarioSkeletonError> {
    let gateway_config_instruction =
        XxxlTypedInstructionSkeleton::InitializeGatewayConfig(
            InitializeGatewayConfigPayloadSkeleton {
                route_id: input.route_id,
                source_chain_id: input.source_chain_id,
                guardian_set_id: input.guardian_set_id,
                is_active: true,
            },
        );

    let gateway_config_data = encode_typed_instruction_skeleton(gateway_config_instruction);
    let gateway_config_metas =
        expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig);

    let gateway_config_plan = plan_local_execution_skeleton(
        XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &gateway_config_data,
            account_metas: &gateway_config_metas,
            context: XxxlLocalExecutionContextSkeleton::InitializeGatewayConfig {
                bump: input.gateway_config_bump,
                gateway_mint_authority_pda: input.gateway_mint_authority_pda,
            },
        },
    )?;

    let gateway_config = match gateway_config_plan {
        XxxlLocalExecutionPlanSkeleton::InitializeGatewayConfig(output) => output.account,
        _ => return Err(XxxlLocalExecutionScenarioSkeletonError::UnexpectedPlanVariant),
    };

    let guardian_set_instruction =
        XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(
            InitializeGuardianSetHeaderPayloadSkeleton {
                guardian_set_id: input.guardian_set_id,
                threshold: input.guardian_threshold,
                guardian_count: input.guardian_count,
            },
        );

    let guardian_set_data = encode_typed_instruction_skeleton(guardian_set_instruction);
    let guardian_set_metas =
        expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGuardianSet);

    let guardian_set_plan = plan_local_execution_skeleton(
        XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &guardian_set_data,
            account_metas: &guardian_set_metas,
            context: XxxlLocalExecutionContextSkeleton::InitializeGuardianSet {
                bump: input.guardian_set_bump,
                status: input.guardian_status,
            },
        },
    )?;

    let guardian_set = match guardian_set_plan {
        XxxlLocalExecutionPlanSkeleton::InitializeGuardianSet(output) => output.account,
        _ => return Err(XxxlLocalExecutionScenarioSkeletonError::UnexpectedPlanVariant),
    };

    let mint_state_instruction = XxxlTypedInstructionSkeleton::InitializeMintState(
        InitializeMintStatePayloadSkeleton {
            mint: input.mint,
            decimals: input.decimals,
        },
    );

    let mint_state_data = encode_typed_instruction_skeleton(mint_state_instruction);
    let mint_state_metas =
        expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeMintState);

    let mint_state_plan = plan_local_execution_skeleton(
        XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &mint_state_data,
            account_metas: &mint_state_metas,
            context: XxxlLocalExecutionContextSkeleton::InitializeMintState {
                bump: input.mint_state_bump,
                mint_authority_pda: input.mint_authority_pda,
                mint_authority_bump: input.mint_authority_bump,
                token_program: input.token_program,
                total_minted: input.initial_total_minted,
                is_active: input.mint_state_is_active,
            },
        },
    )?;

    let initial_mint_state = match mint_state_plan {
        XxxlLocalExecutionPlanSkeleton::InitializeMintState(output) => output.account,
        _ => return Err(XxxlLocalExecutionScenarioSkeletonError::UnexpectedPlanVariant),
    };

    let consume_instruction =
        XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
            ConsumeGatewayMintPayloadBoundarySkeleton {
                message_hash: input.message_hash,
                canonical_event_key: input.canonical_event_key,
                mint_amount: input.mint_amount,
            },
        );

    let consume_data = encode_typed_instruction_skeleton(consume_instruction);
    let consume_metas =
        expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

    let consume_plan = plan_local_execution_skeleton(
        XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &consume_data,
            account_metas: &consume_metas,
            context: XxxlLocalExecutionContextSkeleton::ConsumeGatewayMint {
                current_mint_state: initial_mint_state,
                processed_event_bump: input.processed_event_bump,
                source_burn_tx_hash: input.source_burn_tx_hash,
                source_burn_event_index: input.source_burn_event_index,
                recipient_hash: input.recipient_hash,
                processed_at_slot: input.processed_at_slot,
            },
        },
    )?;

    let (next_mint_state, processed_event) = match consume_plan {
        XxxlLocalExecutionPlanSkeleton::ConsumeGatewayMint(output) => (
            output.state_transition_output.next_mint_state,
            output.state_transition_output.processed_event,
        ),
        _ => return Err(XxxlLocalExecutionScenarioSkeletonError::UnexpectedPlanVariant),
    };

    Ok(XxxlLocalExecutionScenarioSkeletonOutput {
        gateway_config,
        guardian_set,
        initial_mint_state,
        next_mint_state,
        processed_event,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> XxxlLocalExecutionScenarioSkeletonInput {
        XxxlLocalExecutionScenarioSkeletonInput {
            route_id: [0x11; 32],
            source_chain_id: 1,
            guardian_set_id: 7,
            gateway_config_bump: 252,
            guardian_set_bump: 251,
            mint_state_bump: 250,
            processed_event_bump: 249,
            gateway_mint_authority_pda: Pubkey::new_unique(),
            mint: Pubkey::new_unique(),
            mint_authority_pda: Pubkey::new_unique(),
            mint_authority_bump: 252,
            token_program: Pubkey::new_unique(),
            decimals: 9,
            guardian_threshold: 2,
            guardian_count: 3,
            guardian_status: 1,
            initial_total_minted: 100,
            mint_state_is_active: true,
            message_hash: [0x22; 32],
            canonical_event_key: [0x33; 32],
            mint_amount: 23,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        }
    }

    #[test]
    fn local_execution_scenario_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_LOCAL_EXECUTION_SCENARIO_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn local_execution_scenario_runs_full_local_sequence() {
        let input = valid_input();
        let output = run_local_execution_scenario_skeleton(input).expect("scenario ok");

        assert_eq!(output.gateway_config.route_id, input.route_id);
        assert_eq!(output.gateway_config.source_chain_id, input.source_chain_id);
        assert_eq!(output.gateway_config.guardian_set_id, input.guardian_set_id);
        assert_eq!(
            output.gateway_config.gateway_mint_authority_pda,
            input.gateway_mint_authority_pda
        );
        assert!(output.gateway_config.is_active);

        assert_eq!(output.guardian_set.guardian_set_id, input.guardian_set_id);
        assert_eq!(output.guardian_set.threshold, input.guardian_threshold);
        assert_eq!(output.guardian_set.guardian_count, input.guardian_count);
        assert_eq!(output.guardian_set.status, input.guardian_status);

        assert_eq!(output.initial_mint_state.mint, input.mint);
        assert_eq!(
            output.initial_mint_state.mint_authority_pda,
            input.mint_authority_pda
        );
        assert_eq!(output.initial_mint_state.decimals, input.decimals);
        assert_eq!(
            output.initial_mint_state.total_minted,
            input.initial_total_minted
        );

        assert_eq!(
            output.next_mint_state.total_minted,
            input.initial_total_minted + input.mint_amount
        );

        assert_eq!(
            output.processed_event.canonical_event_key,
            input.canonical_event_key
        );
        assert_eq!(output.processed_event.message_hash, input.message_hash);
        assert_eq!(
            output.processed_event.source_burn_tx_hash,
            input.source_burn_tx_hash
        );
        assert_eq!(
            output.processed_event.source_burn_event_index,
            input.source_burn_event_index
        );
        assert_eq!(output.processed_event.recipient_hash, input.recipient_hash);
        assert_eq!(output.processed_event.minted_amount, input.mint_amount);
        assert_eq!(
            output.processed_event.processed_at_slot,
            input.processed_at_slot
        );
    }

    #[test]
    fn local_execution_scenario_rejects_invalid_guardian_threshold() {
        let mut input = valid_input();
        input.guardian_threshold = 4;
        input.guardian_count = 3;

        let result = run_local_execution_scenario_skeleton(input);

        assert!(matches!(
            result,
            Err(XxxlLocalExecutionScenarioSkeletonError::Plan(_))
        ));
    }

    #[test]
    fn local_execution_scenario_rejects_inactive_mint_state_before_consume() {
        let mut input = valid_input();
        input.mint_state_is_active = false;

        let result = run_local_execution_scenario_skeleton(input);

        assert!(matches!(
            result,
            Err(XxxlLocalExecutionScenarioSkeletonError::Plan(_))
        ));
    }

    #[test]
    fn local_execution_scenario_rejects_total_minted_overflow() {
        let mut input = valid_input();
        input.initial_total_minted = u128::MAX;
        input.mint_amount = 1;

        let result = run_local_execution_scenario_skeleton(input);

        assert!(matches!(
            result,
            Err(XxxlLocalExecutionScenarioSkeletonError::Plan(_))
        ));
    }

    #[test]
    fn local_execution_scenario_allows_zero_mint_amount_as_boundary_only() {
        let mut input = valid_input();
        input.initial_total_minted = 7;
        input.mint_amount = 0;

        let output = run_local_execution_scenario_skeleton(input).expect("scenario ok");

        assert_eq!(output.initial_mint_state.total_minted, 7);
        assert_eq!(output.next_mint_state.total_minted, 7);
        assert_eq!(output.processed_event.minted_amount, 0);
    }
}

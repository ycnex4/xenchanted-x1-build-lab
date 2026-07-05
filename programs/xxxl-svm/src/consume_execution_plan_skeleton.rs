use crate::{
    account_validation_skeleton::XxxlAccountMetaSkeleton,
    consume_state_transition_skeleton::{
        apply_consume_gateway_mint_state_transition_skeleton,
        ConsumeGatewayMintStateTransitionSkeletonInput,
        ConsumeGatewayMintStateTransitionSkeletonOutput,
        XxxlConsumeStateTransitionSkeletonError,
    },
    state_account_layout_skeleton::MintStateAccountLayoutSkeleton,
    state_instruction_skeleton::XxxlGatewayInstructionTag,
    typed_instruction_skeleton::XxxlTypedInstructionSkeleton,
    validated_dispatch_skeleton::{
        plan_validated_dispatch_skeleton, XxxlValidatedDispatchSkeletonError,
        XxxlValidatedDispatchSkeletonPlan,
    },
};

pub const XXXL_CONSUME_EXECUTION_PLAN_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlConsumeExecutionPlanSkeletonError {
    ValidatedDispatch(XxxlValidatedDispatchSkeletonError),
    InvalidInstructionKind {
        expected: XxxlGatewayInstructionTag,
        actual: XxxlGatewayInstructionTag,
    },
    StateTransition(XxxlConsumeStateTransitionSkeletonError),
}

impl From<XxxlValidatedDispatchSkeletonError> for XxxlConsumeExecutionPlanSkeletonError {
    fn from(value: XxxlValidatedDispatchSkeletonError) -> Self {
        Self::ValidatedDispatch(value)
    }
}

impl From<XxxlConsumeStateTransitionSkeletonError> for XxxlConsumeExecutionPlanSkeletonError {
    fn from(value: XxxlConsumeStateTransitionSkeletonError) -> Self {
        Self::StateTransition(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsumeGatewayMintExecutionPlanSkeletonInput<'a> {
    pub instruction_data: &'a [u8],
    pub account_metas: &'a [XxxlAccountMetaSkeleton],
    pub current_mint_state: MintStateAccountLayoutSkeleton,
    pub processed_event_bump: u8,
    pub source_burn_tx_hash: [u8; 32],
    pub source_burn_event_index: u32,
    pub recipient_hash: [u8; 32],
    pub processed_at_slot: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsumeGatewayMintExecutionPlanSkeletonOutput {
    pub validated_dispatch_plan: XxxlValidatedDispatchSkeletonPlan,
    pub state_transition_output: ConsumeGatewayMintStateTransitionSkeletonOutput,
}

pub fn plan_consume_gateway_mint_execution_skeleton(
    input: ConsumeGatewayMintExecutionPlanSkeletonInput<'_>,
) -> Result<ConsumeGatewayMintExecutionPlanSkeletonOutput, XxxlConsumeExecutionPlanSkeletonError> {
    let validated_dispatch_plan =
        plan_validated_dispatch_skeleton(input.instruction_data, input.account_metas)?;

    if validated_dispatch_plan.dispatch_plan.tag != XxxlGatewayInstructionTag::ConsumeGatewayMint {
        return Err(XxxlConsumeExecutionPlanSkeletonError::InvalidInstructionKind {
            expected: XxxlGatewayInstructionTag::ConsumeGatewayMint,
            actual: validated_dispatch_plan.dispatch_plan.tag,
        });
    }

    let payload = match validated_dispatch_plan.dispatch_plan.instruction {
        XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(payload) => payload,
        _ => {
            return Err(XxxlConsumeExecutionPlanSkeletonError::InvalidInstructionKind {
                expected: XxxlGatewayInstructionTag::ConsumeGatewayMint,
                actual: validated_dispatch_plan.dispatch_plan.tag,
            });
        }
    };

    let state_transition_output =
        apply_consume_gateway_mint_state_transition_skeleton(
            ConsumeGatewayMintStateTransitionSkeletonInput {
                current_mint_state: input.current_mint_state,
                payload,
                processed_event_bump: input.processed_event_bump,
                source_burn_tx_hash: input.source_burn_tx_hash,
                source_burn_event_index: input.source_burn_event_index,
                recipient_hash: input.recipient_hash,
                processed_at_slot: input.processed_at_slot,
            },
        )?;

    Ok(ConsumeGatewayMintExecutionPlanSkeletonOutput {
        validated_dispatch_plan,
        state_transition_output,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        account_validation_skeleton::expected_account_metas_for_tag,
        instruction_payload_skeleton::{
            ConsumeGatewayMintPayloadBoundarySkeleton, InitializeGatewayConfigPayloadSkeleton,
        },
        typed_instruction_skeleton::{
            encode_typed_instruction_skeleton, XxxlTypedInstructionSkeleton,
        },
    };
    use solana_program::pubkey::Pubkey;

    fn active_mint_state(total_minted: u128) -> MintStateAccountLayoutSkeleton {
        MintStateAccountLayoutSkeleton {
            version: 1,
            bump: 250,
            mint: Pubkey::new_unique(),
            mint_authority_pda: Pubkey::new_unique(),
            mint_authority_bump: 252,
            token_program: Pubkey::new_unique(),
            decimals: 9,
            total_minted,
            is_active: true,
        }
    }

    fn consume_instruction_data(mint_amount: u128) -> Vec<u8> {
        encode_typed_instruction_skeleton(
            XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                ConsumeGatewayMintPayloadBoundarySkeleton {
                    message_hash: [0x22; 32],
                    canonical_event_key: [0x33; 32],
                    mint_amount,
                },
            ),
        )
    }

    #[test]
    fn consume_execution_plan_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_CONSUME_EXECUTION_PLAN_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn consume_execution_plan_accepts_valid_consume_boundary() {
        let data = consume_instruction_data(23);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        let input = ConsumeGatewayMintExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            current_mint_state: active_mint_state(100),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        let output = plan_consume_gateway_mint_execution_skeleton(input).expect("plan ok");

        assert_eq!(
            output.validated_dispatch_plan.dispatch_plan.tag,
            XxxlGatewayInstructionTag::ConsumeGatewayMint
        );
        assert_eq!(
            output.state_transition_output.next_mint_state.total_minted,
            123
        );
        assert_eq!(
            output.state_transition_output.processed_event.canonical_event_key,
            [0x33; 32]
        );
        assert_eq!(
            output.state_transition_output.processed_event.minted_amount,
            23
        );
    }

    #[test]
    fn consume_execution_plan_rejects_non_consume_instruction() {
        let instruction =
            XxxlTypedInstructionSkeleton::InitializeGatewayConfig(
                InitializeGatewayConfigPayloadSkeleton {
                    route_id: [0x11; 32],
                    source_chain_id: 1,
                    guardian_set_id: 7,
                    is_active: true,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig);

        let input = ConsumeGatewayMintExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            current_mint_state: active_mint_state(0),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        assert_eq!(
            plan_consume_gateway_mint_execution_skeleton(input),
            Err(XxxlConsumeExecutionPlanSkeletonError::InvalidInstructionKind {
                expected: XxxlGatewayInstructionTag::ConsumeGatewayMint,
                actual: XxxlGatewayInstructionTag::InitializeGatewayConfig,
            })
        );
    }

    #[test]
    fn consume_execution_plan_rejects_invalid_account_metas() {
        let data = consume_instruction_data(23);
        let mut metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        metas[0].is_signer = false;

        let input = ConsumeGatewayMintExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            current_mint_state: active_mint_state(0),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        let err = plan_consume_gateway_mint_execution_skeleton(input).expect_err("must reject");

        assert!(matches!(
            err,
            XxxlConsumeExecutionPlanSkeletonError::ValidatedDispatch(_)
        ));
    }

    #[test]
    fn consume_execution_plan_rejects_inactive_mint_state() {
        let data = consume_instruction_data(23);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);
        let mut mint_state = active_mint_state(0);
        mint_state.is_active = false;

        let input = ConsumeGatewayMintExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            current_mint_state: mint_state,
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        assert_eq!(
            plan_consume_gateway_mint_execution_skeleton(input),
            Err(XxxlConsumeExecutionPlanSkeletonError::StateTransition(
                XxxlConsumeStateTransitionSkeletonError::MintStateInactive
            ))
        );
    }

    #[test]
    fn consume_execution_plan_rejects_total_minted_overflow() {
        let data = consume_instruction_data(1);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        let input = ConsumeGatewayMintExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            current_mint_state: active_mint_state(u128::MAX),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        assert_eq!(
            plan_consume_gateway_mint_execution_skeleton(input),
            Err(XxxlConsumeExecutionPlanSkeletonError::StateTransition(
                XxxlConsumeStateTransitionSkeletonError::MintAmountOverflow
            ))
        );
    }
}

use crate::{
    account_validation_skeleton::XxxlAccountMetaSkeleton,
    consume_execution_plan_skeleton::{
        plan_consume_gateway_mint_execution_skeleton,
        ConsumeGatewayMintExecutionPlanSkeletonInput,
        ConsumeGatewayMintExecutionPlanSkeletonOutput,
        XxxlConsumeExecutionPlanSkeletonError,
    },
    initialization_execution_plan_skeleton::{
        plan_initialize_gateway_config_execution_skeleton,
        plan_initialize_guardian_set_execution_skeleton,
        plan_initialize_mint_state_execution_skeleton,
        InitializeGatewayConfigExecutionPlanSkeletonInput,
        InitializeGatewayConfigExecutionPlanSkeletonOutput,
        InitializeGuardianSetExecutionPlanSkeletonInput,
        InitializeGuardianSetExecutionPlanSkeletonOutput,
        InitializeMintStateExecutionPlanSkeletonInput,
        InitializeMintStateExecutionPlanSkeletonOutput,
        XxxlInitializationExecutionPlanSkeletonError,
    },
    state_account_layout_skeleton::MintStateAccountLayoutSkeleton,
    state_instruction_skeleton::XxxlGatewayInstructionTag,
    validated_dispatch_skeleton::{
        plan_validated_dispatch_skeleton, XxxlValidatedDispatchSkeletonError,
    },
};
use solana_program::pubkey::Pubkey;

pub const XXXL_LOCAL_EXECUTION_PLAN_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalExecutionContextSkeleton {
    InitializeGatewayConfig {
        bump: u8,
        gateway_mint_authority_pda: Pubkey,
    },
    InitializeGuardianSet {
        bump: u8,
        status: u8,
    },
    InitializeMintState {
        bump: u8,
        mint_authority_pda: Pubkey,
        mint_authority_bump: u8,
        token_program: Pubkey,
        total_minted: u128,
        is_active: bool,
    },
    ConsumeGatewayMint {
        current_mint_state: MintStateAccountLayoutSkeleton,
        processed_event_bump: u8,
        source_burn_tx_hash: [u8; 32],
        source_burn_event_index: u32,
        recipient_hash: [u8; 32],
        processed_at_slot: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalExecutionPlanSkeleton {
    InitializeGatewayConfig(InitializeGatewayConfigExecutionPlanSkeletonOutput),
    InitializeGuardianSet(InitializeGuardianSetExecutionPlanSkeletonOutput),
    InitializeMintState(InitializeMintStateExecutionPlanSkeletonOutput),
    ConsumeGatewayMint(ConsumeGatewayMintExecutionPlanSkeletonOutput),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlLocalExecutionPlanSkeletonError {
    ValidatedDispatch(XxxlValidatedDispatchSkeletonError),
    ContextInstructionMismatch {
        expected: XxxlGatewayInstructionTag,
        actual: XxxlGatewayInstructionTag,
    },
    Initialization(XxxlInitializationExecutionPlanSkeletonError),
    Consume(XxxlConsumeExecutionPlanSkeletonError),
}

impl From<XxxlValidatedDispatchSkeletonError> for XxxlLocalExecutionPlanSkeletonError {
    fn from(value: XxxlValidatedDispatchSkeletonError) -> Self {
        Self::ValidatedDispatch(value)
    }
}

impl From<XxxlInitializationExecutionPlanSkeletonError> for XxxlLocalExecutionPlanSkeletonError {
    fn from(value: XxxlInitializationExecutionPlanSkeletonError) -> Self {
        Self::Initialization(value)
    }
}

impl From<XxxlConsumeExecutionPlanSkeletonError> for XxxlLocalExecutionPlanSkeletonError {
    fn from(value: XxxlConsumeExecutionPlanSkeletonError) -> Self {
        Self::Consume(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XxxlLocalExecutionPlanSkeletonInput<'a> {
    pub instruction_data: &'a [u8],
    pub account_metas: &'a [XxxlAccountMetaSkeleton],
    pub context: XxxlLocalExecutionContextSkeleton,
}

pub fn expected_tag_for_local_execution_context_skeleton(
    context: &XxxlLocalExecutionContextSkeleton,
) -> XxxlGatewayInstructionTag {
    match context {
        XxxlLocalExecutionContextSkeleton::InitializeGatewayConfig { .. } => {
            XxxlGatewayInstructionTag::InitializeGatewayConfig
        }
        XxxlLocalExecutionContextSkeleton::InitializeGuardianSet { .. } => {
            XxxlGatewayInstructionTag::InitializeGuardianSet
        }
        XxxlLocalExecutionContextSkeleton::InitializeMintState { .. } => {
            XxxlGatewayInstructionTag::InitializeMintState
        }
        XxxlLocalExecutionContextSkeleton::ConsumeGatewayMint { .. } => {
            XxxlGatewayInstructionTag::ConsumeGatewayMint
        }
    }
}

pub fn plan_local_execution_skeleton(
    input: XxxlLocalExecutionPlanSkeletonInput<'_>,
) -> Result<XxxlLocalExecutionPlanSkeleton, XxxlLocalExecutionPlanSkeletonError> {
    let validated_dispatch_plan =
        plan_validated_dispatch_skeleton(input.instruction_data, input.account_metas)?;

    let expected_tag = expected_tag_for_local_execution_context_skeleton(&input.context);
    let actual_tag = validated_dispatch_plan.dispatch_plan.tag;

    if expected_tag != actual_tag {
        return Err(XxxlLocalExecutionPlanSkeletonError::ContextInstructionMismatch {
            expected: expected_tag,
            actual: actual_tag,
        });
    }

    match input.context {
        XxxlLocalExecutionContextSkeleton::InitializeGatewayConfig {
            bump,
            gateway_mint_authority_pda,
        } => Ok(XxxlLocalExecutionPlanSkeleton::InitializeGatewayConfig(
            plan_initialize_gateway_config_execution_skeleton(
                InitializeGatewayConfigExecutionPlanSkeletonInput {
                    instruction_data: input.instruction_data,
                    account_metas: input.account_metas,
                    bump,
                    gateway_mint_authority_pda,
                },
            )?,
        )),
        XxxlLocalExecutionContextSkeleton::InitializeGuardianSet { bump, status } => {
            Ok(XxxlLocalExecutionPlanSkeleton::InitializeGuardianSet(
                plan_initialize_guardian_set_execution_skeleton(
                    InitializeGuardianSetExecutionPlanSkeletonInput {
                        instruction_data: input.instruction_data,
                        account_metas: input.account_metas,
                        bump,
                        status,
                    },
                )?,
            ))
        }
        XxxlLocalExecutionContextSkeleton::InitializeMintState {
            bump,
            mint_authority_pda,
            mint_authority_bump,
            token_program,
            total_minted,
            is_active,
        } => Ok(XxxlLocalExecutionPlanSkeleton::InitializeMintState(
            plan_initialize_mint_state_execution_skeleton(
                InitializeMintStateExecutionPlanSkeletonInput {
                    instruction_data: input.instruction_data,
                    account_metas: input.account_metas,
                    bump,
                    mint_authority_pda,
                    mint_authority_bump,
                    token_program,
                    total_minted,
                    is_active,
                },
            )?,
        )),
        XxxlLocalExecutionContextSkeleton::ConsumeGatewayMint {
            current_mint_state,
            processed_event_bump,
            source_burn_tx_hash,
            source_burn_event_index,
            recipient_hash,
            processed_at_slot,
        } => Ok(XxxlLocalExecutionPlanSkeleton::ConsumeGatewayMint(
            plan_consume_gateway_mint_execution_skeleton(
                ConsumeGatewayMintExecutionPlanSkeletonInput {
                    instruction_data: input.instruction_data,
                    account_metas: input.account_metas,
                    current_mint_state,
                    processed_event_bump,
                    source_burn_tx_hash,
                    source_burn_event_index,
                    recipient_hash,
                    processed_at_slot,
                },
            )?,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        account_validation_skeleton::expected_account_metas_for_tag,
        instruction_payload_skeleton::{
            ConsumeGatewayMintPayloadBoundarySkeleton, InitializeGatewayConfigPayloadSkeleton,
            InitializeGuardianSetHeaderPayloadSkeleton, InitializeMintStatePayloadSkeleton,
        },
        typed_instruction_skeleton::{
            encode_typed_instruction_skeleton, XxxlTypedInstructionSkeleton,
        },
    };

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

    #[test]
    fn local_execution_plan_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_LOCAL_EXECUTION_PLAN_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn local_execution_plans_initialize_gateway_config() {
        let mint_authority = Pubkey::new_unique();

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

        let plan = plan_local_execution_skeleton(XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            context: XxxlLocalExecutionContextSkeleton::InitializeGatewayConfig {
                bump: 252,
                gateway_mint_authority_pda: mint_authority,
            },
        })
        .expect("plan ok");

        match plan {
            XxxlLocalExecutionPlanSkeleton::InitializeGatewayConfig(output) => {
                assert_eq!(
                    output.validated_dispatch_plan.dispatch_plan.tag,
                    XxxlGatewayInstructionTag::InitializeGatewayConfig
                );
                assert_eq!(output.account.gateway_mint_authority_pda, mint_authority);
                assert!(output.account.is_active);
            }
            _ => panic!("unexpected plan variant"),
        }
    }

    #[test]
    fn local_execution_plans_initialize_guardian_set() {
        let instruction =
            XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(
                InitializeGuardianSetHeaderPayloadSkeleton {
                    guardian_set_id: 7,
                    threshold: 2,
                    guardian_count: 3,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGuardianSet);

        let plan = plan_local_execution_skeleton(XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            context: XxxlLocalExecutionContextSkeleton::InitializeGuardianSet {
                bump: 251,
                status: 1,
            },
        })
        .expect("plan ok");

        match plan {
            XxxlLocalExecutionPlanSkeleton::InitializeGuardianSet(output) => {
                assert_eq!(output.account.guardian_set_id, 7);
                assert_eq!(output.account.threshold, 2);
                assert_eq!(output.account.guardian_count, 3);
                assert_eq!(output.account.status, 1);
            }
            _ => panic!("unexpected plan variant"),
        }
    }

    #[test]
    fn local_execution_plans_initialize_mint_state() {
        let mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let token_program = Pubkey::new_unique();

        let instruction = XxxlTypedInstructionSkeleton::InitializeMintState(
            InitializeMintStatePayloadSkeleton { mint, decimals: 9 },
        );

        let data = encode_typed_instruction_skeleton(instruction);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeMintState);

        let plan = plan_local_execution_skeleton(XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            context: XxxlLocalExecutionContextSkeleton::InitializeMintState {
                bump: 250,
                mint_authority_pda: mint_authority,
                mint_authority_bump: 252,
                token_program,
                total_minted: 0,
                is_active: true,
            },
        })
        .expect("plan ok");

        match plan {
            XxxlLocalExecutionPlanSkeleton::InitializeMintState(output) => {
                assert_eq!(output.account.mint, mint);
                assert_eq!(output.account.mint_authority_pda, mint_authority);
                assert_eq!(output.account.token_program, token_program);
                assert_eq!(output.account.decimals, 9);
                assert!(output.account.is_active);
            }
            _ => panic!("unexpected plan variant"),
        }
    }

    #[test]
    fn local_execution_plans_consume_gateway_mint() {
        let instruction =
            XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                ConsumeGatewayMintPayloadBoundarySkeleton {
                    message_hash: [0x22; 32],
                    canonical_event_key: [0x33; 32],
                    mint_amount: 23,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        let plan = plan_local_execution_skeleton(XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            context: XxxlLocalExecutionContextSkeleton::ConsumeGatewayMint {
                current_mint_state: active_mint_state(100),
                processed_event_bump: 249,
                source_burn_tx_hash: [0x44; 32],
                source_burn_event_index: 5,
                recipient_hash: [0x55; 32],
                processed_at_slot: 99,
            },
        })
        .expect("plan ok");

        match plan {
            XxxlLocalExecutionPlanSkeleton::ConsumeGatewayMint(output) => {
                assert_eq!(
                    output.validated_dispatch_plan.dispatch_plan.tag,
                    XxxlGatewayInstructionTag::ConsumeGatewayMint
                );
                assert_eq!(
                    output.state_transition_output.next_mint_state.total_minted,
                    123
                );
                assert_eq!(
                    output.state_transition_output.processed_event.minted_amount,
                    23
                );
            }
            _ => panic!("unexpected plan variant"),
        }
    }

    #[test]
    fn local_execution_rejects_context_instruction_mismatch() {
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

        assert_eq!(
            plan_local_execution_skeleton(XxxlLocalExecutionPlanSkeletonInput {
                instruction_data: &data,
                account_metas: &metas,
                context: XxxlLocalExecutionContextSkeleton::ConsumeGatewayMint {
                    current_mint_state: active_mint_state(0),
                    processed_event_bump: 249,
                    source_burn_tx_hash: [0x44; 32],
                    source_burn_event_index: 5,
                    recipient_hash: [0x55; 32],
                    processed_at_slot: 99,
                },
            }),
            Err(XxxlLocalExecutionPlanSkeletonError::ContextInstructionMismatch {
                expected: XxxlGatewayInstructionTag::ConsumeGatewayMint,
                actual: XxxlGatewayInstructionTag::InitializeGatewayConfig,
            })
        );
    }

    #[test]
    fn local_execution_rejects_invalid_account_metas_before_context_execution() {
        let instruction =
            XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                ConsumeGatewayMintPayloadBoundarySkeleton {
                    message_hash: [0x22; 32],
                    canonical_event_key: [0x33; 32],
                    mint_amount: 23,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let mut metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        metas[0].is_signer = false;

        let err = plan_local_execution_skeleton(XxxlLocalExecutionPlanSkeletonInput {
            instruction_data: &data,
            account_metas: &metas,
            context: XxxlLocalExecutionContextSkeleton::ConsumeGatewayMint {
                current_mint_state: active_mint_state(0),
                processed_event_bump: 249,
                source_burn_tx_hash: [0x44; 32],
                source_burn_event_index: 5,
                recipient_hash: [0x55; 32],
                processed_at_slot: 99,
            },
        })
        .expect_err("must reject");

        assert!(matches!(
            err,
            XxxlLocalExecutionPlanSkeletonError::ValidatedDispatch(_)
        ));
    }
}

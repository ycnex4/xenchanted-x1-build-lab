use crate::{
    account_validation_skeleton::XxxlAccountMetaSkeleton,
    state_account_layout_skeleton::{
        GatewayConfigAccountLayoutSkeleton, GuardianSetHeaderAccountLayoutSkeleton,
        MintStateAccountLayoutSkeleton,
    },
    state_initialization_skeleton::{
        build_gateway_config_account_layout_skeleton,
        build_guardian_set_header_account_layout_skeleton,
        build_mint_state_account_layout_skeleton,
        encode_and_verify_gateway_config_initialization_skeleton,
        encode_and_verify_guardian_set_header_initialization_skeleton,
        encode_and_verify_mint_state_initialization_skeleton,
        GatewayConfigInitializationSkeletonInput,
        GuardianSetHeaderInitializationSkeletonInput,
        MintStateInitializationSkeletonInput,
        XxxlStateInitializationSkeletonError,
    },
    state_instruction_skeleton::XxxlGatewayInstructionTag,
    typed_instruction_skeleton::XxxlTypedInstructionSkeleton,
    validated_dispatch_skeleton::{
        plan_validated_dispatch_skeleton, XxxlValidatedDispatchSkeletonError,
        XxxlValidatedDispatchSkeletonPlan,
    },
};
use solana_program::pubkey::Pubkey;

pub const XXXL_INITIALIZATION_EXECUTION_PLAN_SKELETON_STATUS: &str =
    "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum XxxlInitializationExecutionPlanSkeletonError {
    ValidatedDispatch(XxxlValidatedDispatchSkeletonError),
    InvalidInstructionKind {
        expected: XxxlGatewayInstructionTag,
        actual: XxxlGatewayInstructionTag,
    },
    StateInitialization(XxxlStateInitializationSkeletonError),
}

impl From<XxxlValidatedDispatchSkeletonError> for XxxlInitializationExecutionPlanSkeletonError {
    fn from(value: XxxlValidatedDispatchSkeletonError) -> Self {
        Self::ValidatedDispatch(value)
    }
}

impl From<XxxlStateInitializationSkeletonError> for XxxlInitializationExecutionPlanSkeletonError {
    fn from(value: XxxlStateInitializationSkeletonError) -> Self {
        Self::StateInitialization(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitializeGatewayConfigExecutionPlanSkeletonInput<'a> {
    pub instruction_data: &'a [u8],
    pub account_metas: &'a [XxxlAccountMetaSkeleton],
    pub bump: u8,
    pub gateway_mint_authority_pda: Pubkey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitializeGuardianSetExecutionPlanSkeletonInput<'a> {
    pub instruction_data: &'a [u8],
    pub account_metas: &'a [XxxlAccountMetaSkeleton],
    pub bump: u8,
    pub status: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitializeMintStateExecutionPlanSkeletonInput<'a> {
    pub instruction_data: &'a [u8],
    pub account_metas: &'a [XxxlAccountMetaSkeleton],
    pub bump: u8,
    pub mint_authority_pda: Pubkey,
    pub mint_authority_bump: u8,
    pub token_program: Pubkey,
    pub total_minted: u128,
    pub is_active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitializeGatewayConfigExecutionPlanSkeletonOutput {
    pub validated_dispatch_plan: XxxlValidatedDispatchSkeletonPlan,
    pub account: GatewayConfigAccountLayoutSkeleton,
    pub encoded_account: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitializeGuardianSetExecutionPlanSkeletonOutput {
    pub validated_dispatch_plan: XxxlValidatedDispatchSkeletonPlan,
    pub account: GuardianSetHeaderAccountLayoutSkeleton,
    pub encoded_account: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InitializeMintStateExecutionPlanSkeletonOutput {
    pub validated_dispatch_plan: XxxlValidatedDispatchSkeletonPlan,
    pub account: MintStateAccountLayoutSkeleton,
    pub encoded_account: Vec<u8>,
}

fn require_expected_tag(
    actual: XxxlGatewayInstructionTag,
    expected: XxxlGatewayInstructionTag,
) -> Result<(), XxxlInitializationExecutionPlanSkeletonError> {
    if actual == expected {
        Ok(())
    } else {
        Err(XxxlInitializationExecutionPlanSkeletonError::InvalidInstructionKind {
            expected,
            actual,
        })
    }
}

pub fn plan_initialize_gateway_config_execution_skeleton(
    input: InitializeGatewayConfigExecutionPlanSkeletonInput<'_>,
) -> Result<
    InitializeGatewayConfigExecutionPlanSkeletonOutput,
    XxxlInitializationExecutionPlanSkeletonError,
> {
    let validated_dispatch_plan =
        plan_validated_dispatch_skeleton(input.instruction_data, input.account_metas)?;

    require_expected_tag(
        validated_dispatch_plan.dispatch_plan.tag,
        XxxlGatewayInstructionTag::InitializeGatewayConfig,
    )?;

    let payload = match validated_dispatch_plan.dispatch_plan.instruction {
        XxxlTypedInstructionSkeleton::InitializeGatewayConfig(payload) => payload,
        _ => {
            return Err(XxxlInitializationExecutionPlanSkeletonError::InvalidInstructionKind {
                expected: XxxlGatewayInstructionTag::InitializeGatewayConfig,
                actual: validated_dispatch_plan.dispatch_plan.tag,
            });
        }
    };

    let initialization_input = GatewayConfigInitializationSkeletonInput {
        payload,
        bump: input.bump,
        gateway_mint_authority_pda: input.gateway_mint_authority_pda,
    };

    let account = build_gateway_config_account_layout_skeleton(initialization_input);
    let encoded_account =
        encode_and_verify_gateway_config_initialization_skeleton(initialization_input)?;

    Ok(InitializeGatewayConfigExecutionPlanSkeletonOutput {
        validated_dispatch_plan,
        account,
        encoded_account,
    })
}

pub fn plan_initialize_guardian_set_execution_skeleton(
    input: InitializeGuardianSetExecutionPlanSkeletonInput<'_>,
) -> Result<
    InitializeGuardianSetExecutionPlanSkeletonOutput,
    XxxlInitializationExecutionPlanSkeletonError,
> {
    let validated_dispatch_plan =
        plan_validated_dispatch_skeleton(input.instruction_data, input.account_metas)?;

    require_expected_tag(
        validated_dispatch_plan.dispatch_plan.tag,
        XxxlGatewayInstructionTag::InitializeGuardianSet,
    )?;

    let payload = match validated_dispatch_plan.dispatch_plan.instruction {
        XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(payload) => payload,
        _ => {
            return Err(XxxlInitializationExecutionPlanSkeletonError::InvalidInstructionKind {
                expected: XxxlGatewayInstructionTag::InitializeGuardianSet,
                actual: validated_dispatch_plan.dispatch_plan.tag,
            });
        }
    };

    let initialization_input = GuardianSetHeaderInitializationSkeletonInput {
        payload,
        bump: input.bump,
        status: input.status,
    };

    let account = build_guardian_set_header_account_layout_skeleton(initialization_input)?;
    let encoded_account =
        encode_and_verify_guardian_set_header_initialization_skeleton(initialization_input)?;

    Ok(InitializeGuardianSetExecutionPlanSkeletonOutput {
        validated_dispatch_plan,
        account,
        encoded_account,
    })
}

pub fn plan_initialize_mint_state_execution_skeleton(
    input: InitializeMintStateExecutionPlanSkeletonInput<'_>,
) -> Result<
    InitializeMintStateExecutionPlanSkeletonOutput,
    XxxlInitializationExecutionPlanSkeletonError,
> {
    let validated_dispatch_plan =
        plan_validated_dispatch_skeleton(input.instruction_data, input.account_metas)?;

    require_expected_tag(
        validated_dispatch_plan.dispatch_plan.tag,
        XxxlGatewayInstructionTag::InitializeMintState,
    )?;

    let payload = match validated_dispatch_plan.dispatch_plan.instruction {
        XxxlTypedInstructionSkeleton::InitializeMintState(payload) => payload,
        _ => {
            return Err(XxxlInitializationExecutionPlanSkeletonError::InvalidInstructionKind {
                expected: XxxlGatewayInstructionTag::InitializeMintState,
                actual: validated_dispatch_plan.dispatch_plan.tag,
            });
        }
    };

    let initialization_input = MintStateInitializationSkeletonInput {
        payload,
        bump: input.bump,
        mint_authority_pda: input.mint_authority_pda,
        mint_authority_bump: input.mint_authority_bump,
        token_program: input.token_program,
        total_minted: input.total_minted,
        is_active: input.is_active,
    };

    let account = build_mint_state_account_layout_skeleton(initialization_input);
    let encoded_account =
        encode_and_verify_mint_state_initialization_skeleton(initialization_input)?;

    Ok(InitializeMintStateExecutionPlanSkeletonOutput {
        validated_dispatch_plan,
        account,
        encoded_account,
    })
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
        state_account_layout_skeleton::{
            GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN, GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN,
            MINT_STATE_ACCOUNT_SKELETON_LEN,
        },
        typed_instruction_skeleton::{
            encode_typed_instruction_skeleton, XxxlTypedInstructionSkeleton,
        },
    };

    #[test]
    fn initialization_execution_plan_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_INITIALIZATION_EXECUTION_PLAN_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn initialize_gateway_config_execution_plan_accepts_valid_instruction() {
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

        let output = plan_initialize_gateway_config_execution_skeleton(
            InitializeGatewayConfigExecutionPlanSkeletonInput {
                instruction_data: &data,
                account_metas: &metas,
                bump: 252,
                gateway_mint_authority_pda: mint_authority,
            },
        )
        .expect("plan ok");

        assert_eq!(
            output.validated_dispatch_plan.dispatch_plan.tag,
            XxxlGatewayInstructionTag::InitializeGatewayConfig
        );
        assert_eq!(output.account.route_id, [0x11; 32]);
        assert_eq!(output.account.source_chain_id, 1);
        assert_eq!(output.account.guardian_set_id, 7);
        assert_eq!(output.account.gateway_mint_authority_pda, mint_authority);
        assert!(output.account.is_active);
        assert_eq!(output.encoded_account.len(), GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN);
    }

    #[test]
    fn initialize_guardian_set_execution_plan_accepts_valid_instruction() {
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

        let output = plan_initialize_guardian_set_execution_skeleton(
            InitializeGuardianSetExecutionPlanSkeletonInput {
                instruction_data: &data,
                account_metas: &metas,
                bump: 251,
                status: 1,
            },
        )
        .expect("plan ok");

        assert_eq!(
            output.validated_dispatch_plan.dispatch_plan.tag,
            XxxlGatewayInstructionTag::InitializeGuardianSet
        );
        assert_eq!(output.account.guardian_set_id, 7);
        assert_eq!(output.account.threshold, 2);
        assert_eq!(output.account.guardian_count, 3);
        assert_eq!(output.account.status, 1);
        assert_eq!(
            output.encoded_account.len(),
            GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN
        );
    }

    #[test]
    fn initialize_mint_state_execution_plan_accepts_valid_instruction() {
        let mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let token_program = Pubkey::new_unique();

        let instruction = XxxlTypedInstructionSkeleton::InitializeMintState(
            InitializeMintStatePayloadSkeleton { mint, decimals: 9 },
        );

        let data = encode_typed_instruction_skeleton(instruction);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeMintState);

        let output = plan_initialize_mint_state_execution_skeleton(
            InitializeMintStateExecutionPlanSkeletonInput {
                instruction_data: &data,
                account_metas: &metas,
                bump: 250,
                mint_authority_pda: mint_authority,
                mint_authority_bump: 252,
                token_program,
                total_minted: 0,
                is_active: true,
            },
        )
        .expect("plan ok");

        assert_eq!(
            output.validated_dispatch_plan.dispatch_plan.tag,
            XxxlGatewayInstructionTag::InitializeMintState
        );
        assert_eq!(output.account.mint, mint);
        assert_eq!(output.account.mint_authority_pda, mint_authority);
        assert_eq!(output.account.token_program, token_program);
        assert_eq!(output.account.decimals, 9);
        assert_eq!(output.account.total_minted, 0);
        assert!(output.account.is_active);
        assert_eq!(output.encoded_account.len(), MINT_STATE_ACCOUNT_SKELETON_LEN);
    }

    #[test]
    fn initialize_gateway_config_execution_plan_rejects_non_gateway_config_instruction() {
        let instruction =
            XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                ConsumeGatewayMintPayloadBoundarySkeleton {
                    message_hash: [0x22; 32],
                    canonical_event_key: [0x33; 32],
                    mint_amount: 1,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        assert_eq!(
            plan_initialize_gateway_config_execution_skeleton(
                InitializeGatewayConfigExecutionPlanSkeletonInput {
                    instruction_data: &data,
                    account_metas: &metas,
                    bump: 252,
                    gateway_mint_authority_pda: Pubkey::new_unique(),
                },
            ),
            Err(XxxlInitializationExecutionPlanSkeletonError::InvalidInstructionKind {
                expected: XxxlGatewayInstructionTag::InitializeGatewayConfig,
                actual: XxxlGatewayInstructionTag::ConsumeGatewayMint,
            })
        );
    }

    #[test]
    fn initialize_gateway_config_execution_plan_rejects_invalid_account_metas() {
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
        let mut metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig);

        metas[0].is_signer = false;

        let err = plan_initialize_gateway_config_execution_skeleton(
            InitializeGatewayConfigExecutionPlanSkeletonInput {
                instruction_data: &data,
                account_metas: &metas,
                bump: 252,
                gateway_mint_authority_pda: Pubkey::new_unique(),
            },
        )
        .expect_err("must reject");

        assert!(matches!(
            err,
            XxxlInitializationExecutionPlanSkeletonError::ValidatedDispatch(_)
        ));
    }
}

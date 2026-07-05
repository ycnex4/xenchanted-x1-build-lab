use crate::{
    account_validation_skeleton::{
        validate_account_metas_for_tag, XxxlAccountMetaSkeleton,
        XxxlAccountValidationSkeletonError,
    },
    dispatch_skeleton::{
        plan_dispatch_skeleton, XxxlDispatchSkeletonError, XxxlDispatchSkeletonPlan,
    },
};

pub const XXXL_VALIDATED_DISPATCH_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlValidatedDispatchSkeletonError {
    Dispatch(XxxlDispatchSkeletonError),
    AccountValidation(XxxlAccountValidationSkeletonError),
}

impl From<XxxlDispatchSkeletonError> for XxxlValidatedDispatchSkeletonError {
    fn from(value: XxxlDispatchSkeletonError) -> Self {
        Self::Dispatch(value)
    }
}

impl From<XxxlAccountValidationSkeletonError> for XxxlValidatedDispatchSkeletonError {
    fn from(value: XxxlAccountValidationSkeletonError) -> Self {
        Self::AccountValidation(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlValidatedDispatchSkeletonPlan {
    pub dispatch_plan: XxxlDispatchSkeletonPlan,
}

pub fn plan_validated_dispatch_skeleton(
    instruction_data: &[u8],
    account_metas: &[XxxlAccountMetaSkeleton],
) -> Result<XxxlValidatedDispatchSkeletonPlan, XxxlValidatedDispatchSkeletonError> {
    let dispatch_plan = plan_dispatch_skeleton(instruction_data, account_metas.len())?;

    validate_account_metas_for_tag(dispatch_plan.tag, account_metas)?;

    Ok(XxxlValidatedDispatchSkeletonPlan { dispatch_plan })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        account_order_skeleton::{
            XxxlAccountOrderSkeletonError, XxxlAccountOwnerExpectationSkeleton,
        },
        account_validation_skeleton::expected_account_metas_for_tag,
        instruction_codec_skeleton::XxxlInstructionCodecSkeletonError,
        instruction_payload_skeleton::{
            ConsumeGatewayMintPayloadBoundarySkeleton, InitializeGatewayConfigPayloadSkeleton,
        },
        state_instruction_skeleton::XxxlGatewayInstructionTag,
        typed_instruction_skeleton::{
            encode_typed_instruction_skeleton, XxxlTypedInstructionSkeleton,
            XxxlTypedInstructionSkeletonError,
        },
    };

    #[test]
    fn validated_dispatch_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_VALIDATED_DISPATCH_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn validated_dispatch_accepts_initialize_gateway_config() {
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

        let plan = plan_validated_dispatch_skeleton(&data, &metas).expect("validated dispatch ok");

        assert_eq!(
            plan.dispatch_plan.tag,
            XxxlGatewayInstructionTag::InitializeGatewayConfig
        );
        assert_eq!(plan.dispatch_plan.instruction, instruction);
        assert_eq!(plan.dispatch_plan.required_accounts.len(), 4);
    }

    #[test]
    fn validated_dispatch_accepts_consume_gateway_mint_boundary() {
        let instruction =
            XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                ConsumeGatewayMintPayloadBoundarySkeleton {
                    message_hash: [0x22; 32],
                    canonical_event_key: [0x33; 32],
                    mint_amount: 123_456,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        let plan = plan_validated_dispatch_skeleton(&data, &metas).expect("validated dispatch ok");

        assert_eq!(plan.dispatch_plan.tag, XxxlGatewayInstructionTag::ConsumeGatewayMint);
        assert_eq!(plan.dispatch_plan.instruction, instruction);
        assert_eq!(plan.dispatch_plan.required_accounts.len(), 11);
    }

    #[test]
    fn validated_dispatch_rejects_invalid_instruction_before_account_validation() {
        assert_eq!(
            plan_validated_dispatch_skeleton(&[u8::MAX], &[]),
            Err(XxxlValidatedDispatchSkeletonError::Dispatch(
                XxxlDispatchSkeletonError::TypedInstruction(
                    XxxlTypedInstructionSkeletonError::Codec(
                        XxxlInstructionCodecSkeletonError::InvalidInstructionTag
                    )
                )
            ))
        );
    }

    #[test]
    fn validated_dispatch_rejects_wrong_account_count() {
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
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeMintState);

        assert_eq!(
            plan_validated_dispatch_skeleton(&data, &metas),
            Err(XxxlValidatedDispatchSkeletonError::Dispatch(
                XxxlDispatchSkeletonError::AccountOrder(
                    XxxlAccountOrderSkeletonError::InvalidAccountCount
                )
            ))
        );
    }

    #[test]
    fn validated_dispatch_rejects_signer_mismatch() {
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

        assert_eq!(
            plan_validated_dispatch_skeleton(&data, &metas),
            Err(XxxlValidatedDispatchSkeletonError::AccountValidation(
                XxxlAccountValidationSkeletonError::SignerMismatch {
                    index: 0,
                    expected: true,
                    actual: false
                }
            ))
        );
    }

    #[test]
    fn validated_dispatch_rejects_writable_mismatch() {
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

        metas[1].is_writable = false;

        assert_eq!(
            plan_validated_dispatch_skeleton(&data, &metas),
            Err(XxxlValidatedDispatchSkeletonError::AccountValidation(
                XxxlAccountValidationSkeletonError::WritableMismatch {
                    index: 1,
                    expected: true,
                    actual: false
                }
            ))
        );
    }

    #[test]
    fn validated_dispatch_rejects_owner_expectation_mismatch() {
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

        metas[2].owner_expectation = XxxlAccountOwnerExpectationSkeleton::SplTokenProgram;

        assert_eq!(
            plan_validated_dispatch_skeleton(&data, &metas),
            Err(XxxlValidatedDispatchSkeletonError::AccountValidation(
                XxxlAccountValidationSkeletonError::OwnerExpectationMismatch {
                    index: 2,
                    expected: XxxlAccountOwnerExpectationSkeleton::SystemProgram,
                    actual: XxxlAccountOwnerExpectationSkeleton::SplTokenProgram
                }
            ))
        );
    }
}

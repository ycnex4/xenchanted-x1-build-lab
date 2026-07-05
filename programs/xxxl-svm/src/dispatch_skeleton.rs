use crate::{
    account_order_skeleton::{
        account_requirements_for_tag, validate_account_count_for_tag,
        XxxlAccountOrderSkeletonError, XxxlAccountRequirementSkeleton,
    },
    state_instruction_skeleton::XxxlGatewayInstructionTag,
    typed_instruction_skeleton::{
        decode_typed_instruction_skeleton, XxxlTypedInstructionSkeleton,
        XxxlTypedInstructionSkeletonError,
    },
};

pub const XXXL_DISPATCH_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlDispatchSkeletonError {
    TypedInstruction(XxxlTypedInstructionSkeletonError),
    AccountOrder(XxxlAccountOrderSkeletonError),
}

impl From<XxxlTypedInstructionSkeletonError> for XxxlDispatchSkeletonError {
    fn from(value: XxxlTypedInstructionSkeletonError) -> Self {
        Self::TypedInstruction(value)
    }
}

impl From<XxxlAccountOrderSkeletonError> for XxxlDispatchSkeletonError {
    fn from(value: XxxlAccountOrderSkeletonError) -> Self {
        Self::AccountOrder(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlDispatchSkeletonPlan {
    pub tag: XxxlGatewayInstructionTag,
    pub instruction: XxxlTypedInstructionSkeleton,
    pub required_accounts: &'static [XxxlAccountRequirementSkeleton],
}

pub fn tag_for_typed_instruction_skeleton(
    instruction: &XxxlTypedInstructionSkeleton,
) -> XxxlGatewayInstructionTag {
    match instruction {
        XxxlTypedInstructionSkeleton::InitializeGatewayConfig(_) => {
            XxxlGatewayInstructionTag::InitializeGatewayConfig
        }
        XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(_) => {
            XxxlGatewayInstructionTag::InitializeGuardianSet
        }
        XxxlTypedInstructionSkeleton::InitializeMintState(_) => {
            XxxlGatewayInstructionTag::InitializeMintState
        }
        XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(_) => {
            XxxlGatewayInstructionTag::ConsumeGatewayMint
        }
    }
}

pub fn plan_dispatch_skeleton(
    instruction_data: &[u8],
    account_count: usize,
) -> Result<XxxlDispatchSkeletonPlan, XxxlDispatchSkeletonError> {
    let instruction = decode_typed_instruction_skeleton(instruction_data)?;
    let tag = tag_for_typed_instruction_skeleton(&instruction);

    validate_account_count_for_tag(tag, account_count)?;

    Ok(XxxlDispatchSkeletonPlan {
        tag,
        instruction,
        required_accounts: account_requirements_for_tag(tag),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        instruction_payload_skeleton::{
            ConsumeGatewayMintPayloadBoundarySkeleton, InitializeGatewayConfigPayloadSkeleton,
            InitializeGuardianSetHeaderPayloadSkeleton, InitializeMintStatePayloadSkeleton,
        },
        typed_instruction_skeleton::encode_typed_instruction_skeleton,
    };
    use solana_program::pubkey::Pubkey;

    #[test]
    fn dispatch_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(XXXL_DISPATCH_SKELETON_STATUS, "LOCAL_ONLY_NOT_DEPLOYABLE");
    }

    #[test]
    fn dispatch_plans_initialize_gateway_config_with_expected_accounts() {
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
        let plan = plan_dispatch_skeleton(&data, 4).expect("dispatch plan ok");

        assert_eq!(plan.tag, XxxlGatewayInstructionTag::InitializeGatewayConfig);
        assert_eq!(plan.instruction, instruction);
        assert_eq!(plan.required_accounts.len(), 4);
    }

    #[test]
    fn dispatch_plans_initialize_guardian_set_with_expected_accounts() {
        let instruction =
            XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(
                InitializeGuardianSetHeaderPayloadSkeleton {
                    guardian_set_id: 7,
                    threshold: 2,
                    guardian_count: 3,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let plan = plan_dispatch_skeleton(&data, 4).expect("dispatch plan ok");

        assert_eq!(plan.tag, XxxlGatewayInstructionTag::InitializeGuardianSet);
        assert_eq!(plan.instruction, instruction);
        assert_eq!(plan.required_accounts.len(), 4);
    }

    #[test]
    fn dispatch_plans_initialize_mint_state_with_expected_accounts() {
        let instruction = XxxlTypedInstructionSkeleton::InitializeMintState(
            InitializeMintStatePayloadSkeleton {
                mint: Pubkey::new_unique(),
                decimals: 9,
            },
        );

        let data = encode_typed_instruction_skeleton(instruction);
        let plan = plan_dispatch_skeleton(&data, 7).expect("dispatch plan ok");

        assert_eq!(plan.tag, XxxlGatewayInstructionTag::InitializeMintState);
        assert_eq!(plan.instruction, instruction);
        assert_eq!(plan.required_accounts.len(), 7);
    }

    #[test]
    fn dispatch_plans_consume_gateway_mint_with_expected_accounts() {
        let instruction =
            XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                ConsumeGatewayMintPayloadBoundarySkeleton {
                    message_hash: [0x22; 32],
                    canonical_event_key: [0x33; 32],
                    mint_amount: 123_456,
                },
            );

        let data = encode_typed_instruction_skeleton(instruction);
        let plan = plan_dispatch_skeleton(&data, 11).expect("dispatch plan ok");

        assert_eq!(plan.tag, XxxlGatewayInstructionTag::ConsumeGatewayMint);
        assert_eq!(plan.instruction, instruction);
        assert_eq!(plan.required_accounts.len(), 11);
    }

    #[test]
    fn dispatch_rejects_invalid_instruction_data() {
        assert_eq!(
            plan_dispatch_skeleton(&[u8::MAX], 0),
            Err(XxxlDispatchSkeletonError::TypedInstruction(
                XxxlTypedInstructionSkeletonError::Codec(
                    crate::instruction_codec_skeleton::XxxlInstructionCodecSkeletonError::InvalidInstructionTag
                )
            ))
        );
    }

    #[test]
    fn dispatch_rejects_wrong_account_count_after_decode() {
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

        assert_eq!(
            plan_dispatch_skeleton(&data, 3),
            Err(XxxlDispatchSkeletonError::AccountOrder(
                XxxlAccountOrderSkeletonError::InvalidAccountCount
            ))
        );
    }
}

use crate::{
    instruction_codec_skeleton::{
        decode_instruction_skeleton, encode_instruction_skeleton,
        XxxlInstructionCodecSkeletonError,
    },
    instruction_payload_skeleton::{
        decode_consume_gateway_mint_payload_boundary_skeleton,
        decode_initialize_gateway_config_payload_skeleton,
        decode_initialize_guardian_set_header_payload_skeleton,
        decode_initialize_mint_state_payload_skeleton,
        encode_consume_gateway_mint_payload_boundary_skeleton,
        encode_initialize_gateway_config_payload_skeleton,
        encode_initialize_guardian_set_header_payload_skeleton,
        encode_initialize_mint_state_payload_skeleton,
        ConsumeGatewayMintPayloadBoundarySkeleton,
        InitializeGatewayConfigPayloadSkeleton,
        InitializeGuardianSetHeaderPayloadSkeleton,
        InitializeMintStatePayloadSkeleton,
        XxxlInstructionPayloadSkeletonError,
    },
    state_instruction_skeleton::XxxlGatewayInstructionTag,
};

pub const XXXL_TYPED_INSTRUCTION_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlTypedInstructionSkeleton {
    InitializeGatewayConfig(InitializeGatewayConfigPayloadSkeleton),
    InitializeGuardianSetHeader(InitializeGuardianSetHeaderPayloadSkeleton),
    InitializeMintState(InitializeMintStatePayloadSkeleton),
    ConsumeGatewayMintBoundary(ConsumeGatewayMintPayloadBoundarySkeleton),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlTypedInstructionSkeletonError {
    Codec(XxxlInstructionCodecSkeletonError),
    Payload(XxxlInstructionPayloadSkeletonError),
}

impl From<XxxlInstructionCodecSkeletonError> for XxxlTypedInstructionSkeletonError {
    fn from(value: XxxlInstructionCodecSkeletonError) -> Self {
        Self::Codec(value)
    }
}

impl From<XxxlInstructionPayloadSkeletonError> for XxxlTypedInstructionSkeletonError {
    fn from(value: XxxlInstructionPayloadSkeletonError) -> Self {
        Self::Payload(value)
    }
}

pub fn encode_typed_instruction_skeleton(instruction: XxxlTypedInstructionSkeleton) -> Vec<u8> {
    match instruction {
        XxxlTypedInstructionSkeleton::InitializeGatewayConfig(payload) => {
            encode_instruction_skeleton(
                XxxlGatewayInstructionTag::InitializeGatewayConfig,
                &encode_initialize_gateway_config_payload_skeleton(payload),
            )
        }
        XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(payload) => {
            encode_instruction_skeleton(
                XxxlGatewayInstructionTag::InitializeGuardianSet,
                &encode_initialize_guardian_set_header_payload_skeleton(payload),
            )
        }
        XxxlTypedInstructionSkeleton::InitializeMintState(payload) => encode_instruction_skeleton(
            XxxlGatewayInstructionTag::InitializeMintState,
            &encode_initialize_mint_state_payload_skeleton(payload),
        ),
        XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(payload) => {
            encode_instruction_skeleton(
                XxxlGatewayInstructionTag::ConsumeGatewayMint,
                &encode_consume_gateway_mint_payload_boundary_skeleton(payload),
            )
        }
    }
}

pub fn decode_typed_instruction_skeleton(
    data: &[u8],
) -> Result<XxxlTypedInstructionSkeleton, XxxlTypedInstructionSkeletonError> {
    let decoded = decode_instruction_skeleton(data)?;

    match decoded.tag {
        XxxlGatewayInstructionTag::InitializeGatewayConfig => {
            Ok(XxxlTypedInstructionSkeleton::InitializeGatewayConfig(
                decode_initialize_gateway_config_payload_skeleton(decoded.payload)?,
            ))
        }
        XxxlGatewayInstructionTag::InitializeGuardianSet => {
            Ok(XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(
                decode_initialize_guardian_set_header_payload_skeleton(decoded.payload)?,
            ))
        }
        XxxlGatewayInstructionTag::InitializeMintState => {
            Ok(XxxlTypedInstructionSkeleton::InitializeMintState(
                decode_initialize_mint_state_payload_skeleton(decoded.payload)?,
            ))
        }
        XxxlGatewayInstructionTag::ConsumeGatewayMint => {
            Ok(XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                decode_consume_gateway_mint_payload_boundary_skeleton(decoded.payload)?,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn typed_instruction_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_TYPED_INSTRUCTION_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn gateway_config_typed_instruction_round_trips() {
        let instruction =
            XxxlTypedInstructionSkeleton::InitializeGatewayConfig(
                InitializeGatewayConfigPayloadSkeleton {
                    route_id: [0x11; 32],
                    source_chain_id: 1,
                    guardian_set_id: 7,
                    is_active: true,
                },
            );

        let encoded = encode_typed_instruction_skeleton(instruction);
        let decoded = decode_typed_instruction_skeleton(&encoded).expect("decode ok");

        assert_eq!(decoded, instruction);
    }

    #[test]
    fn guardian_set_typed_instruction_round_trips() {
        let instruction =
            XxxlTypedInstructionSkeleton::InitializeGuardianSetHeader(
                InitializeGuardianSetHeaderPayloadSkeleton {
                    guardian_set_id: 7,
                    threshold: 2,
                    guardian_count: 3,
                },
            );

        let encoded = encode_typed_instruction_skeleton(instruction);
        let decoded = decode_typed_instruction_skeleton(&encoded).expect("decode ok");

        assert_eq!(decoded, instruction);
    }

    #[test]
    fn mint_state_typed_instruction_round_trips() {
        let instruction = XxxlTypedInstructionSkeleton::InitializeMintState(
            InitializeMintStatePayloadSkeleton {
                mint: Pubkey::new_unique(),
                decimals: 9,
            },
        );

        let encoded = encode_typed_instruction_skeleton(instruction);
        let decoded = decode_typed_instruction_skeleton(&encoded).expect("decode ok");

        assert_eq!(decoded, instruction);
    }

    #[test]
    fn consume_gateway_mint_boundary_typed_instruction_round_trips() {
        let instruction =
            XxxlTypedInstructionSkeleton::ConsumeGatewayMintBoundary(
                ConsumeGatewayMintPayloadBoundarySkeleton {
                    message_hash: [0x22; 32],
                    canonical_event_key: [0x33; 32],
                    mint_amount: 123_456,
                },
            );

        let encoded = encode_typed_instruction_skeleton(instruction);
        let decoded = decode_typed_instruction_skeleton(&encoded).expect("decode ok");

        assert_eq!(decoded, instruction);
    }

    #[test]
    fn typed_instruction_decode_rejects_invalid_tag() {
        assert_eq!(
            decode_typed_instruction_skeleton(&[u8::MAX]),
            Err(XxxlTypedInstructionSkeletonError::Codec(
                XxxlInstructionCodecSkeletonError::InvalidInstructionTag
            ))
        );
    }

    #[test]
    fn typed_instruction_decode_rejects_wrong_payload_length() {
        assert_eq!(
            decode_typed_instruction_skeleton(&[u8::from(
                XxxlGatewayInstructionTag::InitializeGatewayConfig
            )]),
            Err(XxxlTypedInstructionSkeletonError::Payload(
                XxxlInstructionPayloadSkeletonError::InvalidPayloadLength
            ))
        );
    }

    #[test]
    fn typed_instruction_decode_rejects_invalid_guardian_threshold() {
        let invalid_guardian_set = InitializeGuardianSetHeaderPayloadSkeleton {
            guardian_set_id: 7,
            threshold: 4,
            guardian_count: 3,
        };

        let encoded = encode_instruction_skeleton(
            XxxlGatewayInstructionTag::InitializeGuardianSet,
            &encode_initialize_guardian_set_header_payload_skeleton(invalid_guardian_set),
        );

        assert_eq!(
            decode_typed_instruction_skeleton(&encoded),
            Err(XxxlTypedInstructionSkeletonError::Payload(
                XxxlInstructionPayloadSkeletonError::GuardianThresholdOutOfBounds
            ))
        );
    }
}

use solana_program::pubkey::Pubkey;

pub const XXXL_INSTRUCTION_PAYLOAD_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

pub const GATEWAY_CONFIG_PAYLOAD_LEN: usize = 49;
pub const GUARDIAN_SET_HEADER_PAYLOAD_LEN: usize = 10;
pub const MINT_STATE_PAYLOAD_LEN: usize = 33;
pub const CONSUME_GATEWAY_MINT_PAYLOAD_BOUNDARY_LEN: usize = 80;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlInstructionPayloadSkeletonError {
    InvalidPayloadLength,
    InvalidBoolean,
    GuardianThresholdOutOfBounds,
    GuardianCountZero,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeGatewayConfigPayloadSkeleton {
    pub route_id: [u8; 32],
    pub source_chain_id: u64,
    pub guardian_set_id: u64,
    pub is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeGuardianSetHeaderPayloadSkeleton {
    pub guardian_set_id: u64,
    pub threshold: u8,
    pub guardian_count: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeMintStatePayloadSkeleton {
    pub mint: Pubkey,
    pub decimals: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConsumeGatewayMintPayloadBoundarySkeleton {
    pub message_hash: [u8; 32],
    pub canonical_event_key: [u8; 32],
    pub mint_amount: u128,
}

fn require_len(input: &[u8], expected: usize) -> Result<(), XxxlInstructionPayloadSkeletonError> {
    if input.len() == expected {
        Ok(())
    } else {
        Err(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength)
    }
}

fn read_array<const N: usize>(
    input: &[u8],
    offset: usize,
) -> Result<[u8; N], XxxlInstructionPayloadSkeletonError> {
    let end = offset
        .checked_add(N)
        .ok_or(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength)?;

    if input.len() < end {
        return Err(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength);
    }

    let mut output = [0_u8; N];
    output.copy_from_slice(&input[offset..end]);
    Ok(output)
}

fn read_u64_le(input: &[u8], offset: usize) -> Result<u64, XxxlInstructionPayloadSkeletonError> {
    Ok(u64::from_le_bytes(read_array::<8>(input, offset)?))
}

fn read_u128_le(input: &[u8], offset: usize) -> Result<u128, XxxlInstructionPayloadSkeletonError> {
    Ok(u128::from_le_bytes(read_array::<16>(input, offset)?))
}

fn read_bool(input: &[u8], offset: usize) -> Result<bool, XxxlInstructionPayloadSkeletonError> {
    match input
        .get(offset)
        .copied()
        .ok_or(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength)?
    {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(XxxlInstructionPayloadSkeletonError::InvalidBoolean),
    }
}

pub fn encode_initialize_gateway_config_payload_skeleton(
    payload: InitializeGatewayConfigPayloadSkeleton,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(GATEWAY_CONFIG_PAYLOAD_LEN);
    output.extend_from_slice(&payload.route_id);
    output.extend_from_slice(&payload.source_chain_id.to_le_bytes());
    output.extend_from_slice(&payload.guardian_set_id.to_le_bytes());
    output.push(u8::from(payload.is_active));
    output
}

pub fn decode_initialize_gateway_config_payload_skeleton(
    input: &[u8],
) -> Result<InitializeGatewayConfigPayloadSkeleton, XxxlInstructionPayloadSkeletonError> {
    require_len(input, GATEWAY_CONFIG_PAYLOAD_LEN)?;

    Ok(InitializeGatewayConfigPayloadSkeleton {
        route_id: read_array::<32>(input, 0)?,
        source_chain_id: read_u64_le(input, 32)?,
        guardian_set_id: read_u64_le(input, 40)?,
        is_active: read_bool(input, 48)?,
    })
}

pub fn encode_initialize_guardian_set_header_payload_skeleton(
    payload: InitializeGuardianSetHeaderPayloadSkeleton,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(GUARDIAN_SET_HEADER_PAYLOAD_LEN);
    output.extend_from_slice(&payload.guardian_set_id.to_le_bytes());
    output.push(payload.threshold);
    output.push(payload.guardian_count);
    output
}

pub fn decode_initialize_guardian_set_header_payload_skeleton(
    input: &[u8],
) -> Result<InitializeGuardianSetHeaderPayloadSkeleton, XxxlInstructionPayloadSkeletonError> {
    require_len(input, GUARDIAN_SET_HEADER_PAYLOAD_LEN)?;

    let payload = InitializeGuardianSetHeaderPayloadSkeleton {
        guardian_set_id: read_u64_le(input, 0)?,
        threshold: input[8],
        guardian_count: input[9],
    };

    validate_guardian_set_header_payload_skeleton(payload)?;

    Ok(payload)
}

pub fn validate_guardian_set_header_payload_skeleton(
    payload: InitializeGuardianSetHeaderPayloadSkeleton,
) -> Result<(), XxxlInstructionPayloadSkeletonError> {
    if payload.guardian_count == 0 {
        return Err(XxxlInstructionPayloadSkeletonError::GuardianCountZero);
    }

    if payload.threshold == 0 || payload.threshold > payload.guardian_count {
        return Err(XxxlInstructionPayloadSkeletonError::GuardianThresholdOutOfBounds);
    }

    Ok(())
}

pub fn encode_initialize_mint_state_payload_skeleton(
    payload: InitializeMintStatePayloadSkeleton,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(MINT_STATE_PAYLOAD_LEN);
    output.extend_from_slice(payload.mint.as_ref());
    output.push(payload.decimals);
    output
}

pub fn decode_initialize_mint_state_payload_skeleton(
    input: &[u8],
) -> Result<InitializeMintStatePayloadSkeleton, XxxlInstructionPayloadSkeletonError> {
    require_len(input, MINT_STATE_PAYLOAD_LEN)?;

    Ok(InitializeMintStatePayloadSkeleton {
        mint: Pubkey::new_from_array(read_array::<32>(input, 0)?),
        decimals: input[32],
    })
}

pub fn encode_consume_gateway_mint_payload_boundary_skeleton(
    payload: ConsumeGatewayMintPayloadBoundarySkeleton,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(CONSUME_GATEWAY_MINT_PAYLOAD_BOUNDARY_LEN);
    output.extend_from_slice(&payload.message_hash);
    output.extend_from_slice(&payload.canonical_event_key);
    output.extend_from_slice(&payload.mint_amount.to_le_bytes());
    output
}

pub fn decode_consume_gateway_mint_payload_boundary_skeleton(
    input: &[u8],
) -> Result<ConsumeGatewayMintPayloadBoundarySkeleton, XxxlInstructionPayloadSkeletonError> {
    require_len(input, CONSUME_GATEWAY_MINT_PAYLOAD_BOUNDARY_LEN)?;

    Ok(ConsumeGatewayMintPayloadBoundarySkeleton {
        message_hash: read_array::<32>(input, 0)?,
        canonical_event_key: read_array::<32>(input, 32)?,
        mint_amount: read_u128_le(input, 64)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_INSTRUCTION_PAYLOAD_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn gateway_config_payload_round_trip_preserves_fields() {
        let payload = InitializeGatewayConfigPayloadSkeleton {
            route_id: [0x11; 32],
            source_chain_id: 1_337,
            guardian_set_id: 7,
            is_active: true,
        };

        let encoded = encode_initialize_gateway_config_payload_skeleton(payload);
        let decoded =
            decode_initialize_gateway_config_payload_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), GATEWAY_CONFIG_PAYLOAD_LEN);
        assert_eq!(decoded, payload);
    }

    #[test]
    fn gateway_config_payload_rejects_invalid_bool() {
        let mut encoded =
            encode_initialize_gateway_config_payload_skeleton(InitializeGatewayConfigPayloadSkeleton {
                route_id: [0x11; 32],
                source_chain_id: 1,
                guardian_set_id: 1,
                is_active: true,
            });

        encoded[48] = 2;

        assert_eq!(
            decode_initialize_gateway_config_payload_skeleton(&encoded),
            Err(XxxlInstructionPayloadSkeletonError::InvalidBoolean)
        );
    }

    #[test]
    fn guardian_set_header_round_trip_preserves_fields() {
        let payload = InitializeGuardianSetHeaderPayloadSkeleton {
            guardian_set_id: 42,
            threshold: 2,
            guardian_count: 3,
        };

        let encoded = encode_initialize_guardian_set_header_payload_skeleton(payload);
        let decoded =
            decode_initialize_guardian_set_header_payload_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), GUARDIAN_SET_HEADER_PAYLOAD_LEN);
        assert_eq!(decoded, payload);
    }

    #[test]
    fn guardian_set_header_rejects_zero_guardian_count() {
        let payload = InitializeGuardianSetHeaderPayloadSkeleton {
            guardian_set_id: 42,
            threshold: 1,
            guardian_count: 0,
        };

        assert_eq!(
            validate_guardian_set_header_payload_skeleton(payload),
            Err(XxxlInstructionPayloadSkeletonError::GuardianCountZero)
        );
    }

    #[test]
    fn guardian_set_header_rejects_invalid_threshold() {
        let zero_threshold = InitializeGuardianSetHeaderPayloadSkeleton {
            guardian_set_id: 42,
            threshold: 0,
            guardian_count: 3,
        };

        let too_high_threshold = InitializeGuardianSetHeaderPayloadSkeleton {
            guardian_set_id: 42,
            threshold: 4,
            guardian_count: 3,
        };

        assert_eq!(
            validate_guardian_set_header_payload_skeleton(zero_threshold),
            Err(XxxlInstructionPayloadSkeletonError::GuardianThresholdOutOfBounds)
        );

        assert_eq!(
            validate_guardian_set_header_payload_skeleton(too_high_threshold),
            Err(XxxlInstructionPayloadSkeletonError::GuardianThresholdOutOfBounds)
        );
    }

    #[test]
    fn mint_state_payload_round_trip_preserves_fields() {
        let payload = InitializeMintStatePayloadSkeleton {
            mint: Pubkey::new_unique(),
            decimals: 9,
        };

        let encoded = encode_initialize_mint_state_payload_skeleton(payload);
        let decoded = decode_initialize_mint_state_payload_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), MINT_STATE_PAYLOAD_LEN);
        assert_eq!(decoded, payload);
    }

    #[test]
    fn consume_gateway_mint_payload_boundary_round_trip_preserves_fields() {
        let payload = ConsumeGatewayMintPayloadBoundarySkeleton {
            message_hash: [0x22; 32],
            canonical_event_key: [0x33; 32],
            mint_amount: 123_456_789,
        };

        let encoded = encode_consume_gateway_mint_payload_boundary_skeleton(payload);
        let decoded =
            decode_consume_gateway_mint_payload_boundary_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), CONSUME_GATEWAY_MINT_PAYLOAD_BOUNDARY_LEN);
        assert_eq!(decoded, payload);
    }

    #[test]
    fn payload_decoders_reject_wrong_lengths() {
        assert_eq!(
            decode_initialize_gateway_config_payload_skeleton(&[]),
            Err(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength)
        );
        assert_eq!(
            decode_initialize_guardian_set_header_payload_skeleton(&[0; 9]),
            Err(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength)
        );
        assert_eq!(
            decode_initialize_mint_state_payload_skeleton(&[0; 32]),
            Err(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength)
        );
        assert_eq!(
            decode_consume_gateway_mint_payload_boundary_skeleton(&[0; 79]),
            Err(XxxlInstructionPayloadSkeletonError::InvalidPayloadLength)
        );
    }
}

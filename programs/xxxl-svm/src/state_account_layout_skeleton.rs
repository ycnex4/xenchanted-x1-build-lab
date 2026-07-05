use crate::state_instruction_skeleton::{
    GATEWAY_CONFIG_DISCRIMINATOR, GUARDIAN_SET_DISCRIMINATOR, MINT_STATE_DISCRIMINATOR,
    PROCESSED_EVENT_DISCRIMINATOR,
};
use solana_program::pubkey::Pubkey;

pub const XXXL_STATE_ACCOUNT_LAYOUT_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

pub const GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN: usize = 91;
pub const GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN: usize = 21;
pub const MINT_STATE_ACCOUNT_SKELETON_LEN: usize = 125;
pub const PROCESSED_EVENT_ACCOUNT_SKELETON_LEN: usize = 166;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlStateAccountLayoutSkeletonError {
    InvalidDataLength,
    InvalidDiscriminator,
    InvalidBoolean,
    GuardianCountZero,
    GuardianThresholdOutOfBounds,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GatewayConfigAccountLayoutSkeleton {
    pub version: u8,
    pub bump: u8,
    pub route_id: [u8; 32],
    pub source_chain_id: u64,
    pub guardian_set_id: u64,
    pub gateway_mint_authority_pda: Pubkey,
    pub is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianSetHeaderAccountLayoutSkeleton {
    pub version: u8,
    pub bump: u8,
    pub guardian_set_id: u64,
    pub threshold: u8,
    pub guardian_count: u8,
    pub status: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MintStateAccountLayoutSkeleton {
    pub version: u8,
    pub bump: u8,
    pub mint: Pubkey,
    pub mint_authority_pda: Pubkey,
    pub mint_authority_bump: u8,
    pub token_program: Pubkey,
    pub decimals: u8,
    pub total_minted: u128,
    pub is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessedEventAccountLayoutSkeleton {
    pub version: u8,
    pub bump: u8,
    pub canonical_event_key: [u8; 32],
    pub message_hash: [u8; 32],
    pub source_burn_tx_hash: [u8; 32],
    pub source_burn_event_index: u32,
    pub recipient_hash: [u8; 32],
    pub minted_amount: u128,
    pub processed_at_slot: u64,
}

fn require_len(input: &[u8], expected: usize) -> Result<(), XxxlStateAccountLayoutSkeletonError> {
    if input.len() == expected {
        Ok(())
    } else {
        Err(XxxlStateAccountLayoutSkeletonError::InvalidDataLength)
    }
}

fn read_array<const N: usize>(
    input: &[u8],
    offset: usize,
) -> Result<[u8; N], XxxlStateAccountLayoutSkeletonError> {
    let end = offset
        .checked_add(N)
        .ok_or(XxxlStateAccountLayoutSkeletonError::InvalidDataLength)?;

    if input.len() < end {
        return Err(XxxlStateAccountLayoutSkeletonError::InvalidDataLength);
    }

    let mut output = [0_u8; N];
    output.copy_from_slice(&input[offset..end]);
    Ok(output)
}

fn read_u32_le(input: &[u8], offset: usize) -> Result<u32, XxxlStateAccountLayoutSkeletonError> {
    Ok(u32::from_le_bytes(read_array::<4>(input, offset)?))
}

fn read_u64_le(input: &[u8], offset: usize) -> Result<u64, XxxlStateAccountLayoutSkeletonError> {
    Ok(u64::from_le_bytes(read_array::<8>(input, offset)?))
}

fn read_u128_le(input: &[u8], offset: usize) -> Result<u128, XxxlStateAccountLayoutSkeletonError> {
    Ok(u128::from_le_bytes(read_array::<16>(input, offset)?))
}

fn read_bool(input: &[u8], offset: usize) -> Result<bool, XxxlStateAccountLayoutSkeletonError> {
    match input
        .get(offset)
        .copied()
        .ok_or(XxxlStateAccountLayoutSkeletonError::InvalidDataLength)?
    {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(XxxlStateAccountLayoutSkeletonError::InvalidBoolean),
    }
}

fn require_discriminator(
    input: &[u8],
    expected: [u8; 8],
) -> Result<(), XxxlStateAccountLayoutSkeletonError> {
    let actual = read_array::<8>(input, 0)?;

    if actual == expected {
        Ok(())
    } else {
        Err(XxxlStateAccountLayoutSkeletonError::InvalidDiscriminator)
    }
}

pub fn encode_gateway_config_account_layout_skeleton(
    account: GatewayConfigAccountLayoutSkeleton,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN);
    output.extend_from_slice(&GATEWAY_CONFIG_DISCRIMINATOR);
    output.push(account.version);
    output.push(account.bump);
    output.extend_from_slice(&account.route_id);
    output.extend_from_slice(&account.source_chain_id.to_le_bytes());
    output.extend_from_slice(&account.guardian_set_id.to_le_bytes());
    output.extend_from_slice(account.gateway_mint_authority_pda.as_ref());
    output.push(u8::from(account.is_active));
    output
}

pub fn decode_gateway_config_account_layout_skeleton(
    input: &[u8],
) -> Result<GatewayConfigAccountLayoutSkeleton, XxxlStateAccountLayoutSkeletonError> {
    require_len(input, GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN)?;
    require_discriminator(input, GATEWAY_CONFIG_DISCRIMINATOR)?;

    Ok(GatewayConfigAccountLayoutSkeleton {
        version: input[8],
        bump: input[9],
        route_id: read_array::<32>(input, 10)?,
        source_chain_id: read_u64_le(input, 42)?,
        guardian_set_id: read_u64_le(input, 50)?,
        gateway_mint_authority_pda: Pubkey::new_from_array(read_array::<32>(input, 58)?),
        is_active: read_bool(input, 90)?,
    })
}

pub fn validate_guardian_set_header_account_layout_skeleton(
    account: GuardianSetHeaderAccountLayoutSkeleton,
) -> Result<(), XxxlStateAccountLayoutSkeletonError> {
    if account.guardian_count == 0 {
        return Err(XxxlStateAccountLayoutSkeletonError::GuardianCountZero);
    }

    if account.threshold == 0 || account.threshold > account.guardian_count {
        return Err(XxxlStateAccountLayoutSkeletonError::GuardianThresholdOutOfBounds);
    }

    Ok(())
}

pub fn encode_guardian_set_header_account_layout_skeleton(
    account: GuardianSetHeaderAccountLayoutSkeleton,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN);
    output.extend_from_slice(&GUARDIAN_SET_DISCRIMINATOR);
    output.push(account.version);
    output.push(account.bump);
    output.extend_from_slice(&account.guardian_set_id.to_le_bytes());
    output.push(account.threshold);
    output.push(account.guardian_count);
    output.push(account.status);
    output
}

pub fn decode_guardian_set_header_account_layout_skeleton(
    input: &[u8],
) -> Result<GuardianSetHeaderAccountLayoutSkeleton, XxxlStateAccountLayoutSkeletonError> {
    require_len(input, GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN)?;
    require_discriminator(input, GUARDIAN_SET_DISCRIMINATOR)?;

    let account = GuardianSetHeaderAccountLayoutSkeleton {
        version: input[8],
        bump: input[9],
        guardian_set_id: read_u64_le(input, 10)?,
        threshold: input[18],
        guardian_count: input[19],
        status: input[20],
    };

    validate_guardian_set_header_account_layout_skeleton(account)?;

    Ok(account)
}

pub fn encode_mint_state_account_layout_skeleton(account: MintStateAccountLayoutSkeleton) -> Vec<u8> {
    let mut output = Vec::with_capacity(MINT_STATE_ACCOUNT_SKELETON_LEN);
    output.extend_from_slice(&MINT_STATE_DISCRIMINATOR);
    output.push(account.version);
    output.push(account.bump);
    output.extend_from_slice(account.mint.as_ref());
    output.extend_from_slice(account.mint_authority_pda.as_ref());
    output.push(account.mint_authority_bump);
    output.extend_from_slice(account.token_program.as_ref());
    output.push(account.decimals);
    output.extend_from_slice(&account.total_minted.to_le_bytes());
    output.push(u8::from(account.is_active));
    output
}

pub fn decode_mint_state_account_layout_skeleton(
    input: &[u8],
) -> Result<MintStateAccountLayoutSkeleton, XxxlStateAccountLayoutSkeletonError> {
    require_len(input, MINT_STATE_ACCOUNT_SKELETON_LEN)?;
    require_discriminator(input, MINT_STATE_DISCRIMINATOR)?;

    Ok(MintStateAccountLayoutSkeleton {
        version: input[8],
        bump: input[9],
        mint: Pubkey::new_from_array(read_array::<32>(input, 10)?),
        mint_authority_pda: Pubkey::new_from_array(read_array::<32>(input, 42)?),
        mint_authority_bump: input[74],
        token_program: Pubkey::new_from_array(read_array::<32>(input, 75)?),
        decimals: input[107],
        total_minted: read_u128_le(input, 108)?,
        is_active: read_bool(input, 124)?,
    })
}

pub fn encode_processed_event_account_layout_skeleton(
    account: ProcessedEventAccountLayoutSkeleton,
) -> Vec<u8> {
    let mut output = Vec::with_capacity(PROCESSED_EVENT_ACCOUNT_SKELETON_LEN);
    output.extend_from_slice(&PROCESSED_EVENT_DISCRIMINATOR);
    output.push(account.version);
    output.push(account.bump);
    output.extend_from_slice(&account.canonical_event_key);
    output.extend_from_slice(&account.message_hash);
    output.extend_from_slice(&account.source_burn_tx_hash);
    output.extend_from_slice(&account.source_burn_event_index.to_le_bytes());
    output.extend_from_slice(&account.recipient_hash);
    output.extend_from_slice(&account.minted_amount.to_le_bytes());
    output.extend_from_slice(&account.processed_at_slot.to_le_bytes());
    output
}

pub fn decode_processed_event_account_layout_skeleton(
    input: &[u8],
) -> Result<ProcessedEventAccountLayoutSkeleton, XxxlStateAccountLayoutSkeletonError> {
    require_len(input, PROCESSED_EVENT_ACCOUNT_SKELETON_LEN)?;
    require_discriminator(input, PROCESSED_EVENT_DISCRIMINATOR)?;

    Ok(ProcessedEventAccountLayoutSkeleton {
        version: input[8],
        bump: input[9],
        canonical_event_key: read_array::<32>(input, 10)?,
        message_hash: read_array::<32>(input, 42)?,
        source_burn_tx_hash: read_array::<32>(input, 74)?,
        source_burn_event_index: read_u32_le(input, 106)?,
        recipient_hash: read_array::<32>(input, 110)?,
        minted_amount: read_u128_le(input, 142)?,
        processed_at_slot: read_u64_le(input, 158)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_account_layout_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_STATE_ACCOUNT_LAYOUT_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn state_account_layout_lengths_are_stable() {
        assert_eq!(GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN, 91);
        assert_eq!(GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN, 21);
        assert_eq!(MINT_STATE_ACCOUNT_SKELETON_LEN, 125);
        assert_eq!(PROCESSED_EVENT_ACCOUNT_SKELETON_LEN, 166);
    }

    #[test]
    fn gateway_config_account_layout_round_trips() {
        let account = GatewayConfigAccountLayoutSkeleton {
            version: 1,
            bump: 252,
            route_id: [0x11; 32],
            source_chain_id: 1,
            guardian_set_id: 7,
            gateway_mint_authority_pda: Pubkey::new_unique(),
            is_active: true,
        };

        let encoded = encode_gateway_config_account_layout_skeleton(account);
        let decoded = decode_gateway_config_account_layout_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN);
        assert_eq!(decoded, account);
    }

    #[test]
    fn guardian_set_header_account_layout_round_trips() {
        let account = GuardianSetHeaderAccountLayoutSkeleton {
            version: 1,
            bump: 251,
            guardian_set_id: 7,
            threshold: 2,
            guardian_count: 3,
            status: 1,
        };

        let encoded = encode_guardian_set_header_account_layout_skeleton(account);
        let decoded =
            decode_guardian_set_header_account_layout_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN);
        assert_eq!(decoded, account);
    }

    #[test]
    fn mint_state_account_layout_round_trips() {
        let account = MintStateAccountLayoutSkeleton {
            version: 1,
            bump: 250,
            mint: Pubkey::new_unique(),
            mint_authority_pda: Pubkey::new_unique(),
            mint_authority_bump: 252,
            token_program: Pubkey::new_unique(),
            decimals: 9,
            total_minted: 123_456,
            is_active: true,
        };

        let encoded = encode_mint_state_account_layout_skeleton(account);
        let decoded = decode_mint_state_account_layout_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), MINT_STATE_ACCOUNT_SKELETON_LEN);
        assert_eq!(decoded, account);
    }

    #[test]
    fn processed_event_account_layout_round_trips() {
        let account = ProcessedEventAccountLayoutSkeleton {
            version: 1,
            bump: 249,
            canonical_event_key: [0x11; 32],
            message_hash: [0x22; 32],
            source_burn_tx_hash: [0x33; 32],
            source_burn_event_index: 4,
            recipient_hash: [0x44; 32],
            minted_amount: 123_456,
            processed_at_slot: 99,
        };

        let encoded = encode_processed_event_account_layout_skeleton(account);
        let decoded = decode_processed_event_account_layout_skeleton(&encoded).expect("decode ok");

        assert_eq!(encoded.len(), PROCESSED_EVENT_ACCOUNT_SKELETON_LEN);
        assert_eq!(decoded, account);
    }

    #[test]
    fn decoder_rejects_wrong_length() {
        assert_eq!(
            decode_gateway_config_account_layout_skeleton(&[]),
            Err(XxxlStateAccountLayoutSkeletonError::InvalidDataLength)
        );
        assert_eq!(
            decode_mint_state_account_layout_skeleton(&[0; 124]),
            Err(XxxlStateAccountLayoutSkeletonError::InvalidDataLength)
        );
    }

    #[test]
    fn decoder_rejects_wrong_discriminator() {
        let account = GatewayConfigAccountLayoutSkeleton {
            version: 1,
            bump: 252,
            route_id: [0x11; 32],
            source_chain_id: 1,
            guardian_set_id: 7,
            gateway_mint_authority_pda: Pubkey::new_unique(),
            is_active: true,
        };

        let mut encoded = encode_gateway_config_account_layout_skeleton(account);
        encoded[0] = 0xff;

        assert_eq!(
            decode_gateway_config_account_layout_skeleton(&encoded),
            Err(XxxlStateAccountLayoutSkeletonError::InvalidDiscriminator)
        );
    }

    #[test]
    fn decoder_rejects_invalid_boolean() {
        let account = GatewayConfigAccountLayoutSkeleton {
            version: 1,
            bump: 252,
            route_id: [0x11; 32],
            source_chain_id: 1,
            guardian_set_id: 7,
            gateway_mint_authority_pda: Pubkey::new_unique(),
            is_active: true,
        };

        let mut encoded = encode_gateway_config_account_layout_skeleton(account);
        encoded[90] = 2;

        assert_eq!(
            decode_gateway_config_account_layout_skeleton(&encoded),
            Err(XxxlStateAccountLayoutSkeletonError::InvalidBoolean)
        );
    }

    #[test]
    fn guardian_set_header_validation_rejects_invalid_threshold() {
        let zero_count = GuardianSetHeaderAccountLayoutSkeleton {
            version: 1,
            bump: 251,
            guardian_set_id: 7,
            threshold: 1,
            guardian_count: 0,
            status: 1,
        };

        let too_high = GuardianSetHeaderAccountLayoutSkeleton {
            version: 1,
            bump: 251,
            guardian_set_id: 7,
            threshold: 4,
            guardian_count: 3,
            status: 1,
        };

        assert_eq!(
            validate_guardian_set_header_account_layout_skeleton(zero_count),
            Err(XxxlStateAccountLayoutSkeletonError::GuardianCountZero)
        );

        assert_eq!(
            validate_guardian_set_header_account_layout_skeleton(too_high),
            Err(XxxlStateAccountLayoutSkeletonError::GuardianThresholdOutOfBounds)
        );
    }
}

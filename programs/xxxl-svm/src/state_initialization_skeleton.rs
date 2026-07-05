use crate::{
    instruction_payload_skeleton::{
        ConsumeGatewayMintPayloadBoundarySkeleton, InitializeGatewayConfigPayloadSkeleton,
        InitializeGuardianSetHeaderPayloadSkeleton, InitializeMintStatePayloadSkeleton,
    },
    state_account_layout_skeleton::{
        decode_gateway_config_account_layout_skeleton,
        decode_guardian_set_header_account_layout_skeleton,
        decode_mint_state_account_layout_skeleton,
        decode_processed_event_account_layout_skeleton,
        encode_gateway_config_account_layout_skeleton,
        encode_guardian_set_header_account_layout_skeleton,
        encode_mint_state_account_layout_skeleton,
        encode_processed_event_account_layout_skeleton,
        GatewayConfigAccountLayoutSkeleton,
        GuardianSetHeaderAccountLayoutSkeleton,
        MintStateAccountLayoutSkeleton,
        ProcessedEventAccountLayoutSkeleton,
        XxxlStateAccountLayoutSkeletonError,
    },
};
use solana_program::pubkey::Pubkey;

pub const XXXL_STATE_INITIALIZATION_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlStateInitializationSkeletonError {
    Layout(XxxlStateAccountLayoutSkeletonError),
}

impl From<XxxlStateAccountLayoutSkeletonError> for XxxlStateInitializationSkeletonError {
    fn from(value: XxxlStateAccountLayoutSkeletonError) -> Self {
        Self::Layout(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GatewayConfigInitializationSkeletonInput {
    pub payload: InitializeGatewayConfigPayloadSkeleton,
    pub bump: u8,
    pub gateway_mint_authority_pda: Pubkey,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianSetHeaderInitializationSkeletonInput {
    pub payload: InitializeGuardianSetHeaderPayloadSkeleton,
    pub bump: u8,
    pub status: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MintStateInitializationSkeletonInput {
    pub payload: InitializeMintStatePayloadSkeleton,
    pub bump: u8,
    pub mint_authority_pda: Pubkey,
    pub mint_authority_bump: u8,
    pub token_program: Pubkey,
    pub total_minted: u128,
    pub is_active: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessedEventMarkingSkeletonInput {
    pub payload: ConsumeGatewayMintPayloadBoundarySkeleton,
    pub bump: u8,
    pub source_burn_tx_hash: [u8; 32],
    pub source_burn_event_index: u32,
    pub recipient_hash: [u8; 32],
    pub processed_at_slot: u64,
}

pub fn build_gateway_config_account_layout_skeleton(
    input: GatewayConfigInitializationSkeletonInput,
) -> GatewayConfigAccountLayoutSkeleton {
    GatewayConfigAccountLayoutSkeleton {
        version: 1,
        bump: input.bump,
        route_id: input.payload.route_id,
        source_chain_id: input.payload.source_chain_id,
        guardian_set_id: input.payload.guardian_set_id,
        gateway_mint_authority_pda: input.gateway_mint_authority_pda,
        is_active: input.payload.is_active,
    }
}

pub fn build_guardian_set_header_account_layout_skeleton(
    input: GuardianSetHeaderInitializationSkeletonInput,
) -> Result<GuardianSetHeaderAccountLayoutSkeleton, XxxlStateInitializationSkeletonError> {
    let account = GuardianSetHeaderAccountLayoutSkeleton {
        version: 1,
        bump: input.bump,
        guardian_set_id: input.payload.guardian_set_id,
        threshold: input.payload.threshold,
        guardian_count: input.payload.guardian_count,
        status: input.status,
    };

    let encoded = encode_guardian_set_header_account_layout_skeleton(account);
    let decoded = decode_guardian_set_header_account_layout_skeleton(&encoded)?;

    Ok(decoded)
}

pub fn build_mint_state_account_layout_skeleton(
    input: MintStateInitializationSkeletonInput,
) -> MintStateAccountLayoutSkeleton {
    MintStateAccountLayoutSkeleton {
        version: 1,
        bump: input.bump,
        mint: input.payload.mint,
        mint_authority_pda: input.mint_authority_pda,
        mint_authority_bump: input.mint_authority_bump,
        token_program: input.token_program,
        decimals: input.payload.decimals,
        total_minted: input.total_minted,
        is_active: input.is_active,
    }
}

pub fn build_processed_event_account_layout_skeleton(
    input: ProcessedEventMarkingSkeletonInput,
) -> ProcessedEventAccountLayoutSkeleton {
    ProcessedEventAccountLayoutSkeleton {
        version: 1,
        bump: input.bump,
        canonical_event_key: input.payload.canonical_event_key,
        message_hash: input.payload.message_hash,
        source_burn_tx_hash: input.source_burn_tx_hash,
        source_burn_event_index: input.source_burn_event_index,
        recipient_hash: input.recipient_hash,
        minted_amount: input.payload.mint_amount,
        processed_at_slot: input.processed_at_slot,
    }
}

pub fn encode_and_verify_gateway_config_initialization_skeleton(
    input: GatewayConfigInitializationSkeletonInput,
) -> Result<Vec<u8>, XxxlStateInitializationSkeletonError> {
    let account = build_gateway_config_account_layout_skeleton(input);
    let encoded = encode_gateway_config_account_layout_skeleton(account);
    let decoded = decode_gateway_config_account_layout_skeleton(&encoded)?;

    if decoded == account {
        Ok(encoded)
    } else {
        Err(XxxlStateInitializationSkeletonError::Layout(
            XxxlStateAccountLayoutSkeletonError::InvalidDataLength,
        ))
    }
}

pub fn encode_and_verify_guardian_set_header_initialization_skeleton(
    input: GuardianSetHeaderInitializationSkeletonInput,
) -> Result<Vec<u8>, XxxlStateInitializationSkeletonError> {
    let account = build_guardian_set_header_account_layout_skeleton(input)?;
    let encoded = encode_guardian_set_header_account_layout_skeleton(account);
    let decoded = decode_guardian_set_header_account_layout_skeleton(&encoded)?;

    if decoded == account {
        Ok(encoded)
    } else {
        Err(XxxlStateInitializationSkeletonError::Layout(
            XxxlStateAccountLayoutSkeletonError::InvalidDataLength,
        ))
    }
}

pub fn encode_and_verify_mint_state_initialization_skeleton(
    input: MintStateInitializationSkeletonInput,
) -> Result<Vec<u8>, XxxlStateInitializationSkeletonError> {
    let account = build_mint_state_account_layout_skeleton(input);
    let encoded = encode_mint_state_account_layout_skeleton(account);
    let decoded = decode_mint_state_account_layout_skeleton(&encoded)?;

    if decoded == account {
        Ok(encoded)
    } else {
        Err(XxxlStateInitializationSkeletonError::Layout(
            XxxlStateAccountLayoutSkeletonError::InvalidDataLength,
        ))
    }
}

pub fn encode_and_verify_processed_event_marking_skeleton(
    input: ProcessedEventMarkingSkeletonInput,
) -> Result<Vec<u8>, XxxlStateInitializationSkeletonError> {
    let account = build_processed_event_account_layout_skeleton(input);
    let encoded = encode_processed_event_account_layout_skeleton(account);
    let decoded = decode_processed_event_account_layout_skeleton(&encoded)?;

    if decoded == account {
        Ok(encoded)
    } else {
        Err(XxxlStateInitializationSkeletonError::Layout(
            XxxlStateAccountLayoutSkeletonError::InvalidDataLength,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_account_layout_skeleton::{
        GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN, GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN,
        MINT_STATE_ACCOUNT_SKELETON_LEN, PROCESSED_EVENT_ACCOUNT_SKELETON_LEN,
    };

    #[test]
    fn state_initialization_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_STATE_INITIALIZATION_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn gateway_config_initialization_builds_expected_account_layout() {
        let mint_authority = Pubkey::new_unique();
        let input = GatewayConfigInitializationSkeletonInput {
            payload: InitializeGatewayConfigPayloadSkeleton {
                route_id: [0x11; 32],
                source_chain_id: 1,
                guardian_set_id: 7,
                is_active: true,
            },
            bump: 252,
            gateway_mint_authority_pda: mint_authority,
        };

        let account = build_gateway_config_account_layout_skeleton(input);

        assert_eq!(account.version, 1);
        assert_eq!(account.bump, 252);
        assert_eq!(account.route_id, [0x11; 32]);
        assert_eq!(account.source_chain_id, 1);
        assert_eq!(account.guardian_set_id, 7);
        assert_eq!(account.gateway_mint_authority_pda, mint_authority);
        assert!(account.is_active);
    }

    #[test]
    fn gateway_config_initialization_encodes_and_decodes() {
        let input = GatewayConfigInitializationSkeletonInput {
            payload: InitializeGatewayConfigPayloadSkeleton {
                route_id: [0x11; 32],
                source_chain_id: 1,
                guardian_set_id: 7,
                is_active: true,
            },
            bump: 252,
            gateway_mint_authority_pda: Pubkey::new_unique(),
        };

        let encoded =
            encode_and_verify_gateway_config_initialization_skeleton(input).expect("encode ok");

        assert_eq!(encoded.len(), GATEWAY_CONFIG_ACCOUNT_SKELETON_LEN);
    }

    #[test]
    fn guardian_set_header_initialization_encodes_and_decodes() {
        let input = GuardianSetHeaderInitializationSkeletonInput {
            payload: InitializeGuardianSetHeaderPayloadSkeleton {
                guardian_set_id: 7,
                threshold: 2,
                guardian_count: 3,
            },
            bump: 251,
            status: 1,
        };

        let encoded =
            encode_and_verify_guardian_set_header_initialization_skeleton(input).expect("encode ok");

        assert_eq!(encoded.len(), GUARDIAN_SET_HEADER_ACCOUNT_SKELETON_LEN);
    }

    #[test]
    fn guardian_set_header_initialization_rejects_invalid_threshold() {
        let input = GuardianSetHeaderInitializationSkeletonInput {
            payload: InitializeGuardianSetHeaderPayloadSkeleton {
                guardian_set_id: 7,
                threshold: 4,
                guardian_count: 3,
            },
            bump: 251,
            status: 1,
        };

        assert_eq!(
            encode_and_verify_guardian_set_header_initialization_skeleton(input),
            Err(XxxlStateInitializationSkeletonError::Layout(
                XxxlStateAccountLayoutSkeletonError::GuardianThresholdOutOfBounds
            ))
        );
    }

    #[test]
    fn mint_state_initialization_encodes_and_decodes() {
        let mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let token_program = Pubkey::new_unique();

        let input = MintStateInitializationSkeletonInput {
            payload: InitializeMintStatePayloadSkeleton { mint, decimals: 9 },
            bump: 250,
            mint_authority_pda: mint_authority,
            mint_authority_bump: 252,
            token_program,
            total_minted: 0,
            is_active: true,
        };

        let encoded = encode_and_verify_mint_state_initialization_skeleton(input)
            .expect("encode ok");

        assert_eq!(encoded.len(), MINT_STATE_ACCOUNT_SKELETON_LEN);

        let decoded = decode_mint_state_account_layout_skeleton(&encoded).expect("decode ok");

        assert_eq!(decoded.mint, mint);
        assert_eq!(decoded.mint_authority_pda, mint_authority);
        assert_eq!(decoded.token_program, token_program);
        assert_eq!(decoded.decimals, 9);
        assert!(decoded.is_active);
    }

    #[test]
    fn processed_event_marking_encodes_and_decodes() {
        let input = ProcessedEventMarkingSkeletonInput {
            payload: ConsumeGatewayMintPayloadBoundarySkeleton {
                message_hash: [0x22; 32],
                canonical_event_key: [0x33; 32],
                mint_amount: 123_456,
            },
            bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        let encoded =
            encode_and_verify_processed_event_marking_skeleton(input).expect("encode ok");

        assert_eq!(encoded.len(), PROCESSED_EVENT_ACCOUNT_SKELETON_LEN);

        let decoded = decode_processed_event_account_layout_skeleton(&encoded).expect("decode ok");

        assert_eq!(decoded.canonical_event_key, [0x33; 32]);
        assert_eq!(decoded.message_hash, [0x22; 32]);
        assert_eq!(decoded.source_burn_tx_hash, [0x44; 32]);
        assert_eq!(decoded.source_burn_event_index, 5);
        assert_eq!(decoded.recipient_hash, [0x55; 32]);
        assert_eq!(decoded.minted_amount, 123_456);
        assert_eq!(decoded.processed_at_slot, 99);
    }
}

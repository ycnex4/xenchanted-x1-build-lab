use crate::{
    instruction_payload_skeleton::ConsumeGatewayMintPayloadBoundarySkeleton,
    state_account_layout_skeleton::{
        decode_mint_state_account_layout_skeleton,
        decode_processed_event_account_layout_skeleton,
        encode_mint_state_account_layout_skeleton,
        encode_processed_event_account_layout_skeleton,
        MintStateAccountLayoutSkeleton,
        ProcessedEventAccountLayoutSkeleton,
        XxxlStateAccountLayoutSkeletonError,
    },
    state_initialization_skeleton::{
        build_processed_event_account_layout_skeleton, ProcessedEventMarkingSkeletonInput,
    },
};

pub const XXXL_CONSUME_STATE_TRANSITION_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlConsumeStateTransitionSkeletonError {
    MintStateInactive,
    MintAmountOverflow,
    Layout(XxxlStateAccountLayoutSkeletonError),
}

impl From<XxxlStateAccountLayoutSkeletonError> for XxxlConsumeStateTransitionSkeletonError {
    fn from(value: XxxlStateAccountLayoutSkeletonError) -> Self {
        Self::Layout(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConsumeGatewayMintStateTransitionSkeletonInput {
    pub current_mint_state: MintStateAccountLayoutSkeleton,
    pub payload: ConsumeGatewayMintPayloadBoundarySkeleton,
    pub processed_event_bump: u8,
    pub source_burn_tx_hash: [u8; 32],
    pub source_burn_event_index: u32,
    pub recipient_hash: [u8; 32],
    pub processed_at_slot: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConsumeGatewayMintStateTransitionSkeletonOutput {
    pub next_mint_state: MintStateAccountLayoutSkeleton,
    pub processed_event: ProcessedEventAccountLayoutSkeleton,
    pub encoded_next_mint_state: Vec<u8>,
    pub encoded_processed_event: Vec<u8>,
}

pub fn apply_consume_gateway_mint_state_transition_skeleton(
    input: ConsumeGatewayMintStateTransitionSkeletonInput,
) -> Result<ConsumeGatewayMintStateTransitionSkeletonOutput, XxxlConsumeStateTransitionSkeletonError>
{
    if !input.current_mint_state.is_active {
        return Err(XxxlConsumeStateTransitionSkeletonError::MintStateInactive);
    }

    let next_total_minted = input
        .current_mint_state
        .total_minted
        .checked_add(input.payload.mint_amount)
        .ok_or(XxxlConsumeStateTransitionSkeletonError::MintAmountOverflow)?;

    let mut next_mint_state = input.current_mint_state;
    next_mint_state.total_minted = next_total_minted;

    let processed_event =
        build_processed_event_account_layout_skeleton(ProcessedEventMarkingSkeletonInput {
            payload: input.payload,
            bump: input.processed_event_bump,
            source_burn_tx_hash: input.source_burn_tx_hash,
            source_burn_event_index: input.source_burn_event_index,
            recipient_hash: input.recipient_hash,
            processed_at_slot: input.processed_at_slot,
        });

    let encoded_next_mint_state = encode_mint_state_account_layout_skeleton(next_mint_state);
    let decoded_next_mint_state =
        decode_mint_state_account_layout_skeleton(&encoded_next_mint_state)?;

    let encoded_processed_event = encode_processed_event_account_layout_skeleton(processed_event);
    let decoded_processed_event =
        decode_processed_event_account_layout_skeleton(&encoded_processed_event)?;

    if decoded_next_mint_state != next_mint_state {
        return Err(XxxlConsumeStateTransitionSkeletonError::Layout(
            XxxlStateAccountLayoutSkeletonError::InvalidDataLength,
        ));
    }

    if decoded_processed_event != processed_event {
        return Err(XxxlConsumeStateTransitionSkeletonError::Layout(
            XxxlStateAccountLayoutSkeletonError::InvalidDataLength,
        ));
    }

    Ok(ConsumeGatewayMintStateTransitionSkeletonOutput {
        next_mint_state,
        processed_event,
        encoded_next_mint_state,
        encoded_processed_event,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_account_layout_skeleton::{
        MINT_STATE_ACCOUNT_SKELETON_LEN, PROCESSED_EVENT_ACCOUNT_SKELETON_LEN,
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

    fn base_payload(mint_amount: u128) -> ConsumeGatewayMintPayloadBoundarySkeleton {
        ConsumeGatewayMintPayloadBoundarySkeleton {
            message_hash: [0x22; 32],
            canonical_event_key: [0x33; 32],
            mint_amount,
        }
    }

    #[test]
    fn consume_state_transition_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_CONSUME_STATE_TRANSITION_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn consume_state_transition_updates_total_minted() {
        let mint_state = active_mint_state(100);
        let input = ConsumeGatewayMintStateTransitionSkeletonInput {
            current_mint_state: mint_state,
            payload: base_payload(23),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        let output =
            apply_consume_gateway_mint_state_transition_skeleton(input).expect("transition ok");

        assert_eq!(output.next_mint_state.total_minted, 123);
        assert_eq!(output.next_mint_state.mint, mint_state.mint);
        assert_eq!(
            output.next_mint_state.mint_authority_pda,
            mint_state.mint_authority_pda
        );
        assert_eq!(output.encoded_next_mint_state.len(), MINT_STATE_ACCOUNT_SKELETON_LEN);
    }

    #[test]
    fn consume_state_transition_builds_processed_event() {
        let input = ConsumeGatewayMintStateTransitionSkeletonInput {
            current_mint_state: active_mint_state(0),
            payload: base_payload(123_456),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        let output =
            apply_consume_gateway_mint_state_transition_skeleton(input).expect("transition ok");

        assert_eq!(output.processed_event.version, 1);
        assert_eq!(output.processed_event.bump, 249);
        assert_eq!(output.processed_event.canonical_event_key, [0x33; 32]);
        assert_eq!(output.processed_event.message_hash, [0x22; 32]);
        assert_eq!(output.processed_event.source_burn_tx_hash, [0x44; 32]);
        assert_eq!(output.processed_event.source_burn_event_index, 5);
        assert_eq!(output.processed_event.recipient_hash, [0x55; 32]);
        assert_eq!(output.processed_event.minted_amount, 123_456);
        assert_eq!(output.processed_event.processed_at_slot, 99);
        assert_eq!(
            output.encoded_processed_event.len(),
            PROCESSED_EVENT_ACCOUNT_SKELETON_LEN
        );
    }

    #[test]
    fn consume_state_transition_rejects_inactive_mint_state() {
        let mut mint_state = active_mint_state(0);
        mint_state.is_active = false;

        let input = ConsumeGatewayMintStateTransitionSkeletonInput {
            current_mint_state: mint_state,
            payload: base_payload(1),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        assert_eq!(
            apply_consume_gateway_mint_state_transition_skeleton(input),
            Err(XxxlConsumeStateTransitionSkeletonError::MintStateInactive)
        );
    }

    #[test]
    fn consume_state_transition_rejects_total_minted_overflow() {
        let input = ConsumeGatewayMintStateTransitionSkeletonInput {
            current_mint_state: active_mint_state(u128::MAX),
            payload: base_payload(1),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        assert_eq!(
            apply_consume_gateway_mint_state_transition_skeleton(input),
            Err(XxxlConsumeStateTransitionSkeletonError::MintAmountOverflow)
        );
    }

    #[test]
    fn consume_state_transition_allows_zero_amount_as_boundary_only() {
        let input = ConsumeGatewayMintStateTransitionSkeletonInput {
            current_mint_state: active_mint_state(7),
            payload: base_payload(0),
            processed_event_bump: 249,
            source_burn_tx_hash: [0x44; 32],
            source_burn_event_index: 5,
            recipient_hash: [0x55; 32],
            processed_at_slot: 99,
        };

        let output =
            apply_consume_gateway_mint_state_transition_skeleton(input).expect("transition ok");

        assert_eq!(output.next_mint_state.total_minted, 7);
        assert_eq!(output.processed_event.minted_amount, 0);
    }
}

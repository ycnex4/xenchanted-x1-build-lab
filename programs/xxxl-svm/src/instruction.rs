use solana_program::program_error::ProgramError;

use crate::error::XxxlError;

pub const CONSUME_GATEWAY_MINT_INSTRUCTION_LEN: usize = 208;
pub const INSTRUCTION_DISCRIMINATOR_LEN: usize = 8;
pub const INSTRUCTION_LAYOUT_VERSION: u16 = 1;

pub const CONSUME_GATEWAY_MINT_DISCRIMINATOR: [u8; INSTRUCTION_DISCRIMINATOR_LEN] =
    [0xf2, 0xf4, 0xa8, 0x68, 0xbb, 0x89, 0xfe, 0x52];

pub const CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT: u8 = 9;
pub const CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX: u8 = 1;
pub const CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX: u8 = 2;
pub const CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX: u8 = 0;
pub const CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX: u8 = 3;
pub const CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX: u8 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlInstruction {
    ConsumeGatewayMint(ConsumeGatewayMintArgs),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConsumeGatewayMintArgs {
    pub raw: [u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN],
    pub account_meta_count: u8,
    pub route_account_index: u8,
    pub guardian_set_account_index: u8,
    pub mint_state_account_index: u8,
    pub processed_event_account_index: u8,
    pub recipient_balance_account_index: u8,
    pub route_id: [u8; 32],
    pub guardian_set_id: [u8; 32],
    pub mint_id: [u8; 32],
    pub canonical_event_key: [u8; 32],
    pub recipient: [u8; 32],
    pub amount: u128,
    pub source_chain_weight_bps: u16,
}

impl XxxlInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() != CONSUME_GATEWAY_MINT_INSTRUCTION_LEN {
            return Err(XxxlError::InvalidInstruction.into());
        }

        if input[0..INSTRUCTION_DISCRIMINATOR_LEN] != CONSUME_GATEWAY_MINT_DISCRIMINATOR {
            return Err(XxxlError::InvalidDiscriminator.into());
        }

        let version = read_u16_le(input, 8);

        if version != INSTRUCTION_LAYOUT_VERSION {
            return Err(XxxlError::InvalidVersion.into());
        }

        let account_meta_count = input[10];
        let route_account_index = input[11];
        let guardian_set_account_index = input[12];
        let mint_state_account_index = input[13];
        let processed_event_account_index = input[14];
        let recipient_balance_account_index = input[15];

        if account_meta_count != CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT
            || route_account_index != CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX
            || guardian_set_account_index != CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX
            || mint_state_account_index != CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX
            || processed_event_account_index != CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX
            || recipient_balance_account_index != CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX
        {
            return Err(XxxlError::InvalidInstruction.into());
        }

        let mut raw = [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN];
        raw.copy_from_slice(input);

        Ok(Self::ConsumeGatewayMint(ConsumeGatewayMintArgs {
            raw,
            account_meta_count,
            route_account_index,
            guardian_set_account_index,
            mint_state_account_index,
            processed_event_account_index,
            recipient_balance_account_index,
            route_id: read_fixed_32(input, 16),
            guardian_set_id: read_fixed_32(input, 48),
            mint_id: read_fixed_32(input, 80),
            canonical_event_key: read_fixed_32(input, 112),
            recipient: read_fixed_32(input, 144),
            amount: read_u128_le(input, 176),
            source_chain_weight_bps: read_u16_le(input, 192),
        }))
    }
}

fn read_fixed_32(input: &[u8], offset: usize) -> [u8; 32] {
    let mut output = [0u8; 32];
    output.copy_from_slice(&input[offset..offset + 32]);
    output
}

fn read_u16_le(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([input[offset], input[offset + 1]])
}

fn read_u128_le(input: &[u8], offset: usize) -> u128 {
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&input[offset..offset + 16]);
    u128::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::program_error::ProgramError;

    #[test]
    fn consume_gateway_mint_instruction_parses_known_layout() {
        let bytes = valid_consume_gateway_mint_instruction();

        let instruction = XxxlInstruction::unpack(&bytes).expect("valid instruction");

        match instruction {
            XxxlInstruction::ConsumeGatewayMint(args) => {
                assert_eq!(args.raw, bytes);
                assert_eq!(args.account_meta_count, 9);
                assert_eq!(args.route_account_index, 1);
                assert_eq!(args.guardian_set_account_index, 2);
                assert_eq!(args.mint_state_account_index, 0);
                assert_eq!(args.processed_event_account_index, 3);
                assert_eq!(args.recipient_balance_account_index, 4);
                assert_eq!(args.route_id, [0x11; 32]);
                assert_eq!(args.guardian_set_id, [0x22; 32]);
                assert_eq!(args.mint_id, [0x33; 32]);
                assert_eq!(args.canonical_event_key, [0x44; 32]);
                assert_eq!(args.recipient, [0x55; 32]);
                assert_eq!(args.amount, 1_000);
                assert_eq!(args.source_chain_weight_bps, 10_000);
            }
        }
    }

    #[test]
    fn consume_gateway_mint_rejects_wrong_instruction_length() {
        let bytes = valid_consume_gateway_mint_instruction();

        assert_custom_error(
            XxxlInstruction::unpack(&bytes[..CONSUME_GATEWAY_MINT_INSTRUCTION_LEN - 1]),
            XxxlError::InvalidInstruction,
        );

        let mut too_long = bytes.to_vec();
        too_long.push(0);

        assert_custom_error(
            XxxlInstruction::unpack(&too_long),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn consume_gateway_mint_rejects_wrong_instruction_discriminator() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[0] ^= 0xff;

        assert_custom_error(
            XxxlInstruction::unpack(&bytes),
            XxxlError::InvalidDiscriminator,
        );
    }

    #[test]
    fn consume_gateway_mint_rejects_wrong_instruction_version() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[8..10].copy_from_slice(&2u16.to_le_bytes());

        assert_custom_error(XxxlInstruction::unpack(&bytes), XxxlError::InvalidVersion);
    }

    #[test]
    fn consume_gateway_mint_rejects_wrong_account_meta_count() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[10] = 8;

        assert_custom_error(
            XxxlInstruction::unpack(&bytes),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn consume_gateway_mint_rejects_wrong_account_index_boundary() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[13] = 9;

        assert_custom_error(
            XxxlInstruction::unpack(&bytes),
            XxxlError::InvalidInstruction,
        );
    }

    fn valid_consume_gateway_mint_instruction() -> [u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN] {
        let mut bytes = [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN];

        bytes[0..8].copy_from_slice(&CONSUME_GATEWAY_MINT_DISCRIMINATOR);
        bytes[8..10].copy_from_slice(&INSTRUCTION_LAYOUT_VERSION.to_le_bytes());
        bytes[10] = CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT;
        bytes[11] = CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX;
        bytes[12] = CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX;
        bytes[13] = CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX;
        bytes[14] = CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX;
        bytes[15] = CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX;
        bytes[16..48].copy_from_slice(&[0x11; 32]);
        bytes[48..80].copy_from_slice(&[0x22; 32]);
        bytes[80..112].copy_from_slice(&[0x33; 32]);
        bytes[112..144].copy_from_slice(&[0x44; 32]);
        bytes[144..176].copy_from_slice(&[0x55; 32]);
        bytes[176..192].copy_from_slice(&1_000u128.to_le_bytes());
        bytes[192..194].copy_from_slice(&10_000u16.to_le_bytes());

        bytes
    }

    fn assert_custom_error(result: Result<XxxlInstruction, ProgramError>, error: XxxlError) {
        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
    }
}

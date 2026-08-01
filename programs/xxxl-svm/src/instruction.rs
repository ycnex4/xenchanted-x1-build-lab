use solana_program::program_error::ProgramError;

use crate::error::XxxlError;

pub const CONSUME_GATEWAY_MINT_INSTRUCTION_LEN: usize = 208;
pub const INSTRUCTION_DISCRIMINATOR_LEN: usize = 8;
pub const INSTRUCTION_LAYOUT_VERSION: u16 = 2;
pub const CONSUME_GATEWAY_MINT_SOURCE_CHAIN_ID_OFFSET: usize = 194;
pub const CONSUME_GATEWAY_MINT_RESERVED_ZERO_START: usize = 202;
pub const CONSUME_GATEWAY_MINT_RESERVED_ZERO_END: usize = 208;

pub const INITIALIZE_GATEWAY_CONFIG_INSTRUCTION_LEN: usize = 128;
pub const INITIALIZE_GUARDIAN_SET_INSTRUCTION_LEN: usize = 320;
pub const INITIALIZE_MINT_STATE_INSTRUCTION_LEN: usize = 112;
pub const INITIALIZE_RECIPIENT_BALANCE_INSTRUCTION_LEN: usize = 80;

pub const INIT_ACCOUNT_META_COUNT: u8 = 4;
pub const INIT_STATE_ACCOUNT_INDEX: u8 = 0;
pub const INIT_AUTHORITY_ACCOUNT_INDEX: u8 = 1;
pub const INIT_RENT_PAYER_ACCOUNT_INDEX: u8 = 2;
pub const INIT_SYSTEM_PROGRAM_ACCOUNT_INDEX: u8 = 3;

pub const MAX_GUARDIANS_IN_INITIALIZE_GUARDIAN_SET: usize = 8;

pub const CONSUME_GATEWAY_MINT_DISCRIMINATOR: [u8; INSTRUCTION_DISCRIMINATOR_LEN] =
    [0xf2, 0xf4, 0xa8, 0x68, 0xbb, 0x89, 0xfe, 0x52];

pub const INITIALIZE_GATEWAY_CONFIG_DISCRIMINATOR: [u8; INSTRUCTION_DISCRIMINATOR_LEN] =
    [0x48, 0x67, 0x63, 0x66, 0x67, 0x2d, 0x69, 0x6e];
pub const INITIALIZE_GUARDIAN_SET_DISCRIMINATOR: [u8; INSTRUCTION_DISCRIMINATOR_LEN] =
    [0x48, 0x67, 0x73, 0x65, 0x74, 0x2d, 0x69, 0x6e];
pub const INITIALIZE_MINT_STATE_DISCRIMINATOR: [u8; INSTRUCTION_DISCRIMINATOR_LEN] =
    [0x48, 0x6d, 0x73, 0x74, 0x61, 0x2d, 0x69, 0x6e];
pub const INITIALIZE_RECIPIENT_BALANCE_DISCRIMINATOR: [u8; INSTRUCTION_DISCRIMINATOR_LEN] =
    [0x48, 0x72, 0x62, 0x61, 0x6c, 0x2d, 0x69, 0x6e];

pub const CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT: u8 = 11;
pub const CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_META_COUNT: u8 = 12;
pub const CONSUME_GATEWAY_MINT_B1_V3_INSTRUCTION_LAYOUT_VERSION: u16 = 3;
pub const CONSUME_GATEWAY_MINT_INSTRUCTIONS_SYSVAR_ACCOUNT_INDEX: u8 = 11;
pub const CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX: u8 = 1;
pub const CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX: u8 = 2;
pub const CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX: u8 = 0;
pub const CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX: u8 = 3;
pub const CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX: u8 = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlInstruction {
    ConsumeGatewayMint(ConsumeGatewayMintArgs),
    InitializeGatewayConfig(InitializeGatewayConfigArgs),
    InitializeGuardianSet(InitializeGuardianSetArgs),
    InitializeMintState(InitializeMintStateArgs),
    InitializeRecipientBalance(InitializeRecipientBalanceArgs),
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
    pub source_chain_id: u64,
    pub source_chain_weight_bps: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeGatewayConfigArgs {
    pub raw: [u8; INITIALIZE_GATEWAY_CONFIG_INSTRUCTION_LEN],
    pub route_id: [u8; 32],
    pub guardian_set_id: [u8; 32],
    pub target_mint: [u8; 32],
    pub source_chain_id: u64,
    pub source_chain_weight_bps: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeGuardianSetArgs {
    pub raw: [u8; INITIALIZE_GUARDIAN_SET_INSTRUCTION_LEN],
    pub guardian_set_id: [u8; 32],
    pub quorum_threshold: u16,
    pub guardian_count: u8,
    pub guardians: [[u8; 32]; MAX_GUARDIANS_IN_INITIALIZE_GUARDIAN_SET],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeMintStateArgs {
    pub raw: [u8; INITIALIZE_MINT_STATE_INSTRUCTION_LEN],
    pub mint_id: [u8; 32],
    pub mint_pubkey: [u8; 32],
    pub decimals: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitializeRecipientBalanceArgs {
    pub raw: [u8; INITIALIZE_RECIPIENT_BALANCE_INSTRUCTION_LEN],
    pub recipient: [u8; 32],
    pub mint: [u8; 32],
}

impl XxxlInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < INSTRUCTION_DISCRIMINATOR_LEN {
            return Err(XxxlError::InvalidInstruction.into());
        }

        let discriminator = &input[0..INSTRUCTION_DISCRIMINATOR_LEN];

        if discriminator == CONSUME_GATEWAY_MINT_DISCRIMINATOR {
            return unpack_consume_gateway_mint(input);
        }

        if discriminator == INITIALIZE_GATEWAY_CONFIG_DISCRIMINATOR {
            return unpack_initialize_gateway_config(input);
        }

        if discriminator == INITIALIZE_GUARDIAN_SET_DISCRIMINATOR {
            return unpack_initialize_guardian_set(input);
        }

        if discriminator == INITIALIZE_MINT_STATE_DISCRIMINATOR {
            return unpack_initialize_mint_state(input);
        }

        if discriminator == INITIALIZE_RECIPIENT_BALANCE_DISCRIMINATOR {
            return unpack_initialize_recipient_balance(input);
        }

        Err(XxxlError::InvalidDiscriminator.into())
    }
}

fn unpack_consume_gateway_mint(input: &[u8]) -> Result<XxxlInstruction, ProgramError> {
    if input.len() != CONSUME_GATEWAY_MINT_INSTRUCTION_LEN {
        return Err(XxxlError::InvalidInstruction.into());
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

    if input[CONSUME_GATEWAY_MINT_RESERVED_ZERO_START..CONSUME_GATEWAY_MINT_RESERVED_ZERO_END]
        .iter()
        .any(|byte| *byte != 0)
    {
        return Err(XxxlError::InvalidInstructionReserved.into());
    }

    let mut raw = [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN];
    raw.copy_from_slice(input);

    Ok(XxxlInstruction::ConsumeGatewayMint(
        ConsumeGatewayMintArgs {
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
            source_chain_id: read_u64_le(input, CONSUME_GATEWAY_MINT_SOURCE_CHAIN_ID_OFFSET),
            source_chain_weight_bps: read_u16_le(input, 192),
        },
    ))
}

fn unpack_initialize_gateway_config(input: &[u8]) -> Result<XxxlInstruction, ProgramError> {
    assert_init_instruction_header(input, INITIALIZE_GATEWAY_CONFIG_INSTRUCTION_LEN)?;

    if input[122..128].iter().any(|byte| *byte != 0) {
        return Err(XxxlError::InvalidInstructionReserved.into());
    }

    let mut raw = [0u8; INITIALIZE_GATEWAY_CONFIG_INSTRUCTION_LEN];
    raw.copy_from_slice(input);

    Ok(XxxlInstruction::InitializeGatewayConfig(
        InitializeGatewayConfigArgs {
            raw,
            route_id: read_fixed_32(input, 16),
            guardian_set_id: read_fixed_32(input, 48),
            target_mint: read_fixed_32(input, 80),
            source_chain_id: read_u64_le(input, 112),
            source_chain_weight_bps: read_u16_le(input, 120),
        },
    ))
}

fn unpack_initialize_guardian_set(input: &[u8]) -> Result<XxxlInstruction, ProgramError> {
    assert_init_instruction_header(input, INITIALIZE_GUARDIAN_SET_INSTRUCTION_LEN)?;

    if input[51..64].iter().any(|byte| *byte != 0) {
        return Err(XxxlError::InvalidInstructionReserved.into());
    }

    let guardian_count = input[50];
    let quorum_threshold = read_u16_le(input, 48);

    if guardian_count == 0
        || guardian_count as usize > MAX_GUARDIANS_IN_INITIALIZE_GUARDIAN_SET
        || quorum_threshold == 0
        || quorum_threshold > guardian_count as u16
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let mut guardians = [[0u8; 32]; MAX_GUARDIANS_IN_INITIALIZE_GUARDIAN_SET];

    for (index, guardian) in guardians.iter_mut().enumerate() {
        let offset = 64 + index * 32;
        *guardian = read_fixed_32(input, offset);
    }

    let mut raw = [0u8; INITIALIZE_GUARDIAN_SET_INSTRUCTION_LEN];
    raw.copy_from_slice(input);

    Ok(XxxlInstruction::InitializeGuardianSet(
        InitializeGuardianSetArgs {
            raw,
            guardian_set_id: read_fixed_32(input, 16),
            quorum_threshold,
            guardian_count,
            guardians,
        },
    ))
}

fn unpack_initialize_mint_state(input: &[u8]) -> Result<XxxlInstruction, ProgramError> {
    assert_init_instruction_header(input, INITIALIZE_MINT_STATE_INSTRUCTION_LEN)?;

    if input[81..112].iter().any(|byte| *byte != 0) {
        return Err(XxxlError::InvalidInstructionReserved.into());
    }

    let mut raw = [0u8; INITIALIZE_MINT_STATE_INSTRUCTION_LEN];
    raw.copy_from_slice(input);

    Ok(XxxlInstruction::InitializeMintState(
        InitializeMintStateArgs {
            raw,
            mint_id: read_fixed_32(input, 16),
            mint_pubkey: read_fixed_32(input, 48),
            decimals: input[80],
        },
    ))
}

fn unpack_initialize_recipient_balance(input: &[u8]) -> Result<XxxlInstruction, ProgramError> {
    assert_init_instruction_header(input, INITIALIZE_RECIPIENT_BALANCE_INSTRUCTION_LEN)?;

    let mut raw = [0u8; INITIALIZE_RECIPIENT_BALANCE_INSTRUCTION_LEN];
    raw.copy_from_slice(input);

    Ok(XxxlInstruction::InitializeRecipientBalance(
        InitializeRecipientBalanceArgs {
            raw,
            recipient: read_fixed_32(input, 16),
            mint: read_fixed_32(input, 48),
        },
    ))
}

fn assert_init_instruction_header(input: &[u8], expected_len: usize) -> Result<(), ProgramError> {
    if input.len() != expected_len {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if read_u16_le(input, 8) != INSTRUCTION_LAYOUT_VERSION {
        return Err(XxxlError::InvalidVersion.into());
    }

    if input[10] != INIT_ACCOUNT_META_COUNT
        || input[11] != INIT_STATE_ACCOUNT_INDEX
        || input[12] != INIT_AUTHORITY_ACCOUNT_INDEX
        || input[13] != INIT_RENT_PAYER_ACCOUNT_INDEX
        || input[14] != INIT_SYSTEM_PROGRAM_ACCOUNT_INDEX
        || input[15] != 0
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    Ok(())
}

fn read_fixed_32(input: &[u8], offset: usize) -> [u8; 32] {
    let mut output = [0u8; 32];
    output.copy_from_slice(&input[offset..offset + 32]);
    output
}

fn read_u16_le(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([input[offset], input[offset + 1]])
}

fn read_u64_le(input: &[u8], offset: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&input[offset..offset + 8]);
    u64::from_le_bytes(bytes)
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

        if let XxxlInstruction::ConsumeGatewayMint(args) = instruction {
            assert_eq!(args.raw, bytes);
            assert_eq!(args.account_meta_count, 11);
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
            assert_eq!(args.source_chain_id, 1);
            assert_eq!(args.source_chain_weight_bps, 10_000);
        } else {
            panic!("unexpected instruction variant");
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
        bytes[8..10].copy_from_slice(&3u16.to_le_bytes());

        assert_custom_error(XxxlInstruction::unpack(&bytes), XxxlError::InvalidVersion);
    }

    #[test]
    fn consume_gateway_mint_rejects_version_1() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[8..10].copy_from_slice(&1u16.to_le_bytes());

        assert_custom_error(XxxlInstruction::unpack(&bytes), XxxlError::InvalidVersion);
    }

    #[test]
    fn consume_gateway_mint_rejects_version_0() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[8..10].copy_from_slice(&0u16.to_le_bytes());

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

    #[test]
    fn consume_gateway_mint_v2_parses_source_chain_id() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        let expected_source_chain_id = 42u64;
        bytes[194..202].copy_from_slice(&expected_source_chain_id.to_le_bytes());

        let instruction = XxxlInstruction::unpack(&bytes).expect("valid v2 instruction");

        if let XxxlInstruction::ConsumeGatewayMint(args) = instruction {
            assert_eq!(args.source_chain_id, expected_source_chain_id);
            assert_eq!(args.raw[194..202], expected_source_chain_id.to_le_bytes());
        } else {
            panic!("unexpected instruction variant");
        }
    }

    #[test]
    fn consume_gateway_mint_v2_parses_max_source_chain_id() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[194..202].copy_from_slice(&u64::MAX.to_le_bytes());

        let instruction = XxxlInstruction::unpack(&bytes).expect("valid v2 instruction");

        if let XxxlInstruction::ConsumeGatewayMint(args) = instruction {
            assert_eq!(args.source_chain_id, u64::MAX);
        } else {
            panic!("unexpected instruction variant");
        }
    }

    #[test]
    fn consume_gateway_mint_rejects_nonzero_reserved_202_207() {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[202..208].copy_from_slice(&[1, 2, 3, 4, 5, 6]);

        assert_custom_error(
            XxxlInstruction::unpack(&bytes),
            XxxlError::InvalidInstructionReserved,
        );
    }

    #[test]
    fn consume_gateway_mint_rejects_nonzero_reserved_any_byte() {
        for index in 202..208 {
            let mut bytes = valid_consume_gateway_mint_instruction();
            bytes[index] = 1;

            assert_custom_error(
                XxxlInstruction::unpack(&bytes),
                XxxlError::InvalidInstructionReserved,
            );
        }
    }

    #[test]
    fn initialize_gateway_config_parses_known_layout() {
        let bytes = valid_initialize_gateway_config_instruction();

        let instruction = XxxlInstruction::unpack(&bytes).expect("valid init gateway config");

        if let XxxlInstruction::InitializeGatewayConfig(args) = instruction {
            assert_eq!(args.route_id, [0x11; 32]);
            assert_eq!(args.guardian_set_id, [0x22; 32]);
            assert_eq!(args.target_mint, [0x33; 32]);
            assert_eq!(args.source_chain_id, 42);
            assert_eq!(args.source_chain_weight_bps, 10_000);
        } else {
            panic!("unexpected instruction variant");
        }
    }

    #[test]
    fn initialize_guardian_set_parses_known_layout() {
        let bytes = valid_initialize_guardian_set_instruction();

        let instruction = XxxlInstruction::unpack(&bytes).expect("valid init guardian set");

        if let XxxlInstruction::InitializeGuardianSet(args) = instruction {
            assert_eq!(args.guardian_set_id, [0x44; 32]);
            assert_eq!(args.quorum_threshold, 3);
            assert_eq!(args.guardian_count, 5);
            assert_eq!(args.guardians[0], [1; 32]);
            assert_eq!(args.guardians[4], [5; 32]);
        } else {
            panic!("unexpected instruction variant");
        }
    }

    #[test]
    fn initialize_guardian_set_rejects_zero_guardian_count() {
        let mut bytes = valid_initialize_guardian_set_instruction();
        bytes[50] = 0;

        assert_custom_error(
            XxxlInstruction::unpack(&bytes),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn initialize_mint_state_parses_known_layout() {
        let bytes = valid_initialize_mint_state_instruction();

        let instruction = XxxlInstruction::unpack(&bytes).expect("valid init mint state");

        if let XxxlInstruction::InitializeMintState(args) = instruction {
            assert_eq!(args.mint_id, [0x66; 32]);
            assert_eq!(args.mint_pubkey, [0x77; 32]);
            assert_eq!(args.decimals, 9);
        } else {
            panic!("unexpected instruction variant");
        }
    }

    #[test]
    fn initialize_recipient_balance_parses_known_layout() {
        let bytes = valid_initialize_recipient_balance_instruction();

        let instruction = XxxlInstruction::unpack(&bytes).expect("valid init recipient balance");

        if let XxxlInstruction::InitializeRecipientBalance(args) = instruction {
            assert_eq!(args.recipient, [0x88; 32]);
            assert_eq!(args.mint, [0x99; 32]);
        } else {
            panic!("unexpected instruction variant");
        }
    }

    #[test]
    fn initialize_instructions_reject_wrong_meta_header() {
        let mut bytes = valid_initialize_gateway_config_instruction();
        bytes[10] = 9;

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
        bytes[194..202].copy_from_slice(&1u64.to_le_bytes());

        bytes
    }

    fn valid_initialize_gateway_config_instruction(
    ) -> [u8; INITIALIZE_GATEWAY_CONFIG_INSTRUCTION_LEN] {
        let mut bytes = [0u8; INITIALIZE_GATEWAY_CONFIG_INSTRUCTION_LEN];

        write_init_header(&mut bytes, &INITIALIZE_GATEWAY_CONFIG_DISCRIMINATOR);
        bytes[16..48].copy_from_slice(&[0x11; 32]);
        bytes[48..80].copy_from_slice(&[0x22; 32]);
        bytes[80..112].copy_from_slice(&[0x33; 32]);
        bytes[112..120].copy_from_slice(&42u64.to_le_bytes());
        bytes[120..122].copy_from_slice(&10_000u16.to_le_bytes());

        bytes
    }

    fn valid_initialize_guardian_set_instruction() -> [u8; INITIALIZE_GUARDIAN_SET_INSTRUCTION_LEN]
    {
        let mut bytes = [0u8; INITIALIZE_GUARDIAN_SET_INSTRUCTION_LEN];

        write_init_header(&mut bytes, &INITIALIZE_GUARDIAN_SET_DISCRIMINATOR);
        bytes[16..48].copy_from_slice(&[0x44; 32]);
        bytes[48..50].copy_from_slice(&3u16.to_le_bytes());
        bytes[50] = 5;

        for index in 0..5 {
            let offset = 64 + index * 32;
            bytes[offset..offset + 32].copy_from_slice(&[(index + 1) as u8; 32]);
        }

        bytes
    }

    fn valid_initialize_mint_state_instruction() -> [u8; INITIALIZE_MINT_STATE_INSTRUCTION_LEN] {
        let mut bytes = [0u8; INITIALIZE_MINT_STATE_INSTRUCTION_LEN];

        write_init_header(&mut bytes, &INITIALIZE_MINT_STATE_DISCRIMINATOR);
        bytes[16..48].copy_from_slice(&[0x66; 32]);
        bytes[48..80].copy_from_slice(&[0x77; 32]);
        bytes[80] = 9;

        bytes
    }

    fn valid_initialize_recipient_balance_instruction(
    ) -> [u8; INITIALIZE_RECIPIENT_BALANCE_INSTRUCTION_LEN] {
        let mut bytes = [0u8; INITIALIZE_RECIPIENT_BALANCE_INSTRUCTION_LEN];

        write_init_header(&mut bytes, &INITIALIZE_RECIPIENT_BALANCE_DISCRIMINATOR);
        bytes[16..48].copy_from_slice(&[0x88; 32]);
        bytes[48..80].copy_from_slice(&[0x99; 32]);

        bytes
    }

    fn write_init_header(bytes: &mut [u8], discriminator: &[u8; INSTRUCTION_DISCRIMINATOR_LEN]) {
        bytes[0..8].copy_from_slice(discriminator);
        bytes[8..10].copy_from_slice(&INSTRUCTION_LAYOUT_VERSION.to_le_bytes());
        bytes[10] = INIT_ACCOUNT_META_COUNT;
        bytes[11] = INIT_STATE_ACCOUNT_INDEX;
        bytes[12] = INIT_AUTHORITY_ACCOUNT_INDEX;
        bytes[13] = INIT_RENT_PAYER_ACCOUNT_INDEX;
        bytes[14] = INIT_SYSTEM_PROGRAM_ACCOUNT_INDEX;
    }

    fn assert_custom_error(result: Result<XxxlInstruction, ProgramError>, error: XxxlError) {
        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
    }
}

use solana_program::program_error::ProgramError;
use xxxl_svm::{
    error::XxxlError,
    instruction::{
        XxxlInstruction, CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT,
        CONSUME_GATEWAY_MINT_DISCRIMINATOR, CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX,
        CONSUME_GATEWAY_MINT_INSTRUCTION_LEN, CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX,
        CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX,
        CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX,
        CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX, INSTRUCTION_LAYOUT_VERSION,
    },
};

#[test]
fn consume_gateway_mint_v2_parses_source_chain_id_from_bytes_194_201() {
    let mut bytes = valid_consume_gateway_mint_instruction();
    let source_chain_id = 0xa7a6_a5a4_a3a2_a1a0u64;
    bytes[194..202].copy_from_slice(&source_chain_id.to_le_bytes());

    let instruction = XxxlInstruction::unpack(&bytes).expect("valid v2 source_chain_id");

    match instruction {
        XxxlInstruction::ConsumeGatewayMint(args) => {
            assert_eq!(args.raw[194..202], bytes[194..202]);
            assert_eq!(args.source_chain_id, source_chain_id);
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
fn consume_gateway_mint_v2_rejects_nonzero_reserved_bytes_202_207() {
    for index in 202..208 {
        let mut bytes = valid_consume_gateway_mint_instruction();
        bytes[index] = 0xff;

        assert_custom_error(
            XxxlInstruction::unpack(&bytes),
            XxxlError::InvalidInstructionReserved,
        );
    }
}

#[test]
fn consume_gateway_mint_v1_rejects_before_reserved_or_source_chain_semantics() {
    let mut bytes = valid_consume_gateway_mint_instruction();
    bytes[8..10].copy_from_slice(&1u16.to_le_bytes());
    bytes[194..202].copy_from_slice(&u64::MAX.to_le_bytes());
    bytes[202] = 1;

    assert_custom_error(XxxlInstruction::unpack(&bytes), XxxlError::InvalidVersion);
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

fn assert_custom_error(result: Result<XxxlInstruction, ProgramError>, error: XxxlError) {
    assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
}

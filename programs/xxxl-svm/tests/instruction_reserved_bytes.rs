use xxxl_svm::instruction::{
    XxxlInstruction, CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT, CONSUME_GATEWAY_MINT_DISCRIMINATOR,
    CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX, CONSUME_GATEWAY_MINT_INSTRUCTION_LEN,
    CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX,
    CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX,
    CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX, CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX,
    INSTRUCTION_LAYOUT_VERSION,
};

#[test]
fn consume_gateway_mint_accepts_nonzero_reserved_bytes_194_208_as_raw_only() {
    let mut bytes = valid_consume_gateway_mint_instruction();

    for (i, byte) in bytes[194..208].iter_mut().enumerate() {
        *byte = 0xa0 + i as u8;
    }

    let instruction = XxxlInstruction::unpack(&bytes).expect("reserved bytes remain raw-only");

    match instruction {
        XxxlInstruction::ConsumeGatewayMint(args) => {
            assert_eq!(args.raw[194..208], bytes[194..208]);
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

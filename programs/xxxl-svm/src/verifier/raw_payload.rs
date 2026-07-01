use core::convert::TryInto;

pub const RAW_PAYLOAD_DECODER_PHASE_33: &str = "RAW_PAYLOAD_DECODER_PHASE_33";
pub const RAW_PAYLOAD_DECODER_VERSION: u8 = 1;

pub const XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE: &[u8] = b"XXXL_GATEWAY_MINT";
pub const XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION: u16 = 1;
pub const XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION: u16 = 2;
pub const MAX_VAR_BYTES_LEN: usize = u16::MAX as usize;

pub const RAW_PAYLOAD_PHASE_23_FIELD_ORDER: [&str; 21] = [
    "message_type",
    "schema_version",
    "instruction_layout_version",
    "route_id",
    "source_chain_id",
    "source_token",
    "source_sender",
    "source_burn_tx_hash",
    "source_burn_event_index",
    "source_block_number",
    "source_block_hash",
    "source_finality_block",
    "canonical_event_key",
    "x1_recipient",
    "burned_amount",
    "source_chain_weight_bps",
    "xxxl_mint_amount",
    "target_mint",
    "guardian_set_id",
    "message_nonce",
    "expiration_slot_or_unix_ts",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RawPayloadDecodeErrorKind {
    Truncated,
    TrailingBytes,
    EmptyVariableBytes,
    UnsupportedMessageType,
    UnsupportedSchemaVersion,
    UnsupportedInstructionLayoutVersion,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawPayloadDecodeError {
    pub kind: RawPayloadDecodeErrorKind,
    pub field: &'static str,
    pub offset: usize,
    pub expected: usize,
    pub actual: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodedGuardianPayloadRaw<'a> {
    pub message_type: &'a [u8],
    pub schema_version: u16,
    pub instruction_layout_version: u16,
    pub route_id: &'a [u8; 32],
    pub source_chain_id: u64,
    pub source_token: &'a [u8],
    pub source_sender: &'a [u8],
    pub source_burn_tx_hash: &'a [u8],
    pub source_burn_event_index: u64,
    pub source_block_number: u64,
    pub source_block_hash: &'a [u8],
    pub source_finality_block: u64,
    pub canonical_event_key: &'a [u8; 32],
    pub x1_recipient: &'a [u8; 32],
    pub burned_amount: u128,
    pub source_chain_weight_bps: u16,
    pub xxxl_mint_amount: u128,
    pub target_mint: &'a [u8; 32],
    pub guardian_set_id: &'a [u8; 32],
    pub message_nonce: &'a [u8; 32],
    pub expiration_slot_or_unix_ts: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RawPayloadDecoderReport {
    pub decoder_id: &'static str,
    pub decoder_version: u8,
    pub field_count: usize,
    pub follows_phase_23_canonical_layout: bool,
    pub rejects_truncated_payloads: bool,
    pub rejects_trailing_bytes: bool,
    pub rejects_empty_variable_bytes: bool,
    pub rejects_structurally_invalid_byte_encoding: bool,
    pub wrong_field_order_structural_only: bool,
    pub all_wrong_field_order_cases_satisfied: bool,
    pub live_route_enabled: bool,
    pub spl_cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub mint_execution_enabled: bool,
    pub runtime_state_mutation_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub production_program_id_selected: bool,
    pub deployment_blockers_removed: bool,
}

pub const RAW_PAYLOAD_DECODER_REPORT: RawPayloadDecoderReport = RawPayloadDecoderReport {
    decoder_id: RAW_PAYLOAD_DECODER_PHASE_33,
    decoder_version: RAW_PAYLOAD_DECODER_VERSION,
    field_count: 21,
    follows_phase_23_canonical_layout: true,
    rejects_truncated_payloads: true,
    rejects_trailing_bytes: true,
    rejects_empty_variable_bytes: true,
    rejects_structurally_invalid_byte_encoding: true,
    wrong_field_order_structural_only: true,
    all_wrong_field_order_cases_satisfied: false,
    live_route_enabled: false,
    spl_cpi_enabled: false,
    invoke_signed_enabled: false,
    mint_execution_enabled: false,
    runtime_state_mutation_enabled: false,
    replay_write_enabled: false,
    processed_event_marking_enabled: false,
    production_program_id_selected: false,
    deployment_blockers_removed: false,
};

struct DecodeCursor<'a> {
    input: &'a [u8],
    offset: usize,
}

impl<'a> DecodeCursor<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self { input, offset: 0 }
    }

    fn remaining(&self) -> usize {
        self.input.len().saturating_sub(self.offset)
    }

    fn take(&mut self, len: usize, field: &'static str) -> Result<&'a [u8], RawPayloadDecodeError> {
        if self.remaining() < len {
            return Err(RawPayloadDecodeError {
                kind: RawPayloadDecodeErrorKind::Truncated,
                field,
                offset: self.offset,
                expected: len,
                actual: self.remaining(),
            });
        }

        let start = self.offset;
        self.offset += len;
        Ok(&self.input[start..self.offset])
    }

    fn read_u16(&mut self, field: &'static str) -> Result<u16, RawPayloadDecodeError> {
        let bytes = self.take(2, field)?;
        Ok(u16::from_le_bytes(
            bytes.try_into().expect("u16 slice length"),
        ))
    }

    fn read_u64(&mut self, field: &'static str) -> Result<u64, RawPayloadDecodeError> {
        let bytes = self.take(8, field)?;
        Ok(u64::from_le_bytes(
            bytes.try_into().expect("u64 slice length"),
        ))
    }

    fn read_u128(&mut self, field: &'static str) -> Result<u128, RawPayloadDecodeError> {
        let bytes = self.take(16, field)?;
        Ok(u128::from_le_bytes(
            bytes.try_into().expect("u128 slice length"),
        ))
    }

    fn read_bytes32(&mut self, field: &'static str) -> Result<&'a [u8; 32], RawPayloadDecodeError> {
        let bytes = self.take(32, field)?;
        Ok(bytes.try_into().expect("bytes32 slice length"))
    }

    fn read_var_bytes(&mut self, field: &'static str) -> Result<&'a [u8], RawPayloadDecodeError> {
        let len_offset = self.offset;
        let len = self.read_u16(field)? as usize;

        if len == 0 {
            return Err(RawPayloadDecodeError {
                kind: RawPayloadDecodeErrorKind::EmptyVariableBytes,
                field,
                offset: len_offset,
                expected: 1,
                actual: 0,
            });
        }

        if len > MAX_VAR_BYTES_LEN {
            return Err(RawPayloadDecodeError {
                kind: RawPayloadDecodeErrorKind::Truncated,
                field,
                offset: len_offset,
                expected: MAX_VAR_BYTES_LEN,
                actual: len,
            });
        }

        self.take(len, field)
    }

    fn finish(self) -> Result<(), RawPayloadDecodeError> {
        if self.offset != self.input.len() {
            return Err(RawPayloadDecodeError {
                kind: RawPayloadDecodeErrorKind::TrailingBytes,
                field: "payload",
                offset: self.offset,
                expected: self.offset,
                actual: self.input.len(),
            });
        }

        Ok(())
    }
}

pub fn decode_guardian_payload_raw<'a>(
    input: &'a [u8],
) -> Result<DecodedGuardianPayloadRaw<'a>, RawPayloadDecodeError> {
    let mut cursor = DecodeCursor::new(input);

    let message_type = cursor.read_var_bytes("message_type")?;
    if message_type != XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE {
        return Err(RawPayloadDecodeError {
            kind: RawPayloadDecodeErrorKind::UnsupportedMessageType,
            field: "message_type",
            offset: 0,
            expected: XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE.len(),
            actual: message_type.len(),
        });
    }

    let schema_version = cursor.read_u16("schema_version")?;
    if schema_version != XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION {
        return Err(RawPayloadDecodeError {
            kind: RawPayloadDecodeErrorKind::UnsupportedSchemaVersion,
            field: "schema_version",
            offset: cursor.offset.saturating_sub(2),
            expected: XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION as usize,
            actual: schema_version as usize,
        });
    }

    let instruction_layout_version = cursor.read_u16("instruction_layout_version")?;
    if instruction_layout_version != XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION {
        return Err(RawPayloadDecodeError {
            kind: RawPayloadDecodeErrorKind::UnsupportedInstructionLayoutVersion,
            field: "instruction_layout_version",
            offset: cursor.offset.saturating_sub(2),
            expected: XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION as usize,
            actual: instruction_layout_version as usize,
        });
    }

    let route_id = cursor.read_bytes32("route_id")?;
    let source_chain_id = cursor.read_u64("source_chain_id")?;
    let source_token = cursor.read_var_bytes("source_token")?;
    let source_sender = cursor.read_var_bytes("source_sender")?;
    let source_burn_tx_hash = cursor.read_var_bytes("source_burn_tx_hash")?;
    let source_burn_event_index = cursor.read_u64("source_burn_event_index")?;
    let source_block_number = cursor.read_u64("source_block_number")?;
    let source_block_hash = cursor.read_var_bytes("source_block_hash")?;
    let source_finality_block = cursor.read_u64("source_finality_block")?;
    let canonical_event_key = cursor.read_bytes32("canonical_event_key")?;
    let x1_recipient = cursor.read_bytes32("x1_recipient")?;
    let burned_amount = cursor.read_u128("burned_amount")?;
    let source_chain_weight_bps = cursor.read_u16("source_chain_weight_bps")?;
    let xxxl_mint_amount = cursor.read_u128("xxxl_mint_amount")?;
    let target_mint = cursor.read_bytes32("target_mint")?;
    let guardian_set_id = cursor.read_bytes32("guardian_set_id")?;
    let message_nonce = cursor.read_bytes32("message_nonce")?;
    let expiration_slot_or_unix_ts = cursor.read_u64("expiration_slot_or_unix_ts")?;

    cursor.finish()?;

    Ok(DecodedGuardianPayloadRaw {
        message_type,
        schema_version,
        instruction_layout_version,
        route_id,
        source_chain_id,
        source_token,
        source_sender,
        source_burn_tx_hash,
        source_burn_event_index,
        source_block_number,
        source_block_hash,
        source_finality_block,
        canonical_event_key,
        x1_recipient,
        burned_amount,
        source_chain_weight_bps,
        xxxl_mint_amount,
        target_mint,
        guardian_set_id,
        message_nonce,
        expiration_slot_or_unix_ts,
    })
}

pub fn raw_payload_decoder_report() -> &'static RawPayloadDecoderReport {
    &RAW_PAYLOAD_DECODER_REPORT
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::read_only_verifier_boundary;

    const VALID_PAYLOAD_HEX: &str = "11005858584c5f474154455741595f4d494e5401000200\
         1111111111111111111111111111111111111111111111111111111111111111\
         010000000000000006000102030405061400\
         aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
         2000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\
         070000000000000015cd5b07000000002000\
         bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb\
         84cd5b0700000000\
         4444444444444444444444444444444444444444444444444444444444444444\
         5555555555555555555555555555555555555555555555555555555555555555\
         0010a5d4e8000000000000000000000010270010a5d4e80000000000000000000000\
         3333333333333333333333333333333333333333333333333333333333333333\
         2222222222222222222222222222222222222222222222222222222222222222\
         6666666666666666666666666666666666666666666666666666666666666666\
         b168de3a00000000";

    fn hex_val(byte: u8) -> u8 {
        match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            b'A'..=b'F' => byte - b'A' + 10,
            _ => panic!("invalid hex byte"),
        }
    }

    fn hex_bytes(hex: &str) -> Vec<u8> {
        let compact: Vec<u8> = hex
            .bytes()
            .filter(|byte| !byte.is_ascii_whitespace())
            .collect();
        assert_eq!(compact.len() % 2, 0);

        compact
            .chunks_exact(2)
            .map(|pair| (hex_val(pair[0]) << 4) | hex_val(pair[1]))
            .collect()
    }

    fn valid_payload() -> Vec<u8> {
        hex_bytes(VALID_PAYLOAD_HEX)
    }

    fn write_u16_le(out: &mut Vec<u8>, value: u16) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u64_le(out: &mut Vec<u8>, value: u64) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn write_u128_le(out: &mut Vec<u8>, value: u128) {
        out.extend_from_slice(&value.to_le_bytes());
    }

    fn write_var(out: &mut Vec<u8>, bytes: &[u8]) {
        write_u16_le(out, bytes.len() as u16);
        out.extend_from_slice(bytes);
    }

    fn structurally_detectable_wrong_field_order_payload() -> Vec<u8> {
        let mut out = Vec::new();

        write_var(&mut out, XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE);
        write_u16_le(&mut out, XXXL_GUARDIAN_PAYLOAD_SCHEMA_VERSION);
        write_u16_le(&mut out, XXXL_GUARDIAN_PAYLOAD_INSTRUCTION_LAYOUT_VERSION);
        out.extend_from_slice(&[0x11; 32]);
        write_var(&mut out, &[1, 2, 3, 4, 5, 6]);
        write_u64_le(&mut out, 1);
        write_var(&mut out, &[0xaa; 20]);
        write_var(&mut out, &[0xaa; 32]);
        write_u64_le(&mut out, 7);
        write_u64_le(&mut out, 123_456_789);
        write_var(&mut out, &[0xbb; 32]);
        write_u64_le(&mut out, 123_456_900);
        out.extend_from_slice(&[0x44; 32]);
        out.extend_from_slice(&[0x55; 32]);
        write_u128_le(&mut out, 1_000_000_000_000);
        write_u16_le(&mut out, 10_000);
        write_u128_le(&mut out, 1_000_000_000_000);
        out.extend_from_slice(&[0x33; 32]);
        out.extend_from_slice(&[0x22; 32]);
        out.extend_from_slice(&[0x66; 32]);
        write_u64_le(&mut out, 987_654_321);

        out
    }

    #[test]
    fn valid_canonical_payload_decodes() {
        let payload = valid_payload();
        let decoded = decode_guardian_payload_raw(&payload).expect("valid canonical payload");

        assert_eq!(payload.len(), 387);
        assert_eq!(decoded.message_type, XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE);
        assert_eq!(decoded.schema_version, 1);
        assert_eq!(decoded.instruction_layout_version, 2);
        assert_eq!(decoded.route_id, &[0x11; 32]);
        assert_eq!(decoded.source_chain_id, 1);
        assert_eq!(decoded.source_token, &[1, 2, 3, 4, 5, 6]);
        assert_eq!(decoded.source_sender, &[0xaa; 20]);
        assert_eq!(decoded.source_burn_tx_hash, &[0xaa; 32]);
        assert_eq!(decoded.source_burn_event_index, 7);
        assert_eq!(decoded.source_block_number, 123_456_789);
        assert_eq!(decoded.source_block_hash, &[0xbb; 32]);
        assert_eq!(decoded.source_finality_block, 123_456_900);
        assert_eq!(decoded.canonical_event_key, &[0x44; 32]);
        assert_eq!(decoded.x1_recipient, &[0x55; 32]);
        assert_eq!(decoded.burned_amount, 1_000_000_000_000);
        assert_eq!(decoded.source_chain_weight_bps, 10_000);
        assert_eq!(decoded.xxxl_mint_amount, 1_000_000_000_000);
        assert_eq!(decoded.target_mint, &[0x33; 32]);
        assert_eq!(decoded.guardian_set_id, &[0x22; 32]);
        assert_eq!(decoded.message_nonce, &[0x66; 32]);
        assert_eq!(decoded.expiration_slot_or_unix_ts, 987_654_321);
    }

    #[test]
    fn decoded_field_order_matches_phase_23() {
        assert_eq!(RAW_PAYLOAD_PHASE_23_FIELD_ORDER.len(), 21);
        assert_eq!(
            RAW_PAYLOAD_PHASE_23_FIELD_ORDER,
            [
                "message_type",
                "schema_version",
                "instruction_layout_version",
                "route_id",
                "source_chain_id",
                "source_token",
                "source_sender",
                "source_burn_tx_hash",
                "source_burn_event_index",
                "source_block_number",
                "source_block_hash",
                "source_finality_block",
                "canonical_event_key",
                "x1_recipient",
                "burned_amount",
                "source_chain_weight_bps",
                "xxxl_mint_amount",
                "target_mint",
                "guardian_set_id",
                "message_nonce",
                "expiration_slot_or_unix_ts",
            ]
        );
    }

    #[test]
    fn truncated_payload_is_rejected() {
        let mut payload = valid_payload();
        payload.pop();

        let error = decode_guardian_payload_raw(&payload).expect_err("truncated payload");

        assert_eq!(error.kind, RawPayloadDecodeErrorKind::Truncated);
        assert_eq!(error.field, "expiration_slot_or_unix_ts");
    }

    #[test]
    fn trailing_bytes_are_rejected() {
        let mut payload = valid_payload();
        payload.push(0);

        let error = decode_guardian_payload_raw(&payload).expect_err("trailing payload");

        assert_eq!(error.kind, RawPayloadDecodeErrorKind::TrailingBytes);
        assert_eq!(error.field, "payload");
    }

    #[test]
    fn empty_variable_length_field_is_rejected() {
        let mut payload = valid_payload();
        let source_token_len_offset = 2 + XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE.len() + 2 + 2 + 32 + 8;
        payload[source_token_len_offset] = 0;
        payload[source_token_len_offset + 1] = 0;

        let error = decode_guardian_payload_raw(&payload).expect_err("empty source token");

        assert_eq!(error.kind, RawPayloadDecodeErrorKind::EmptyVariableBytes);
        assert_eq!(error.field, "source_token");
    }

    #[test]
    fn malformed_length_prefixed_encoding_is_rejected() {
        let mut payload = valid_payload();
        let source_token_len_offset = 2 + XXXL_GUARDIAN_PAYLOAD_MESSAGE_TYPE.len() + 2 + 2 + 32 + 8;
        payload[source_token_len_offset] = 0xff;
        payload[source_token_len_offset + 1] = 0xff;

        let error = decode_guardian_payload_raw(&payload).expect_err("oversized source token");

        assert_eq!(error.kind, RawPayloadDecodeErrorKind::Truncated);
        assert_eq!(error.field, "source_token");
    }

    #[test]
    fn wrong_byte_encoding_fixture_is_rejected_at_decoder_level() {
        let mut payload = valid_payload();
        payload[0] = 0x12;

        let error = decode_guardian_payload_raw(&payload).expect_err("wrong message length");

        assert_eq!(
            error.kind,
            RawPayloadDecodeErrorKind::UnsupportedMessageType
        );
        assert_eq!(error.field, "message_type");
    }

    #[test]
    fn structurally_detectable_wrong_field_order_is_rejected() {
        let payload = structurally_detectable_wrong_field_order_payload();

        let error = decode_guardian_payload_raw(&payload).expect_err("structural wrong order");

        assert_eq!(error.kind, RawPayloadDecodeErrorKind::EmptyVariableBytes);
        assert_eq!(error.field, "source_sender");
    }

    #[test]
    fn decoder_report_is_read_only_and_partial_for_field_order() {
        let report = raw_payload_decoder_report();

        assert_eq!(report.decoder_id, RAW_PAYLOAD_DECODER_PHASE_33);
        assert_eq!(report.decoder_version, 1);
        assert_eq!(report.field_count, 21);
        assert!(report.follows_phase_23_canonical_layout);
        assert!(report.rejects_truncated_payloads);
        assert!(report.rejects_trailing_bytes);
        assert!(report.rejects_empty_variable_bytes);
        assert!(report.rejects_structurally_invalid_byte_encoding);
        assert!(report.wrong_field_order_structural_only);
        assert!(!report.all_wrong_field_order_cases_satisfied);
        assert!(!report.live_route_enabled);
        assert!(!report.spl_cpi_enabled);
        assert!(!report.invoke_signed_enabled);
        assert!(!report.mint_execution_enabled);
        assert!(!report.runtime_state_mutation_enabled);
        assert!(!report.replay_write_enabled);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.production_program_id_selected);
        assert!(!report.deployment_blockers_removed);
    }

    #[test]
    fn phase_32_safety_flags_remain_false() {
        let boundary = read_only_verifier_boundary();

        assert!(!boundary.live_route_enabled);
        assert!(!boundary.spl_cpi_enabled);
        assert!(!boundary.invoke_signed_enabled);
        assert!(!boundary.mint_execution_enabled);
        assert!(!boundary.runtime_state_mutation_enabled);
        assert!(!boundary.replay_write_enabled);
        assert!(!boundary.processed_event_marking_enabled);
        assert!(!boundary.production_program_id_selected);
        assert!(!boundary.deployment_blockers_removed);
    }
}

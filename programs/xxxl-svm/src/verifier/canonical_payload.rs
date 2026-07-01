#[allow(deprecated)]
use solana_program::keccak;

use super::{
    decode_guardian_payload_raw, raw_payload_decoder_report, read_only_verifier_boundary,
    RawPayloadDecodeError, RAW_PAYLOAD_DECODER_PHASE_33,
};

pub const CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34: &str =
    "CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34";
pub const CANONICAL_PAYLOAD_HASH_VALIDATOR_VERSION: u8 = 1;
pub const XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1: &str = "XXXL_GUARDIAN_PAYLOAD_HASH_V1";

pub const XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1: [u8; 32] = [
    0xf1, 0x95, 0x8b, 0xbf, 0x04, 0xd4, 0x5d, 0xdb, 0xc5, 0xa9, 0xf9, 0x3f, 0x20, 0x0f, 0x50, 0x05,
    0xee, 0x47, 0xb0, 0x5c, 0xf6, 0x1a, 0x90, 0xfa, 0xf4, 0xd9, 0x3c, 0xd6, 0xe3, 0xec, 0xcd, 0x66,
];

pub const XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1: [u8; 32] = [
    0xab, 0x0e, 0xe5, 0x9a, 0x12, 0x68, 0xf3, 0xee, 0xbf, 0x4a, 0x9d, 0x42, 0x72, 0x56, 0x40, 0xce,
    0x68, 0x22, 0x6e, 0x64, 0x2a, 0x61, 0xda, 0xbd, 0x5f, 0x90, 0x4e, 0x76, 0x80, 0xf0, 0x80, 0x15,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanonicalPayloadHashValidationErrorKind {
    RawPayloadDecode,
    HashMismatch,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalPayloadHashValidationError {
    pub kind: CanonicalPayloadHashValidationErrorKind,
    pub raw_payload_error: Option<RawPayloadDecodeError>,
    pub expected_hash: Option<[u8; 32]>,
    pub actual_hash: Option<[u8; 32]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalPayloadHashValidationReport {
    pub validator_id: &'static str,
    pub validator_version: u8,
    pub hash_domain_label: &'static str,
    pub phase_23_domain_separator: [u8; 32],
    pub phase_23_valid_payload_hash: [u8; 32],
    pub raw_payload_decode_required_first: bool,
    pub raw_payload_decoder_phase: &'static str,
    pub recomputes_hash_from_payload_bytes: bool,
    pub caller_provided_payload_hash_trusted: bool,
    pub hash_domain_matches_phase_23: bool,
    pub source_proof_verified: bool,
    pub route_config_verified: bool,
    pub guardian_quorum_verified: bool,
    pub amount_cap_enforced: bool,
    pub target_mint_account_checked: bool,
    pub replay_check_enabled: bool,
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

pub const CANONICAL_PAYLOAD_HASH_VALIDATION_REPORT: CanonicalPayloadHashValidationReport =
    CanonicalPayloadHashValidationReport {
        validator_id: CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
        validator_version: CANONICAL_PAYLOAD_HASH_VALIDATOR_VERSION,
        hash_domain_label: XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1,
        phase_23_domain_separator: XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1,
        phase_23_valid_payload_hash: XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
        raw_payload_decode_required_first: true,
        raw_payload_decoder_phase: RAW_PAYLOAD_DECODER_PHASE_33,
        recomputes_hash_from_payload_bytes: true,
        caller_provided_payload_hash_trusted: false,
        hash_domain_matches_phase_23: true,
        source_proof_verified: false,
        route_config_verified: false,
        guardian_quorum_verified: false,
        amount_cap_enforced: false,
        target_mint_account_checked: false,
        replay_check_enabled: false,
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

pub fn compute_guardian_payload_hash_domain_separator() -> [u8; 32] {
    keccak::hash(XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1.as_bytes()).to_bytes()
}

pub fn compute_guardian_payload_hash(
    input: &[u8],
) -> Result<[u8; 32], CanonicalPayloadHashValidationError> {
    let domain_separator = compute_guardian_payload_hash_domain_separator();

    compute_guardian_payload_hash_with_domain_separator(input, &domain_separator)
}

pub fn validate_guardian_payload_hash(
    input: &[u8],
    expected_hash: &[u8; 32],
) -> Result<(), CanonicalPayloadHashValidationError> {
    let actual_hash = compute_guardian_payload_hash(input)?;

    if &actual_hash != expected_hash {
        return Err(CanonicalPayloadHashValidationError {
            kind: CanonicalPayloadHashValidationErrorKind::HashMismatch,
            raw_payload_error: None,
            expected_hash: Some(*expected_hash),
            actual_hash: Some(actual_hash),
        });
    }

    Ok(())
}

pub fn canonical_payload_hash_validation_report() -> &'static CanonicalPayloadHashValidationReport {
    &CANONICAL_PAYLOAD_HASH_VALIDATION_REPORT
}

fn compute_guardian_payload_hash_with_domain_separator(
    input: &[u8],
    domain_separator: &[u8; 32],
) -> Result<[u8; 32], CanonicalPayloadHashValidationError> {
    decode_guardian_payload_raw(input).map_err(|error| CanonicalPayloadHashValidationError {
        kind: CanonicalPayloadHashValidationErrorKind::RawPayloadDecode,
        raw_payload_error: Some(error),
        expected_hash: None,
        actual_hash: None,
    })?;

    Ok(keccak::hashv(&[domain_separator, input]).to_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        decode_guardian_payload_raw, RAW_PAYLOAD_DECODER_PHASE_33,
        READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
    };

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

    #[test]
    fn marker_version_and_domain_are_stable() {
        assert_eq!(
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34,
            "CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34"
        );
        assert_eq!(CANONICAL_PAYLOAD_HASH_VALIDATOR_VERSION, 1);
        assert_eq!(
            XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1,
            "XXXL_GUARDIAN_PAYLOAD_HASH_V1"
        );
        assert_eq!(
            compute_guardian_payload_hash_domain_separator(),
            XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1
        );
    }

    #[test]
    fn canonical_payload_hash_matches_phase_23_ts_vector() {
        let payload = valid_payload();

        let actual_hash =
            compute_guardian_payload_hash(&payload).expect("valid canonical payload hash");

        assert_eq!(actual_hash, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1);
    }

    #[test]
    fn validation_accepts_matching_recomputed_hash() {
        let payload = valid_payload();

        validate_guardian_payload_hash(&payload, &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1)
            .expect("matching recomputed payload hash");
    }

    #[test]
    fn validation_rejects_wrong_expected_hash() {
        let payload = valid_payload();
        let mut wrong_hash = XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1;
        wrong_hash[0] ^= 0xff;

        let error =
            validate_guardian_payload_hash(&payload, &wrong_hash).expect_err("wrong expected hash");

        assert_eq!(
            error.kind,
            CanonicalPayloadHashValidationErrorKind::HashMismatch
        );
        assert_eq!(error.expected_hash, Some(wrong_hash));
        assert_eq!(error.actual_hash, Some(XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1));
        assert_eq!(error.raw_payload_error, None);
    }

    #[test]
    fn malformed_raw_payload_is_rejected_before_hash_acceptance() {
        let mut payload = valid_payload();
        payload.pop();

        let error = validate_guardian_payload_hash(&payload, &XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1)
            .expect_err("malformed raw payload");

        assert_eq!(
            error.kind,
            CanonicalPayloadHashValidationErrorKind::RawPayloadDecode
        );
        assert!(error.raw_payload_error.is_some());
        assert_eq!(error.expected_hash, None);
        assert_eq!(error.actual_hash, None);
    }

    #[test]
    fn wrong_domain_material_does_not_match_phase_23_hash() {
        let payload = valid_payload();
        let mut wrong_domain = XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1;
        wrong_domain[31] ^= 0x01;

        let wrong_hash =
            compute_guardian_payload_hash_with_domain_separator(&payload, &wrong_domain)
                .expect("valid payload under wrong test domain");

        assert_ne!(wrong_hash, XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1);
    }

    #[test]
    fn report_states_hash_is_recomputed_and_caller_hash_is_not_trusted() {
        let report = canonical_payload_hash_validation_report();

        assert_eq!(
            report.validator_id,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34
        );
        assert_eq!(
            report.validator_version,
            CANONICAL_PAYLOAD_HASH_VALIDATOR_VERSION
        );
        assert_eq!(
            report.hash_domain_label,
            XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1
        );
        assert_eq!(
            report.phase_23_domain_separator,
            XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1
        );
        assert_eq!(
            report.phase_23_valid_payload_hash,
            XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1
        );
        assert!(report.raw_payload_decode_required_first);
        assert_eq!(
            report.raw_payload_decoder_phase,
            RAW_PAYLOAD_DECODER_PHASE_33
        );
        assert!(report.recomputes_hash_from_payload_bytes);
        assert!(!report.caller_provided_payload_hash_trusted);
        assert!(report.hash_domain_matches_phase_23);
    }

    #[test]
    fn report_keeps_execution_and_unfinished_verifier_surfaces_disabled() {
        let report = canonical_payload_hash_validation_report();

        assert!(!report.source_proof_verified);
        assert!(!report.route_config_verified);
        assert!(!report.guardian_quorum_verified);
        assert!(!report.amount_cap_enforced);
        assert!(!report.target_mint_account_checked);
        assert!(!report.replay_check_enabled);
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
    fn phase_32_boundary_safety_flags_remain_false() {
        let boundary = read_only_verifier_boundary();

        assert_eq!(
            boundary.scaffold_id,
            READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32
        );
        assert!(!boundary.execution_enabled());
        assert!(!boundary.deployment_unlocked());
        assert!(!boundary.live_route_enabled);
        assert!(!boundary.spl_cpi_enabled);
        assert!(!boundary.invoke_signed_enabled);
        assert!(!boundary.mint_execution_enabled);
        assert!(!boundary.runtime_state_mutation_enabled);
        assert!(!boundary.replay_write_enabled);
        assert!(!boundary.processed_event_marking_enabled);
    }

    #[test]
    fn phase_33_decoder_still_accepts_valid_payload() {
        let payload = valid_payload();
        let decoded = decode_guardian_payload_raw(&payload).expect("phase 33 decoder");
        let decoder_report = raw_payload_decoder_report();

        assert_eq!(payload.len(), 387);
        assert_eq!(decoded.source_chain_id, 1);
        assert_eq!(decoded.source_chain_weight_bps, 10_000);
        assert_eq!(decoded.expiration_slot_or_unix_ts, 987_654_321);
        assert_eq!(decoder_report.decoder_id, RAW_PAYLOAD_DECODER_PHASE_33);
        assert!(decoder_report.follows_phase_23_canonical_layout);
    }
}

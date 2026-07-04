use solana_program::{hash::hashv, pubkey::Pubkey};

use super::b1c_ed25519_evidence_parser::B1CParsedEd25519Evidence;

pub const PHASE_41K6_B1C_4_PAYLOAD_HASH_BINDING_PHASE: &str = "41K.6-B1C.4";
pub const PHASE_41K6_B1C_4_PAYLOAD_HASH_BINDING_VERSION: &str = "0.1.0";
pub const B1C_AUTHORIZATION_PAYLOAD_DOMAIN: &[u8] = b"consume_gateway_mint_authorization_v1";
pub const B1C_AUTHORIZATION_PAYLOAD_HASH_LEN: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CAuthorizationPayloadContext {
    pub processed_event: Pubkey,
    pub mint: Pubkey,
    pub recipient: Pubkey,
    pub amount: u64,
    pub guardian_set_id: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CPayloadHashBindingStatus {
    Bound,
    NoParsedEvidence,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CPayloadHashBindingRejectionKind {
    NoParsedEvidence,
    SignedMessageWrongLength,
    PayloadHashMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CPayloadBoundEvidence {
    pub signer_public_key: [u8; 32],
    pub source_instruction_index: usize,
    pub signed_message: [u8; 32],
    pub matches_expected_payload_hash: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CPayloadHashBindingResult {
    pub status: B1CPayloadHashBindingStatus,
    pub rejection_kind: Option<B1CPayloadHashBindingRejectionKind>,
    pub expected_payload_hash: [u8; 32],
    pub checked_evidence_count: usize,
    pub bound_evidence_count: usize,
    pub bound_evidence: Vec<B1CPayloadBoundEvidence>,
    pub computes_hash_locally: bool,
    pub accepts_caller_provided_hash: bool,
    pub binds_payload_hash: bool,
    pub validates_guardian_membership: bool,
    pub counts_unique_guardians: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CPayloadHashBindingReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub hash_algorithm: &'static str,
    pub domain: &'static [u8],
    pub computes_hash_locally: bool,
    pub accepts_caller_provided_hash: bool,
    pub includes_current_slot: bool,
    pub binds_payload_hash: bool,
    pub validates_guardian_membership: bool,
    pub counts_unique_guardians: bool,
    pub authorizes_handler_execution: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

pub const B1C_PAYLOAD_HASH_BINDING_REPORT: B1CPayloadHashBindingReport =
    B1CPayloadHashBindingReport {
        phase: PHASE_41K6_B1C_4_PAYLOAD_HASH_BINDING_PHASE,
        version: PHASE_41K6_B1C_4_PAYLOAD_HASH_BINDING_VERSION,
        hash_algorithm: "sha256",
        domain: B1C_AUTHORIZATION_PAYLOAD_DOMAIN,
        computes_hash_locally: true,
        accepts_caller_provided_hash: false,
        includes_current_slot: false,
        binds_payload_hash: true,
        validates_guardian_membership: false,
        counts_unique_guardians: false,
        authorizes_handler_execution: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    };

pub fn b1c_payload_hash_binding_report() -> &'static B1CPayloadHashBindingReport {
    &B1C_PAYLOAD_HASH_BINDING_REPORT
}

pub fn compute_b1c_expected_authorization_payload_hash(
    context: &B1CAuthorizationPayloadContext,
) -> [u8; 32] {
    let amount = context.amount.to_le_bytes();
    let guardian_set_id = context.guardian_set_id.to_le_bytes();

    let hash = hashv(&[
        B1C_AUTHORIZATION_PAYLOAD_DOMAIN,
        context.processed_event.as_ref(),
        context.mint.as_ref(),
        context.recipient.as_ref(),
        &amount,
        &guardian_set_id,
    ]);

    let mut out = [0u8; 32];
    out.copy_from_slice(hash.as_ref());
    out
}

pub fn bind_b1c_parsed_ed25519_evidence_to_expected_payload_hash(
    context: &B1CAuthorizationPayloadContext,
    parsed_evidence: &[B1CParsedEd25519Evidence],
) -> B1CPayloadHashBindingResult {
    let expected_payload_hash = compute_b1c_expected_authorization_payload_hash(context);

    if parsed_evidence.is_empty() {
        return result(
            B1CPayloadHashBindingStatus::NoParsedEvidence,
            Some(B1CPayloadHashBindingRejectionKind::NoParsedEvidence),
            expected_payload_hash,
            0,
            Vec::new(),
        );
    }

    let mut bound = Vec::new();

    for evidence in parsed_evidence.iter() {
        if evidence.signed_message.len() != B1C_AUTHORIZATION_PAYLOAD_HASH_LEN {
            return result(
                B1CPayloadHashBindingStatus::Rejected,
                Some(B1CPayloadHashBindingRejectionKind::SignedMessageWrongLength),
                expected_payload_hash,
                bound.len() + 1,
                Vec::new(),
            );
        }

        if evidence.signed_message.as_slice() != expected_payload_hash {
            return result(
                B1CPayloadHashBindingStatus::Rejected,
                Some(B1CPayloadHashBindingRejectionKind::PayloadHashMismatch),
                expected_payload_hash,
                bound.len() + 1,
                Vec::new(),
            );
        }

        bound.push(B1CPayloadBoundEvidence {
            signer_public_key: evidence.signer_public_key,
            source_instruction_index: evidence.source_instruction_index,
            signed_message: expected_payload_hash,
            matches_expected_payload_hash: true,
        });
    }

    result(
        B1CPayloadHashBindingStatus::Bound,
        None,
        expected_payload_hash,
        parsed_evidence.len(),
        bound,
    )
}

fn result(
    status: B1CPayloadHashBindingStatus,
    rejection_kind: Option<B1CPayloadHashBindingRejectionKind>,
    expected_payload_hash: [u8; 32],
    checked_evidence_count: usize,
    bound_evidence: Vec<B1CPayloadBoundEvidence>,
) -> B1CPayloadHashBindingResult {
    B1CPayloadHashBindingResult {
        status,
        rejection_kind,
        expected_payload_hash,
        checked_evidence_count,
        bound_evidence_count: bound_evidence.len(),
        bound_evidence,
        computes_hash_locally: true,
        accepts_caller_provided_hash: false,
        binds_payload_hash: status == B1CPayloadHashBindingStatus::Bound,
        validates_guardian_membership: false,
        counts_unique_guardians: false,
        authorization_enabled: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::b1c_ed25519_evidence_parser::{
        B1CEd25519EvidenceParsingStatus, B1CParsedEd25519Evidence, ED25519_SIGNATURE_LEN,
    };

    fn context() -> B1CAuthorizationPayloadContext {
        B1CAuthorizationPayloadContext {
            processed_event: Pubkey::new_from_array([1; 32]),
            mint: Pubkey::new_from_array([2; 32]),
            recipient: Pubkey::new_from_array([3; 32]),
            amount: 123,
            guardian_set_id: 7,
        }
    }

    fn evidence(message: Vec<u8>, signer_byte: u8) -> B1CParsedEd25519Evidence {
        B1CParsedEd25519Evidence {
            status: B1CEd25519EvidenceParsingStatus::Parsed,
            source_instruction_index: signer_byte as usize,
            signer_public_key: [signer_byte; 32],
            signature: [0x55; ED25519_SIGNATURE_LEN],
            signed_message: message,
            instruction_data_len: 128,
            signature_offset: 16,
            public_key_offset: 80,
            message_data_offset: 112,
            message_data_size: 32,
            runtime_verified_by_ed25519_precompile: true,
            parsed_from_prior_ed25519_instruction: true,
            single_signature_layout: true,
            self_contained_current_instruction_offsets_only: true,
            accepts_caller_provided_signature_claims: false,
            accepts_frontend_or_watcher_proof: false,
            binds_payload_hash: false,
            validates_guardian_membership: false,
            counts_unique_guardians: false,
            authorization_enabled: false,
            processed_event_marking_enabled: false,
            cpi_enabled: false,
            live_route_enabled: false,
        }
    }

    fn assert_execution_flags_false(result: &B1CPayloadHashBindingResult) {
        assert!(!result.accepts_caller_provided_hash);
        assert!(!result.validates_guardian_membership);
        assert!(!result.counts_unique_guardians);
        assert!(!result.authorization_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn report_documents_b1c4_scope() {
        let report = b1c_payload_hash_binding_report();

        assert_eq!(report.phase, PHASE_41K6_B1C_4_PAYLOAD_HASH_BINDING_PHASE);
        assert_eq!(report.hash_algorithm, "sha256");
        assert_eq!(report.domain, B1C_AUTHORIZATION_PAYLOAD_DOMAIN);
        assert!(report.computes_hash_locally);
        assert!(!report.accepts_caller_provided_hash);
        assert!(!report.includes_current_slot);
        assert!(report.binds_payload_hash);
        assert!(!report.validates_guardian_membership);
        assert!(!report.counts_unique_guardians);
        assert!(!report.authorizes_handler_execution);
    }

    #[test]
    fn same_parameters_produce_same_hash() {
        assert_eq!(
            compute_b1c_expected_authorization_payload_hash(&context()),
            compute_b1c_expected_authorization_payload_hash(&context())
        );
    }

    #[test]
    fn payload_fields_change_hash() {
        let base = compute_b1c_expected_authorization_payload_hash(&context());

        let mut c = context();
        c.processed_event = Pubkey::new_from_array([9; 32]);
        assert_ne!(base, compute_b1c_expected_authorization_payload_hash(&c));

        let mut c = context();
        c.mint = Pubkey::new_from_array([9; 32]);
        assert_ne!(base, compute_b1c_expected_authorization_payload_hash(&c));

        let mut c = context();
        c.recipient = Pubkey::new_from_array([9; 32]);
        assert_ne!(base, compute_b1c_expected_authorization_payload_hash(&c));

        let mut c = context();
        c.amount += 1;
        assert_ne!(base, compute_b1c_expected_authorization_payload_hash(&c));

        let mut c = context();
        c.guardian_set_id += 1;
        assert_ne!(base, compute_b1c_expected_authorization_payload_hash(&c));
    }

    #[test]
    fn matching_evidence_passes_payload_binding() {
        let c = context();
        let hash = compute_b1c_expected_authorization_payload_hash(&c);
        let result = bind_b1c_parsed_ed25519_evidence_to_expected_payload_hash(
            &c,
            &[evidence(hash.to_vec(), 1), evidence(hash.to_vec(), 2)],
        );

        assert_eq!(result.status, B1CPayloadHashBindingStatus::Bound);
        assert_eq!(result.rejection_kind, None);
        assert_eq!(result.checked_evidence_count, 2);
        assert_eq!(result.bound_evidence_count, 2);
        assert!(result.binds_payload_hash);
        assert_execution_flags_false(&result);
    }

    #[test]
    fn mismatched_evidence_rejects() {
        let c = context();
        let result = bind_b1c_parsed_ed25519_evidence_to_expected_payload_hash(
            &c,
            &[evidence(vec![0xAA; 32], 1)],
        );

        assert_eq!(result.status, B1CPayloadHashBindingStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1CPayloadHashBindingRejectionKind::PayloadHashMismatch)
        );
        assert_eq!(result.bound_evidence_count, 0);
        assert!(!result.binds_payload_hash);
        assert_execution_flags_false(&result);
    }

    #[test]
    fn wrong_signed_message_length_rejects() {
        let c = context();
        let result = bind_b1c_parsed_ed25519_evidence_to_expected_payload_hash(
            &c,
            &[evidence(vec![0xAA; 31], 1)],
        );

        assert_eq!(result.status, B1CPayloadHashBindingStatus::Rejected);
        assert_eq!(
            result.rejection_kind,
            Some(B1CPayloadHashBindingRejectionKind::SignedMessageWrongLength)
        );
        assert_execution_flags_false(&result);
    }

    #[test]
    fn empty_evidence_rejects_without_authorization() {
        let c = context();
        let result = bind_b1c_parsed_ed25519_evidence_to_expected_payload_hash(&c, &[]);

        assert_eq!(result.status, B1CPayloadHashBindingStatus::NoParsedEvidence);
        assert_eq!(
            result.rejection_kind,
            Some(B1CPayloadHashBindingRejectionKind::NoParsedEvidence)
        );
        assert_execution_flags_false(&result);
    }
}

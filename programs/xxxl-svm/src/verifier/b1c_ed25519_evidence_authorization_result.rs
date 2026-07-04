use solana_program::pubkey::Pubkey;

pub const PHASE_41K6_B1C_1_RESULT_TYPES_PHASE: &str = "41K.6-B1C.1";
pub const PHASE_41K6_B1C_1_RESULT_TYPES_VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CEd25519EvidenceAuthorizationStatus {
    Authorized,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CEd25519EvidenceAuthorizationRejectionKind {
    GuardianSetNotLoaded,
    InvalidInstructionSysvar,
    NoPriorInstructions,
    NoPriorEd25519Instructions,
    InvalidEd25519Evidence,
    InvalidPayloadBinding,
    InsufficientGuardians,
    DuplicateGuardianOnly,
    NonGuardianEvidenceOnly,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CEd25519EvidenceAuthorizationEstablished {
    pub status: B1CEd25519EvidenceAuthorizationStatus,
    pub threshold: u8,
    pub unique_guardian_count: usize,
    pub counted_guardian_public_keys: Vec<Pubkey>,
    pub rejected_evidence_count: usize,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CEd25519EvidenceAuthorizationRejected {
    pub status: B1CEd25519EvidenceAuthorizationStatus,
    pub kind: B1CEd25519EvidenceAuthorizationRejectionKind,
    pub threshold: Option<u8>,
    pub unique_guardian_count: usize,
    pub counted_guardian_public_keys: Vec<Pubkey>,
    pub rejected_evidence_count: usize,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum B1CEd25519EvidenceAuthorizationResult {
    Authorized(B1CEd25519EvidenceAuthorizationEstablished),
    Rejected(B1CEd25519EvidenceAuthorizationRejected),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CEd25519EvidenceAuthorizationResultReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub result_type_only: bool,
    pub reads_instructions_sysvar: bool,
    pub parses_ed25519_evidence: bool,
    pub binds_payload_hash: bool,
    pub counts_unique_guardians: bool,
    pub authorizes_handler_execution: bool,
    pub processed_event_marking_enabled_by_default: bool,
    pub cpi_enabled_by_default: bool,
    pub live_route_enabled_by_default: bool,
}

pub const B1C_ED25519_EVIDENCE_AUTHORIZATION_RESULT_REPORT:
    B1CEd25519EvidenceAuthorizationResultReport = B1CEd25519EvidenceAuthorizationResultReport {
    phase: PHASE_41K6_B1C_1_RESULT_TYPES_PHASE,
    version: PHASE_41K6_B1C_1_RESULT_TYPES_VERSION,
    result_type_only: true,
    reads_instructions_sysvar: false,
    parses_ed25519_evidence: false,
    binds_payload_hash: false,
    counts_unique_guardians: false,
    authorizes_handler_execution: false,
    processed_event_marking_enabled_by_default: false,
    cpi_enabled_by_default: false,
    live_route_enabled_by_default: false,
};

pub fn b1c_ed25519_evidence_authorization_result_report(
) -> &'static B1CEd25519EvidenceAuthorizationResultReport {
    &B1C_ED25519_EVIDENCE_AUTHORIZATION_RESULT_REPORT
}

impl B1CEd25519EvidenceAuthorizationResult {
    pub fn authorized(threshold: u8, counted_guardian_public_keys: Vec<Pubkey>) -> Self {
        Self::Authorized(B1CEd25519EvidenceAuthorizationEstablished {
            status: B1CEd25519EvidenceAuthorizationStatus::Authorized,
            threshold,
            unique_guardian_count: counted_guardian_public_keys.len(),
            counted_guardian_public_keys,
            rejected_evidence_count: 0,
            authorization_enabled: true,
            processed_event_marking_enabled: true,
            cpi_enabled: true,
            live_route_enabled: true,
        })
    }

    pub fn rejected(
        kind: B1CEd25519EvidenceAuthorizationRejectionKind,
        threshold: Option<u8>,
        unique_guardian_count: usize,
        counted_guardian_public_keys: Vec<Pubkey>,
        rejected_evidence_count: usize,
    ) -> Self {
        Self::Rejected(B1CEd25519EvidenceAuthorizationRejected {
            status: B1CEd25519EvidenceAuthorizationStatus::Rejected,
            kind,
            threshold,
            unique_guardian_count,
            counted_guardian_public_keys,
            rejected_evidence_count,
            authorization_enabled: false,
            processed_event_marking_enabled: false,
            cpi_enabled: false,
            live_route_enabled: false,
        })
    }

    pub fn authorization_enabled(&self) -> bool {
        match self {
            Self::Authorized(result) => result.authorization_enabled,
            Self::Rejected(result) => result.authorization_enabled,
        }
    }

    pub fn processed_event_marking_enabled(&self) -> bool {
        match self {
            Self::Authorized(result) => result.processed_event_marking_enabled,
            Self::Rejected(result) => result.processed_event_marking_enabled,
        }
    }

    pub fn cpi_enabled(&self) -> bool {
        match self {
            Self::Authorized(result) => result.cpi_enabled,
            Self::Rejected(result) => result.cpi_enabled,
        }
    }

    pub fn live_route_enabled(&self) -> bool {
        match self {
            Self::Authorized(result) => result.live_route_enabled,
            Self::Rejected(result) => result.live_route_enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn guardian(byte: u8) -> Pubkey {
        Pubkey::new_from_array([byte; 32])
    }

    #[test]
    fn report_documents_b1c1_result_type_only_boundary() {
        let report = b1c_ed25519_evidence_authorization_result_report();

        assert_eq!(report.phase, PHASE_41K6_B1C_1_RESULT_TYPES_PHASE);
        assert_eq!(report.version, PHASE_41K6_B1C_1_RESULT_TYPES_VERSION);
        assert!(report.result_type_only);
        assert!(!report.reads_instructions_sysvar);
        assert!(!report.parses_ed25519_evidence);
        assert!(!report.binds_payload_hash);
        assert!(!report.counts_unique_guardians);
        assert!(!report.authorizes_handler_execution);
        assert!(!report.processed_event_marking_enabled_by_default);
        assert!(!report.cpi_enabled_by_default);
        assert!(!report.live_route_enabled_by_default);
    }

    #[test]
    fn authorized_result_sets_authorization_flags_inside_feature_gated_type_only() {
        let result =
            B1CEd25519EvidenceAuthorizationResult::authorized(2, vec![guardian(1), guardian(2)]);

        assert!(result.authorization_enabled());
        assert!(result.processed_event_marking_enabled());
        assert!(result.cpi_enabled());
        assert!(result.live_route_enabled());

        match result {
            B1CEd25519EvidenceAuthorizationResult::Authorized(authorized) => {
                assert_eq!(
                    authorized.status,
                    B1CEd25519EvidenceAuthorizationStatus::Authorized
                );
                assert_eq!(authorized.threshold, 2);
                assert_eq!(authorized.unique_guardian_count, 2);
                assert_eq!(authorized.counted_guardian_public_keys.len(), 2);
                assert_eq!(authorized.rejected_evidence_count, 0);
            }
            B1CEd25519EvidenceAuthorizationResult::Rejected(_) => {
                panic!("expected authorized result")
            }
        }
    }

    #[test]
    fn insufficient_guardians_rejection_keeps_all_execution_flags_false() {
        let result = B1CEd25519EvidenceAuthorizationResult::rejected(
            B1CEd25519EvidenceAuthorizationRejectionKind::InsufficientGuardians,
            Some(3),
            2,
            vec![guardian(1), guardian(2)],
            0,
        );

        assert!(!result.authorization_enabled());
        assert!(!result.processed_event_marking_enabled());
        assert!(!result.cpi_enabled());
        assert!(!result.live_route_enabled());

        match result {
            B1CEd25519EvidenceAuthorizationResult::Rejected(rejected) => {
                assert_eq!(
                    rejected.status,
                    B1CEd25519EvidenceAuthorizationStatus::Rejected
                );
                assert_eq!(
                    rejected.kind,
                    B1CEd25519EvidenceAuthorizationRejectionKind::InsufficientGuardians
                );
                assert_eq!(rejected.threshold, Some(3));
                assert_eq!(rejected.unique_guardian_count, 2);
                assert_eq!(rejected.counted_guardian_public_keys.len(), 2);
            }
            B1CEd25519EvidenceAuthorizationResult::Authorized(_) => {
                panic!("expected rejected result")
            }
        }
    }

    #[test]
    fn invalid_payload_binding_rejection_keeps_all_execution_flags_false() {
        let result = B1CEd25519EvidenceAuthorizationResult::rejected(
            B1CEd25519EvidenceAuthorizationRejectionKind::InvalidPayloadBinding,
            Some(2),
            0,
            Vec::new(),
            1,
        );

        assert!(!result.authorization_enabled());
        assert!(!result.processed_event_marking_enabled());
        assert!(!result.cpi_enabled());
        assert!(!result.live_route_enabled());
    }

    #[test]
    fn guardian_set_not_loaded_rejection_keeps_all_execution_flags_false() {
        let result = B1CEd25519EvidenceAuthorizationResult::rejected(
            B1CEd25519EvidenceAuthorizationRejectionKind::GuardianSetNotLoaded,
            None,
            0,
            Vec::new(),
            0,
        );

        assert!(!result.authorization_enabled());
        assert!(!result.processed_event_marking_enabled());
        assert!(!result.cpi_enabled());
        assert!(!result.live_route_enabled());
    }
}

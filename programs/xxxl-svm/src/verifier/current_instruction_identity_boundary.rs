use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41C2_CURRENT_INSTRUCTION_IDENTITY_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41C2_CURRENT_INSTRUCTION_IDENTITY_BOUNDARY_PHASE: &str = "41C2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41C2CurrentInstructionIdentityStatus {
    MissingCurrentInstructionIdentity,
    InconsistentCurrentInstructionIdentity,
    CurrentInstructionIdentityBound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C2CurrentInstructionIdentityDescriptor {
    pub current_instruction_identity_supplied: bool,
    pub program_id_matches_expected_program: bool,
    pub discriminator_matches_expected_instruction: bool,
    pub payload_binding_matches_expected_context: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C2CurrentInstructionIdentityResult {
    pub status: Phase41C2CurrentInstructionIdentityStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub current_instruction_identity_boundary_enabled: bool,
    pub current_instruction_identity_derived_from_runtime: bool,
    pub reads_concrete_instruction_content: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C2CurrentInstructionIdentityBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub model_or_boundary_only: bool,
    pub current_instruction_identity_boundary_enabled: bool,
    pub real_runtime_sysvar_population_deferred: bool,
    pub account_info_parser_implemented: bool,
    pub load_instruction_deferred: bool,
    pub reads_concrete_instruction_content: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub allowed_result_count: usize,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41C2_ALLOWED_IDENTITY_STATUSES: [Phase41C2CurrentInstructionIdentityStatus; 3] = [
    Phase41C2CurrentInstructionIdentityStatus::MissingCurrentInstructionIdentity,
    Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity,
    Phase41C2CurrentInstructionIdentityStatus::CurrentInstructionIdentityBound,
];

pub const PHASE_41C2_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: false,
    load_instruction_called: false,
    load_instruction_enabled: false,
    concrete_runtime_api_selected: true,
    current_instruction_identity_derived_from_runtime: false,
    ed25519_signature_verification_performed: false,
    cryptographic_signature_proof_accepted: false,
    verification_evidence_accepted: false,
    quorum_counting_enabled: false,
    authorization_enabled: false,
    replay_write_enabled: false,
    processed_event_marking_enabled: false,
    account_mutation_enabled: false,
    cpi_enabled: false,
    invoke_signed_enabled: false,
    spl_token_mint_to_enabled: false,
    process_instruction_handler_added: false,
    live_route_enabled: false,
};

pub const PHASE_41C2_CURRENT_INSTRUCTION_IDENTITY_BOUNDARY_REPORT:
    Phase41C2CurrentInstructionIdentityBoundaryReport =
    Phase41C2CurrentInstructionIdentityBoundaryReport {
        phase: PHASE_41C2_CURRENT_INSTRUCTION_IDENTITY_BOUNDARY_PHASE,
        version: PHASE_41C2_CURRENT_INSTRUCTION_IDENTITY_BOUNDARY_VERSION,
        model_or_boundary_only: true,
        current_instruction_identity_boundary_enabled: true,
        real_runtime_sysvar_population_deferred: true,
        account_info_parser_implemented: false,
        load_instruction_deferred: true,
        reads_concrete_instruction_content: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        allowed_result_count: 3,
        safety_flags: PHASE_41C2_SAFETY_FLAGS,
    };

pub fn bind_current_instruction_identity_boundary(
    descriptor: Phase41C2CurrentInstructionIdentityDescriptor,
) -> Phase41C2CurrentInstructionIdentityResult {
    let status = if !descriptor.current_instruction_identity_supplied {
        Phase41C2CurrentInstructionIdentityStatus::MissingCurrentInstructionIdentity
    } else if !descriptor.program_id_matches_expected_program
        || !descriptor.discriminator_matches_expected_instruction
        || !descriptor.payload_binding_matches_expected_context
    {
        Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
    } else {
        Phase41C2CurrentInstructionIdentityStatus::CurrentInstructionIdentityBound
    };

    let rejection_case = match status {
        Phase41C2CurrentInstructionIdentityStatus::MissingCurrentInstructionIdentity
        | Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity => {
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        }
        Phase41C2CurrentInstructionIdentityStatus::CurrentInstructionIdentityBound => None,
    };

    Phase41C2CurrentInstructionIdentityResult {
        status,
        rejection_case,
        current_instruction_identity_boundary_enabled: true,
        current_instruction_identity_derived_from_runtime: false,
        reads_concrete_instruction_content: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
    }
}

pub fn phase_41c2_current_instruction_identity_boundary_report(
) -> Phase41C2CurrentInstructionIdentityBoundaryReport {
    PHASE_41C2_CURRENT_INSTRUCTION_IDENTITY_BOUNDARY_REPORT
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_descriptor() -> Phase41C2CurrentInstructionIdentityDescriptor {
        Phase41C2CurrentInstructionIdentityDescriptor {
            current_instruction_identity_supplied: true,
            program_id_matches_expected_program: true,
            discriminator_matches_expected_instruction: true,
            payload_binding_matches_expected_context: true,
        }
    }

    #[test]
    fn missing_current_instruction_identity_maps_to_phase_41b_rejection_case() {
        let result = bind_current_instruction_identity_boundary(
            Phase41C2CurrentInstructionIdentityDescriptor {
                current_instruction_identity_supplied: false,
                program_id_matches_expected_program: false,
                discriminator_matches_expected_instruction: false,
                payload_binding_matches_expected_context: false,
            },
        );

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::MissingCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
        assert!(result.current_instruction_identity_boundary_enabled);
        assert!(!result.current_instruction_identity_derived_from_runtime);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn inconsistent_program_id_maps_to_missing_current_identity_rejection_case() {
        let mut descriptor = valid_descriptor();
        descriptor.program_id_matches_expected_program = false;

        let result = bind_current_instruction_identity_boundary(descriptor);

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
    }

    #[test]
    fn inconsistent_discriminator_maps_to_missing_current_identity_rejection_case() {
        let mut descriptor = valid_descriptor();
        descriptor.discriminator_matches_expected_instruction = false;

        let result = bind_current_instruction_identity_boundary(descriptor);

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
    }

    #[test]
    fn inconsistent_payload_binding_maps_to_missing_current_identity_rejection_case() {
        let mut descriptor = valid_descriptor();
        descriptor.payload_binding_matches_expected_context = false;

        let result = bind_current_instruction_identity_boundary(descriptor);

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
    }

    #[test]
    fn valid_current_instruction_identity_is_non_authorizing() {
        let result = bind_current_instruction_identity_boundary(valid_descriptor());

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::CurrentInstructionIdentityBound
        );
        assert_eq!(result.rejection_case, None);
        assert!(result.current_instruction_identity_boundary_enabled);
        assert!(!result.current_instruction_identity_derived_from_runtime);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn phase_41c2_declares_only_three_structural_results() {
        assert_eq!(PHASE_41C2_ALLOWED_IDENTITY_STATUSES.len(), 3);
        assert_eq!(
            PHASE_41C2_ALLOWED_IDENTITY_STATUSES,
            [
                Phase41C2CurrentInstructionIdentityStatus::MissingCurrentInstructionIdentity,
                Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity,
                Phase41C2CurrentInstructionIdentityStatus::CurrentInstructionIdentityBound,
            ]
        );
    }

    #[test]
    fn phase_41c2_safety_flags_keep_real_runtime_read_deferred() {
        let flags = PHASE_41C2_SAFETY_FLAGS;

        assert!(!flags.raw_instructions_sysvar_parser_implemented);
        assert!(!flags.account_info_parser_implemented);
        assert!(!flags.load_instruction_called);
        assert!(!flags.load_instruction_enabled);
        assert!(flags.concrete_runtime_api_selected);
        assert!(!flags.current_instruction_identity_derived_from_runtime);
        assert!(!flags.ed25519_signature_verification_performed);
        assert!(!flags.cryptographic_signature_proof_accepted);
        assert!(!flags.verification_evidence_accepted);
        assert!(!flags.quorum_counting_enabled);
        assert!(!flags.authorization_enabled);
        assert!(!flags.replay_write_enabled);
        assert!(!flags.processed_event_marking_enabled);
        assert!(!flags.account_mutation_enabled);
        assert!(!flags.cpi_enabled);
        assert!(!flags.invoke_signed_enabled);
        assert!(!flags.spl_token_mint_to_enabled);
        assert!(!flags.process_instruction_handler_added);
        assert!(!flags.live_route_enabled);
    }

    #[test]
    fn phase_41c2_report_preserves_boundary() {
        let report = phase_41c2_current_instruction_identity_boundary_report();

        assert_eq!(report.phase, "41C2");
        assert_eq!(report.version, "0.1.0");
        assert!(report.model_or_boundary_only);
        assert!(report.current_instruction_identity_boundary_enabled);
        assert!(report.real_runtime_sysvar_population_deferred);
        assert!(!report.account_info_parser_implemented);
        assert!(report.load_instruction_deferred);
        assert!(!report.reads_concrete_instruction_content);
        assert!(!report.locates_prior_ed25519_instruction);
        assert!(!report.accepts_verification_evidence);
        assert!(!report.authorizes_execution);
        assert_eq!(report.allowed_result_count, 3);

        assert!(report.safety_flags.concrete_runtime_api_selected);
        assert!(
            !report
                .safety_flags
                .current_instruction_identity_derived_from_runtime
        );
        assert!(!report.safety_flags.load_instruction_called);
        assert!(!report.safety_flags.load_instruction_enabled);
        assert!(!report.safety_flags.verification_evidence_accepted);
        assert!(!report.safety_flags.quorum_counting_enabled);
        assert!(!report.safety_flags.authorization_enabled);
        assert!(!report.safety_flags.spl_token_mint_to_enabled);
        assert!(!report.safety_flags.live_route_enabled);
    }

    #[test]
    fn phase_41c2_does_not_cross_into_phase_41c3_or_authorization_scope() {
        let result = bind_current_instruction_identity_boundary(valid_descriptor());

        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }
}

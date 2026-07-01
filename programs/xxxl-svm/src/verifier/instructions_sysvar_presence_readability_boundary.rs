use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41C1_INSTRUCTIONS_SYSVAR_PRESENCE_READABILITY_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41C1_INSTRUCTIONS_SYSVAR_PRESENCE_READABILITY_BOUNDARY_PHASE: &str = "41C1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41C1RuntimeApiSelection {
    SvmInstructionsSysvarContainerPresenceReadabilityBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41C1InstructionsSysvarReadStatus {
    MissingInstructionsSysvar,
    UnreadableInstructionsSysvar,
    PresentAndReadable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C1InstructionsSysvarContainerView {
    pub instructions_sysvar_supplied: bool,
    pub instructions_sysvar_readable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C1InstructionsSysvarPresenceReadabilityResult {
    pub status: Phase41C1InstructionsSysvarReadStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub selected_runtime_api: Phase41C1RuntimeApiSelection,
    pub concrete_runtime_api_selected: bool,
    pub reads_concrete_instruction_content: bool,
    pub derives_current_instruction_identity: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41C1InstructionsSysvarPresenceReadabilityBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub selected_runtime_api: Phase41C1RuntimeApiSelection,
    pub concrete_runtime_api_selected: bool,
    pub model_or_boundary_only: bool,
    pub allowed_result_count: usize,
    pub load_instruction_deferred: bool,
    pub account_info_parser_implemented: bool,
    pub reads_concrete_instruction_content: bool,
    pub derives_current_instruction_identity: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41C1_SELECTED_RUNTIME_API: Phase41C1RuntimeApiSelection =
    Phase41C1RuntimeApiSelection::SvmInstructionsSysvarContainerPresenceReadabilityBoundary;

pub const PHASE_41C1_ALLOWED_READ_STATUSES: [Phase41C1InstructionsSysvarReadStatus; 3] = [
    Phase41C1InstructionsSysvarReadStatus::MissingInstructionsSysvar,
    Phase41C1InstructionsSysvarReadStatus::UnreadableInstructionsSysvar,
    Phase41C1InstructionsSysvarReadStatus::PresentAndReadable,
];

pub const PHASE_41C1_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
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

pub const PHASE_41C1_INSTRUCTIONS_SYSVAR_PRESENCE_READABILITY_BOUNDARY_REPORT:
    Phase41C1InstructionsSysvarPresenceReadabilityBoundaryReport =
    Phase41C1InstructionsSysvarPresenceReadabilityBoundaryReport {
        phase: PHASE_41C1_INSTRUCTIONS_SYSVAR_PRESENCE_READABILITY_BOUNDARY_PHASE,
        version: PHASE_41C1_INSTRUCTIONS_SYSVAR_PRESENCE_READABILITY_BOUNDARY_VERSION,
        selected_runtime_api: PHASE_41C1_SELECTED_RUNTIME_API,
        concrete_runtime_api_selected: true,
        model_or_boundary_only: true,
        allowed_result_count: 3,
        load_instruction_deferred: true,
        account_info_parser_implemented: false,
        reads_concrete_instruction_content: false,
        derives_current_instruction_identity: false,
        locates_prior_ed25519_instruction: false,
        safety_flags: PHASE_41C1_SAFETY_FLAGS,
    };

pub fn check_instructions_sysvar_presence_readability(
    container: Phase41C1InstructionsSysvarContainerView,
) -> Phase41C1InstructionsSysvarPresenceReadabilityResult {
    let (status, rejection_case) = if !container.instructions_sysvar_supplied {
        (
            Phase41C1InstructionsSysvarReadStatus::MissingInstructionsSysvar,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar),
        )
    } else if !container.instructions_sysvar_readable {
        (
            Phase41C1InstructionsSysvarReadStatus::UnreadableInstructionsSysvar,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
        )
    } else {
        (
            Phase41C1InstructionsSysvarReadStatus::PresentAndReadable,
            None,
        )
    };

    Phase41C1InstructionsSysvarPresenceReadabilityResult {
        status,
        rejection_case,
        selected_runtime_api: PHASE_41C1_SELECTED_RUNTIME_API,
        concrete_runtime_api_selected: true,
        reads_concrete_instruction_content: false,
        derives_current_instruction_identity: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
    }
}

pub fn phase_41c1_instructions_sysvar_presence_readability_boundary_report(
) -> Phase41C1InstructionsSysvarPresenceReadabilityBoundaryReport {
    PHASE_41C1_INSTRUCTIONS_SYSVAR_PRESENCE_READABILITY_BOUNDARY_REPORT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_instructions_sysvar_maps_to_phase_41b_rejection_case() {
        let result = check_instructions_sysvar_presence_readability(
            Phase41C1InstructionsSysvarContainerView {
                instructions_sysvar_supplied: false,
                instructions_sysvar_readable: false,
            },
        );

        assert_eq!(
            result.status,
            Phase41C1InstructionsSysvarReadStatus::MissingInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar)
        );
        assert!(result.concrete_runtime_api_selected);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.derives_current_instruction_identity);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn unreadable_instructions_sysvar_maps_to_phase_41b_rejection_case() {
        let result = check_instructions_sysvar_presence_readability(
            Phase41C1InstructionsSysvarContainerView {
                instructions_sysvar_supplied: true,
                instructions_sysvar_readable: false,
            },
        );

        assert_eq!(
            result.status,
            Phase41C1InstructionsSysvarReadStatus::UnreadableInstructionsSysvar
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar)
        );
        assert!(result.concrete_runtime_api_selected);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.derives_current_instruction_identity);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn present_and_readable_instructions_sysvar_has_no_rejection_case() {
        let result = check_instructions_sysvar_presence_readability(
            Phase41C1InstructionsSysvarContainerView {
                instructions_sysvar_supplied: true,
                instructions_sysvar_readable: true,
            },
        );

        assert_eq!(
            result.status,
            Phase41C1InstructionsSysvarReadStatus::PresentAndReadable
        );
        assert_eq!(result.rejection_case, None);
        assert!(result.concrete_runtime_api_selected);
        assert!(!result.reads_concrete_instruction_content);
        assert!(!result.derives_current_instruction_identity);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
    }

    #[test]
    fn phase_41c1_declares_only_three_structural_results() {
        assert_eq!(PHASE_41C1_ALLOWED_READ_STATUSES.len(), 3);
        assert_eq!(
            PHASE_41C1_ALLOWED_READ_STATUSES,
            [
                Phase41C1InstructionsSysvarReadStatus::MissingInstructionsSysvar,
                Phase41C1InstructionsSysvarReadStatus::UnreadableInstructionsSysvar,
                Phase41C1InstructionsSysvarReadStatus::PresentAndReadable,
            ]
        );
    }

    #[test]
    fn phase_41c1_sets_only_concrete_runtime_api_selected_true() {
        let flags = PHASE_41C1_SAFETY_FLAGS;

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
    fn phase_41c1_report_preserves_boundary() {
        let report = phase_41c1_instructions_sysvar_presence_readability_boundary_report();

        assert_eq!(report.phase, "41C1");
        assert_eq!(report.version, "0.1.0");
        assert_eq!(
            report.selected_runtime_api,
            Phase41C1RuntimeApiSelection::SvmInstructionsSysvarContainerPresenceReadabilityBoundary
        );
        assert!(report.concrete_runtime_api_selected);
        assert!(report.model_or_boundary_only);
        assert_eq!(report.allowed_result_count, 3);
        assert!(report.load_instruction_deferred);
        assert!(!report.account_info_parser_implemented);
        assert!(!report.reads_concrete_instruction_content);
        assert!(!report.derives_current_instruction_identity);
        assert!(!report.locates_prior_ed25519_instruction);

        assert!(report.safety_flags.concrete_runtime_api_selected);
        assert!(
            !report
                .safety_flags
                .raw_instructions_sysvar_parser_implemented
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
    fn phase_41c1_does_not_cross_into_phase_41c2_or_41c3_scope() {
        let present = check_instructions_sysvar_presence_readability(
            Phase41C1InstructionsSysvarContainerView {
                instructions_sysvar_supplied: true,
                instructions_sysvar_readable: true,
            },
        );

        assert!(!present.reads_concrete_instruction_content);
        assert!(!present.derives_current_instruction_identity);
        assert!(!present.locates_prior_ed25519_instruction);
        assert!(!present.accepts_verification_evidence);
    }
}

use super::ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B;

pub const ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D: &str =
    "ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D";
pub const ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FutureEd25519VerificationEvidenceRequirement {
    PriorEd25519Instruction,
    InstructionsSysvarRuntimeRead,
    CurrentInstructionIdentity,
    Ed25519ProgramIdMatch,
    SupportedOffsetLayout,
    Phase37LayoutConstraints,
    Phase38InstructionDataParsing,
    Phase34PayloadHashMatch,
    GuardianPublicKeyInActiveSet,
    GuardianSetIdMatch,
    RouteBinding,
    TargetMintBinding,
    RecipientBinding,
    AmountBinding,
    ExpirationOrFinalityBinding,
    DeterministicFailureReason,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FutureEd25519VerificationEvidenceRejectionCase {
    MissingInstructionsSysvar,
    UnreadableInstructionsSysvar,
    MissingCurrentInstructionIdentity,
    Ed25519InstructionNotFound,
    Ed25519InstructionAfterCurrentInstruction,
    WrongEd25519ProgramId,
    MalformedEd25519InstructionData,
    UnsupportedOffsetLayout,
    PublicKeyMismatch,
    MessageHashMismatch,
    GuardianSetMismatch,
    DuplicateGuardianEvidence,
    AmbiguousCandidateEvidence,
    ExpiredEvidence,
    WrongRoute,
    WrongTargetMint,
    WrongRecipient,
    WrongAmount,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ed25519VerificationEvidenceIntegrationDesignReport {
    pub design_id: &'static str,
    pub design_version: u8,
    pub phase_40b_model_required: bool,
    pub phase_40b_model_phase: &'static str,
    pub future_prior_ed25519_instruction_required: bool,
    pub future_instructions_sysvar_runtime_read_required: bool,
    pub future_current_instruction_identity_required: bool,
    pub future_ed25519_program_id_binding_required: bool,
    pub future_phase_37_layout_constraints_required: bool,
    pub future_phase_38_parser_constraints_required: bool,
    pub future_phase_34_payload_hash_binding_required: bool,
    pub future_guardian_public_key_binding_required: bool,
    pub future_guardian_set_binding_required: bool,
    pub future_route_binding_required: bool,
    pub future_target_mint_binding_required: bool,
    pub future_recipient_binding_required: bool,
    pub future_amount_binding_required: bool,
    pub future_expiration_or_finality_binding_required: bool,
    pub deterministic_error_surface_required: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub account_info_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub ed25519_signature_verification_performed: bool,
    pub cryptographic_signature_proof_accepted: bool,
    pub verification_evidence_accepted: bool,
    pub quorum_counting_enabled: bool,
    pub authorization_enabled: bool,
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

pub const FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS:
    [FutureEd25519VerificationEvidenceRequirement; 16] = [
    FutureEd25519VerificationEvidenceRequirement::PriorEd25519Instruction,
    FutureEd25519VerificationEvidenceRequirement::InstructionsSysvarRuntimeRead,
    FutureEd25519VerificationEvidenceRequirement::CurrentInstructionIdentity,
    FutureEd25519VerificationEvidenceRequirement::Ed25519ProgramIdMatch,
    FutureEd25519VerificationEvidenceRequirement::SupportedOffsetLayout,
    FutureEd25519VerificationEvidenceRequirement::Phase37LayoutConstraints,
    FutureEd25519VerificationEvidenceRequirement::Phase38InstructionDataParsing,
    FutureEd25519VerificationEvidenceRequirement::Phase34PayloadHashMatch,
    FutureEd25519VerificationEvidenceRequirement::GuardianPublicKeyInActiveSet,
    FutureEd25519VerificationEvidenceRequirement::GuardianSetIdMatch,
    FutureEd25519VerificationEvidenceRequirement::RouteBinding,
    FutureEd25519VerificationEvidenceRequirement::TargetMintBinding,
    FutureEd25519VerificationEvidenceRequirement::RecipientBinding,
    FutureEd25519VerificationEvidenceRequirement::AmountBinding,
    FutureEd25519VerificationEvidenceRequirement::ExpirationOrFinalityBinding,
    FutureEd25519VerificationEvidenceRequirement::DeterministicFailureReason,
];

pub const FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES:
    [FutureEd25519VerificationEvidenceRejectionCase; 18] = [
    FutureEd25519VerificationEvidenceRejectionCase::MissingInstructionsSysvar,
    FutureEd25519VerificationEvidenceRejectionCase::UnreadableInstructionsSysvar,
    FutureEd25519VerificationEvidenceRejectionCase::MissingCurrentInstructionIdentity,
    FutureEd25519VerificationEvidenceRejectionCase::Ed25519InstructionNotFound,
    FutureEd25519VerificationEvidenceRejectionCase::Ed25519InstructionAfterCurrentInstruction,
    FutureEd25519VerificationEvidenceRejectionCase::WrongEd25519ProgramId,
    FutureEd25519VerificationEvidenceRejectionCase::MalformedEd25519InstructionData,
    FutureEd25519VerificationEvidenceRejectionCase::UnsupportedOffsetLayout,
    FutureEd25519VerificationEvidenceRejectionCase::PublicKeyMismatch,
    FutureEd25519VerificationEvidenceRejectionCase::MessageHashMismatch,
    FutureEd25519VerificationEvidenceRejectionCase::GuardianSetMismatch,
    FutureEd25519VerificationEvidenceRejectionCase::DuplicateGuardianEvidence,
    FutureEd25519VerificationEvidenceRejectionCase::AmbiguousCandidateEvidence,
    FutureEd25519VerificationEvidenceRejectionCase::ExpiredEvidence,
    FutureEd25519VerificationEvidenceRejectionCase::WrongRoute,
    FutureEd25519VerificationEvidenceRejectionCase::WrongTargetMint,
    FutureEd25519VerificationEvidenceRejectionCase::WrongRecipient,
    FutureEd25519VerificationEvidenceRejectionCase::WrongAmount,
];

pub const ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_REPORT:
    Ed25519VerificationEvidenceIntegrationDesignReport =
    Ed25519VerificationEvidenceIntegrationDesignReport {
        design_id: ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D,
        design_version: ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_VERSION,
        phase_40b_model_required: true,
        phase_40b_model_phase: ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B,
        future_prior_ed25519_instruction_required: true,
        future_instructions_sysvar_runtime_read_required: true,
        future_current_instruction_identity_required: true,
        future_ed25519_program_id_binding_required: true,
        future_phase_37_layout_constraints_required: true,
        future_phase_38_parser_constraints_required: true,
        future_phase_34_payload_hash_binding_required: true,
        future_guardian_public_key_binding_required: true,
        future_guardian_set_binding_required: true,
        future_route_binding_required: true,
        future_target_mint_binding_required: true,
        future_recipient_binding_required: true,
        future_amount_binding_required: true,
        future_expiration_or_finality_binding_required: true,
        deterministic_error_surface_required: true,
        raw_instructions_sysvar_parser_implemented: false,
        account_info_parser_implemented: false,
        load_instruction_called: false,
        ed25519_signature_verification_performed: false,
        cryptographic_signature_proof_accepted: false,
        verification_evidence_accepted: false,
        quorum_counting_enabled: false,
        authorization_enabled: false,
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

pub fn ed25519_verification_evidence_integration_design_report(
) -> &'static Ed25519VerificationEvidenceIntegrationDesignReport {
    &ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_REPORT
}

pub fn future_ed25519_verification_evidence_requirements(
) -> &'static [FutureEd25519VerificationEvidenceRequirement] {
    &FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS
}

pub fn future_ed25519_verification_evidence_rejection_cases(
) -> &'static [FutureEd25519VerificationEvidenceRejectionCase] {
    &FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        ed25519_verification_evidence_model_report, read_only_verifier_boundary,
        ED25519_VERIFICATION_EVIDENCE_MODEL_REPORT, READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
    };

    #[test]
    fn marker_and_report_are_stable() {
        let report = ed25519_verification_evidence_integration_design_report();

        assert_eq!(
            report.design_id,
            ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D
        );
        assert_eq!(
            report.design_version,
            ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_VERSION
        );
        assert!(report.phase_40b_model_required);
        assert_eq!(
            report.phase_40b_model_phase,
            ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B
        );
    }

    #[test]
    fn future_requirements_are_explicit_and_ordered() {
        let requirements = future_ed25519_verification_evidence_requirements();

        assert_eq!(requirements.len(), 16);
        assert_eq!(
            requirements[0],
            FutureEd25519VerificationEvidenceRequirement::PriorEd25519Instruction
        );
        assert_eq!(
            requirements[1],
            FutureEd25519VerificationEvidenceRequirement::InstructionsSysvarRuntimeRead
        );
        assert_eq!(
            requirements[7],
            FutureEd25519VerificationEvidenceRequirement::Phase34PayloadHashMatch
        );
        assert_eq!(
            requirements[15],
            FutureEd25519VerificationEvidenceRequirement::DeterministicFailureReason
        );
    }

    #[test]
    fn future_rejection_cases_are_explicit_and_ordered() {
        let rejection_cases = future_ed25519_verification_evidence_rejection_cases();

        assert_eq!(rejection_cases.len(), 18);
        assert_eq!(
            rejection_cases[0],
            FutureEd25519VerificationEvidenceRejectionCase::MissingInstructionsSysvar
        );
        assert_eq!(
            rejection_cases[4],
            FutureEd25519VerificationEvidenceRejectionCase::Ed25519InstructionAfterCurrentInstruction
        );
        assert_eq!(
            rejection_cases[8],
            FutureEd25519VerificationEvidenceRejectionCase::PublicKeyMismatch
        );
        assert_eq!(
            rejection_cases[17],
            FutureEd25519VerificationEvidenceRejectionCase::WrongAmount
        );
    }

    #[test]
    fn design_requires_future_runtime_bindings_but_implements_none() {
        let report = ed25519_verification_evidence_integration_design_report();

        assert!(report.future_prior_ed25519_instruction_required);
        assert!(report.future_instructions_sysvar_runtime_read_required);
        assert!(report.future_current_instruction_identity_required);
        assert!(report.future_ed25519_program_id_binding_required);
        assert!(report.future_phase_37_layout_constraints_required);
        assert!(report.future_phase_38_parser_constraints_required);
        assert!(report.future_phase_34_payload_hash_binding_required);
        assert!(report.future_guardian_public_key_binding_required);
        assert!(report.future_guardian_set_binding_required);
        assert!(report.future_route_binding_required);
        assert!(report.future_target_mint_binding_required);
        assert!(report.future_recipient_binding_required);
        assert!(report.future_amount_binding_required);
        assert!(report.future_expiration_or_finality_binding_required);
        assert!(report.deterministic_error_surface_required);

        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.account_info_parser_implemented);
        assert!(!report.load_instruction_called);
    }

    #[test]
    fn design_accepts_no_signature_proof_or_verification_evidence() {
        let report = ed25519_verification_evidence_integration_design_report();

        assert!(!report.ed25519_signature_verification_performed);
        assert!(!report.cryptographic_signature_proof_accepted);
        assert!(!report.verification_evidence_accepted);
    }

    #[test]
    fn design_preserves_quorum_authorization_and_execution_boundaries() {
        let report = ed25519_verification_evidence_integration_design_report();

        assert!(!report.quorum_counting_enabled);
        assert!(!report.authorization_enabled);
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
    fn phase_40b_model_remains_required_and_non_authorizing() {
        let report = ed25519_verification_evidence_integration_design_report();
        let phase_40b_report = ed25519_verification_evidence_model_report();

        assert!(report.phase_40b_model_required);
        assert_eq!(
            report.phase_40b_model_phase,
            ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B
        );
        assert_eq!(
            phase_40b_report.model_id,
            ED25519_VERIFICATION_EVIDENCE_MODEL_REPORT.model_id
        );
        assert!(!phase_40b_report.verification_evidence_acceptance_enabled);
        assert!(!phase_40b_report.ed25519_signature_verification_enabled);
        assert!(!phase_40b_report.cryptographic_signature_proof_accepted);
        assert!(!phase_40b_report.quorum_counting_enabled);
        assert!(!phase_40b_report.authorization_enabled);
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
}

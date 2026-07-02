use solana_program::pubkey::Pubkey;

use super::current_instruction_identity_boundary::{
    bind_current_instruction_identity_boundary, Phase41C2CurrentInstructionIdentityDescriptor,
    Phase41C2CurrentInstructionIdentityStatus,
};
use super::instructions_sysvar_access_contract_model::{
    Phase41BRejectionCase, Phase41BSafetyFlags,
};

pub const PHASE_41D2_CURRENT_INSTRUCTION_IDENTITY_RUNTIME_BOUNDARY_VERSION: &str = "0.1.0";
pub const PHASE_41D2_CURRENT_INSTRUCTION_IDENTITY_RUNTIME_BOUNDARY_PHASE: &str = "41D2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41D2RuntimeIdentitySource {
    EntrypointProgramIdAndInstructionData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D2CurrentInstructionIdentityRuntimeContext<'a> {
    pub entrypoint_program_id: Option<&'a Pubkey>,
    pub entrypoint_instruction_data: Option<&'a [u8]>,
    pub expected_program_id: &'a Pubkey,
    pub expected_instruction_discriminator: &'a [u8],
    pub payload_binding_matches_expected_context: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D2CurrentInstructionIdentityRuntimeResult {
    pub status: Phase41C2CurrentInstructionIdentityStatus,
    pub rejection_case: Option<Phase41BRejectionCase>,
    pub descriptor: Phase41C2CurrentInstructionIdentityDescriptor,
    pub runtime_identity_source: Phase41D2RuntimeIdentitySource,
    pub current_instruction_identity_derived_from_runtime: bool,
    pub uses_entrypoint_program_id: bool,
    pub uses_entrypoint_instruction_data: bool,
    pub reads_instruction_data_discriminator_prefix: bool,
    pub uses_payload_binding_result: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41D2CurrentInstructionIdentityRuntimeBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub runtime_identity_source: Phase41D2RuntimeIdentitySource,
    pub maps_to_phase_41c2_descriptor_boundary: bool,
    pub account_info_parser_implemented: bool,
    pub raw_instructions_sysvar_parser_implemented: bool,
    pub load_instruction_called: bool,
    pub load_instruction_enabled: bool,
    pub current_instruction_identity_derived_from_runtime: bool,
    pub locates_prior_ed25519_instruction: bool,
    pub accepts_verification_evidence: bool,
    pub authorizes_execution: bool,
    pub safety_flags: Phase41BSafetyFlags,
}

pub const PHASE_41D2_SAFETY_FLAGS: Phase41BSafetyFlags = Phase41BSafetyFlags {
    raw_instructions_sysvar_parser_implemented: false,
    account_info_parser_implemented: true,
    load_instruction_called: false,
    load_instruction_enabled: false,
    concrete_runtime_api_selected: true,
    current_instruction_identity_derived_from_runtime: true,
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

pub const PHASE_41D2_CURRENT_INSTRUCTION_IDENTITY_RUNTIME_BOUNDARY_REPORT:
    Phase41D2CurrentInstructionIdentityRuntimeBoundaryReport =
    Phase41D2CurrentInstructionIdentityRuntimeBoundaryReport {
        phase: PHASE_41D2_CURRENT_INSTRUCTION_IDENTITY_RUNTIME_BOUNDARY_PHASE,
        version: PHASE_41D2_CURRENT_INSTRUCTION_IDENTITY_RUNTIME_BOUNDARY_VERSION,
        runtime_identity_source:
            Phase41D2RuntimeIdentitySource::EntrypointProgramIdAndInstructionData,
        maps_to_phase_41c2_descriptor_boundary: true,
        account_info_parser_implemented: true,
        raw_instructions_sysvar_parser_implemented: false,
        load_instruction_called: false,
        load_instruction_enabled: false,
        current_instruction_identity_derived_from_runtime: true,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
        safety_flags: PHASE_41D2_SAFETY_FLAGS,
    };

pub fn derive_current_instruction_identity_from_entrypoint_context(
    context: Phase41D2CurrentInstructionIdentityRuntimeContext<'_>,
) -> Phase41D2CurrentInstructionIdentityRuntimeResult {
    let descriptor = match (
        context.entrypoint_program_id,
        context.entrypoint_instruction_data,
    ) {
        (Some(entrypoint_program_id), Some(entrypoint_instruction_data))
            if !context.expected_instruction_discriminator.is_empty() =>
        {
            let discriminator_matches_expected_instruction = match entrypoint_instruction_data
                .get(0..context.expected_instruction_discriminator.len())
            {
                Some(actual_discriminator) => {
                    actual_discriminator == context.expected_instruction_discriminator
                }
                None => false,
            };

            Phase41C2CurrentInstructionIdentityDescriptor {
                current_instruction_identity_supplied: true,
                program_id_matches_expected_program: entrypoint_program_id
                    == context.expected_program_id,
                discriminator_matches_expected_instruction,
                payload_binding_matches_expected_context: context
                    .payload_binding_matches_expected_context,
            }
        }
        _ => Phase41C2CurrentInstructionIdentityDescriptor {
            current_instruction_identity_supplied: false,
            program_id_matches_expected_program: false,
            discriminator_matches_expected_instruction: false,
            payload_binding_matches_expected_context: false,
        },
    };

    let boundary_result = bind_current_instruction_identity_boundary(descriptor);

    Phase41D2CurrentInstructionIdentityRuntimeResult {
        status: boundary_result.status,
        rejection_case: boundary_result.rejection_case,
        descriptor,
        runtime_identity_source:
            Phase41D2RuntimeIdentitySource::EntrypointProgramIdAndInstructionData,
        current_instruction_identity_derived_from_runtime: true,
        uses_entrypoint_program_id: context.entrypoint_program_id.is_some(),
        uses_entrypoint_instruction_data: context.entrypoint_instruction_data.is_some(),
        reads_instruction_data_discriminator_prefix: context.entrypoint_instruction_data.is_some()
            && !context.expected_instruction_discriminator.is_empty(),
        uses_payload_binding_result: context.entrypoint_program_id.is_some()
            && context.entrypoint_instruction_data.is_some()
            && !context.expected_instruction_discriminator.is_empty(),
        raw_instructions_sysvar_parser_implemented: false,
        load_instruction_called: false,
        load_instruction_enabled: false,
        locates_prior_ed25519_instruction: false,
        accepts_verification_evidence: false,
        authorizes_execution: false,
    }
}

pub fn phase_41d2_current_instruction_identity_runtime_boundary_report(
) -> Phase41D2CurrentInstructionIdentityRuntimeBoundaryReport {
    PHASE_41D2_CURRENT_INSTRUCTION_IDENTITY_RUNTIME_BOUNDARY_REPORT
}

#[cfg(test)]
mod tests {
    use solana_program::pubkey::Pubkey;

    use super::*;

    const EXPECTED_DISCRIMINATOR: [u8; 4] = [0x58, 0x58, 0x58, 0x4c];

    fn expected_program_id() -> Pubkey {
        Pubkey::new_from_array([1; 32])
    }

    fn wrong_program_id() -> Pubkey {
        Pubkey::new_from_array([2; 32])
    }

    fn valid_context<'a>(
        program_id: &'a Pubkey,
        instruction_data: &'a [u8],
    ) -> Phase41D2CurrentInstructionIdentityRuntimeContext<'a> {
        Phase41D2CurrentInstructionIdentityRuntimeContext {
            entrypoint_program_id: Some(program_id),
            entrypoint_instruction_data: Some(instruction_data),
            expected_program_id: program_id,
            expected_instruction_discriminator: &EXPECTED_DISCRIMINATOR,
            payload_binding_matches_expected_context: true,
        }
    }

    #[test]
    fn missing_program_id_maps_to_missing_current_instruction_identity() {
        let expected = expected_program_id();
        let instruction_data = [0x58, 0x58, 0x58, 0x4c];

        let result = derive_current_instruction_identity_from_entrypoint_context(
            Phase41D2CurrentInstructionIdentityRuntimeContext {
                entrypoint_program_id: None,
                entrypoint_instruction_data: Some(&instruction_data),
                expected_program_id: &expected,
                expected_instruction_discriminator: &EXPECTED_DISCRIMINATOR,
                payload_binding_matches_expected_context: true,
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
        assert!(!result.descriptor.current_instruction_identity_supplied);
        assert!(result.current_instruction_identity_derived_from_runtime);
        assert!(!result.load_instruction_called);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn missing_instruction_data_maps_to_missing_current_instruction_identity() {
        let expected = expected_program_id();

        let result = derive_current_instruction_identity_from_entrypoint_context(
            Phase41D2CurrentInstructionIdentityRuntimeContext {
                entrypoint_program_id: Some(&expected),
                entrypoint_instruction_data: None,
                expected_program_id: &expected,
                expected_instruction_discriminator: &EXPECTED_DISCRIMINATOR,
                payload_binding_matches_expected_context: true,
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
        assert!(!result.descriptor.current_instruction_identity_supplied);
        assert!(result.current_instruction_identity_derived_from_runtime);
        assert!(!result.load_instruction_called);
    }

    #[test]
    fn empty_expected_discriminator_maps_to_missing_current_instruction_identity() {
        let expected = expected_program_id();
        let instruction_data = [0x58, 0x58, 0x58, 0x4c];

        let result = derive_current_instruction_identity_from_entrypoint_context(
            Phase41D2CurrentInstructionIdentityRuntimeContext {
                entrypoint_program_id: Some(&expected),
                entrypoint_instruction_data: Some(&instruction_data),
                expected_program_id: &expected,
                expected_instruction_discriminator: &[],
                payload_binding_matches_expected_context: true,
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
        assert!(!result.descriptor.current_instruction_identity_supplied);
        assert!(!result.reads_instruction_data_discriminator_prefix);
    }

    #[test]
    fn wrong_program_id_maps_to_inconsistent_current_instruction_identity() {
        let expected = expected_program_id();
        let wrong = wrong_program_id();
        let instruction_data = [0x58, 0x58, 0x58, 0x4c, 0x01];

        let result = derive_current_instruction_identity_from_entrypoint_context(
            Phase41D2CurrentInstructionIdentityRuntimeContext {
                entrypoint_program_id: Some(&wrong),
                entrypoint_instruction_data: Some(&instruction_data),
                expected_program_id: &expected,
                expected_instruction_discriminator: &EXPECTED_DISCRIMINATOR,
                payload_binding_matches_expected_context: true,
            },
        );

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
        assert!(result.descriptor.current_instruction_identity_supplied);
        assert!(!result.descriptor.program_id_matches_expected_program);
        assert!(result.descriptor.discriminator_matches_expected_instruction);
    }

    #[test]
    fn short_instruction_data_maps_to_inconsistent_current_instruction_identity() {
        let expected = expected_program_id();
        let instruction_data = [0x58, 0x58];

        let result = derive_current_instruction_identity_from_entrypoint_context(valid_context(
            &expected,
            &instruction_data,
        ));

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
        assert!(result.descriptor.current_instruction_identity_supplied);
        assert!(!result.descriptor.discriminator_matches_expected_instruction);
        assert!(result.reads_instruction_data_discriminator_prefix);
    }

    #[test]
    fn discriminator_mismatch_maps_to_inconsistent_current_instruction_identity() {
        let expected = expected_program_id();
        let instruction_data = [0x58, 0x58, 0x58, 0x00, 0x01];

        let result = derive_current_instruction_identity_from_entrypoint_context(valid_context(
            &expected,
            &instruction_data,
        ));

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
        assert!(result.descriptor.current_instruction_identity_supplied);
        assert!(!result.descriptor.discriminator_matches_expected_instruction);
    }

    #[test]
    fn payload_binding_mismatch_maps_to_inconsistent_current_instruction_identity() {
        let expected = expected_program_id();
        let instruction_data = [0x58, 0x58, 0x58, 0x4c, 0x01];

        let result = derive_current_instruction_identity_from_entrypoint_context(
            Phase41D2CurrentInstructionIdentityRuntimeContext {
                entrypoint_program_id: Some(&expected),
                entrypoint_instruction_data: Some(&instruction_data),
                expected_program_id: &expected,
                expected_instruction_discriminator: &EXPECTED_DISCRIMINATOR,
                payload_binding_matches_expected_context: false,
            },
        );

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::InconsistentCurrentInstructionIdentity
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41BRejectionCase::MissingCurrentInstructionIdentity)
        );
        assert!(result.descriptor.current_instruction_identity_supplied);
        assert!(!result.descriptor.payload_binding_matches_expected_context);
    }

    #[test]
    fn valid_entrypoint_identity_maps_to_current_instruction_identity_bound() {
        let expected = expected_program_id();
        let instruction_data = [0x58, 0x58, 0x58, 0x4c, 0x01];

        let result = derive_current_instruction_identity_from_entrypoint_context(valid_context(
            &expected,
            &instruction_data,
        ));

        assert_eq!(
            result.status,
            Phase41C2CurrentInstructionIdentityStatus::CurrentInstructionIdentityBound
        );
        assert_eq!(result.rejection_case, None);
        assert!(result.descriptor.current_instruction_identity_supplied);
        assert!(result.descriptor.program_id_matches_expected_program);
        assert!(result.descriptor.discriminator_matches_expected_instruction);
        assert!(result.descriptor.payload_binding_matches_expected_context);
        assert!(result.current_instruction_identity_derived_from_runtime);
        assert!(result.uses_entrypoint_program_id);
        assert!(result.uses_entrypoint_instruction_data);
        assert!(result.reads_instruction_data_discriminator_prefix);
        assert!(result.uses_payload_binding_result);
        assert!(!result.raw_instructions_sysvar_parser_implemented);
        assert!(!result.load_instruction_called);
        assert!(!result.load_instruction_enabled);
        assert!(!result.locates_prior_ed25519_instruction);
        assert!(!result.accepts_verification_evidence);
        assert!(!result.authorizes_execution);
    }

    #[test]
    fn phase_41d2_flips_only_current_identity_runtime_flag_after_41d1() {
        let flags = PHASE_41D2_SAFETY_FLAGS;

        assert!(!flags.raw_instructions_sysvar_parser_implemented);
        assert!(flags.account_info_parser_implemented);
        assert!(!flags.load_instruction_called);
        assert!(!flags.load_instruction_enabled);
        assert!(flags.concrete_runtime_api_selected);
        assert!(flags.current_instruction_identity_derived_from_runtime);
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
    fn phase_41d2_report_preserves_runtime_identity_boundary() {
        let report = phase_41d2_current_instruction_identity_runtime_boundary_report();

        assert_eq!(report.phase, "41D2");
        assert_eq!(report.version, "0.1.0");
        assert_eq!(
            report.runtime_identity_source,
            Phase41D2RuntimeIdentitySource::EntrypointProgramIdAndInstructionData
        );
        assert!(report.maps_to_phase_41c2_descriptor_boundary);
        assert!(report.account_info_parser_implemented);
        assert!(!report.raw_instructions_sysvar_parser_implemented);
        assert!(!report.load_instruction_called);
        assert!(!report.load_instruction_enabled);
        assert!(report.current_instruction_identity_derived_from_runtime);
        assert!(!report.locates_prior_ed25519_instruction);
        assert!(!report.accepts_verification_evidence);
        assert!(!report.authorizes_execution);
    }
}

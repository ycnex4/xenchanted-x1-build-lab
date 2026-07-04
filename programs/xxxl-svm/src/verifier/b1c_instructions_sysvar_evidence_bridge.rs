use solana_program::account_info::AccountInfo;

use super::{
    establish_phase_41k_1_instructions_sysvar_live_wiring_boundary,
    B1CEd25519EvidenceAuthorizationRejectionKind, Phase41K1InstructionsSysvarLiveWiringResult,
    Phase41K1InstructionsSysvarLiveWiringStatus,
};

pub const PHASE_41K6_B1C_2_INSTRUCTIONS_SYSVAR_EVIDENCE_PHASE: &str = "41K.6-B1C.2";
pub const PHASE_41K6_B1C_2_INSTRUCTIONS_SYSVAR_EVIDENCE_VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B1CInstructionsSysvarEvidenceStatus {
    PriorEd25519EvidenceLoaded,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CLoadedPriorEd25519EvidenceDescriptor {
    pub instruction_index: usize,
    pub instruction_data_len: usize,
    pub strictly_prior_to_current_instruction: bool,
    pub program_id_is_ed25519: bool,
    pub loaded_from_real_instructions_sysvar: bool,
    pub loaded_with_checked_runtime_api: bool,
    pub accepts_caller_provided_instruction_bytes: bool,
    pub accepts_frontend_or_watcher_ed25519_proof: bool,
    pub parses_ed25519_instruction_data: bool,
    pub binds_payload_hash: bool,
    pub counts_unique_guardians: bool,
    pub authorizes_execution: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CInstructionsSysvarEvidenceLoaded {
    pub status: B1CInstructionsSysvarEvidenceStatus,
    pub current_instruction_index: usize,
    pub loaded_prior_instruction_count: usize,
    pub inspected_prior_instruction_count: usize,
    pub discarded_non_ed25519_prior_instruction_count: usize,
    pub prior_ed25519_precompile_count: usize,
    pub prior_ed25519_precompile_instructions: Vec<B1CLoadedPriorEd25519EvidenceDescriptor>,
    pub uses_real_instructions_sysvar_account_info: bool,
    pub checks_instructions_sysvar_account_id: bool,
    pub load_current_index_checked_used: bool,
    pub load_instruction_at_checked_used: bool,
    pub accepts_caller_provided_instruction_bytes: bool,
    pub accepts_frontend_or_watcher_ed25519_proof: bool,
    pub parses_ed25519_instruction_data: bool,
    pub binds_payload_hash: bool,
    pub counts_unique_guardians: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct B1CInstructionsSysvarEvidenceRejected {
    pub status: B1CInstructionsSysvarEvidenceStatus,
    pub rejection_kind: B1CEd25519EvidenceAuthorizationRejectionKind,
    pub source_status: Phase41K1InstructionsSysvarLiveWiringStatus,
    pub current_instruction_index: Option<usize>,
    pub loaded_prior_instruction_count: usize,
    pub inspected_prior_instruction_count: usize,
    pub discarded_non_ed25519_prior_instruction_count: usize,
    pub prior_ed25519_precompile_count: usize,
    pub uses_real_instructions_sysvar_account_info: bool,
    pub checks_instructions_sysvar_account_id: bool,
    pub load_current_index_checked_used: bool,
    pub load_instruction_at_checked_used: bool,
    pub accepts_caller_provided_instruction_bytes: bool,
    pub accepts_frontend_or_watcher_ed25519_proof: bool,
    pub parses_ed25519_instruction_data: bool,
    pub binds_payload_hash: bool,
    pub counts_unique_guardians: bool,
    pub authorization_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum B1CInstructionsSysvarEvidenceResult {
    Loaded(B1CInstructionsSysvarEvidenceLoaded),
    Rejected(B1CInstructionsSysvarEvidenceRejected),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct B1CInstructionsSysvarEvidenceBridgeReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub consumes_phase_41k1_live_wiring_boundary: bool,
    pub reads_real_instructions_sysvar_account_info: bool,
    pub checks_instructions_sysvar_account_id: bool,
    pub loads_current_instruction_index: bool,
    pub loads_prior_instructions: bool,
    pub filters_prior_ed25519_precompile_instructions: bool,
    pub parses_ed25519_instruction_data: bool,
    pub binds_payload_hash: bool,
    pub counts_unique_guardians: bool,
    pub authorizes_handler_execution: bool,
    pub processed_event_marking_enabled: bool,
    pub cpi_enabled: bool,
    pub live_route_enabled: bool,
}

pub const B1C_INSTRUCTIONS_SYSVAR_EVIDENCE_BRIDGE_REPORT:
    B1CInstructionsSysvarEvidenceBridgeReport = B1CInstructionsSysvarEvidenceBridgeReport {
    phase: PHASE_41K6_B1C_2_INSTRUCTIONS_SYSVAR_EVIDENCE_PHASE,
    version: PHASE_41K6_B1C_2_INSTRUCTIONS_SYSVAR_EVIDENCE_VERSION,
    consumes_phase_41k1_live_wiring_boundary: true,
    reads_real_instructions_sysvar_account_info: true,
    checks_instructions_sysvar_account_id: true,
    loads_current_instruction_index: true,
    loads_prior_instructions: true,
    filters_prior_ed25519_precompile_instructions: true,
    parses_ed25519_instruction_data: false,
    binds_payload_hash: false,
    counts_unique_guardians: false,
    authorizes_handler_execution: false,
    processed_event_marking_enabled: false,
    cpi_enabled: false,
    live_route_enabled: false,
};

pub fn b1c_instructions_sysvar_evidence_bridge_report(
) -> &'static B1CInstructionsSysvarEvidenceBridgeReport {
    &B1C_INSTRUCTIONS_SYSVAR_EVIDENCE_BRIDGE_REPORT
}

pub fn load_b1c_instructions_sysvar_evidence_bridge(
    instructions_sysvar_account: Option<&AccountInfo<'_>>,
) -> B1CInstructionsSysvarEvidenceResult {
    let phase_41k1_result =
        establish_phase_41k_1_instructions_sysvar_live_wiring_boundary(instructions_sysvar_account);

    derive_b1c_instructions_sysvar_evidence_from_41k1(&phase_41k1_result)
}

pub fn derive_b1c_instructions_sysvar_evidence_from_41k1(
    source: &Phase41K1InstructionsSysvarLiveWiringResult,
) -> B1CInstructionsSysvarEvidenceResult {
    if source.status
        != Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded
    {
        return B1CInstructionsSysvarEvidenceResult::Rejected(
            B1CInstructionsSysvarEvidenceRejected {
                status: B1CInstructionsSysvarEvidenceStatus::Rejected,
                rejection_kind: map_rejection_kind(source.status),
                source_status: source.status,
                current_instruction_index: source.current_instruction_index,
                loaded_prior_instruction_count: source.loaded_prior_instruction_count,
                inspected_prior_instruction_count: source.inspected_prior_instruction_count,
                discarded_non_ed25519_prior_instruction_count: source
                    .discarded_non_ed25519_prior_instruction_count,
                prior_ed25519_precompile_count: source.prior_ed25519_precompile_count,
                uses_real_instructions_sysvar_account_info: source
                    .uses_real_instructions_sysvar_account_info,
                checks_instructions_sysvar_account_id: source.checks_instructions_sysvar_account_id,
                load_current_index_checked_used: source.load_current_index_checked_used,
                load_instruction_at_checked_used: source.load_instruction_at_checked_used,
                accepts_caller_provided_instruction_bytes: false,
                accepts_frontend_or_watcher_ed25519_proof: false,
                parses_ed25519_instruction_data: false,
                binds_payload_hash: false,
                counts_unique_guardians: false,
                authorization_enabled: false,
                processed_event_marking_enabled: false,
                cpi_enabled: false,
                live_route_enabled: false,
            },
        );
    }

    let current_instruction_index = source
        .current_instruction_index
        .expect("41K.1 loaded prior Ed25519 evidence must include current index");

    let prior_ed25519_precompile_instructions = source
        .prior_ed25519_precompile_instructions
        .iter()
        .map(|prior| B1CLoadedPriorEd25519EvidenceDescriptor {
            instruction_index: prior.instruction_index,
            instruction_data_len: prior.instruction_data_len,
            strictly_prior_to_current_instruction: prior.strictly_prior_to_current_instruction,
            program_id_is_ed25519: prior.program_id_is_ed25519,
            loaded_from_real_instructions_sysvar: prior.loaded_from_real_instructions_sysvar,
            loaded_with_checked_runtime_api: prior.loaded_with_checked_runtime_api,
            accepts_caller_provided_instruction_bytes: false,
            accepts_frontend_or_watcher_ed25519_proof: false,
            parses_ed25519_instruction_data: false,
            binds_payload_hash: false,
            counts_unique_guardians: false,
            authorizes_execution: false,
        })
        .collect::<Vec<_>>();

    B1CInstructionsSysvarEvidenceResult::Loaded(B1CInstructionsSysvarEvidenceLoaded {
        status: B1CInstructionsSysvarEvidenceStatus::PriorEd25519EvidenceLoaded,
        current_instruction_index,
        loaded_prior_instruction_count: source.loaded_prior_instruction_count,
        inspected_prior_instruction_count: source.inspected_prior_instruction_count,
        discarded_non_ed25519_prior_instruction_count: source
            .discarded_non_ed25519_prior_instruction_count,
        prior_ed25519_precompile_count: source.prior_ed25519_precompile_count,
        prior_ed25519_precompile_instructions,
        uses_real_instructions_sysvar_account_info: source
            .uses_real_instructions_sysvar_account_info,
        checks_instructions_sysvar_account_id: source.checks_instructions_sysvar_account_id,
        load_current_index_checked_used: source.load_current_index_checked_used,
        load_instruction_at_checked_used: source.load_instruction_at_checked_used,
        accepts_caller_provided_instruction_bytes: false,
        accepts_frontend_or_watcher_ed25519_proof: false,
        parses_ed25519_instruction_data: false,
        binds_payload_hash: false,
        counts_unique_guardians: false,
        authorization_enabled: false,
        processed_event_marking_enabled: false,
        cpi_enabled: false,
        live_route_enabled: false,
    })
}

fn map_rejection_kind(
    status: Phase41K1InstructionsSysvarLiveWiringStatus,
) -> B1CEd25519EvidenceAuthorizationRejectionKind {
    match status {
        Phase41K1InstructionsSysvarLiveWiringStatus::MissingInstructionsSysvar
        | Phase41K1InstructionsSysvarLiveWiringStatus::CurrentInstructionIndexUnavailable
        | Phase41K1InstructionsSysvarLiveWiringStatus::PriorIndexRangeUnavailable
        | Phase41K1InstructionsSysvarLiveWiringStatus::PriorInstructionLoadingFailed => {
            B1CEd25519EvidenceAuthorizationRejectionKind::InvalidInstructionSysvar
        }
        Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorInstructions => {
            B1CEd25519EvidenceAuthorizationRejectionKind::NoPriorInstructions
        }
        Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorEd25519PrecompileInstructions => {
            B1CEd25519EvidenceAuthorizationRejectionKind::NoPriorEd25519Instructions
        }
        Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded => {
            B1CEd25519EvidenceAuthorizationRejectionKind::InvalidEd25519Evidence
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verifier::{
        instructions_sysvar_access_contract_model::Phase41BRejectionCase,
        Phase41K1LoadedPriorEd25519PrecompileInstruction,
    };

    fn loaded_source() -> Phase41K1InstructionsSysvarLiveWiringResult {
        source_result(
            Phase41K1InstructionsSysvarLiveWiringStatus::PriorEd25519PrecompileInstructionsLoaded,
            None,
            Some(3),
            3,
            3,
            1,
            vec![
                prior_ed25519_descriptor(0, 144),
                prior_ed25519_descriptor(2, 144),
            ],
            true,
            true,
            true,
        )
    }

    fn source_result(
        status: Phase41K1InstructionsSysvarLiveWiringStatus,
        rejection_case: Option<Phase41BRejectionCase>,
        current_instruction_index: Option<usize>,
        loaded_prior_instruction_count: usize,
        inspected_prior_instruction_count: usize,
        discarded_non_ed25519_prior_instruction_count: usize,
        prior_ed25519_precompile_instructions: Vec<
            Phase41K1LoadedPriorEd25519PrecompileInstruction,
        >,
        uses_real_instructions_sysvar_account_info: bool,
        checks_instructions_sysvar_account_id: bool,
        load_instruction_at_checked_used: bool,
    ) -> Phase41K1InstructionsSysvarLiveWiringResult {
        let prior_ed25519_precompile_count = prior_ed25519_precompile_instructions.len();

        Phase41K1InstructionsSysvarLiveWiringResult {
            status,
            rejection_case,
            current_instruction_index,
            loaded_prior_instruction_count,
            inspected_prior_instruction_count,
            discarded_non_ed25519_prior_instruction_count,
            prior_ed25519_precompile_count,
            prior_ed25519_precompile_instructions,
            consumes_phase_41d3_1_current_index_boundary: true,
            consumes_phase_41d3_2_1_prior_range_boundary: true,
            consumes_phase_41d3_2_2_checked_prior_loading_boundary: true,
            uses_real_instructions_sysvar_account_info,
            checks_instructions_sysvar_account_id,
            load_current_index_checked_used: true,
            load_instruction_at_checked_used,
            current_index_caller_provided: false,
            enumerates_n_prior_ed25519_precompile_instructions: true,
            model_a_live_wiring_precondition_preserved: prior_ed25519_precompile_count > 0,
            model_a_sysvar_input_precondition_preserved: prior_ed25519_precompile_count > 0,
            full_model_a_handler_execution_context_enforced_here: false,
            requires_41k5_handler_execution_context_enforcement: true,
            all_loaded_ed25519_precompiles_strictly_prior: prior_ed25519_precompile_count > 0,
            all_loaded_ed25519_precompiles_program_id_checked: prior_ed25519_precompile_count > 0,
            accepts_caller_provided_instruction_bytes: false,
            accepts_frontend_or_watcher_ed25519_proof: false,
            guardian_set_runtime_loading_enabled: false,
            processed_registry_runtime_loading_enabled: false,
            replay_write_enabled: false,
            processed_event_marking_enabled: false,
            account_mutation_enabled: false,
            cpi_enabled: false,
            invoke_signed_enabled: false,
            spl_token_mint_to_enabled: false,
            process_instruction_handler_added: false,
            live_route_enabled: false,
        }
    }

    fn prior_ed25519_descriptor(
        instruction_index: usize,
        instruction_data_len: usize,
    ) -> Phase41K1LoadedPriorEd25519PrecompileInstruction {
        Phase41K1LoadedPriorEd25519PrecompileInstruction {
            instruction_index,
            instruction_data_len,
            strictly_prior_to_current_instruction: true,
            program_id_is_ed25519: true,
            loaded_from_real_instructions_sysvar: true,
            loaded_with_checked_runtime_api: true,
            runtime_data_only: true,
            model_a_applies_to_this_prior_precompile: true,
            accepts_verification_evidence: false,
            authorizes_execution: false,
        }
    }

    #[test]
    fn report_documents_b1c2_bridge_scope() {
        let report = b1c_instructions_sysvar_evidence_bridge_report();

        assert_eq!(
            report.phase,
            PHASE_41K6_B1C_2_INSTRUCTIONS_SYSVAR_EVIDENCE_PHASE
        );
        assert_eq!(
            report.version,
            PHASE_41K6_B1C_2_INSTRUCTIONS_SYSVAR_EVIDENCE_VERSION
        );
        assert!(report.consumes_phase_41k1_live_wiring_boundary);
        assert!(report.reads_real_instructions_sysvar_account_info);
        assert!(report.checks_instructions_sysvar_account_id);
        assert!(report.loads_current_instruction_index);
        assert!(report.loads_prior_instructions);
        assert!(report.filters_prior_ed25519_precompile_instructions);
        assert!(!report.parses_ed25519_instruction_data);
        assert!(!report.binds_payload_hash);
        assert!(!report.counts_unique_guardians);
        assert!(!report.authorizes_handler_execution);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn derives_loaded_prior_ed25519_evidence_without_authorizing_execution() {
        let result = derive_b1c_instructions_sysvar_evidence_from_41k1(&loaded_source());

        match result {
            B1CInstructionsSysvarEvidenceResult::Loaded(loaded) => {
                assert_eq!(
                    loaded.status,
                    B1CInstructionsSysvarEvidenceStatus::PriorEd25519EvidenceLoaded
                );
                assert_eq!(loaded.current_instruction_index, 3);
                assert_eq!(loaded.loaded_prior_instruction_count, 3);
                assert_eq!(loaded.inspected_prior_instruction_count, 3);
                assert_eq!(loaded.discarded_non_ed25519_prior_instruction_count, 1);
                assert_eq!(loaded.prior_ed25519_precompile_count, 2);
                assert_eq!(loaded.prior_ed25519_precompile_instructions.len(), 2);
                assert!(loaded.uses_real_instructions_sysvar_account_info);
                assert!(loaded.checks_instructions_sysvar_account_id);
                assert!(loaded.load_current_index_checked_used);
                assert!(loaded.load_instruction_at_checked_used);
                assert!(!loaded.accepts_caller_provided_instruction_bytes);
                assert!(!loaded.accepts_frontend_or_watcher_ed25519_proof);
                assert!(!loaded.parses_ed25519_instruction_data);
                assert!(!loaded.binds_payload_hash);
                assert!(!loaded.counts_unique_guardians);
                assert!(!loaded.authorization_enabled);
                assert!(!loaded.processed_event_marking_enabled);
                assert!(!loaded.cpi_enabled);
                assert!(!loaded.live_route_enabled);

                for prior in loaded.prior_ed25519_precompile_instructions.iter() {
                    assert!(prior.strictly_prior_to_current_instruction);
                    assert!(prior.program_id_is_ed25519);
                    assert!(prior.loaded_from_real_instructions_sysvar);
                    assert!(prior.loaded_with_checked_runtime_api);
                    assert!(!prior.accepts_caller_provided_instruction_bytes);
                    assert!(!prior.accepts_frontend_or_watcher_ed25519_proof);
                    assert!(!prior.parses_ed25519_instruction_data);
                    assert!(!prior.binds_payload_hash);
                    assert!(!prior.counts_unique_guardians);
                    assert!(!prior.authorizes_execution);
                }
            }
            B1CInstructionsSysvarEvidenceResult::Rejected(_) => {
                panic!("expected loaded prior Ed25519 evidence")
            }
        }
    }

    #[test]
    fn maps_missing_instructions_sysvar_to_invalid_instruction_sysvar_rejection() {
        let source = source_result(
            Phase41K1InstructionsSysvarLiveWiringStatus::MissingInstructionsSysvar,
            Some(Phase41BRejectionCase::MissingInstructionsSysvar),
            None,
            0,
            0,
            0,
            Vec::new(),
            false,
            false,
            false,
        );

        assert_rejected(
            derive_b1c_instructions_sysvar_evidence_from_41k1(&source),
            B1CEd25519EvidenceAuthorizationRejectionKind::InvalidInstructionSysvar,
            Phase41K1InstructionsSysvarLiveWiringStatus::MissingInstructionsSysvar,
        );
    }

    #[test]
    fn maps_no_prior_instructions_to_no_prior_instructions_rejection() {
        let source = source_result(
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorInstructions,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            Some(0),
            0,
            0,
            0,
            Vec::new(),
            true,
            true,
            true,
        );

        assert_rejected(
            derive_b1c_instructions_sysvar_evidence_from_41k1(&source),
            B1CEd25519EvidenceAuthorizationRejectionKind::NoPriorInstructions,
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorInstructions,
        );
    }

    #[test]
    fn maps_no_prior_ed25519_to_no_prior_ed25519_rejection() {
        let source = source_result(
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorEd25519PrecompileInstructions,
            Some(Phase41BRejectionCase::Ed25519InstructionNotFound),
            Some(2),
            2,
            2,
            2,
            Vec::new(),
            true,
            true,
            true,
        );

        assert_rejected(
            derive_b1c_instructions_sysvar_evidence_from_41k1(&source),
            B1CEd25519EvidenceAuthorizationRejectionKind::NoPriorEd25519Instructions,
            Phase41K1InstructionsSysvarLiveWiringStatus::NoPriorEd25519PrecompileInstructions,
        );
    }

    #[test]
    fn rejected_bridge_result_keeps_all_execution_flags_false() {
        let source = source_result(
            Phase41K1InstructionsSysvarLiveWiringStatus::PriorInstructionLoadingFailed,
            Some(Phase41BRejectionCase::UnreadableInstructionsSysvar),
            Some(2),
            1,
            0,
            0,
            Vec::new(),
            true,
            true,
            true,
        );

        match derive_b1c_instructions_sysvar_evidence_from_41k1(&source) {
            B1CInstructionsSysvarEvidenceResult::Rejected(rejected) => {
                assert_eq!(
                    rejected.rejection_kind,
                    B1CEd25519EvidenceAuthorizationRejectionKind::InvalidInstructionSysvar
                );
                assert!(!rejected.accepts_caller_provided_instruction_bytes);
                assert!(!rejected.accepts_frontend_or_watcher_ed25519_proof);
                assert!(!rejected.parses_ed25519_instruction_data);
                assert!(!rejected.binds_payload_hash);
                assert!(!rejected.counts_unique_guardians);
                assert!(!rejected.authorization_enabled);
                assert!(!rejected.processed_event_marking_enabled);
                assert!(!rejected.cpi_enabled);
                assert!(!rejected.live_route_enabled);
            }
            B1CInstructionsSysvarEvidenceResult::Loaded(_) => {
                panic!("expected rejected result")
            }
        }
    }

    fn assert_rejected(
        result: B1CInstructionsSysvarEvidenceResult,
        expected_kind: B1CEd25519EvidenceAuthorizationRejectionKind,
        expected_source_status: Phase41K1InstructionsSysvarLiveWiringStatus,
    ) {
        match result {
            B1CInstructionsSysvarEvidenceResult::Rejected(rejected) => {
                assert_eq!(
                    rejected.status,
                    B1CInstructionsSysvarEvidenceStatus::Rejected
                );
                assert_eq!(rejected.rejection_kind, expected_kind);
                assert_eq!(rejected.source_status, expected_source_status);
                assert!(!rejected.authorization_enabled);
                assert!(!rejected.processed_event_marking_enabled);
                assert!(!rejected.cpi_enabled);
                assert!(!rejected.live_route_enabled);
            }
            B1CInstructionsSysvarEvidenceResult::Loaded(_) => {
                panic!("expected rejected result")
            }
        }
    }
}

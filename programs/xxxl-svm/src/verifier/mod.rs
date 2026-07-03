pub mod boundary;
pub mod canonical_payload;
pub mod ed25519_evidence_layout;
pub mod ed25519_instruction_data_parser;
pub mod ed25519_prior_instruction_ordering;
pub mod ed25519_verification_evidence;
pub mod ed25519_verification_evidence_coverage_matrix;
pub mod ed25519_verification_evidence_integration_design;
pub mod errors;
pub mod guardian_membership_validation_boundary;
pub mod guardian_quorum;
pub mod guardian_set_account_loading_boundary;
pub mod instructions_sysvar_evidence_scanner;
pub mod instructions_sysvar_live_wiring_boundary;
pub mod raw_payload;
pub mod types;

pub use boundary::{
    future_runtime_parity_case_reports, read_only_verifier_boundary,
    read_only_verifier_scaffold_report, runtime_verifier_boundary_components,
    verifier_error_categories, ReadOnlyVerifierBoundary, ReadOnlyVerifierScaffoldReport,
    FUTURE_RUNTIME_PARITY_CASES, FUTURE_RUNTIME_PARITY_CASE_REPORTS,
    READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32, READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_VERSION,
    READ_ONLY_VERIFIER_BOUNDARY, READ_ONLY_VERIFIER_SCAFFOLD_REPORT,
    RUNTIME_VERIFIER_BOUNDARY_COMPONENTS,
};
pub use canonical_payload::{
    canonical_payload_hash_validation_report, compute_guardian_payload_hash,
    compute_guardian_payload_hash_domain_separator, validate_guardian_payload_hash,
    CanonicalPayloadHashValidationError, CanonicalPayloadHashValidationErrorKind,
    CanonicalPayloadHashValidationReport, CANONICAL_PAYLOAD_HASH_VALIDATION_REPORT,
    CANONICAL_PAYLOAD_HASH_VALIDATOR_PHASE_34, CANONICAL_PAYLOAD_HASH_VALIDATOR_VERSION,
    XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_SEPARATOR_V1, XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_V1,
    XXXL_GUARDIAN_PAYLOAD_VALID_HASH_V1,
};
pub use ed25519_evidence_layout::{
    ed25519_evidence_layout_model_report, validate_ed25519_evidence_layout,
    Ed25519EvidenceLayoutDescriptor, Ed25519EvidenceLayoutError, Ed25519EvidenceLayoutErrorKind,
    Ed25519EvidenceLayoutModelReport, Ed25519EvidenceLayoutModelResult,
    Ed25519SignatureOffsetsModel, CURRENT_INSTRUCTION_INDEX_SENTINEL,
    ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_PHASE_37,
    ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_REPORT,
    ED25519_INSTRUCTION_EVIDENCE_LAYOUT_MODEL_VERSION, ED25519_INSTRUCTION_HEADER_LEN,
    ED25519_PROGRAM_ID_REFERENCE, ED25519_PUBLIC_KEY_LEN, ED25519_SIGNATURE_LEN,
    ED25519_SIGNATURE_OFFSETS_RECORD_LEN, EXPECTED_MESSAGE_LEN, EXPECTED_SIGNATURE_COUNT,
};
pub use ed25519_instruction_data_parser::{
    ed25519_instruction_data_parser_report, parse_ed25519_instruction_data_evidence,
    Ed25519InstructionDataParserError, Ed25519InstructionDataParserErrorKind,
    Ed25519InstructionDataParserReport, ParsedEd25519InstructionEvidence,
    ED25519_INSTRUCTION_DATA_PARSER_PHASE_38, ED25519_INSTRUCTION_DATA_PARSER_REPORT,
    ED25519_INSTRUCTION_DATA_PARSER_VERSION,
};
pub use ed25519_prior_instruction_ordering::{
    ed25519_prior_instruction_ordering_model_report, model_ed25519_prior_instruction_ordering,
    Ed25519PriorInstructionOrderingError, Ed25519PriorInstructionOrderingErrorKind,
    Ed25519PriorInstructionOrderingModelReport, Ed25519PriorInstructionOrderingResult,
    ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_PHASE_40E,
    ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_REPORT,
    ED25519_PRIOR_INSTRUCTION_ORDERING_MODEL_VERSION,
};
pub use ed25519_verification_evidence::{
    ed25519_verification_evidence_model_report, model_ed25519_verification_evidence_boundary,
    Ed25519VerificationEvidenceBoundaryError, Ed25519VerificationEvidenceBoundaryErrorKind,
    Ed25519VerificationEvidenceBoundaryResult, Ed25519VerificationEvidenceModelReport,
    ED25519_VERIFICATION_EVIDENCE_MODEL_PHASE_40B, ED25519_VERIFICATION_EVIDENCE_MODEL_REPORT,
    ED25519_VERIFICATION_EVIDENCE_MODEL_VERSION,
};
pub use ed25519_verification_evidence_coverage_matrix::{
    coverage_for_requirement, ed25519_verification_evidence_coverage_matrix_report,
    ed25519_verification_evidence_requirement_coverage_matrix,
    every_phase_40d_requirement_has_coverage_entry, Ed25519VerificationEvidenceCoverageCategory,
    Ed25519VerificationEvidenceCoverageMatrixReport,
    Ed25519VerificationEvidenceRequirementCoverage,
    ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_PHASE_40F,
    ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_REPORT,
    ED25519_VERIFICATION_EVIDENCE_COVERAGE_MATRIX_VERSION,
    ED25519_VERIFICATION_EVIDENCE_REQUIREMENT_COVERAGE_MATRIX,
};
pub use ed25519_verification_evidence_integration_design::{
    ed25519_verification_evidence_integration_design_report,
    future_ed25519_verification_evidence_rejection_cases,
    future_ed25519_verification_evidence_requirements,
    Ed25519VerificationEvidenceIntegrationDesignReport,
    FutureEd25519VerificationEvidenceRejectionCase, FutureEd25519VerificationEvidenceRequirement,
    ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_PHASE_40D,
    ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_REPORT,
    ED25519_VERIFICATION_EVIDENCE_INTEGRATION_DESIGN_VERSION,
    FUTURE_ED25519_VERIFICATION_EVIDENCE_REJECTION_CASES,
    FUTURE_ED25519_VERIFICATION_EVIDENCE_REQUIREMENTS,
};
pub use errors::{VerifierErrorCategory, VERIFIER_ERROR_CATEGORIES};
pub use guardian_membership_validation_boundary::{
    establish_guardian_membership_validation, guardian_membership_validation_boundary_report,
    AuthoritativeGuardianSetRef, AuthoritativeGuardianSetSource, GuardianMembershipValidated,
    GuardianMembershipValidationBoundaryReport, GuardianMembershipValidationError,
    GuardianMembershipValidationErrorKind, GuardianMembershipValidationStatus,
    GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_PHASE_41H,
    GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_REPORT,
    GUARDIAN_MEMBERSHIP_VALIDATION_BOUNDARY_VERSION,
};
pub use guardian_quorum::{
    guardian_quorum_structural_report, verify_guardian_quorum_structural, GuardianApprovalClaim,
    GuardianApprovalRef, GuardianPublicKey, GuardianQuorumStructuralError,
    GuardianQuorumStructuralErrorKind, GuardianQuorumStructuralReport,
    GuardianQuorumStructuralResult, GuardianSetRef, GUARDIAN_QUORUM_STRUCTURAL_REPORT,
    GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35, GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_VERSION,
};
pub use guardian_set_account_loading_boundary::{
    decode_phase_41k_2_guardian_set_account_data, find_phase_41k_2_guardian_set_pda,
    load_phase_41k_2_guardian_set_account_info,
    phase_41k_2_guardian_set_account_loading_boundary_report,
    Phase41K2GuardianSetAccountLoadingBoundaryReport, Phase41K2GuardianSetAccountLoadingResult,
    Phase41K2GuardianSetAccountLoadingStatus, Phase41K2GuardianSetAccountRejectionCase,
    GUARDIAN_PUBLIC_KEY_LEN, GUARDIAN_SET_ACTIVE_STATUS_OFFSET, GUARDIAN_SET_GUARDIAN_COUNT_OFFSET,
    GUARDIAN_SET_GUARDIAN_KEYS_OFFSET, GUARDIAN_SET_GUARDIAN_SET_ID_OFFSET,
    GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET, GUARDIAN_SET_STATUS_ACTIVE,
    GUARDIAN_SET_STATUS_DEPRECATED, GUARDIAN_SET_STATUS_INACTIVE, MAX_SUPPORTED_GUARDIAN_COUNT,
    PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_PHASE,
    PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_REPORT,
    PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_VERSION,
};
pub use instructions_sysvar_evidence_scanner::{
    instructions_sysvar_evidence_scanner_report, scan_instructions_sysvar_for_ed25519_evidence,
    InstructionsSysvarEvidenceScannerError, InstructionsSysvarEvidenceScannerErrorKind,
    InstructionsSysvarEvidenceScannerReport, InstructionsSysvarInstructionView,
    ScannedEd25519InstructionEvidence, INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39,
    INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_REPORT, INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_VERSION,
};
pub use instructions_sysvar_live_wiring_boundary::{
    establish_phase_41k_1_instructions_sysvar_live_wiring_boundary,
    phase_41k_1_instructions_sysvar_live_wiring_boundary_report,
    Phase41K1InstructionsSysvarLiveWiringBoundaryReport,
    Phase41K1InstructionsSysvarLiveWiringResult, Phase41K1InstructionsSysvarLiveWiringStatus,
    Phase41K1LoadedPriorEd25519PrecompileInstruction,
    PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_PHASE,
    PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_REPORT,
    PHASE_41K_1_INSTRUCTIONS_SYSVAR_LIVE_WIRING_BOUNDARY_VERSION,
};
pub use processed_registry_account_loading_boundary::{
    decode_phase_41k_3_processed_event_account_data, find_phase_41k_3_processed_event_pda,
    load_phase_41k_3_processed_registry_account_info,
    phase_41k_3_processed_registry_account_loading_boundary_report,
    Phase41K3ProcessedRegistryAccountLoadingBoundaryReport,
    Phase41K3ProcessedRegistryAccountLoadingResult,
    Phase41K3ProcessedRegistryAccountLoadingStatus,
    Phase41K3ProcessedRegistryAccountRejectionCase, Phase41K3ProcessedRegistryLoadWitness,
    PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_PHASE,
    PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_REPORT,
    PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_VERSION,
    PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET, PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET,
    PROCESSED_EVENT_CONSUMED_OFFSET, PROCESSED_EVENT_CONSUMED_SLOT_OFFSET,
    PROCESSED_EVENT_PDA_SEED_0, PROCESSED_EVENT_PDA_SEED_1,
    PROCESSED_EVENT_RECIPIENT_OFFSET, PROCESSED_EVENT_ROUTE_ID_OFFSET,
};
pub use quorum_authorization_boundary::{
    establish_guardian_quorum_authorization, guardian_quorum_authorization_boundary_report,
    GuardianQuorumAuthorizationAttempt, GuardianQuorumAuthorizationAttemptOutcome,
    GuardianQuorumAuthorizationAttemptStatus, GuardianQuorumAuthorizationBoundaryReport,
    GuardianQuorumAuthorizationError, GuardianQuorumAuthorizationErrorKind,
    GuardianQuorumAuthorizationEstablished, GuardianQuorumAuthorizationStatus,
    GUARDIAN_QUORUM_AUTHORIZATION_BOUNDARY_REPORT, QUORUM_AUTHORIZATION_BOUNDARY_PHASE_41I,
    QUORUM_AUTHORIZATION_BOUNDARY_VERSION,
};
pub use raw_payload::{
    decode_guardian_payload_raw, raw_payload_decoder_report, DecodedGuardianPayloadRaw,
    RawPayloadDecodeError, RawPayloadDecodeErrorKind, RawPayloadDecoderReport,
    RAW_PAYLOAD_DECODER_PHASE_33, RAW_PAYLOAD_DECODER_REPORT, RAW_PAYLOAD_DECODER_VERSION,
    RAW_PAYLOAD_PHASE_23_FIELD_ORDER,
};
pub use replay_protection_boundary::{
    establish_replay_protection_eligibility, replay_protection_boundary_report,
    AuthoritativeProcessedRegistryViewRef, ProcessedRegistryViewSource,
    ReplayProtectionBoundaryReport, ReplayProtectionEligibilityError,
    ReplayProtectionEligibilityErrorKind, ReplayProtectionEligibilityEstablished,
    ReplayProtectionEligibilityStatus, REPLAY_PROTECTION_BOUNDARY_PHASE_41J,
    REPLAY_PROTECTION_BOUNDARY_REPORT, REPLAY_PROTECTION_BOUNDARY_VERSION,
};
pub use types::{
    FutureRuntimeParityCase, FutureRuntimeParityCaseReport, RuntimeVerifierBoundaryComponent,
    VerifierBoundaryStatus,
};
pub mod checked_ed25519_byte_extraction_boundary;
pub mod checked_prior_instruction_loading_runtime_boundary;
pub mod current_instruction_identity_boundary;
pub mod current_instruction_identity_runtime_boundary;
pub mod current_instruction_index_runtime_boundary;
pub mod ed25519_instruction_byte_parsing_boundary;
pub mod ed25519_signature_verification_boundary;
pub mod instructions_sysvar_access_contract_model;
pub mod instructions_sysvar_accountinfo_presence_readability_runtime_boundary;
pub mod instructions_sysvar_presence_readability_boundary;
pub mod payload_hash_binding_boundary;
pub mod prefilter_phase_41c3_candidate_descriptor_runtime_boundary;
pub mod prior_ed25519_lookup_ordering_boundary;
pub mod prior_instruction_index_range_runtime_boundary;
pub mod processed_registry_account_loading_boundary;
pub mod quorum_authorization_boundary;
pub mod replay_protection_boundary;

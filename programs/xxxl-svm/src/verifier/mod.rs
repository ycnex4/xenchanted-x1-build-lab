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
pub mod instructions_sysvar_evidence_scanner;
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
pub use instructions_sysvar_evidence_scanner::{
    instructions_sysvar_evidence_scanner_report, scan_instructions_sysvar_for_ed25519_evidence,
    InstructionsSysvarEvidenceScannerError, InstructionsSysvarEvidenceScannerErrorKind,
    InstructionsSysvarEvidenceScannerReport, InstructionsSysvarInstructionView,
    ScannedEd25519InstructionEvidence, INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_PHASE_39,
    INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_REPORT, INSTRUCTIONS_SYSVAR_EVIDENCE_SCANNER_VERSION,
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
pub mod quorum_authorization_boundary;

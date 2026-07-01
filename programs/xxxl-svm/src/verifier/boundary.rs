use super::{
    errors::{VerifierErrorCategory, VERIFIER_ERROR_CATEGORIES},
    types::{
        FutureRuntimeParityCase, FutureRuntimeParityCaseReport, RuntimeVerifierBoundaryComponent,
        VerifierBoundaryStatus,
    },
};

pub const READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32: &str =
    "READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32";

pub const READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_VERSION: u8 = 1;

pub const RUNTIME_VERIFIER_BOUNDARY_COMPONENTS: [RuntimeVerifierBoundaryComponent; 10] = [
    RuntimeVerifierBoundaryComponent::RawPayloadDecoder,
    RuntimeVerifierBoundaryComponent::CanonicalPayloadValidation,
    RuntimeVerifierBoundaryComponent::SourceProofIdentityVerifier,
    RuntimeVerifierBoundaryComponent::GuardianApprovalAndQuorumVerifier,
    RuntimeVerifierBoundaryComponent::RouteBindingVerifier,
    RuntimeVerifierBoundaryComponent::TargetMintLegitimacyVerifier,
    RuntimeVerifierBoundaryComponent::ReplayVerifier,
    RuntimeVerifierBoundaryComponent::ExpirationVerifier,
    RuntimeVerifierBoundaryComponent::AmountControlVerifier,
    RuntimeVerifierBoundaryComponent::DeterministicErrorSurface,
];

pub const FUTURE_RUNTIME_PARITY_CASES: [FutureRuntimeParityCase; 7] = [
    FutureRuntimeParityCase::WrongFieldOrder,
    FutureRuntimeParityCase::WrongByteEncoding,
    FutureRuntimeParityCase::WrongCanonicalEventKeyPreimage,
    FutureRuntimeParityCase::WrongSourceBurnTxHash,
    FutureRuntimeParityCase::WrongSourceBurnEventIndex,
    FutureRuntimeParityCase::AmountOverRouteCap,
    FutureRuntimeParityCase::InvalidTargetMint,
];

pub const FUTURE_RUNTIME_PARITY_CASE_REPORTS: [FutureRuntimeParityCaseReport; 7] = [
    FutureRuntimeParityCaseReport {
        case: FutureRuntimeParityCase::WrongFieldOrder,
        case_id: "wrong-field-order",
        status: VerifierBoundaryStatus::FutureVerifierObligation,
        implemented: false,
        required_component: RuntimeVerifierBoundaryComponent::RawPayloadDecoder,
        reason: "Raw payload field order decoding remains a future verifier obligation.",
    },
    FutureRuntimeParityCaseReport {
        case: FutureRuntimeParityCase::WrongByteEncoding,
        case_id: "wrong-byte-encoding",
        status: VerifierBoundaryStatus::FutureVerifierObligation,
        implemented: false,
        required_component: RuntimeVerifierBoundaryComponent::RawPayloadDecoder,
        reason: "Raw payload byte encoding checks remain a future verifier obligation.",
    },
    FutureRuntimeParityCaseReport {
        case: FutureRuntimeParityCase::WrongCanonicalEventKeyPreimage,
        case_id: "wrong-canonical-event-key-preimage",
        status: VerifierBoundaryStatus::FutureVerifierObligation,
        implemented: false,
        required_component: RuntimeVerifierBoundaryComponent::SourceProofIdentityVerifier,
        reason: "Canonical event key preimage verification remains a future verifier obligation.",
    },
    FutureRuntimeParityCaseReport {
        case: FutureRuntimeParityCase::WrongSourceBurnTxHash,
        case_id: "wrong-source-burn-tx-hash",
        status: VerifierBoundaryStatus::FutureVerifierObligation,
        implemented: false,
        required_component: RuntimeVerifierBoundaryComponent::SourceProofIdentityVerifier,
        reason: "Source burn transaction hash verification remains a future verifier obligation.",
    },
    FutureRuntimeParityCaseReport {
        case: FutureRuntimeParityCase::WrongSourceBurnEventIndex,
        case_id: "wrong-source-burn-event-index",
        status: VerifierBoundaryStatus::FutureVerifierObligation,
        implemented: false,
        required_component: RuntimeVerifierBoundaryComponent::SourceProofIdentityVerifier,
        reason: "Source burn event index verification remains a future verifier obligation.",
    },
    FutureRuntimeParityCaseReport {
        case: FutureRuntimeParityCase::AmountOverRouteCap,
        case_id: "amount-over-route-cap",
        status: VerifierBoundaryStatus::FutureVerifierObligation,
        implemented: false,
        required_component: RuntimeVerifierBoundaryComponent::AmountControlVerifier,
        reason: "Route amount cap verification remains a future verifier obligation.",
    },
    FutureRuntimeParityCaseReport {
        case: FutureRuntimeParityCase::InvalidTargetMint,
        case_id: "invalid-target-mint",
        status: VerifierBoundaryStatus::FutureVerifierObligation,
        implemented: false,
        required_component: RuntimeVerifierBoundaryComponent::TargetMintLegitimacyVerifier,
        reason: "Target mint account legitimacy verification remains a future verifier obligation.",
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadOnlyVerifierBoundary {
    pub scaffold_id: &'static str,
    pub scaffold_version: u8,
    pub status: VerifierBoundaryStatus,
    pub components: &'static [RuntimeVerifierBoundaryComponent],
    pub future_runtime_cases: &'static [FutureRuntimeParityCaseReport],
    pub error_categories: &'static [VerifierErrorCategory],
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

impl ReadOnlyVerifierBoundary {
    pub fn execution_enabled(&self) -> bool {
        self.live_route_enabled
            || self.spl_cpi_enabled
            || self.invoke_signed_enabled
            || self.mint_execution_enabled
            || self.runtime_state_mutation_enabled
            || self.replay_write_enabled
            || self.processed_event_marking_enabled
    }

    pub fn deployment_unlocked(&self) -> bool {
        self.production_program_id_selected || self.deployment_blockers_removed
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReadOnlyVerifierScaffoldReport {
    pub scaffold_id: &'static str,
    pub scaffold_version: u8,
    pub status: VerifierBoundaryStatus,
    pub component_count: usize,
    pub future_runtime_case_count: usize,
    pub error_category_count: usize,
    pub all_future_cases_not_implemented: bool,
    pub execution_enabled: bool,
    pub deployment_unlocked: bool,
}

pub const READ_ONLY_VERIFIER_BOUNDARY: ReadOnlyVerifierBoundary = ReadOnlyVerifierBoundary {
    scaffold_id: READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
    scaffold_version: READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_VERSION,
    status: VerifierBoundaryStatus::ReadOnlyScaffoldOnly,
    components: &RUNTIME_VERIFIER_BOUNDARY_COMPONENTS,
    future_runtime_cases: &FUTURE_RUNTIME_PARITY_CASE_REPORTS,
    error_categories: &VERIFIER_ERROR_CATEGORIES,
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

pub const READ_ONLY_VERIFIER_SCAFFOLD_REPORT: ReadOnlyVerifierScaffoldReport =
    ReadOnlyVerifierScaffoldReport {
        scaffold_id: READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32,
        scaffold_version: READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_VERSION,
        status: VerifierBoundaryStatus::ReadOnlyScaffoldOnly,
        component_count: 10,
        future_runtime_case_count: 7,
        error_category_count: 11,
        all_future_cases_not_implemented: true,
        execution_enabled: false,
        deployment_unlocked: false,
    };

pub fn read_only_verifier_boundary() -> &'static ReadOnlyVerifierBoundary {
    &READ_ONLY_VERIFIER_BOUNDARY
}

pub fn read_only_verifier_scaffold_report() -> &'static ReadOnlyVerifierScaffoldReport {
    &READ_ONLY_VERIFIER_SCAFFOLD_REPORT
}

pub fn runtime_verifier_boundary_components() -> &'static [RuntimeVerifierBoundaryComponent] {
    &RUNTIME_VERIFIER_BOUNDARY_COMPONENTS
}

pub fn future_runtime_parity_case_reports() -> &'static [FutureRuntimeParityCaseReport] {
    &FUTURE_RUNTIME_PARITY_CASE_REPORTS
}

pub fn verifier_error_categories() -> &'static [VerifierErrorCategory] {
    &VERIFIER_ERROR_CATEGORIES
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaffold_marker_and_version_are_stable() {
        let boundary = read_only_verifier_boundary();
        let report = read_only_verifier_scaffold_report();

        assert_eq!(
            boundary.scaffold_id,
            "READ_ONLY_RUNTIME_VERIFIER_SCAFFOLD_PHASE_32"
        );
        assert_eq!(boundary.scaffold_version, 1);
        assert_eq!(
            boundary.status,
            VerifierBoundaryStatus::ReadOnlyScaffoldOnly
        );
        assert_eq!(boundary.status.code(), "READ_ONLY_SCAFFOLD_ONLY");
        assert_eq!(report.scaffold_id, boundary.scaffold_id);
        assert_eq!(report.scaffold_version, boundary.scaffold_version);
    }

    #[test]
    fn boundary_lists_exactly_phase_31_components() {
        let components = runtime_verifier_boundary_components();

        assert_eq!(components.len(), 10);
        assert_eq!(
            components,
            [
                RuntimeVerifierBoundaryComponent::RawPayloadDecoder,
                RuntimeVerifierBoundaryComponent::CanonicalPayloadValidation,
                RuntimeVerifierBoundaryComponent::SourceProofIdentityVerifier,
                RuntimeVerifierBoundaryComponent::GuardianApprovalAndQuorumVerifier,
                RuntimeVerifierBoundaryComponent::RouteBindingVerifier,
                RuntimeVerifierBoundaryComponent::TargetMintLegitimacyVerifier,
                RuntimeVerifierBoundaryComponent::ReplayVerifier,
                RuntimeVerifierBoundaryComponent::ExpirationVerifier,
                RuntimeVerifierBoundaryComponent::AmountControlVerifier,
                RuntimeVerifierBoundaryComponent::DeterministicErrorSurface,
            ]
        );

        for component in components {
            assert!(!component.code().is_empty());
        }
    }

    #[test]
    fn future_runtime_cases_are_carried_forward_and_not_implemented() {
        let reports = future_runtime_parity_case_reports();
        let case_ids: Vec<&str> = reports.iter().map(|report| report.case_id).collect();

        assert_eq!(reports.len(), 7);
        assert_eq!(
            case_ids,
            vec![
                "wrong-field-order",
                "wrong-byte-encoding",
                "wrong-canonical-event-key-preimage",
                "wrong-source-burn-tx-hash",
                "wrong-source-burn-event-index",
                "amount-over-route-cap",
                "invalid-target-mint",
            ]
        );

        for report in reports {
            assert_eq!(report.case.case_id(), report.case_id);
            assert_eq!(
                report.status,
                VerifierBoundaryStatus::FutureVerifierObligation
            );
            assert_eq!(report.status.code(), "FUTURE_VERIFIER_OBLIGATION");
            assert!(!report.implemented);
            assert!(!report.reason.is_empty());
        }
    }

    #[test]
    fn error_categories_include_phase_31_surface() {
        let categories = verifier_error_categories();

        assert_eq!(categories.len(), 11);
        assert!(categories.contains(&VerifierErrorCategory::RawPayloadDecode));
        assert!(categories.contains(&VerifierErrorCategory::CanonicalPayloadValidation));
        assert!(categories.contains(&VerifierErrorCategory::PayloadHashDomain));
        assert!(categories.contains(&VerifierErrorCategory::GuardianSignature));
        assert!(categories.contains(&VerifierErrorCategory::GuardianQuorum));
        assert!(categories.contains(&VerifierErrorCategory::RouteBinding));
        assert!(categories.contains(&VerifierErrorCategory::SourceProofIdentity));
        assert!(categories.contains(&VerifierErrorCategory::Replay));
        assert!(categories.contains(&VerifierErrorCategory::Expiration));
        assert!(categories.contains(&VerifierErrorCategory::AmountCap));
        assert!(categories.contains(&VerifierErrorCategory::TargetMintLegitimacy));

        for category in categories {
            assert!(category.code().ends_with("_ERROR"));
        }
    }

    #[test]
    fn safety_flags_remain_disabled() {
        let boundary = read_only_verifier_boundary();

        assert!(!boundary.live_route_enabled);
        assert!(!boundary.spl_cpi_enabled);
        assert!(!boundary.invoke_signed_enabled);
        assert!(!boundary.mint_execution_enabled);
        assert!(!boundary.runtime_state_mutation_enabled);
        assert!(!boundary.replay_write_enabled);
        assert!(!boundary.processed_event_marking_enabled);
        assert!(!boundary.production_program_id_selected);
        assert!(!boundary.deployment_blockers_removed);
        assert!(!boundary.execution_enabled());
        assert!(!boundary.deployment_unlocked());
    }

    #[test]
    fn scaffold_report_matches_read_only_boundary() {
        let boundary = read_only_verifier_boundary();
        let report = read_only_verifier_scaffold_report();

        assert_eq!(report.status, boundary.status);
        assert_eq!(report.component_count, boundary.components.len());
        assert_eq!(
            report.future_runtime_case_count,
            boundary.future_runtime_cases.len()
        );
        assert_eq!(report.error_category_count, boundary.error_categories.len());
        assert!(report.all_future_cases_not_implemented);
        assert!(!report.execution_enabled);
        assert!(!report.deployment_unlocked);
    }
}

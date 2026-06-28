#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlRuntimeDeploymentStatus {
    ScaffoldOnlyNotDeployable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlRuntimeDeploymentBlocker {
    PlaceholderProgramId,
    LiveRouteDisabled,
    SplCpiExecutionDisabled,
    ProductionGuardianSetUnset,
    ProductionProofLogUnset,
    ExternalReviewIncomplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlRuntimeDeploymentBlockerReport {
    pub blocker: XxxlRuntimeDeploymentBlocker,
    pub code: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlRuntimeDeploymentReport {
    pub status: XxxlRuntimeDeploymentStatus,
    pub status_code: &'static str,
    pub status_description: &'static str,
    pub deployable: bool,
    pub blockers: &'static [XxxlRuntimeDeploymentBlockerReport],
}

pub const XXXL_RUNTIME_DEPLOYMENT_STATUS: XxxlRuntimeDeploymentStatus =
    XxxlRuntimeDeploymentStatus::ScaffoldOnlyNotDeployable;

pub const XXXL_RUNTIME_DEPLOYMENT_BLOCKERS: [XxxlRuntimeDeploymentBlocker; 6] = [
    XxxlRuntimeDeploymentBlocker::PlaceholderProgramId,
    XxxlRuntimeDeploymentBlocker::LiveRouteDisabled,
    XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled,
    XxxlRuntimeDeploymentBlocker::ProductionGuardianSetUnset,
    XxxlRuntimeDeploymentBlocker::ProductionProofLogUnset,
    XxxlRuntimeDeploymentBlocker::ExternalReviewIncomplete,
];

pub const XXXL_RUNTIME_DEPLOYMENT_BLOCKER_REPORTS: [XxxlRuntimeDeploymentBlockerReport; 6] = [
    XxxlRuntimeDeploymentBlockerReport {
        blocker: XxxlRuntimeDeploymentBlocker::PlaceholderProgramId,
        code: "PLACEHOLDER_PROGRAM_ID",
        description: "The runtime still exposes a placeholder Program ID boundary.",
    },
    XxxlRuntimeDeploymentBlockerReport {
        blocker: XxxlRuntimeDeploymentBlocker::LiveRouteDisabled,
        code: "LIVE_ROUTE_DISABLED",
        description: "Live route activation from process_instruction remains disabled.",
    },
    XxxlRuntimeDeploymentBlockerReport {
        blocker: XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled,
        code: "SPL_CPI_EXECUTION_DISABLED",
        description: "SPL Token mint_to CPI execution remains disabled.",
    },
    XxxlRuntimeDeploymentBlockerReport {
        blocker: XxxlRuntimeDeploymentBlocker::ProductionGuardianSetUnset,
        code: "PRODUCTION_GUARDIAN_SET_UNSET",
        description: "The production guardian set is not configured or externally documented.",
    },
    XxxlRuntimeDeploymentBlockerReport {
        blocker: XxxlRuntimeDeploymentBlocker::ProductionProofLogUnset,
        code: "PRODUCTION_PROOF_LOG_UNSET",
        description: "The production proof-log and public audit trail are not configured.",
    },
    XxxlRuntimeDeploymentBlockerReport {
        blocker: XxxlRuntimeDeploymentBlocker::ExternalReviewIncomplete,
        code: "EXTERNAL_REVIEW_INCOMPLETE",
        description: "External review is not complete for deployment activation.",
    },
];

pub const XXXL_RUNTIME_DEPLOYMENT_REPORT: XxxlRuntimeDeploymentReport =
    XxxlRuntimeDeploymentReport {
        status: XXXL_RUNTIME_DEPLOYMENT_STATUS,
        status_code: "SCAFFOLD_ONLY_NOT_DEPLOYABLE",
        status_description: "The XXXL SVM runtime is a scaffold-only build and is not deployable.",
        deployable: false,
        blockers: &XXXL_RUNTIME_DEPLOYMENT_BLOCKER_REPORTS,
    };

impl XxxlRuntimeDeploymentStatus {
    pub fn code(self) -> &'static str {
        match self {
            XxxlRuntimeDeploymentStatus::ScaffoldOnlyNotDeployable => {
                "SCAFFOLD_ONLY_NOT_DEPLOYABLE"
            }
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            XxxlRuntimeDeploymentStatus::ScaffoldOnlyNotDeployable => {
                "The XXXL SVM runtime is a scaffold-only build and is not deployable."
            }
        }
    }
}

impl XxxlRuntimeDeploymentBlocker {
    pub fn code(self) -> &'static str {
        match self {
            XxxlRuntimeDeploymentBlocker::PlaceholderProgramId => "PLACEHOLDER_PROGRAM_ID",
            XxxlRuntimeDeploymentBlocker::LiveRouteDisabled => "LIVE_ROUTE_DISABLED",
            XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled => "SPL_CPI_EXECUTION_DISABLED",
            XxxlRuntimeDeploymentBlocker::ProductionGuardianSetUnset => {
                "PRODUCTION_GUARDIAN_SET_UNSET"
            }
            XxxlRuntimeDeploymentBlocker::ProductionProofLogUnset => "PRODUCTION_PROOF_LOG_UNSET",
            XxxlRuntimeDeploymentBlocker::ExternalReviewIncomplete => "EXTERNAL_REVIEW_INCOMPLETE",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            XxxlRuntimeDeploymentBlocker::PlaceholderProgramId => {
                "The runtime still exposes a placeholder Program ID boundary."
            }
            XxxlRuntimeDeploymentBlocker::LiveRouteDisabled => {
                "Live route activation from process_instruction remains disabled."
            }
            XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled => {
                "SPL Token mint_to CPI execution remains disabled."
            }
            XxxlRuntimeDeploymentBlocker::ProductionGuardianSetUnset => {
                "The production guardian set is not configured or externally documented."
            }
            XxxlRuntimeDeploymentBlocker::ProductionProofLogUnset => {
                "The production proof-log and public audit trail are not configured."
            }
            XxxlRuntimeDeploymentBlocker::ExternalReviewIncomplete => {
                "External review is not complete for deployment activation."
            }
        }
    }
}

pub fn xxxl_runtime_deployment_status() -> XxxlRuntimeDeploymentStatus {
    XXXL_RUNTIME_DEPLOYMENT_STATUS
}

pub fn xxxl_runtime_deployment_status_code() -> &'static str {
    XXXL_RUNTIME_DEPLOYMENT_STATUS.code()
}

pub fn xxxl_runtime_deployment_status_description() -> &'static str {
    XXXL_RUNTIME_DEPLOYMENT_STATUS.description()
}

pub fn xxxl_runtime_deployment_blockers() -> &'static [XxxlRuntimeDeploymentBlocker] {
    &XXXL_RUNTIME_DEPLOYMENT_BLOCKERS
}

pub fn xxxl_runtime_deployment_blocker_reports() -> &'static [XxxlRuntimeDeploymentBlockerReport] {
    &XXXL_RUNTIME_DEPLOYMENT_BLOCKER_REPORTS
}

pub fn xxxl_runtime_deployment_report() -> &'static XxxlRuntimeDeploymentReport {
    &XXXL_RUNTIME_DEPLOYMENT_REPORT
}

pub fn xxxl_runtime_is_deployable() -> bool {
    false
}

pub fn live_route_activation_from_process_instruction_enabled_for_deployment_status() -> bool {
    crate::processor::LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED
}

pub fn spl_cpi_execution_enabled_for_deployment_status() -> bool {
    crate::cpi::spl_mint_to_cpi_execution_enabled()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{XXXL_PROGRAM_ID_PLACEHOLDER, XXXL_RUNTIME_STATUS};

    #[test]
    fn runtime_status_is_scaffold_only_not_deployable() {
        assert_eq!(
            xxxl_runtime_deployment_status(),
            XxxlRuntimeDeploymentStatus::ScaffoldOnlyNotDeployable
        );
        assert!(!xxxl_runtime_is_deployable());
        assert_eq!(XXXL_RUNTIME_STATUS, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
    }

    #[test]
    fn runtime_status_code_and_description_are_human_readable() {
        assert_eq!(
            xxxl_runtime_deployment_status_code(),
            "SCAFFOLD_ONLY_NOT_DEPLOYABLE"
        );
        assert_eq!(
            xxxl_runtime_deployment_status_description(),
            "The XXXL SVM runtime is a scaffold-only build and is not deployable."
        );
    }

    #[test]
    fn runtime_deployment_blockers_are_explicit() {
        let blockers = xxxl_runtime_deployment_blockers();

        assert_eq!(blockers.len(), 6);
        assert!(blockers.contains(&XxxlRuntimeDeploymentBlocker::PlaceholderProgramId));
        assert!(blockers.contains(&XxxlRuntimeDeploymentBlocker::LiveRouteDisabled));
        assert!(blockers.contains(&XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled));
        assert!(blockers.contains(&XxxlRuntimeDeploymentBlocker::ProductionGuardianSetUnset));
        assert!(blockers.contains(&XxxlRuntimeDeploymentBlocker::ProductionProofLogUnset));
        assert!(blockers.contains(&XxxlRuntimeDeploymentBlocker::ExternalReviewIncomplete));
    }

    #[test]
    fn runtime_deployment_blocker_codes_are_stable() {
        let blockers = xxxl_runtime_deployment_blockers();

        assert_eq!(blockers[0].code(), "PLACEHOLDER_PROGRAM_ID");
        assert_eq!(blockers[1].code(), "LIVE_ROUTE_DISABLED");
        assert_eq!(blockers[2].code(), "SPL_CPI_EXECUTION_DISABLED");
        assert_eq!(blockers[3].code(), "PRODUCTION_GUARDIAN_SET_UNSET");
        assert_eq!(blockers[4].code(), "PRODUCTION_PROOF_LOG_UNSET");
        assert_eq!(blockers[5].code(), "EXTERNAL_REVIEW_INCOMPLETE");
    }

    #[test]
    fn runtime_deployment_blocker_descriptions_are_human_readable() {
        for blocker in xxxl_runtime_deployment_blockers() {
            assert!(!blocker.description().is_empty());
            assert!(blocker.description().contains('.') || blocker.description().contains('_'));
        }

        assert_eq!(
            XxxlRuntimeDeploymentBlocker::LiveRouteDisabled.description(),
            "Live route activation from process_instruction remains disabled."
        );
        assert_eq!(
            XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled.description(),
            "SPL Token mint_to CPI execution remains disabled."
        );
    }

    #[test]
    fn runtime_deployment_blocker_reports_match_blocker_methods() {
        let blockers = xxxl_runtime_deployment_blockers();
        let reports = xxxl_runtime_deployment_blocker_reports();

        assert_eq!(reports.len(), blockers.len());

        for (index, report) in reports.iter().enumerate() {
            let blocker = blockers[index];

            assert_eq!(report.blocker, blocker);
            assert_eq!(report.code, blocker.code());
            assert_eq!(report.description, blocker.description());
        }
    }

    #[test]
    fn runtime_deployment_report_is_stable_and_not_deployable() {
        let report = xxxl_runtime_deployment_report();

        assert_eq!(
            report.status,
            XxxlRuntimeDeploymentStatus::ScaffoldOnlyNotDeployable
        );
        assert_eq!(report.status_code, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
        assert_eq!(
            report.status_description,
            "The XXXL SVM runtime is a scaffold-only build and is not deployable."
        );
        assert!(!report.deployable);
        assert_eq!(report.blockers.len(), 6);
        assert_eq!(report.blockers[0].code, "PLACEHOLDER_PROGRAM_ID");
        assert_eq!(report.blockers[1].code, "LIVE_ROUTE_DISABLED");
        assert_eq!(report.blockers[2].code, "SPL_CPI_EXECUTION_DISABLED");
    }

    #[test]
    fn runtime_deployment_report_matches_runtime_flags() {
        let report = xxxl_runtime_deployment_report();

        assert_eq!(report.deployable, xxxl_runtime_is_deployable());
        assert_eq!(report.status_code, xxxl_runtime_deployment_status_code());
        assert_eq!(
            report.status_description,
            xxxl_runtime_deployment_status_description()
        );
        assert!(!live_route_activation_from_process_instruction_enabled_for_deployment_status());
        assert!(!spl_cpi_execution_enabled_for_deployment_status());
    }

    #[test]
    fn runtime_deployment_status_matches_disabled_runtime_flags() {
        assert!(!live_route_activation_from_process_instruction_enabled_for_deployment_status());
        assert!(!spl_cpi_execution_enabled_for_deployment_status());
        assert!(!xxxl_runtime_is_deployable());
    }

    #[test]
    fn runtime_deployment_status_keeps_placeholder_program_id_visible() {
        assert_eq!(
            XXXL_PROGRAM_ID_PLACEHOLDER,
            "XXXLProgram111111111111111111111111111111111"
        );
        assert!(xxxl_runtime_deployment_blockers()
            .contains(&XxxlRuntimeDeploymentBlocker::PlaceholderProgramId));
        assert_eq!(
            XxxlRuntimeDeploymentBlocker::PlaceholderProgramId.description(),
            "The runtime still exposes a placeholder Program ID boundary."
        );
    }
}

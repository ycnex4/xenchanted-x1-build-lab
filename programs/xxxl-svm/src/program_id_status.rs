use crate::{
    deployment_status::{xxxl_runtime_deployment_report_has_blocker, XxxlRuntimeDeploymentBlocker},
    XXXL_PROGRAM_ID_PLACEHOLDER,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlProgramIdReadinessStatus {
    X1TestnetProgramIdBoundaryReviewedActivationBlocked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlProgramIdReadinessReport {
    pub status: XxxlProgramIdReadinessStatus,
    pub status_code: &'static str,
    pub status_description: &'static str,
    pub configured_program_id: &'static str,
    pub deployable_path_ready: bool,
    pub blocker: XxxlRuntimeDeploymentBlocker,
    pub blocker_code: &'static str,
    pub resolution: &'static str,
}

pub const XXXL_PROGRAM_ID_READINESS_STATUS: XxxlProgramIdReadinessStatus =
    XxxlProgramIdReadinessStatus::X1TestnetProgramIdBoundaryReviewedActivationBlocked;

pub const XXXL_PROGRAM_ID_READINESS_REPORT: XxxlProgramIdReadinessReport =
    XxxlProgramIdReadinessReport {
        status: XXXL_PROGRAM_ID_READINESS_STATUS,
        status_code: "X1_TESTNET_PROGRAM_ID_BOUNDARY_REVIEWED_ACTIVATION_BLOCKED",
        status_description: "The X1 testnet Program ID boundary is reviewed at source level; activation remains blocked.",
        configured_program_id: XXXL_PROGRAM_ID_PLACEHOLDER,
        deployable_path_ready: true,
        blocker: XxxlRuntimeDeploymentBlocker::PlaceholderProgramId,
        blocker_code: "PLACEHOLDER_PROGRAM_ID",
        resolution: "Set and review the real Program ID and regenerate all Program-ID-dependent PDA fixtures.",
    };

impl XxxlProgramIdReadinessStatus {
    pub fn code(self) -> &'static str {
        match self {
            XxxlProgramIdReadinessStatus::X1TestnetProgramIdBoundaryReviewedActivationBlocked => {
                "X1_TESTNET_PROGRAM_ID_BOUNDARY_REVIEWED_ACTIVATION_BLOCKED"
            }
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            XxxlProgramIdReadinessStatus::X1TestnetProgramIdBoundaryReviewedActivationBlocked => {
                "The X1 testnet Program ID boundary is reviewed at source level; activation remains blocked."
            }
        }
    }
}

pub fn xxxl_program_id_readiness_status() -> XxxlProgramIdReadinessStatus {
    XXXL_PROGRAM_ID_READINESS_STATUS
}

pub fn xxxl_program_id_readiness_report() -> &'static XxxlProgramIdReadinessReport {
    &XXXL_PROGRAM_ID_READINESS_REPORT
}

pub fn xxxl_program_id_placeholder_boundary_is_active() -> bool {
    true
}

pub fn xxxl_program_id_deployable_path_ready() -> bool {
    xxxl_program_id_readiness_report().deployable_path_ready
}

pub fn xxxl_program_id_placeholder_blocker_is_active_in_deployment_report() -> bool {
    xxxl_runtime_deployment_report_has_blocker(xxxl_program_id_readiness_report().blocker)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_id_readiness_status_is_reviewed_activation_blocked() {
        let status = xxxl_program_id_readiness_status();

        assert_eq!(
            status,
            XxxlProgramIdReadinessStatus::X1TestnetProgramIdBoundaryReviewedActivationBlocked
        );
        assert_eq!(
            status.code(),
            "X1_TESTNET_PROGRAM_ID_BOUNDARY_REVIEWED_ACTIVATION_BLOCKED"
        );
        assert_eq!(
            status.description(),
            "The X1 testnet Program ID boundary is reviewed at source level; activation remains blocked."
        );
    }

    #[test]
    fn program_id_readiness_report_is_reviewed_and_activation_blocked() {
        let report = xxxl_program_id_readiness_report();

        assert_eq!(
            report.status,
            XxxlProgramIdReadinessStatus::X1TestnetProgramIdBoundaryReviewedActivationBlocked
        );
        assert_eq!(
            report.status_code,
            "X1_TESTNET_PROGRAM_ID_BOUNDARY_REVIEWED_ACTIVATION_BLOCKED"
        );
        assert_eq!(
            report.status_description,
            "The X1 testnet Program ID boundary is reviewed at source level; activation remains blocked."
        );
        assert_eq!(report.configured_program_id, XXXL_PROGRAM_ID_PLACEHOLDER);
        assert!(report.deployable_path_ready);
        assert_eq!(
            report.blocker,
            XxxlRuntimeDeploymentBlocker::PlaceholderProgramId
        );
        assert_eq!(report.blocker_code, "PLACEHOLDER_PROGRAM_ID");
    }

    #[test]
    fn program_id_readiness_report_matches_deployment_blocker_metadata() {
        let report = xxxl_program_id_readiness_report();

        assert_eq!(report.blocker.code(), report.blocker_code);
        assert_eq!(report.blocker.resolution(), report.resolution);
    }

    #[test]
    fn program_id_placeholder_blocker_is_active_in_deployment_report() {
        assert!(xxxl_program_id_placeholder_blocker_is_active_in_deployment_report());
    }

    #[test]
    fn program_id_source_ready_but_placeholder_binding_safety_lock_remains_active() {
        assert!(xxxl_program_id_placeholder_boundary_is_active());
        assert!(xxxl_program_id_deployable_path_ready());
    }
}

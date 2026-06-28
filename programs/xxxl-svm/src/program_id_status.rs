use crate::{deployment_status::XxxlRuntimeDeploymentBlocker, XXXL_PROGRAM_ID_PLACEHOLDER};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlProgramIdReadinessStatus {
    Placeholder,
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
    XxxlProgramIdReadinessStatus::Placeholder;

pub const XXXL_PROGRAM_ID_READINESS_REPORT: XxxlProgramIdReadinessReport =
    XxxlProgramIdReadinessReport {
        status: XXXL_PROGRAM_ID_READINESS_STATUS,
        status_code: "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
        status_description: "The XXXL runtime still exposes a placeholder Program ID boundary.",
        configured_program_id: XXXL_PROGRAM_ID_PLACEHOLDER,
        deployable_path_ready: false,
        blocker: XxxlRuntimeDeploymentBlocker::PlaceholderProgramId,
        blocker_code: "PLACEHOLDER_PROGRAM_ID",
        resolution: "Set and review the real Program ID and regenerate all Program-ID-dependent PDA fixtures.",
    };

impl XxxlProgramIdReadinessStatus {
    pub fn code(self) -> &'static str {
        match self {
            XxxlProgramIdReadinessStatus::Placeholder => "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            XxxlProgramIdReadinessStatus::Placeholder => {
                "The XXXL runtime still exposes a placeholder Program ID boundary."
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
    matches!(
        xxxl_program_id_readiness_status(),
        XxxlProgramIdReadinessStatus::Placeholder
    )
}

pub fn xxxl_program_id_deployable_path_ready() -> bool {
    xxxl_program_id_readiness_report().deployable_path_ready
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn program_id_readiness_status_is_placeholder() {
        let status = xxxl_program_id_readiness_status();

        assert_eq!(status, XxxlProgramIdReadinessStatus::Placeholder);
        assert_eq!(status.code(), "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
        assert_eq!(
            status.description(),
            "The XXXL runtime still exposes a placeholder Program ID boundary."
        );
    }

    #[test]
    fn program_id_readiness_report_is_blocking() {
        let report = xxxl_program_id_readiness_report();

        assert_eq!(report.status, XxxlProgramIdReadinessStatus::Placeholder);
        assert_eq!(report.status_code, "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
        assert_eq!(
            report.status_description,
            "The XXXL runtime still exposes a placeholder Program ID boundary."
        );
        assert_eq!(report.configured_program_id, XXXL_PROGRAM_ID_PLACEHOLDER);
        assert!(!report.deployable_path_ready);
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
    fn program_id_placeholder_boundary_remains_active() {
        assert!(xxxl_program_id_placeholder_boundary_is_active());
        assert!(!xxxl_program_id_deployable_path_ready());
    }
}

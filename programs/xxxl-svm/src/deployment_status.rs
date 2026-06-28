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

pub fn xxxl_runtime_deployment_status() -> XxxlRuntimeDeploymentStatus {
    XXXL_RUNTIME_DEPLOYMENT_STATUS
}

pub fn xxxl_runtime_deployment_blockers() -> &'static [XxxlRuntimeDeploymentBlocker] {
    &XXXL_RUNTIME_DEPLOYMENT_BLOCKERS
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
    }
}

use crate::{
    deployment_status::{
        live_route_activation_from_process_instruction_enabled_for_deployment_status,
        spl_cpi_execution_enabled_for_deployment_status, xxxl_runtime_is_deployable,
        xxxl_runtime_predeploy_gate_allows_deploy,
    },
    program_id_status::{
        xxxl_program_id_placeholder_blocker_is_active_in_deployment_report,
        xxxl_program_id_placeholder_boundary_is_active,
    },
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlRuntimeSafetyInvariantSummary {
    pub runtime_deployable: bool,
    pub predeploy_gate_allows_deploy: bool,
    pub program_id_placeholder_boundary_active: bool,
    pub program_id_placeholder_blocker_active_in_deployment_report: bool,
    pub live_route_activation_enabled: bool,
    pub spl_cpi_execution_enabled: bool,
}

impl XxxlRuntimeSafetyInvariantSummary {
    pub fn blocking_invariants_hold(self) -> bool {
        !self.runtime_deployable
            && !self.predeploy_gate_allows_deploy
            && self.program_id_placeholder_boundary_active
            && self.program_id_placeholder_blocker_active_in_deployment_report
            && !self.live_route_activation_enabled
            && !self.spl_cpi_execution_enabled
    }
}

pub fn xxxl_runtime_safety_invariant_summary() -> XxxlRuntimeSafetyInvariantSummary {
    XxxlRuntimeSafetyInvariantSummary {
        runtime_deployable: xxxl_runtime_is_deployable(),
        predeploy_gate_allows_deploy: xxxl_runtime_predeploy_gate_allows_deploy(),
        program_id_placeholder_boundary_active: xxxl_program_id_placeholder_boundary_is_active(),
        program_id_placeholder_blocker_active_in_deployment_report:
            xxxl_program_id_placeholder_blocker_is_active_in_deployment_report(),
        live_route_activation_enabled:
            live_route_activation_from_process_instruction_enabled_for_deployment_status(),
        spl_cpi_execution_enabled: spl_cpi_execution_enabled_for_deployment_status(),
    }
}

pub fn xxxl_runtime_blocking_safety_invariants_hold() -> bool {
    xxxl_runtime_safety_invariant_summary().blocking_invariants_hold()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlPredeployGateSafetyConsistencyReport {
    pub blocking_safety_invariants_hold: bool,
    pub predeploy_gate_allows_deploy: bool,
    pub consistent: bool,
}

pub fn xxxl_predeploy_gate_safety_consistency_report() -> XxxlPredeployGateSafetyConsistencyReport {
    let summary = xxxl_runtime_safety_invariant_summary();

    XxxlPredeployGateSafetyConsistencyReport {
        blocking_safety_invariants_hold: summary.blocking_invariants_hold(),
        predeploy_gate_allows_deploy: summary.predeploy_gate_allows_deploy,
        consistent: !(summary.blocking_invariants_hold() && summary.predeploy_gate_allows_deploy),
    }
}

pub fn xxxl_predeploy_gate_is_consistent_with_safety_invariants() -> bool {
    xxxl_predeploy_gate_safety_consistency_report().consistent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlLiveRouteSafetyConsistencyReport {
    pub blocking_safety_invariants_hold: bool,
    pub live_route_activation_enabled: bool,
    pub consistent: bool,
}

pub fn xxxl_live_route_safety_consistency_report() -> XxxlLiveRouteSafetyConsistencyReport {
    let summary = xxxl_runtime_safety_invariant_summary();

    XxxlLiveRouteSafetyConsistencyReport {
        blocking_safety_invariants_hold: summary.blocking_invariants_hold(),
        live_route_activation_enabled: summary.live_route_activation_enabled,
        consistent: !(summary.blocking_invariants_hold() && summary.live_route_activation_enabled),
    }
}

pub fn xxxl_live_route_is_consistent_with_safety_invariants() -> bool {
    xxxl_live_route_safety_consistency_report().consistent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlSplCpiSafetyConsistencyReport {
    pub blocking_safety_invariants_hold: bool,
    pub spl_cpi_execution_enabled: bool,
    pub consistent: bool,
}

pub fn xxxl_spl_cpi_safety_consistency_report() -> XxxlSplCpiSafetyConsistencyReport {
    let summary = xxxl_runtime_safety_invariant_summary();

    XxxlSplCpiSafetyConsistencyReport {
        blocking_safety_invariants_hold: summary.blocking_invariants_hold(),
        spl_cpi_execution_enabled: summary.spl_cpi_execution_enabled,
        consistent: !(summary.blocking_invariants_hold() && summary.spl_cpi_execution_enabled),
    }
}

pub fn xxxl_spl_cpi_is_consistent_with_safety_invariants() -> bool {
    xxxl_spl_cpi_safety_consistency_report().consistent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_safety_invariant_summary_matches_current_blocking_state() {
        let summary = xxxl_runtime_safety_invariant_summary();

        assert!(!summary.runtime_deployable);
        assert!(!summary.predeploy_gate_allows_deploy);
        assert!(summary.program_id_placeholder_boundary_active);
        assert!(summary.program_id_placeholder_blocker_active_in_deployment_report);
        assert!(!summary.live_route_activation_enabled);
        assert!(!summary.spl_cpi_execution_enabled);
    }

    #[test]
    fn runtime_blocking_safety_invariants_hold_for_current_scaffold() {
        let summary = xxxl_runtime_safety_invariant_summary();

        assert!(summary.blocking_invariants_hold());
        assert!(xxxl_runtime_blocking_safety_invariants_hold());
    }
    #[test]
    fn predeploy_gate_safety_consistency_report_is_blocked_and_consistent() {
        let report = xxxl_predeploy_gate_safety_consistency_report();

        assert!(report.blocking_safety_invariants_hold);
        assert!(!report.predeploy_gate_allows_deploy);
        assert!(report.consistent);
    }

    #[test]
    fn predeploy_gate_is_consistent_with_current_safety_invariants() {
        assert!(xxxl_predeploy_gate_is_consistent_with_safety_invariants());
    }
    #[test]
    fn live_route_safety_consistency_report_is_disabled_and_consistent() {
        let report = xxxl_live_route_safety_consistency_report();

        assert!(report.blocking_safety_invariants_hold);
        assert!(!report.live_route_activation_enabled);
        assert!(report.consistent);
    }

    #[test]
    fn live_route_is_consistent_with_current_safety_invariants() {
        assert!(xxxl_live_route_is_consistent_with_safety_invariants());
    }
    #[test]
    fn spl_cpi_safety_consistency_report_is_disabled_and_consistent() {
        let report = xxxl_spl_cpi_safety_consistency_report();

        assert!(report.blocking_safety_invariants_hold);
        assert!(!report.spl_cpi_execution_enabled);
        assert!(report.consistent);
    }

    #[test]
    fn spl_cpi_is_consistent_with_current_safety_invariants() {
        assert!(xxxl_spl_cpi_is_consistent_with_safety_invariants());
    }
}

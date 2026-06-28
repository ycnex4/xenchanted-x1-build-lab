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
}

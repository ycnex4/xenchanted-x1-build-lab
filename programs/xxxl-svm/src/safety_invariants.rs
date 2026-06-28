use crate::deployment_status::{
    xxxl_runtime_deployment_report_has_blocker, XxxlRuntimeDeploymentBlocker,
};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlActivationSafetyConsistencySummary {
    pub predeploy_gate_consistent: bool,
    pub live_route_consistent: bool,
    pub spl_cpi_consistent: bool,
    pub all_activation_gates_consistent: bool,
}

pub fn xxxl_activation_safety_consistency_summary() -> XxxlActivationSafetyConsistencySummary {
    let predeploy_gate_consistent = xxxl_predeploy_gate_is_consistent_with_safety_invariants();
    let live_route_consistent = xxxl_live_route_is_consistent_with_safety_invariants();
    let spl_cpi_consistent = xxxl_spl_cpi_is_consistent_with_safety_invariants();

    XxxlActivationSafetyConsistencySummary {
        predeploy_gate_consistent,
        live_route_consistent,
        spl_cpi_consistent,
        all_activation_gates_consistent: predeploy_gate_consistent
            && live_route_consistent
            && spl_cpi_consistent,
    }
}

pub fn xxxl_all_activation_gates_are_consistent_with_safety_invariants() -> bool {
    xxxl_activation_safety_consistency_summary().all_activation_gates_consistent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlRuntimeSafetyLockSummary {
    pub blocking_safety_invariants_hold: bool,
    pub activation_gates_consistent: bool,
    pub runtime_deployable: bool,
    pub runtime_locked: bool,
}

pub fn xxxl_runtime_safety_lock_summary() -> XxxlRuntimeSafetyLockSummary {
    let safety_summary = xxxl_runtime_safety_invariant_summary();
    let blocking_safety_invariants_hold = safety_summary.blocking_invariants_hold();
    let activation_gates_consistent =
        xxxl_all_activation_gates_are_consistent_with_safety_invariants();

    XxxlRuntimeSafetyLockSummary {
        blocking_safety_invariants_hold,
        activation_gates_consistent,
        runtime_deployable: safety_summary.runtime_deployable,
        runtime_locked: blocking_safety_invariants_hold
            && activation_gates_consistent
            && !safety_summary.runtime_deployable,
    }
}

pub fn xxxl_runtime_safety_lock_is_active() -> bool {
    xxxl_runtime_safety_lock_summary().runtime_locked
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlSafetyLockDeploymentGateConsistencyReport {
    pub runtime_safety_lock_active: bool,
    pub predeploy_gate_allows_deploy: bool,
    pub consistent: bool,
}

pub fn xxxl_safety_lock_deployment_gate_consistency_report(
) -> XxxlSafetyLockDeploymentGateConsistencyReport {
    let lock_summary = xxxl_runtime_safety_lock_summary();
    let predeploy_report = xxxl_predeploy_gate_safety_consistency_report();

    XxxlSafetyLockDeploymentGateConsistencyReport {
        runtime_safety_lock_active: lock_summary.runtime_locked,
        predeploy_gate_allows_deploy: predeploy_report.predeploy_gate_allows_deploy,
        consistent: !(lock_summary.runtime_locked && predeploy_report.predeploy_gate_allows_deploy),
    }
}

pub fn xxxl_safety_lock_is_consistent_with_deployment_gate() -> bool {
    xxxl_safety_lock_deployment_gate_consistency_report().consistent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlRuntimeSafetyLockEvidenceSummary {
    pub runtime_safety_lock_active: bool,
    pub program_id_placeholder_boundary_active: bool,
    pub placeholder_blocker_active_in_deployment_report: bool,
    pub live_route_disabled: bool,
    pub spl_cpi_execution_disabled: bool,
    pub predeploy_gate_blocked: bool,
    pub evidence_complete: bool,
}

pub fn xxxl_runtime_safety_lock_evidence_summary() -> XxxlRuntimeSafetyLockEvidenceSummary {
    let safety_summary = xxxl_runtime_safety_invariant_summary();
    let lock_summary = xxxl_runtime_safety_lock_summary();
    let predeploy_report = xxxl_predeploy_gate_safety_consistency_report();

    let live_route_disabled = !safety_summary.live_route_activation_enabled;
    let spl_cpi_execution_disabled = !safety_summary.spl_cpi_execution_enabled;
    let predeploy_gate_blocked = !predeploy_report.predeploy_gate_allows_deploy;

    XxxlRuntimeSafetyLockEvidenceSummary {
        runtime_safety_lock_active: lock_summary.runtime_locked,
        program_id_placeholder_boundary_active: safety_summary
            .program_id_placeholder_boundary_active,
        placeholder_blocker_active_in_deployment_report: safety_summary
            .program_id_placeholder_blocker_active_in_deployment_report,
        live_route_disabled,
        spl_cpi_execution_disabled,
        predeploy_gate_blocked,
        evidence_complete: lock_summary.runtime_locked
            && safety_summary.program_id_placeholder_boundary_active
            && safety_summary.program_id_placeholder_blocker_active_in_deployment_report
            && live_route_disabled
            && spl_cpi_execution_disabled
            && predeploy_gate_blocked,
    }
}

pub fn xxxl_runtime_safety_lock_evidence_is_complete() -> bool {
    xxxl_runtime_safety_lock_evidence_summary().evidence_complete
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlDeploymentBlockerEvidenceConsistencyReport {
    pub safety_lock_evidence_complete: bool,
    pub placeholder_program_id_blocker_present: bool,
    pub live_route_disabled_blocker_present: bool,
    pub spl_cpi_execution_disabled_blocker_present: bool,
    pub evidence_consistent: bool,
}

pub fn xxxl_deployment_blocker_evidence_consistency_report(
) -> XxxlDeploymentBlockerEvidenceConsistencyReport {
    let safety_lock_evidence_complete = xxxl_runtime_safety_lock_evidence_is_complete();
    let placeholder_program_id_blocker_present = xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::PlaceholderProgramId,
    );
    let live_route_disabled_blocker_present =
        xxxl_runtime_deployment_report_has_blocker(XxxlRuntimeDeploymentBlocker::LiveRouteDisabled);
    let spl_cpi_execution_disabled_blocker_present = xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled,
    );

    XxxlDeploymentBlockerEvidenceConsistencyReport {
        safety_lock_evidence_complete,
        placeholder_program_id_blocker_present,
        live_route_disabled_blocker_present,
        spl_cpi_execution_disabled_blocker_present,
        evidence_consistent: safety_lock_evidence_complete
            && placeholder_program_id_blocker_present
            && live_route_disabled_blocker_present
            && spl_cpi_execution_disabled_blocker_present,
    }
}

pub fn xxxl_deployment_blocker_evidence_is_consistent() -> bool {
    xxxl_deployment_blocker_evidence_consistency_report().evidence_consistent
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlRuntimeSafetyUnlockCriteriaSummary {
    pub runtime_safety_lock_active: bool,
    pub real_program_id_selected: bool,
    pub production_pda_fixtures_verified: bool,
    pub deployment_blockers_cleared: bool,
    pub live_route_review_complete: bool,
    pub spl_cpi_review_complete: bool,
    pub external_review_complete: bool,
    pub unlock_ready: bool,
    pub unlock_blocked: bool,
}

pub fn xxxl_runtime_safety_unlock_criteria_summary() -> XxxlRuntimeSafetyUnlockCriteriaSummary {
    let runtime_safety_lock_active = xxxl_runtime_safety_lock_is_active();

    let real_program_id_selected = false;
    let production_pda_fixtures_verified = false;
    let live_route_review_complete = false;
    let spl_cpi_review_complete = false;
    let external_review_complete = false;

    let deployment_blockers_cleared = !xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::PlaceholderProgramId,
    ) && !xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::LiveRouteDisabled,
    ) && !xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::SplCpiExecutionDisabled,
    ) && !xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::ProductionGuardianSetUnset,
    ) && !xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::ProductionProofLogUnset,
    ) && !xxxl_runtime_deployment_report_has_blocker(
        XxxlRuntimeDeploymentBlocker::ExternalReviewIncomplete,
    );

    let unlock_ready = real_program_id_selected
        && production_pda_fixtures_verified
        && deployment_blockers_cleared
        && live_route_review_complete
        && spl_cpi_review_complete
        && external_review_complete;

    XxxlRuntimeSafetyUnlockCriteriaSummary {
        runtime_safety_lock_active,
        real_program_id_selected,
        production_pda_fixtures_verified,
        deployment_blockers_cleared,
        live_route_review_complete,
        spl_cpi_review_complete,
        external_review_complete,
        unlock_ready,
        unlock_blocked: runtime_safety_lock_active && !unlock_ready,
    }
}

pub fn xxxl_runtime_safety_unlock_is_ready() -> bool {
    xxxl_runtime_safety_unlock_criteria_summary().unlock_ready
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlRuntimeSafetyReleaseDecisionReport {
    pub runtime_safety_lock_active: bool,
    pub unlock_ready: bool,
    pub unlock_criteria_not_ready: bool,
    pub deployment_blocker_evidence_consistent: bool,
    pub release_allowed: bool,
    pub release_blocked: bool,
    pub primary_blocker_code: &'static str,
}

pub fn xxxl_runtime_safety_release_decision_report() -> XxxlRuntimeSafetyReleaseDecisionReport {
    let unlock_summary = xxxl_runtime_safety_unlock_criteria_summary();
    let runtime_safety_lock_active = unlock_summary.runtime_safety_lock_active;
    let unlock_ready = unlock_summary.unlock_ready;
    let unlock_criteria_not_ready = !unlock_ready;
    let deployment_blocker_evidence_consistent = xxxl_deployment_blocker_evidence_is_consistent();

    let release_allowed = !runtime_safety_lock_active && unlock_ready;
    let primary_blocker_code = if runtime_safety_lock_active {
        "RUNTIME_SAFETY_LOCK_ACTIVE"
    } else if unlock_criteria_not_ready {
        "UNLOCK_CRITERIA_NOT_READY"
    } else {
        "NONE"
    };

    XxxlRuntimeSafetyReleaseDecisionReport {
        runtime_safety_lock_active,
        unlock_ready,
        unlock_criteria_not_ready,
        deployment_blocker_evidence_consistent,
        release_allowed,
        release_blocked: !release_allowed,
        primary_blocker_code,
    }
}

pub fn xxxl_runtime_safety_release_is_allowed() -> bool {
    xxxl_runtime_safety_release_decision_report().release_allowed
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
    #[test]
    fn activation_safety_consistency_summary_collects_all_current_gates() {
        let summary = xxxl_activation_safety_consistency_summary();

        assert!(summary.predeploy_gate_consistent);
        assert!(summary.live_route_consistent);
        assert!(summary.spl_cpi_consistent);
        assert!(summary.all_activation_gates_consistent);
    }

    #[test]
    fn all_activation_gates_are_consistent_with_current_safety_invariants() {
        assert!(xxxl_all_activation_gates_are_consistent_with_safety_invariants());
    }
    #[test]
    fn runtime_safety_lock_summary_collects_current_lock_state() {
        let summary = xxxl_runtime_safety_lock_summary();

        assert!(summary.blocking_safety_invariants_hold);
        assert!(summary.activation_gates_consistent);
        assert!(!summary.runtime_deployable);
        assert!(summary.runtime_locked);
    }

    #[test]
    fn runtime_safety_lock_is_active_for_current_scaffold() {
        assert!(xxxl_runtime_safety_lock_is_active());
    }
    #[test]
    fn safety_lock_deployment_gate_consistency_report_is_locked_and_blocked() {
        let report = xxxl_safety_lock_deployment_gate_consistency_report();

        assert!(report.runtime_safety_lock_active);
        assert!(!report.predeploy_gate_allows_deploy);
        assert!(report.consistent);
    }

    #[test]
    fn safety_lock_is_consistent_with_current_deployment_gate() {
        assert!(xxxl_safety_lock_is_consistent_with_deployment_gate());
    }
    #[test]
    fn runtime_safety_lock_evidence_summary_collects_current_evidence() {
        let summary = xxxl_runtime_safety_lock_evidence_summary();

        assert!(summary.runtime_safety_lock_active);
        assert!(summary.program_id_placeholder_boundary_active);
        assert!(summary.placeholder_blocker_active_in_deployment_report);
        assert!(summary.live_route_disabled);
        assert!(summary.spl_cpi_execution_disabled);
        assert!(summary.predeploy_gate_blocked);
        assert!(summary.evidence_complete);
    }

    #[test]
    fn runtime_safety_lock_evidence_is_complete_for_current_scaffold() {
        assert!(xxxl_runtime_safety_lock_evidence_is_complete());
    }
    #[test]
    fn deployment_blocker_evidence_consistency_report_matches_current_lock_evidence() {
        let report = xxxl_deployment_blocker_evidence_consistency_report();

        assert!(report.safety_lock_evidence_complete);
        assert!(report.placeholder_program_id_blocker_present);
        assert!(report.live_route_disabled_blocker_present);
        assert!(report.spl_cpi_execution_disabled_blocker_present);
        assert!(report.evidence_consistent);
    }

    #[test]
    fn deployment_blocker_evidence_is_consistent_for_current_scaffold() {
        assert!(xxxl_deployment_blocker_evidence_is_consistent());
    }
    #[test]
    fn runtime_safety_unlock_criteria_summary_blocks_current_scaffold() {
        let summary = xxxl_runtime_safety_unlock_criteria_summary();

        assert!(summary.runtime_safety_lock_active);
        assert!(!summary.real_program_id_selected);
        assert!(!summary.production_pda_fixtures_verified);
        assert!(!summary.deployment_blockers_cleared);
        assert!(!summary.live_route_review_complete);
        assert!(!summary.spl_cpi_review_complete);
        assert!(!summary.external_review_complete);
        assert!(!summary.unlock_ready);
        assert!(summary.unlock_blocked);
    }

    #[test]
    fn runtime_safety_unlock_is_not_ready_for_current_scaffold() {
        assert!(!xxxl_runtime_safety_unlock_is_ready());
    }
    #[test]
    fn runtime_safety_release_decision_report_blocks_current_scaffold() {
        let report = xxxl_runtime_safety_release_decision_report();

        assert!(report.runtime_safety_lock_active);
        assert!(!report.unlock_ready);
        assert!(report.unlock_criteria_not_ready);
        assert!(report.deployment_blocker_evidence_consistent);
        assert!(!report.release_allowed);
        assert!(report.release_blocked);
        assert_eq!(report.primary_blocker_code, "RUNTIME_SAFETY_LOCK_ACTIVE");
    }

    #[test]
    fn runtime_safety_release_is_not_allowed_for_current_scaffold() {
        assert!(!xxxl_runtime_safety_release_is_allowed());
    }
}

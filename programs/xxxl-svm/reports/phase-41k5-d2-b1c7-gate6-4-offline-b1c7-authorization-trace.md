# Phase 41K.5 D2 / B1C7 Gate 6.4 Offline B1C7 Authorization Trace

Date UTC: 2026-08-16T22:17:04Z

## Scope

This report records a local/offline B1C7 authorization trace using the captured Gate 6.4 transaction package and captured testnet guardian_set account data.

No transaction was sent. No simulation was performed. No mint, replay, rollback, deploy, or upgrade was executed.

## Safety Flags

- transactions_executed: `false`
- live_broadcast: `false`
- guarded_mint_executed: `false`
- replay_executed: `false`
- rollback_executed: `false`
- raw_signatures_committed: `false`
- signed_transaction_committed: `false`

## Repository State

- Branch: `audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z`
- Head commit before trace: `dd951c62c0400c0447393f7b07d6ac35eff80870`
- Git status before trace: `0` changed files

## Trace Inputs

- Previous Gate 6.4 evidence:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-live-guarded-mint-retry-autopayer-20260816T211352Z
```

- State scan evidence:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-root-cause-triage-state-scan-envfix2-20260816T220346Z
```

## Offline Trace Result

- trace_code: `0`
- connected_status: `ParsedPriorEd25519Evidence`
- payload_binding_status: `Bound`
- guardian_set_status: `GuardianSetAccountDataRejected`
- membership_status: `Rejected`
- quorum_status: `SKIPPED_NO_MEMBERSHIP`
- authorization_status: `Rejected`
- authorization_rejection_kind: `Some(GuardianSetNotDecoded)`
- authorization_enabled: `false`
- auth_hash_match: `true`
- parsed_evidence_count: `0`
- bound_evidence_count: `0`
- membership_validated_signer_count: `0`
- unique_guardian_count: `0`
- authorization_quorum_met: `false`

## Decision

Offline B1C7 authorization trace result: `OFFLINE_B1C7_REJECTED_BOUNDARY_IDENTIFIED`

If this trace is Authorized while live simulation returned Custom(1), the remaining likely root cause is the actual runtime instructions sysvar path: `load_current_index_checked` / `load_instruction_at_checked` / runtime sysvar representation.

If this trace is Rejected, the reported B1C sub-boundary identifies the local rejection source.

## Trace Details

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-offline-b1c7-authorization-trace-20260816T221643Z/results/offline_b1c7_authorization_trace.txt
```

## Non-Claims

This report does not claim:

- live guarded mint execution;
- replay approval;
- replay execution;
- second mint approval;
- production readiness;
- rollback execution.

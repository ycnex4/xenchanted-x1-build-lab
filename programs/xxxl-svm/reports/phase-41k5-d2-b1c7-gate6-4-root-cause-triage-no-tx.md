# Phase 41K.5 D2 / B1C7 Gate 6.4 Root-Cause Triage — No TX

Date UTC: 2026-08-16T22:03:50Z

## Scope

This report records local/read-only root-cause triage after Gate 6.4 simulation returned `InstructionError [3, Custom(1)]`.

No transaction was sent. No mint was executed. No replay, rollback, deploy, or upgrade was executed.

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
- Head commit before triage: `4915d99f4fbb75ce30e8b25ceb946115f2a738b8`
- Git status before triage: `0` changed files

## Inputs

- Previous Gate 6.4 simulation evidence:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-live-guarded-mint-retry-autopayer-20260816T211352Z
```

- Blocked diagnostics evidence:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-blocked-simulation-diagnostics-20260816T211917Z
```

## Prior Confirmed Checks

- PDA derivations: `PASS`
- B1 V3 account flags: `PASS`

## Guardian Set Scan

- guardian_set_owner: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- guardian_set_data_len: `320`
- guardian_set_id_present: `true`
- all_five_guardians_present_in_guardian_set_raw: `true`

## State Account Scan

- mint_state_contains_target_mint: `true`
- mint_state_contains_gateway_authority: `true`
- gateway_config_contains_route_id: `true`
- gateway_config_contains_guardian_set_id: `true`
- gateway_config_contains_target_mint: `true`
- recipient_balance_owner_match: `true`
- recipient_balance_mint_match: `true`
- recipient_balance_balance_zero: `true`
- target_mint_supply_zero: `true`
- recipient_ata_amount_zero: `true`
- root_cause_external_state_scan_ok: `true`

## Decode Details

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-root-cause-triage-state-scan-envfix2-20260816T220346Z/results/root_cause_state_scan.txt
```

## Decision

Gate 6.4 remains blocked at simulation.

If this report is PASS, the likely remaining root cause is inside the B1C7 on-chain authorization path, especially checked prior instruction loading, Ed25519 evidence parsing/connection, payload binding, membership validation, or quorum counting.

## Non-Claims

This report does not claim:

- live guarded mint execution;
- replay approval;
- replay execution;
- second mint approval;
- production readiness;
- rollback execution.

# Phase 41K.5 D2 / B1C7 Gate 6.4 Guardian Set Strict Offset Dump

Date UTC: 2026-08-16T22:19:33Z

## Scope

This report records a local/no-tx strict offset dump of the guardian_set account after offline B1C7 authorization trace rejected with `GuardianSetAccountDataRejected / InactiveOrDeprecatedGuardianSet`.

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
- Head commit before dump: `ac69d8fd8832cd0df14c6c10eb674128adbc95c8`
- Git status before dump: `0` changed files

## Inputs

- State scan evidence:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-root-cause-triage-state-scan-envfix2-20260816T220346Z
```

- Offline B1C7 trace evidence:

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-offline-b1c7-authorization-trace-20260816T221643Z
```

## Strict Offset Result

- dump_code: `0`
- active_status_u8_at_10: `0`
- threshold_u16_at_12: `3`
- guardian_count_u8_at_14: `5`
- active_status_ok: `false`
- threshold_ok: `true`
- guardian_count_ok: `true`
- guardian_set_id_ok: `true`
- strict_guardian_set_decode_expected_result: `REJECT`

## Dump Details

```text
/home/sergey/xenchanted-stage20-activation-evidence-c332814/d2-b1c7-gate6-4-guardian-set-strict-offset-dump-20260816T221931Z/results/guardian_set_offset_dump.txt
```

## Decision

Gate 6.4 remains blocked.

If `active_status_u8_at_10 != 1`, the root cause is a guardian_set account activation/layout mismatch rather than an Ed25519/sysvar/payload issue.

## Non-Claims

This report does not claim:

- live guarded mint execution;
- replay approval;
- replay execution;
- second mint approval;
- production readiness;
- rollback execution.

# XXXL Phase 41K.3 — Processed-Registry Loading Plan Amendment 2

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Applies to:

- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`

## Status

This amendment resolves the two blocking issues found by Theo and Demon review of the amended 41K.3 plan.

Previous verdicts:

- Theo: REQUIRES FIXES
- Demon: REQUIRES FIXES

Blocking issues:

1. `consumed == false` lifecycle was still undecided.
2. 41J list-based processed registry view vs per-event PDA point lookup reconciliation was still undecided.

This Amendment 2 makes both decisions explicit.

Amendment 2 supersedes conflicting wording in the base plan and Amendment 1.

## Decision 1 — Account Presence And Missing PDA Semantics

41K.3 requires the expected processed-event PDA account to be supplied in the instruction account list.

A fully missing `AccountInfo` / `None` is a loader rejection.

Reason:

The program cannot verify the PDA address, account lifecycle state, or prepare the later atomic mark/create flow if the expected processed-event account is not supplied.

Therefore:

- missing `AccountInfo` -> reject;
- supplied account with wrong key -> reject;
- supplied expected PDA in the accepted uninitialized runtime representation -> unprocessed / eligible;
- supplied expected PDA already initialized as consumed -> already processed / replay reject.

This reconciles the two meanings of "missing PDA":

- missing from instruction accounts -> reject;
- supplied expected PDA but runtime-uninitialized / not-yet-created representation -> unprocessed.

## Decision 2 — Uninitialized Expected PDA Means Unprocessed

An expected processed-event PDA may be supplied before it is initialized.

This state means:

`unprocessed / eligible for future atomic mark`

The loader may classify the supplied expected PDA as uninitialized only if the implementation proves a strict runtime representation, for example:

- account key equals expected processed-event PDA;
- account is not executable;
- account is not a signer;
- account data is empty, or otherwise matches the chosen Solana representation for not-yet-initialized account state;
- account owner / lamports / data conditions match the chosen Solana representation.

The exact representation must be implemented and tested in code.

No account data is trusted in the uninitialized state.

The uninitialized state must not be based on arbitrary caller data.

## Decision 3 — Initialized `consumed == false` Is Invalid

41K.3 does not support a persisted initialized processed-event PDA with `consumed == false`.

Accepted lifecycle:

### State A — Expected PDA supplied, uninitialized

Meaning:

- event is not processed;
- replay eligibility may pass;
- later 41K.4 / 41K.5 may create / initialize / mark atomically.

### State B — Expected PDA supplied, initialized and `consumed == true`

Meaning:

- event is already processed;
- replay eligibility must reject.

### State C — Expected PDA supplied, initialized and `consumed == false`

Decision:

Reject as invalid lifecycle state.

Suggested rejection name for code:

`InitializedButUnconsumedProcessedEvent`

Rationale:

A durable initialized-but-not-consumed processed-event PDA creates an ambiguous lifecycle state and unnecessary pre-initialization surface.

The live mark path should create / initialize / set consumed state atomically, rather than leaving a durable initialized `consumed == false` state.

## Decision 4 — Existing `mark_processed_event_consumed(...)` Is Not Accepted As Live Semantics

The current helper:

`mark_processed_event_consumed(...)`

exists in runtime state code, but it is not part of 41K.3 and must not be treated as accepted live mark semantics.

Before 41K.4 code acceptance, the mark path must be reviewed against the lifecycle chosen here.

If the helper assumes a pre-existing initialized `consumed == false` account, it must be replaced, constrained, or wrapped by a create/init/mark atomic path.

## Decision 5 — Writable Account Semantics

41K.3 remains read-only in behavior.

However, 41K.3 must not reject an account solely because it is writable.

Reason:

The later 41K.4 / 41K.5 atomic mark/create flow may require the same expected processed-event PDA to be writable in the same transaction.

Therefore:

- writable observed: allowed;
- mutation: disabled;
- replay write: disabled;
- processed event marking: disabled;
- CPI: disabled;
- SPL mint: disabled;
- handler/live route: disabled.

## Decision 6 — 41J Reconciliation Uses Option A Adapter

41K.3 chooses Option A from Amendment 1.

Do not modify already accepted 41J replay-protection semantics during the 41K.3 loading plan.

Instead, add a future type-enforced, internally constructible adapter from the 41K.3 point-lookup result to the existing 41J list-based `AuthoritativeProcessedRegistryViewRef`.

Adapter semantics:

- unprocessed / uninitialized expected PDA -> empty processed list;
- processed / initialized `consumed == true` -> one-item processed list containing `canonical_event_key`;
- invalid lifecycle state -> rejection before adapter construction.

Rationale:

This preserves the already reviewed 41J boundary and avoids reopening 41J during 41K.3.

The adapter must be internal and type-enforced.

It must not be left to handler discipline.

It must preserve all accepted 41J invariants:

- no caller-supplied replay key;
- no caller-supplied decoded payload;
- canonical_event_key derived internally from raw payload;
- caller-supplied registry rejected;
- unauthenticated registry rejected;
- no write in eligibility phase;
- processed marking intent only until the atomic mark phase.

Option B, a future point-lookup-specific 41J runtime interface, may be reconsidered later as an optimization or cleanup, but it is not chosen for 41K.3 plan acceptance.

## Decision 7 — Processed-Event Identity Fields

41K.3 initialized-account validation must check:

- expected PDA derived from `canonical_event_key`;
- stored `canonical_event_key`;
- stored `route_id`;
- stored `recipient`.

This is sufficient for 41K.3 loading and replay classification.

Forward note for 41K.4:

The atomic mark path must also bind the marked amount / mint amount to the quorum-authorized payload.

A processed-event account must not be markable with an amount different from the internally decoded and authorized payload amount.

## SVM Account Lifecycle Notes

Replay-protection PDAs are durable one-way protocol records.

The protocol should not support closing and recreating processed-event PDAs as a normal lifecycle.

For 41K.3:

- supplied expected PDA in accepted uninitialized representation -> unprocessed;
- initialized XXXL-owned account with correct discriminator/version and `consumed == true` -> processed;
- initialized XXXL-owned account with correct discriminator/version and `consumed == false` -> invalid;
- initialized account with zero/wrong discriminator -> invalid/corrupted;
- wrong owner for initialized account -> reject before data trust.

Rent / close / recreate policy is carried forward to 41K.4 / 41K.5:

- created processed-event PDAs should be rent-exempt;
- replay-protection accounts should not have a close path;
- a close/recreate transition must not be accepted as valid replay state;
- production proof logging must preserve processed-event auditability.

## Active Deployment Blockers Remain

41K.3 does not remove any deployment blockers.

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`;
- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`;
- `PRODUCTION_GUARDIAN_SET_UNSET`;
- `PRODUCTION_PROOF_LOG_UNSET`;
- `SPL_CPI_EXECUTION_DISABLED`;
- `LIVE_ROUTE_DISABLED`;
- `EXTERNAL_REVIEW_INCOMPLETE`.

Relevant blocker mapping for 41K.3:

- `PRODUCTION_PROGRAM_ID_UNSET` blocks final production PDA derivation assumptions;
- `PRODUCTION_PROOF_LOG_UNSET` blocks production-grade processed-event / replay audit trail assumptions.

## Updated Acceptance Gate

The 41K.3 plan is acceptable only if reviewers agree that:

- missing `AccountInfo` is rejected;
- supplied expected PDA in accepted uninitialized runtime representation means unprocessed / eligible;
- supplied wrong PDA is rejected;
- initialized `consumed == true` means already processed;
- initialized `consumed == false` is rejected as invalid lifecycle state;
- writable account is allowed but never mutated in 41K.3;
- 41J reconciliation uses Option A adapter to existing 41J list-based interface;
- future adapter is type-enforced and internal;
- replay write remains disabled;
- processed event marking remains disabled;
- account mutation remains disabled;
- atomic check-mark-mint remains deferred to 41K.4 / 41K.5.

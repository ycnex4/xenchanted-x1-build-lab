# XXXL Phase 41K.3 — Processed-Registry Loading Plan Amendment 1

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Applies to:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`

## Status

This amendment records pre-review guidance for the 41K.3 processed-registry PDA loading plan.

It supersedes any earlier wording in the base plan that implied:

- missing processed-event PDA should always reject;
- writable processed-event account should always reject;
- existing 41J list-based processed registry view can be used with per-event PDA lookup without an explicit adapter decision.

## Amendment A — Missing PDA Semantics

Missing or uninitialized expected processed-event PDA should represent an unprocessed event state, not replay rejection.

Rationale:

A first-time event has not been marked yet. Therefore, its processed-event PDA may not exist or may not be initialized yet.

41K.3 must classify this as:

`unprocessed / eligible for later atomic mark`

not as:

`already processed` or `invalid replay state`.

Required constraint:

The expected PDA must still be derived from:

`["xxxl", "processed-event", canonical_event_key]`

and the supplied account, when present, must match the expected PDA address.

41K.3 must not allow an unrelated account to stand in for an unprocessed event.

## Amendment B — Writable Account Semantics

41K.3 remains read-only in behavior.

However, 41K.3 must not blindly reject a processed-event account only because it is writable.

Reason:

Later 41K.4 / 41K.5 atomic mark/create flow may need the same processed-event PDA to be writable in the same transaction.

Therefore:

- 41K.3 must not mutate account data;
- 41K.3 must not enable replay write;
- 41K.3 must not enable processed marking;
- 41K.3 may observe and report writability;
- writability alone must not be a rejection reason.

## Amendment C — Processed State Model

41K.3 must explicitly model these states before implementation:

### State 1 — Expected PDA Not Initialized

Meaning:

- event is not processed yet;
- replay eligibility may pass;
- later phase may create / initialize / mark atomically.

### State 2 — Initialized Processed Event With `consumed == true`

Meaning:

- event is already processed;
- replay eligibility must reject.

### State 3 — Initialized Processed Event With `consumed == false`

This state needs an explicit decision.

Either:

- support it as an initialized but unconsumed state and test it; or
- reject it / remove it from the lifecycle model and make initialized processed-event PDA imply processed.

This must not remain ambiguous before 41K.3 code acceptance.

## Amendment D — 41J List Interface vs Per-Event PDA Lookup

Current 41J uses `AuthoritativeProcessedRegistryViewRef` with a list-like model:

`processed_canonical_event_keys: &[[u8; 32]]`

The current 41J replay check is effectively:

`processed_canonical_event_keys.contains(canonical_event_key)`

41K.3 uses a per-event PDA lookup model:

`expected processed-event PDA for this canonical_event_key -> processed or unprocessed`

These two models do not match automatically.

Before 41K.3 code acceptance, the plan must choose one approach:

### Option A — Adapter To Existing 41J

A successful 41K.3 lookup creates an internal authoritative adapter:

- unprocessed -> empty processed list;
- processed -> one-item processed list containing canonical_event_key.

The adapter must be type-enforced and not externally constructible.

### Option B — Refine 41J Runtime Interface

41J gains a point-lookup runtime processed-event view instead of relying only on a list of processed keys.

This must preserve all accepted 41J invariants:

- no caller-supplied replay key;
- no caller-supplied decoded payload;
- canonical_event_key derived internally;
- caller-supplied registry rejected;
- unauthenticated registry rejected.

## Amendment E — Updated Review Questions

Reviewers should answer:

1. Is missing / uninitialized expected processed-event PDA correctly treated as unprocessed?
2. What exact runtime representation should implementation use for not-yet-initialized processed-event PDA?
3. Should writable processed-event account be allowed in 41K.3 as long as 41K.3 does not mutate?
4. Should initialized `consumed == false` be supported or rejected?
5. Should 41K.3 adapt to current 41J list interface or should 41J get a point-lookup interface?
6. Are route_id / recipient checks sufficient together with canonical_event_key, or is another identity field required?

## Updated Acceptance Gate

41K.3 plan should not be accepted until these decisions are explicit:

- missing / uninitialized expected PDA semantics;
- writable account semantics;
- initialized consumed=false lifecycle semantics;
- 41J list-vs-point-lookup reconciliation;
- type-enforced adapter requirement before live handler wiring.

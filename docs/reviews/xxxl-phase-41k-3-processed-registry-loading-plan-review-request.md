# XXXL Phase 41K.3 — Processed-Registry PDA Loading Plan Review Request

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Plan document:

`docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`

Base main:

`d983663 Merge XXXL phase 41K.2 guardian-set loading implementation acceptance`

Plan commit:

`49d05c6 Document phase 41K.3 processed-registry loading plan`

## Scope

Пожалуйста, проверь Phase 41K.3 processed-registry PDA loading plan.

Это plan review, не code review.

41K.3 должен быть read/loading boundary для real processed-event PDA.

41K.3 не должен включать write path, processed marking, account mutation, CPI, SPL mint, handler или live route.

## Proposed Runtime Boundary

`real processed-event AccountInfo`
→ expected processed-event PDA derivation
→ account presence policy
→ non-signer / read-only checks
→ owner check before data trust
→ PDA key check before data trust
→ checked data borrow
→ checked processed-event data decode
→ consumed / unconsumed read state
→ structured program-controlled on-chain loading result

## Proposed PDA Seed Format

`["xxxl", "processed-event", canonical_event_key]`

## Main Review Decisions

Please verify before implementation:

1. Is the proposed PDA seed format correct?
2. Should missing processed-event PDA be rejected in 41K.3, or represented as an unprocessed read state for later phases?
3. Should 41K.3 remain read-only and non-writable, with all writes deferred?
4. Is `consumed == true` the correct already-processed signal?
5. Should successful 41K.3 loading later convert to `AuthoritativeProcessedRegistryViewRef` through a single type-enforced adapter?

## Review Focus

Please verify:

- owner check before data trust;
- PDA check before data trust;
- signer/writable rejection in this slice;
- discriminator/version checks;
- stored canonical_event_key / route_id / recipient checks;
- no write path enabled;
- no processed marking enabled;
- no account mutation enabled;
- no CPI / mint / handler / live route enabled.

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is this sufficient before 41K.3 implementation:

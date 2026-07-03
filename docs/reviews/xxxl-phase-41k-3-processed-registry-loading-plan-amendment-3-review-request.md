# XXXL Phase 41K.3 — Processed-Registry Loading Plan Amendment 3 Review Request

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Review target:

- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-2.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-3.md`

## Scope

Please review Amendment 3 as a fix to Claude hostile audit verdict:

`REQUIRES FIXES`

Amendment 3 should resolve these issues:

1. canonical PDA bump handling;
2. exact uninitialized expected PDA representation;
3. lamport-dusting DoS risk;
4. canonical_event_key sufficiency as sole PDA seed identity;
5. required 41K.4 atomic create/init/consume invariant;
6. Option A adapter invariants;
7. type-enforcement pattern.

## Decisions Made In Amendment 3

1. PDA derivation must use canonical `Pubkey::find_program_address`.
2. Caller-supplied bump is never trusted.
3. Supplied expected uninitialized PDA means:
   - expected key;
   - system-program owner;
   - empty data;
   - not signer;
   - not executable.
4. Lamports do not affect uninitialized classification.
5. System-owned empty PDA with nonzero lamports remains unprocessed / eligible.
6. XXXL-owned zero/wrong discriminator is invalid, not unprocessed.
7. `canonical_event_key` is accepted as sole seed identity only as canonical source-event identity from accepted payload binding.
8. Stored route_id / recipient checks remain required as initialized-account integrity checks.
9. 41K.4 must enforce atomic create/init/consume and never create durable initialized `consumed == false`.
10. Option A adapter assumes one replay-eligibility check per event and 41J membership-only semantics.
11. Adapter construction must be internal and type-enforced.
12. All write / mark / mutation / CPI / mint / handler / live route surfaces remain disabled in 41K.3.

## Review Focus

Please verify:

- Does Amendment 3 fully resolve canonical bump risk?
- Does Amendment 3 fully resolve lamport-dusting DoS risk at plan level?
- Is the exact uninitialized representation safe?
- Is it correct that lamports do not affect uninitialized classification?
- Is XXXL-owned zero/wrong discriminator correctly invalid rather than unprocessed?
- Is canonical_event_key sufficiency now stated correctly?
- Are route_id / recipient checks correctly framed as integrity checks?
- Is the 41K.4 atomicity invariant strong enough?
- Are Option A adapter assumptions sufficient?
- Does 41K.3 still stay inside read/loading/classification boundary?
- Are any remaining issues blocking before 41K.3 implementation?

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is Amendment 3 sufficient before 41K.3 plan acceptance:

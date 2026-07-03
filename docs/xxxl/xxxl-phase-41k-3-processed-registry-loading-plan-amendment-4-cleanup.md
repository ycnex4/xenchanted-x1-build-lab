# XXXL Phase 41K.3 — Processed-Registry Loading Plan Amendment 4 Cleanup

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Applies to:

- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-2.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-3.md`

## Status

This cleanup records non-blocking notes from Claude hostile audit of Amendment 3.

Claude verdict:

`ACCEPT WITH NOTES`

Required fixes:

None.

This cleanup does not reopen 41K.3 plan architecture.

It removes test-list drift and clarifies fail-closed implementation expectations before 41K.3 code work begins.

## Cleanup 1 — Base Test List Drift

The base plan originally listed a future test for writable rejection.

That old requirement is superseded.

41K.3 must not reject a processed-event account solely because it is writable.

Correct test requirement:

- writable processed-event account is allowed;
- 41K.3 does not mutate account data;
- replay write remains disabled;
- processed marking remains disabled;
- account mutation remains disabled.

The old writable-rejection test is withdrawn.

## Cleanup 2 — Panic-Safety Test Preserved

The base plan requirement remains active:

The production decoder / loader path must not use unchecked slicing, `unwrap`, or `expect`.

This must stay in the 41K.3 code test/review checklist.

## Cleanup 3 — Total Fail-Closed Classification

41K.3 processed-event account classification must be a total fail-closed function.

Only these states may be accepted/classified:

### Accepted uninitialized / unprocessed state

All must be true:

- account key equals expected canonical processed-event PDA;
- owner is system program;
- data length is zero;
- account is not signer;
- account is not executable.

Lamports are ignored for classification.

### Accepted initialized / processed state

All must be true:

- account key equals expected canonical processed-event PDA;
- owner is XXXL program;
- discriminator is valid;
- version is supported;
- stored canonical_event_key matches expected;
- stored route_id matches expected;
- stored recipient matches expected;
- consumed is true;
- account is not signer;
- account is not executable.

Any state not proven to be one of the accepted states must reject.

## Cleanup 4 — Explicit Invalid States

The following states must reject:

- missing `AccountInfo`;
- wrong account key / non-canonical PDA;
- signer account;
- executable account;
- system-owned account with nonzero data length;
- XXXL-owned account with zero discriminator;
- XXXL-owned account with wrong discriminator;
- XXXL-owned account with unsupported version;
- XXXL-owned account with `consumed == false`;
- non-system, non-XXXL owner;
- malformed borrow / data length / layout;
- any state not explicitly accepted.

System-owned account with nonzero lamports and zero data remains uninitialized / unprocessed.

System-owned account with nonzero data is invalid.

## Cleanup 5 — Signer / Executable Rule Applies To All States

Signer and executable rejection apply to all supplied processed-event account states.

This is not only an uninitialized-state rule.

Both initialized and uninitialized processed-event accounts must reject if:

- `is_signer == true`; or
- `executable == true`.

## Cleanup 6 — Type-Enforcement Language Strengthened

The future adapter construction requirement is mandatory.

The adapter must be internal and type-enforced.

Preferred implementation pattern:

- private-field newtype / witness type;
- single constructor that requires successful 41K.3 processed-event PDA load result;
- no public constructor that allows handler code or caller-controlled data to create an authoritative processed-registry view directly.

`pub(crate)` alone is not sufficient if unrelated code inside the crate can construct the authoritative view without the successful-load witness.

Handler discipline is not sufficient.

## Updated 41K.3 Code Review Checklist

41K.3 code review must verify:

1. canonical `Pubkey::find_program_address` derivation;
2. caller-supplied bump ignored / never trusted;
3. non-canonical PDA rejected;
4. missing `AccountInfo` rejected;
5. signer rejected in all states;
6. executable rejected in all states;
7. system-owned empty-data expected PDA with zero lamports classified uninitialized / unprocessed;
8. system-owned empty-data expected PDA with nonzero lamports classified uninitialized / unprocessed;
9. system-owned nonzero-data expected PDA rejected;
10. XXXL-owned zero discriminator rejected;
11. XXXL-owned wrong discriminator rejected;
12. XXXL-owned unsupported version rejected;
13. XXXL-owned `consumed == false` rejected;
14. XXXL-owned valid `consumed == true` classified processed / replay rejection;
15. route_id / recipient / canonical_event_key integrity checks enforced;
16. writable account allowed but not mutated;
17. no replay write enabled;
18. no processed marking enabled;
19. no account mutation enabled;
20. no CPI / mint / handler / live route enabled;
21. no unchecked slicing / `unwrap` / `expect` in production loader/decoder path;
22. adapter cannot be constructed without successful 41K.3 load witness;
23. caller-supplied / unauthenticated processed-registry views remain rejected;
24. 41J uses the adapted processed-list only for single-key membership of the current canonical_event_key;
25. canonical_event_key binding to source-event identity is verified against accepted Stage 1 / 41I payload path.

## Carry-Forward To 41K.4

41K.4 must prove lamport-dusted atomic initialization.

Required future test:

A system-owned expected processed-event PDA with empty data and nonzero lamports, including a dust amount such as `rent_exempt_min / 2`, must still be safely initialized and consumed atomically without leaving a durable initialized `consumed == false` state.

41K.4 must not rely on a naive `system_instruction::create_account` path that fails on pre-funded / dusted accounts.

The future implementation should use a reviewed allocate / assign / top-up or equivalent idempotent initialization path.

If this cannot be implemented safely, the 41K.3 lifecycle model must be reopened before live route.

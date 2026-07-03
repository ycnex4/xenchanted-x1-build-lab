# XXXL Phase 41K.2 — Guardian-Set Account Loading Implementation Acceptance

Date: 2026-07-03

Status: accepted implementation with notes

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-2-guardian-set-loading-implementation`

Base main:

`1ac03a3 Merge XXXL phase 41K.2 guardian-set loading plan acceptance`

Accepted commits:

- `cedbff5 Add phase 41K.2 guardian-set decoder boundary`
- `9971efe Add phase 41K.2 guardian-set account loader`
- `b98be09 Document phase 41K.2 guardian-set loading implementation review`
- `6611695 Refine phase 41K.2 account check progress flags`

## Final Verdict

Phase 41K.2 guardian-set account/PDA loading implementation is accepted with notes.

Required fixes: none.

41K.2 is sufficient before proceeding to 41K.3 processed-registry PDA loading.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Non-blocking notes: 1
- Sufficient before 41K.2 implementation acceptance: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Non-blocking notes: 1
- Sufficient before 41K.2 implementation acceptance: yes

## Notes Addressed Before Acceptance

Demon reported a minor observability note:

- `account_key_checked`;
- `account_owner_checked`;
- `pda_checked`.

The note was addressed before final acceptance in:

`6611695 Refine phase 41K.2 account check progress flags`

The updated flags now reflect actual check progress on rejection paths.

## Accepted Implementation

New module:

`programs/xxxl-svm/src/verifier/guardian_set_account_loading_boundary.rs`

Exports:

`programs/xxxl-svm/src/verifier/mod.rs`

The accepted 41K.2 boundary implements:

- real guardian-set `AccountInfo` loading;
- fixed guardian-set PDA seed format;
- guardian-set account presence check;
- non-signer precondition;
- read-only precondition;
- expected program owner check before account data trust;
- expected PDA identity check before account data trust;
- checked account data borrow;
- checked account data decode;
- zero discriminator rejection;
- wrong discriminator rejection;
- unsupported layout version rejection;
- inactive/deprecated guardian-set rejection;
- empty guardian-set rejection;
- invalid threshold rejection;
- threshold greater than guardian_count rejection;
- guardian count above max-supported rejection;
- stored guardian_set_id match against expected guardian_set_id;
- duplicate guardian public key rejection;
- panic-safe raw account data reads through checked offsets / `.get()` / `checked_add`;
- no production `unwrap`;
- no production `expect`;
- no unchecked production slicing.

## PDA Seed Format

The accepted guardian-set PDA seed format is:

`["xxxl", "guardian-set", guardian_set_id]`

## Safety Flags

Only the guardian-set runtime loading surface is enabled:

`guardian_set_runtime_loading_enabled: true`

All later runtime surfaces remain disabled:

- `processed_registry_runtime_loading_enabled: false`;
- `replay_write_enabled: false`;
- `processed_event_marking_enabled: false`;
- `account_mutation_enabled: false`;
- `cpi_enabled: false`;
- `invoke_signed_enabled: false`;
- `spl_token_mint_to_enabled: false`;
- `process_instruction_handler_added: false`;
- `live_route_enabled: false`.

## Non-Blocking Architectural Note

Both Theo and Demon reported the same non-blocking architectural note:

41K.2 correctly validates real guardian-set `AccountInfo` / PDA / owner / account data, but final `AuthoritativeGuardianSetRef` construction is still intentionally deferred.

Before 41K.5 live-handler wiring, the path from a successful 41K.2 load to:

`AuthoritativeGuardianSetRef::from_program_controlled_on_chain_source(...)`

must be type-enforced through a single adapter, not left to handler discipline.

This is not a blocker for 41K.2 acceptance.

It is a required future gate before a guardian-set can feed 41H / 41I in the live handler path.

## Test Status

Focused guardian-set account loading tests passed.

Full xxxl-svm test suite passed after implementation.

Full xxxl-svm test suite passed again after the observability flag patch.

Additional local checks:

- `production-safety-scan: OK`;
- `git diff --check: OK`.

## Still Disabled

41K.2 does not enable:

- processed-registry PDA loading;
- replay write;
- processed event marking;
- atomic check-mark-mint;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint;
- process instruction handler;
- live route;
- guardian-set governance;
- guardian-set update instruction;
- production guardian-set deployment.

## Next Gate

41K.3 real processed-registry PDA loading.

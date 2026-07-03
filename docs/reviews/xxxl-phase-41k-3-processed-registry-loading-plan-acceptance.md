# XXXL Phase 41K.3 — Processed-Registry Loading Plan Acceptance

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Accepted HEAD:

`57acfcb Clean up phase 41K.3 processed-registry plan notes`

## Final Verdict

ACCEPTED FOR 41K.3 CODE IMPLEMENTATION

## Review Results

Theo:

`ACCEPT`

Demon:

`ACCEPT`

Claude:

`ACCEPT WITH NOTES`

Required fixes:

None.

## Accepted Plan Documents

- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-2.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-3.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-4-cleanup.md`

Amendments supersede conflicting earlier wording.

## Accepted Decisions

1. Missing `AccountInfo` is rejected.
2. Supplied expected PDA in accepted uninitialized runtime representation means unprocessed / eligible.
3. Supplied wrong PDA is rejected.
4. Initialized `consumed == true` means already processed.
5. Initialized `consumed == false` is rejected as invalid lifecycle state.
6. Writable account is allowed, but 41K.3 must not mutate.
7. Canonical PDA derivation uses `Pubkey::find_program_address`.
8. Caller-supplied bump is never trusted.
9. Lamports do not affect uninitialized classification.
10. System-owned empty-data expected PDA with nonzero lamports remains unprocessed / eligible.
11. XXXL-owned zero/wrong discriminator is invalid, not unprocessed.
12. `canonical_event_key` is accepted as sole seed identity only as canonical source-event identity from accepted payload binding.
13. Stored route_id / recipient checks are initialized-account integrity checks.
14. 41J reconciliation uses Option A: internal type-enforced adapter to existing 41J list-based interface.
15. 41K.4 must enforce atomic create/init/consume and never create durable initialized `consumed == false`.
16. Existing `mark_processed_event_consumed(...)` is not accepted as live semantics without later 41K.4 review.
17. 41K.4 must bind marked amount / mint amount to the quorum-authorized payload.
18. Rent / close / recreate lifecycle risks are documented and carried forward.
19. Active deployment blockers remain unchanged.

## Accepted Safety Flags

41K.3 may enable:

- `processed_registry_runtime_loading_enabled: true`

41K.3 must keep disabled:

- `replay_write_enabled: false`;
- `processed_event_marking_enabled: false`;
- `account_mutation_enabled: false`;
- `cpi_enabled: false`;
- `invoke_signed_enabled: false`;
- `spl_token_mint_to_enabled: false`;
- `process_instruction_handler_added: false`;
- `live_route_enabled: false`.

## 41K.3 Code Review Gates

41K.3 implementation review must verify:

- canonical PDA derivation and canonical bump;
- caller-supplied bump ignored;
- total fail-closed account classification;
- exact uninitialized expected PDA representation;
- lamport-dusted system-owned empty-data expected PDA remains unprocessed;
- system-owned nonzero-data expected PDA rejects;
- XXXL-owned zero/wrong discriminator rejects;
- initialized `consumed == false` rejects;
- initialized `consumed == true` means already processed;
- signer / executable rejection across all states;
- no unchecked slicing / `unwrap` / `expect`;
- type-enforced adapter construction;
- 41J membership-only semantics;
- canonical_event_key binding and collision resistance against the accepted Stage 1 / 41I payload path;
- no replay write;
- no processed marking;
- no mutation / CPI / mint / handler / live route.

## 41K.4 / 41K.5 Carry-Forward Gates

Future phases must enforce:

- no close path for processed-event PDAs;
- rent-exempt processed-event PDA creation;
- no close/recreate replay path;
- atomic create/init/set-consumed mark path;
- robust initialization of lamport-dusted expected PDA accounts;
- no naive `create_account` path that fails on pre-funded / dusted accounts;
- amount / mint binding to quorum-authorized payload;
- production proof logging before deployment readiness.

## Deployment Blockers

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`;
- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`;
- `PRODUCTION_GUARDIAN_SET_UNSET`;
- `PRODUCTION_PROOF_LOG_UNSET`;
- `SPL_CPI_EXECUTION_DISABLED`;
- `LIVE_ROUTE_DISABLED`;
- `EXTERNAL_REVIEW_INCOMPLETE`.

## Next

Proceed to 41K.3 code implementation.

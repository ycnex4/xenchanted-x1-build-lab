# Phase 41K.5 — Live Atomic `consume_gateway_mint` Wiring Plan

## Status

Phase 41K.4 is merged to `main`.

Current `main` checkpoint:

- `3ac365b` — Merge phase 41K.4 atomic processed-event marking implementation.
- Post-merge validation was green.
- 41K.4 proved the processed-event marking primitive with Mollusk/SVM.
- 41K.4 did not activate the production live route.

## Goal

Wire the real `consume_gateway_mint` path so that a valid gateway mint is consumed atomically:

1. decode instruction;
2. validate account contract;
3. validate route/source/mint/recipient/amount;
4. verify guardian quorum / authorization boundary;
5. check processed-event eligibility;
6. mark processed-event PDA as consumed;
7. mint XXXL to the recipient token account;
8. return success only if all required state transitions are completed in one transaction.

The final live path must not expose a route where marking happens without authorization or mint happens without processed-event marking.

## Non-goals

This phase does not cover:

- production deployment;
- production guardian key rotation;
- relayer service implementation;
- frontend gateway UI;
- off-chain watcher implementation;
- mainnet operational runbooks.

Those are later MVP layers after the on-chain atomic path is correct.

## Current blockers in `main`

### 1. Live route is still disabled

`process_consume_gateway_mint` currently builds the runtime execution plan and returns `Ok(())` without live execution.

The current behavior is intentional from previous phases, but 41K.5 must replace planning-only behavior with a guarded live path after tests prove atomicity.

### 2. Current account contract is too small for live marking

Current `CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS` is 9.

The current account list covers:

0. mint_state
1. gateway_config
2. guardian_set
3. processed_event
4. recipient_balance
5. spl_token_mint
6. recipient_token_account
7. mint_authority_pda
8. token_program

41K.4 live marking requires additional accounts:

9. rent_payer
10. system_program

Without these, the live route cannot create/top-up/allocate/assign a system-owned processed-event PDA.

### 3. Current prepare boundary assumes processed-event is already initialized

Current `prepare_consume_gateway_mint_cpi_boundary` treats `processed_event_account` as a program-owned, rent-exempt account before execution.

That is incompatible with 41K.4, where a valid unprocessed event can enter as a system-owned empty/dusted PDA and be created atomically during consume.

41K.5 must split the account validation model:

- program-owned validation for already-initialized program accounts;
- processed-event PDA eligibility validation through the 41K.3/41K.4 loader;
- post-mark re-decode validation after marking.

### 4. SPL mint CPI is still guarded off

`guarded_mint_to_cpi_execution_gate_boundary` currently refuses live CPI because the execution flag is false.

41K.5 must either:

- introduce a live-only execution boundary that can be tested end-to-end; or
- transition the existing guarded boundary only after atomic mark+mint tests prove safety.

### 5. `ProcessedEventMarkingWitness` is still forgeable

The witness currently exposes public fields.

If the witness is used as a mint gate in 41K.5, it must be hardened:

- private fields;
- read-only accessors only;
- constructed only by `mark_processed_event_atomic`;
- not manually forgeable by tests or caller code.

### 6. `consumed_slot` must come from `Clock`

41K.4 test harness accepted `consumed_slot` from instruction data because it was a marking-only harness.

The live path must use `Clock::get()?.slot`.

Caller-supplied consumed slot must not be trusted.

## Proposed 41K.5 implementation slices

### 41K.5A — Planning checkpoint and account-boundary map

Document the live account contract and define the exact transition from 9 accounts to 11 accounts.

Expected output:

- this plan document;
- explicit account ordering for live `consume_gateway_mint`;
- list of validation changes;
- tests that prove the old planning route remains disabled until live route is intentionally activated.

### 41K.5B — Witness hardening

Make `ProcessedEventMarkingWitness` non-forgeable.

Expected output:

- private fields;
- accessors;
- constructor remains inside the marking boundary only;
- tests updated;
- no behavior change to marking.

### 41K.5C — Live account contract preparation

Extend the consume account contract to include:

9. rent_payer
10. system_program

Expected output:

- new required account count;
- validation for rent_payer signer+writable;
- validation for system_program id;
- processed-event validation moved away from unconditional program-owner/rent-exempt assertion;
- negative tests for missing/wrong rent_payer/system_program.

### 41K.5D — Atomic mark + mint boundary

Create a single internal boundary that:

1. prepares route/mint/recipient validations;
2. verifies event eligibility;
3. calls `mark_processed_event_atomic`;
4. uses the marking witness as the only permission to mint;
5. calls SPL Token `mint_to` through `invoke_signed`.

Expected output:

- no standalone live marking route;
- no mint without marking witness;
- consumed amount equals XXXL mint amount;
- consumed slot comes from Clock.

### 41K.5E — Mollusk/SVM end-to-end tests

Prove the live path with runtime tests:

1. valid consume:
   - processed-event PDA becomes consumed;
   - recipient token account balance increases by amount;
   - mint supply increases by amount.

2. replay:
   - second consume fails;
   - no second mint.

3. mint failure after marking attempt:
   - transaction fails atomically;
   - processed-event PDA is not left consumed;
   - recipient balance and mint supply unchanged.

4. invalid guardian/quorum:
   - no marking;
   - no mint.

5. wrong recipient/token account:
   - no marking;
   - no mint.

6. insufficient rent payer:
   - no marking;
   - no mint.

## Claude limit mode

Claude should not be used during intermediate implementation.

Claude should receive only a final 41K.5 audit packet containing:

- branch link;
- compare link;
- patch links for key commits;
- validation summary;
- exact blocker/non-blocker question.

Claude should be asked not to re-review accepted 41K.4 commits unless the new delta invalidates them.

## Acceptance criteria

Phase 41K.5 is not complete until:

- live route is either safely activated or explicitly staged behind a narrowly scoped tested gate;
- processed-event marking and SPL mint are proven atomic in Mollusk/SVM;
- no replay can mint twice;
- no mint can happen without a successful processed-event mark;
- no processed-event mark can remain if mint fails;
- witness cannot be forged;
- consumed slot comes from Clock;
- full `xxxl-svm` suite passes.

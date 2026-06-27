# XXXL Mollusk Readiness Harness Plan

Status: COMPLETED.

This stage records the readiness plan for a future Mollusk-based runtime harness for the XXXL SVM program.

It is intentionally doc-only.

No runtime code is changed in this stage.

## Goal

Prepare the project for realistic SVM runtime testing after the guarded live-handler wiring fixture.

The current runtime has:

- process_instruction entrypoint
- consume_gateway_mint decoder
- canonical 9-account boundary
- account owner/rent checks
- SPL mint and recipient token account validation
- gateway mint authority PDA validation
- guarded live-handler fixture
- atomic execution-plan fixture
- live route activation disabled by default

The next runtime test layer should verify these boundaries under a more realistic Solana/SVM execution harness.

## Why Mollusk next

Unit tests already cover deterministic parsing, account views, CPI boundary construction, PDA derivation, and guarded plan construction.

Mollusk should be used next because it can exercise the program closer to runtime shape:

- entrypoint invocation
- instruction data delivery
- account metas
- AccountInfo state
- writable/signer/executable flags
- program-owned accounts
- SPL Token accounts
- PDA account presence
- failure behavior through process_instruction

This is safer than jumping directly into live route mutation.

## Non-goals

This stage does not:

- add Mollusk as a dependency
- create the actual Mollusk test harness
- activate live route execution
- invoke SPL mint_to from process_instruction
- enable minting from a live handler
- change runtime behavior
- change account layout
- change guardian signature handling
- change upgrade/freeze/finalization policy

## Current runtime boundary to preserve

The harness must preserve the current policy:

- process_instruction remains scaffold-only unless a later explicit stage changes it
- live route activation remains false by default
- mint_to invocation from process_instruction remains false by default
- manual mint remains impossible
- hidden emission remains impossible
- Build-derived supply rights remain impossible
- route activation must stay explicit and separately reviewed

## Required positive harness case

A first Mollusk harness should construct a valid consume_gateway_mint instruction with the canonical 9 accounts:

| Index | Account |
|---:|---|
| 0 | mint_state |
| 1 | gateway_config |
| 2 | guardian_set |
| 3 | processed_event |
| 4 | recipient_balance |
| 5 | spl_token_mint |
| 6 | recipient_token_account |
| 7 | mint_authority_pda |
| 8 | token_program |

Expected result for the current scaffold stage:

- process_instruction returns success
- no SPL mint occurs
- no processed_event mutation occurs through process_instruction
- no recipient_balance mutation occurs through process_instruction
- live route remains disabled

This positive case is not a mint test. It is a scaffold-boundary test.

## Required account meta checks

The harness should explicitly model and test:

- required account count is 9
- account order matches the consume_gateway_mint layout
- mint_state is writable
- gateway_config is writable or read-only according to final handler need
- guardian_set is read-only unless future runtime logic requires otherwise
- processed_event is writable before live mutation stages
- recipient_balance is writable before live mutation stages
- spl_token_mint is writable before live SPL mint_to
- recipient_token_account is writable before live SPL mint_to
- mint_authority_pda is present and not user-signer controlled
- token_program is executable and equals canonical SPL Token program

## Required invalid instruction cases

The harness should reject:

- wrong instruction length
- wrong instruction discriminator
- wrong instruction version
- wrong account_meta_count
- wrong route account index
- wrong guardian set account index
- wrong mint state account index
- wrong processed event account index
- wrong recipient balance account index
- zero amount
- amount larger than u64::MAX

## Required account boundary failure cases

The harness should reject or preserve no-state-change behavior for:

- missing account
- extra account if strict account count is required
- wrong token_program key
- wrong program owner for mint_state
- wrong program owner for gateway_config
- wrong program owner for guardian_set
- wrong program owner for processed_event
- wrong program owner for recipient_balance
- non-rent-exempt program-owned account
- non-rent-exempt SPL mint
- non-rent-exempt recipient token account
- wrong mint_state discriminator
- wrong gateway_config discriminator
- wrong guardian_set discriminator
- wrong processed_event discriminator
- wrong recipient_balance discriminator
- wrong runtime layout version
- truncated account data

## Required PDA failure cases

The harness should reject:

- wrong gateway mint authority PDA
- correct PDA with wrong bump in mint_state
- mint_state pointing to an arbitrary mint authority
- SPL mint authority not matching the gateway PDA
- PDA derived from wrong program id
- PDA account replaced by user-controlled account

## Required route and guardian boundary cases

The harness should reject:

- gateway_config.route_id mismatch
- gateway_config.guardian_set_id mismatch
- gateway_config.target_mint mismatch
- gateway_config.source_chain_weight_bps mismatch
- guardian_set.guardian_set_id mismatch

Guardian signature verification remains outside this stage.

The harness should not imply that guardian approval verification has moved into runtime.

## Required replay boundary cases

The harness should reject:

- processed_event already consumed
- processed_event canonical_event_key mismatch
- processed_event route_id mismatch
- processed_event recipient mismatch

For future live mutation stages, the harness must also verify:

- failed replay check does not credit recipient balance
- failed replay check does not invoke SPL mint_to
- failed replay check does not mark a new event consumed

## Required recipient token account cases

The harness should reject:

- uninitialized recipient token account
- recipient token account mint mismatch
- recipient token account owner mismatch
- recipient token account not owned by SPL Token
- recipient token account not rent-exempt

## Required SPL mint cases

The harness should reject:

- uninitialized SPL mint
- SPL mint not owned by SPL Token
- SPL mint authority mismatch
- SPL mint not rent-exempt

## Required atomicity invariants for future live stage

Before any future live mint path is enabled, the harness must prove:

- all account checks happen before mutation
- replay is checked before balance credit
- overflow is checked before marking processed
- if processed_event mark fails, recipient balance is not credited
- if balance credit fails, processed_event is not marked
- if SPL mint_to fails, the whole instruction fails atomically
- successful live execution marks processed exactly once
- successful live execution credits exactly args.amount
- successful live execution mints exactly args.amount
- no alternative account path can mint more than args.amount

## Required scaffold invariants now

For the current stage, Mollusk should assert:

- process_instruction succeeds for a valid scaffold instruction
- process_instruction does not call mint_to
- process_instruction does not mutate processed_event
- process_instruction does not mutate recipient_balance
- LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED remains false
- execution plans built by guarded fixture keep live_route_activation_enabled false
- execution plans built by guarded fixture keep mint_to_invocation_from_process_instruction_enabled false

## Suggested harness structure

Suggested future files:

- programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs
- programs/xxxl-svm/tests/fixtures/consume_gateway_mint.rs
- programs/xxxl-svm/tests/fixtures/accounts.rs
- programs/xxxl-svm/tests/fixtures/instruction.rs

Suggested fixture builders:

- valid_program_id
- valid_gateway_mint_authority_pda
- valid_consume_gateway_mint_instruction
- valid_mint_state_account
- valid_gateway_config_account
- valid_guardian_set_account
- valid_processed_event_account
- valid_recipient_balance_account
- valid_spl_mint_account
- valid_recipient_token_account
- valid_token_program_account

Suggested mutation helpers:

- corrupt_discriminator
- corrupt_version
- corrupt_owner
- corrupt_rent
- corrupt_pda
- corrupt_route_id
- corrupt_guardian_set_id
- corrupt_target_mint
- corrupt_processed_event_consumed
- corrupt_recipient_owner
- corrupt_recipient_mint
- corrupt_amount

## Dependency policy

Mollusk dependency should be added in a separate stage.

Before adding it:

- identify compatible Mollusk version for current Solana dependency set
- ensure cargo audit remains acceptable
- ensure cargo deny remains acceptable
- avoid broad dependency upgrades unless required
- keep any advisory exception explicit and documented
- avoid moving runtime dependencies unless necessary

## Completion criteria for future Mollusk harness stage

A future implementation stage is complete only when:

- Mollusk dependency is added intentionally
- harness compiles
- positive scaffold instruction case passes
- invalid instruction cases pass
- invalid account boundary cases pass
- PDA failure cases pass
- route/guardian boundary cases pass
- replay boundary cases pass
- current no-live-mint invariant is proven
- cargo fmt/test/clippy/audit/deny remain green

## Conclusion

The project is ready to plan a Mollusk harness, but not to activate live minting.

The next implementation should be test-harness-first, not live-handler-first.

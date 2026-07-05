# Phase 41K.6 B6.36 — Local-validator success/failure matrix design

Status:

LOCAL_VALIDATOR_MATRIX_DESIGN_NOT_EXECUTED

Current decision:

NO-GO

## Purpose

This document defines the future local-validator-only success and failure matrix.

It extends:

- B6.29 local-validator dry-run design map
- B6.30 local-validator fixture inventory map
- B6.31 local-validator fixture generator design
- B6.32 local-validator fixture generator schema
- B6.33 local-only fixture generator skeleton
- B6.34 local fixture generator safety checkpoint
- B6.35 local-validator command boundary map

This is docs-only.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Current blocker H status

Blocker H:

local validator dry-run

Current status:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Current decision remains:

NO-GO.

## Matrix principle

The future local-validator matrix must prove two things:

1. Success path mutates only the expected accounts and only in the expected way.
2. Failure paths leave all mutable accounts byte-identical.

The default invariant is:

Failure leaves all mutable accounts unchanged.

## Success path

The required success path is:

ConsumeGatewayMint success.

Preconditions:

- local validator only
- local program only
- local fixtures only
- local SPL mint only
- local recipient token account only
- local gateway_config initialized
- local guardian_set initialized
- local mint_state initialized and active
- processed_event unconsumed
- valid instruction tag
- valid payload layout
- valid account order
- valid signer flags
- valid writable flags
- valid owner expectations
- valid route id
- valid source chain id
- valid guardian set id
- valid mint id
- valid mint authority PDA
- valid mint authority bump
- valid recipient token account
- valid recipient owner
- valid recipient mint
- valid amount greater than zero
- no replay

Expected success result:

- instruction succeeds
- mint_state.total_minted increases by mint_amount
- processed_event becomes consumed or marked
- recipient token balance increases by mint_amount if SPL CPI is enabled in a later local-only step
- gateway_config remains unchanged
- guardian_set remains unchanged
- unrelated accounts remain unchanged
- expected logs are emitted
- no secrets are printed

Current status:

NOT EXECUTED BY THIS CHECKPOINT.

## Success snapshot requirements

The future success test must record:

- before gateway_config bytes
- before guardian_set bytes
- before mint_state bytes
- before processed_event bytes
- before SPL mint bytes
- before recipient token account bytes
- after gateway_config bytes
- after guardian_set bytes
- after mint_state bytes
- after processed_event bytes
- after SPL mint bytes
- after recipient token account bytes

Expected success mutations:

- mint_state bytes change only in total_minted field
- processed_event bytes change from unconsumed to consumed or marked
- SPL mint supply changes only if SPL CPI is enabled in a later local-only step
- recipient token account balance changes only if SPL CPI is enabled in a later local-only step

Expected unchanged accounts:

- gateway_config
- guardian_set
- any sysvar fixtures
- any system program fixture
- any token program fixture

## Failure matrix categories

The future local-validator failure matrix must include at least these categories:

1. Account count failures.
2. Account order failures.
3. Signer flag failures.
4. Writable flag failures.
5. Owner failures.
6. PDA failures.
7. Discriminator failures.
8. Route failures.
9. Guardian set failures.
10. Mint state failures.
11. Processed event replay failures.
12. SPL mint failures.
13. Recipient token account failures.
14. Amount failures.
15. Payload failures.
16. Rent failures.
17. Guardian authorization failures.
18. Inactive state failures.

## Required failure cases

The future matrix should include these failure cases:

- wrong account count
- wrong account order
- missing required signer
- unexpected signer where forbidden
- readonly account where writable is required
- writable account where readonly is expected
- wrong gateway_config owner
- wrong guardian_set owner
- wrong mint_state owner
- wrong processed_event owner
- wrong SPL mint owner
- wrong recipient token account owner
- wrong mint authority PDA
- wrong mint authority bump
- wrong processed_event PDA
- wrong gateway_config discriminator
- wrong guardian_set discriminator
- wrong mint_state discriminator
- wrong processed_event discriminator
- wrong route id
- wrong source chain id
- wrong gateway_config guardian_set_id
- wrong guardian_set_id
- wrong mint_state mint id
- wrong mint_state token_program
- processed_event already consumed
- processed_event canonical_event_key mismatch
- processed_event recipient_hash mismatch
- wrong SPL mint authority
- wrong recipient token account mint
- wrong recipient token account owner
- zero mint amount
- amount overflow
- malformed instruction data
- empty instruction data
- invalid instruction tag
- truncated payload
- oversized payload
- truncated gateway_config
- truncated guardian_set
- truncated mint_state
- truncated processed_event
- low-rent gateway_config
- low-rent guardian_set
- low-rent mint_state
- low-rent processed_event
- low-rent SPL mint
- low-rent recipient token account
- insufficient guardian quorum
- duplicate guardian approval
- unknown guardian approval
- invalid guardian signature package
- signature over wrong message
- inactive gateway_config
- inactive guardian_set
- inactive mint_state

## Required no-mutation invariant for failures

For every failure case, the future local-validator dry-run must verify:

- gateway_config bytes unchanged
- guardian_set bytes unchanged
- mint_state bytes unchanged
- processed_event bytes unchanged
- SPL mint bytes unchanged
- recipient token account bytes unchanged

The future dry-run must compare byte snapshots before and after.

Expected comparison:

byte-identical

Any mutation on failure must fail the dry-run.

## Expected error evidence

For every failure case, the future matrix must record:

- failure_case_id
- scenario_id
- expected_error_label
- actual_error_label
- expected_no_mutation
- actual_no_mutation
- checked_account_ids
- before_snapshot_id
- after_snapshot_id
- pass_or_fail

A failure case passes only if:

- expected error matches actual error
- all checked mutable accounts are byte-identical
- forbidden logs are absent
- no live RPC or testnet endpoint is used
- no signing material is referenced

## Forbidden evidence

The future matrix output must not include:

- private keys
- keypair paths
- seed phrases
- mnemonic data
- production guardian material
- live RPC URLs
- testnet submit commands
- production account addresses
- production SPL mint addresses

## Required safety checks before future execution

Before the future local-validator matrix can run, the command plan must prove:

- local validator only
- no live RPC
- no testnet RPC
- no production program id
- no production mint
- no production recipient account
- no keypair path
- no private material
- no submit command
- no deploy command to live network
- no upgrade command to live network
- fixture safety report PASS
- deterministic fixture set id
- deterministic snapshots
- cleanup plan exists

## Matrix output requirements

A future local-validator matrix run should output:

- success_result.json
- failure_matrix_result.json
- mutation_invariance_result.json
- safety_report.json
- logs_sanitized.txt
- cleanup_report.txt

This checkpoint does not create these files.

## Explicit non-closure

This checkpoint does not close blocker H.

It defines the future success/failure matrix design only.

Current blocker H state:

OPEN_DESIGN_STARTED

Current decision remains:

NO-GO.

## Next safe step

The next safe step is a docs-only rollback and recovery plan map for blocker G.

No local-validator execution is approved by this checkpoint.

Current decision remains:

NO-GO.

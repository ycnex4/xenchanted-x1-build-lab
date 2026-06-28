# XXXL Account Contract Review Boundary

Status: COMPLETED.

This document records the Phase 1 account contract review boundary for the XXXL SVM runtime scaffold.

This is a documentation-only boundary.

No runtime code is changed by this stage.

## Purpose

The goal is to document the existing consume_gateway_mint account contract as a reviewable boundary before any production-path mutation, SPL CPI execution, live route activation, or deployment behavior is enabled.

The current runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

The release decision remains:

- release allowed: `false`
- release blocked: `true`
- primary blocker code: `RUNTIME_SAFETY_LOCK_ACTIVE`

This document does not remove the `ACCOUNT_CONTRACT_UNREVIEWED` deployment blocker.

The blocker must remain active until the account contract is reviewed, tested to the required standard, and accepted in a future reviewed boundary.

## Source of truth

The current code-level source of truth is:

- `programs/xxxl-svm/src/account_contract.rs`

The processor integration points are:

- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/instruction.rs`

The deployment blocker source is:

- `programs/xxxl-svm/src/deployment_status.rs`

The safety lock and release decision source is:

- `programs/xxxl-svm/src/safety_invariants.rs`

## Current account count

The current consume_gateway_mint account contract contains exactly 9 accounts.

The current instruction account meta count is also 9.

The runtime must reject an instruction if the account list length or encoded account meta count does not match the expected count.

## Account index mapping

Index 0: `mint_state`

- Access: readonly
- Signer: no
- Owner model: program-owned
- Current role: stores mint identity and gateway mint authority PDA binding
- Validation intent:
  - must be owned by this program
  - must be rent-exempt
  - must decode as a MintState account
  - mint pubkey must match instruction `mint_id`
  - gateway mint authority PDA must match the provided mint authority PDA account
- Substitution protection:
  - program ownership check
  - rent exemption check
  - account layout/discriminator/version validation through account view decoding
  - instruction mint binding
  - authority PDA binding
- Failure behavior:
  - reject before execution plan is accepted
  - no production state mutation
  - no SPL CPI execution

Index 1: `gateway_config`

- Access: readonly
- Signer: no
- Owner model: program-owned
- Current role: stores route policy for the consume_gateway_mint path
- Validation intent:
  - must be owned by this program
  - must be rent-exempt
  - must decode as a GatewayConfig account
  - route id must match instruction `route_id`
  - guardian set id must match instruction `guardian_set_id`
  - target mint must match instruction `mint_id`
  - source chain weight bps must match instruction `source_chain_weight_bps`
- Substitution protection:
  - program ownership check
  - rent exemption check
  - route id binding
  - guardian set binding
  - target mint binding
  - route weight binding
- Failure behavior:
  - reject before execution plan is accepted
  - no production state mutation
  - no SPL CPI execution

Index 2: `guardian_set`

- Access: readonly
- Signer: no
- Owner model: program-owned
- Current role: stores guardian set metadata referenced by the route
- Validation intent:
  - must be owned by this program
  - must be rent-exempt
  - must decode as a GuardianSet account
  - guardian set id must match instruction `guardian_set_id`
- Substitution protection:
  - program ownership check
  - rent exemption check
  - guardian set id binding
  - route-to-guardian set binding through gateway config
- Failure behavior:
  - reject before execution plan is accepted
  - no production state mutation
  - no SPL CPI execution

Index 3: `processed_event`

- Access: writable
- Signer: no
- Owner model: program-owned
- Current role: processed event / replay protection account
- Validation intent:
  - must be owned by this program
  - must be rent-exempt
  - must decode as a ProcessedEvent account
  - must not already be consumed
  - canonical event key must match instruction `canonical_event_key`
  - route id must match instruction `route_id`
  - recipient must match instruction `recipient`
- Substitution protection:
  - program ownership check
  - rent exemption check
  - consumed flag check
  - canonical event key binding
  - route binding
  - recipient binding
- Failure behavior:
  - reject before execution plan is accepted
  - no successful replay
  - no SPL CPI execution
- Mutation note:
  - this is a future mutation target
  - this boundary does not enable production mutation

Index 4: `recipient_balance`

- Access: writable
- Signer: no
- Owner model: program-owned
- Current role: local runtime recipient balance/accounting view
- Validation intent:
  - must be owned by this program
  - must be rent-exempt
  - must decode as a RecipientBalance account
  - owner must match instruction `recipient`
  - mint must match instruction `mint_id`
- Substitution protection:
  - program ownership check
  - rent exemption check
  - recipient owner binding
  - mint binding
- Failure behavior:
  - reject before execution plan is accepted
  - no production state mutation
  - no SPL CPI execution
- Mutation note:
  - this is a future mutation target
  - this boundary does not enable production mutation

Index 5: `spl_token_mint`

- Access: writable
- Signer: no
- Owner model: SPL Token owned
- Current role: XXXL SPL Token mint account
- Validation intent:
  - must be rent-exempt
  - must decode as initialized SPL Token Mint
  - mint authority must be the gateway mint authority PDA
  - mint pubkey must match instruction `mint_id` through the execution path
- Substitution protection:
  - SPL Token account layout validation
  - initialized mint validation
  - mint authority binding
  - instruction mint binding
- Failure behavior:
  - reject before SPL CPI can be considered reachable
  - no SPL CPI execution
- Writable rationale:
  - future SPL Token `mint_to` would mutate mint supply
  - current runtime still keeps SPL CPI execution disabled

Index 6: `recipient_token_account`

- Access: writable
- Signer: no
- Owner model: SPL Token owned
- Current role: recipient token account / ATA for XXXL
- Validation intent:
  - must be rent-exempt
  - must decode as initialized SPL Token account
  - token account owner must match instruction `recipient`
  - token account mint must match instruction `mint_id`
  - must satisfy the recipient ATA boundary
- Substitution protection:
  - SPL Token account layout validation
  - recipient owner binding
  - mint binding
  - ATA boundary validation
- Failure behavior:
  - reject before SPL CPI can be considered reachable
  - no SPL CPI execution
- Writable rationale:
  - future SPL Token `mint_to` would mutate recipient token balance
  - current runtime still keeps SPL CPI execution disabled

Index 7: `mint_authority_pda`

- Access: readonly
- Signer: no
- Owner model: program-derived address
- Current role: gateway mint authority PDA
- Validation intent:
  - must match the PDA derived from the current program id and gateway mint authority seeds
  - bump must match the bump recorded in mint state
  - no external signer is accepted
- Substitution protection:
  - PDA derivation check
  - bump check
  - mint state authority binding
- Failure behavior:
  - reject before execution plan is accepted
  - no SPL CPI execution
- Future signer note:
  - future `invoke_signed` must use reviewed PDA seeds only
  - this boundary does not enable `invoke_signed`

Index 8: `token_program`

- Access: readonly
- Signer: no
- Owner model: SPL Token program
- Current role: SPL Token program id for future mint_to CPI
- Validation intent:
  - account key must equal the canonical SPL Token program id
- Substitution protection:
  - hard token program id check
- Failure behavior:
  - reject before SPL CPI can be considered reachable
  - no SPL CPI execution

## Current non-included accounts

The current consume_gateway_mint account contract does not include a system program account.

The current consume_gateway_mint account contract does not include an explicit rent sysvar account.

The current processor obtains rent and clock through sysvar access, not through account indices in the consume_gateway_mint account list.

If future implementation requires explicit sysvar or system-program accounts, that must be handled in a separate reviewed boundary.

## Writable account set

The current writable account set is:

- `processed_event`
- `recipient_balance`
- `spl_token_mint`
- `recipient_token_account`

Writable does not mean currently production-mutated.

Current live route execution and SPL CPI execution remain disabled.

## Readonly account set

The current readonly account set is:

- `mint_state`
- `gateway_config`
- `guardian_set`
- `mint_authority_pda`
- `token_program`

## Signer requirements

The current account contract requires no external signer accounts.

The mint authority PDA is not an external signer.

Any future `invoke_signed` path must remain disabled until separately reviewed.

## PDA accounts

The current explicit PDA account is:

- `mint_authority_pda`

The following program-owned accounts may later need PDA derivation constraints to be fully specified for production:

- `mint_state`
- `gateway_config`
- `guardian_set`
- `processed_event`
- `recipient_balance`

This document records the current account contract but does not claim production PDA fixture readiness.

Real Program ID selection and production PDA fixture regeneration remain separate roadmap phases.

## Caller-supplied accounts

The relayer or caller supplies the account list to the instruction.

The runtime must treat caller-supplied accounts as untrusted until all account contract checks pass.

The account list must not allow substitution of:

- route config
- guardian set
- processed event account
- recipient balance account
- token mint
- recipient token account
- mint authority PDA
- SPL Token program

## Existing validation boundaries

The current validation boundary checks include:

- account count
- instruction account meta count
- expected writable flags
- expected signer flags
- program-owned account owner checks
- rent exemption checks
- SPL Token program id check
- SPL Token mint initialization
- mint authority binding
- recipient token account / ATA boundary
- route id binding
- guardian set binding
- target mint binding
- source chain weight binding
- processed event replay state
- canonical event key binding
- recipient binding
- non-zero amount and u64 amount boundary for SPL Token mint_to planning

## Review questions

A reviewer should verify:

1. Is the 9-account order sufficient for the future consume_gateway_mint path?
2. Are all writable accounts justified?
3. Are any writable accounts unnecessarily writable?
4. Are readonly accounts protected against substitution?
5. Is the absence of external signer accounts correct?
6. Is the mint authority PDA boundary sufficient before future `invoke_signed` planning?
7. Should any program-owned accounts require explicit PDA derivation checks before mutation?
8. Is the SPL Token mint account validation sufficient before future mint_to CPI?
9. Is recipient token account / ATA validation sufficient?
10. Are rent and clock sysvar assumptions acceptable without explicit account indices?
11. Are failure modes strict enough to guarantee no production mutation on account-contract failure?
12. Are additional Mollusk account-substitution tests required before unlock?

## Required test matrix before blocker removal

The following tests are required before `ACCOUNT_CONTRACT_UNREVIEWED` can be removed or replaced:

- wrong account count
- wrong instruction account meta count
- wrong account order
- wrong writable flag
- required writable account passed readonly
- readonly account passed writable
- unexpected signer
- missing required account
- wrong program owner for program-owned account
- wrong SPL Token program id
- wrong SPL Token mint account
- wrong mint authority PDA
- wrong mint authority bump
- wrong recipient token account
- wrong recipient token owner
- wrong recipient token mint
- wrong gateway config route id
- wrong gateway config guardian set id
- wrong gateway config target mint
- wrong gateway config source chain weight
- wrong guardian set id
- already consumed processed event
- wrong canonical event key in processed event
- wrong processed event route id
- wrong processed event recipient
- wrong recipient balance owner
- wrong recipient balance mint
- zero amount
- amount larger than SPL Token u64 range

## Existing visible coverage

The current code already includes account contract tests for:

- expected length and indices
- processor index mapping
- instruction index mapping for encoded account indices
- writable account classification
- no external signer requirement
- owner model documentation
- out-of-range lookup rejection

The current processor integration tests include visible coverage for:

- successful CPI boundary preparation after decode and validation
- wrong account count
- gateway route mismatch
- consumed processed event rejection
- wrong mint authority PDA rejection
- wrong SPL mint owner rejection
- wrong recipient token mint rejection

This is useful evidence, but this document does not claim the blocker is cleared.

## Test gap closure evidence

The `XXXL account contract test gap closure` stage adds focused
processor-boundary coverage for security-sensitive account substitution and
binding cases.

New direct processor-boundary tests cover:

- wrong account order
- wrong program owner for a program-owned account path
- wrong SPL Token program id
- wrong SPL Token mint authority
- wrong mint authority bump
- wrong gateway config guardian set id
- wrong gateway config target mint
- wrong gateway config source chain weight
- wrong guardian set id
- wrong processed event canonical event key
- wrong processed event route id
- wrong processed event recipient
- wrong recipient balance owner
- wrong recipient balance mint
- amount larger than SPL Token `u64` range

Existing lower-level tests remain the intended coverage for account count,
encoded account meta count, account index mapping, writable/readonly flags,
signer requirements, owner model classification, recipient token account
validation, consumed-event rejection, zero amount rejection, and CPI
planning-only rejection boundaries.

This improved test evidence does not complete external review.

This improved test evidence does not remove `ACCOUNT_CONTRACT_UNREVIEWED`.

## Review evidence package

A reviewer-facing evidence package is available at:

- `docs/xxxl/xxxl-account-contract-review-evidence-package.md`

The package collects the current account contract, threat model, test evidence,
remaining blockers, validation expectations, and reviewer questions in one
place.

The package is evidence only. It is not approval, not blocker removal, and not
runtime unlock.

## Non-goals

This boundary does not implement runtime mutation.

This boundary does not enable runtime account writes in the live process_instruction path.

This boundary does not enable live route execution.

This boundary does not enable SPL CPI execution.

This boundary does not enable `invoke_signed`.

This boundary does not enable SPL Token `mint_to`.

This boundary does not enable XXXL minting.

This boundary does not select a real Program ID.

This boundary does not regenerate production PDA fixtures.

This boundary does not remove deployment blockers.

This boundary does not change deployability predicates.

## Decision

The existing consume_gateway_mint account contract is documented as a review boundary.

The current runtime remains scaffold-only, locked, unreleasable, and not deployable.

The `ACCOUNT_CONTRACT_UNREVIEWED` blocker remains active until separate review and required test evidence are complete.

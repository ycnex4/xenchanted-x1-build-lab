# Stage 2.5 Compute Budget Strategy

This document records the compute budget strategy for the Stage 2.5 token mint CPI prototype.

## Decision

Stage 2.5 client/test transactions should include a ComputeBudgetProgram.setComputeUnitLimit instruction before gateway submit_mint_approval.

This is a client/test execution strategy.

It is not an on-chain protocol rule.

## Reason

The Stage 2.5 gateway transaction is heavier than a simple token transfer.

The transaction may include:

1. ComputeBudgetProgram.setComputeUnitLimit
2. Ed25519 guardian signature instruction
3. optional non-Ed25519 interleaving instruction
4. Ed25519 guardian signature instruction
5. submit_mint_approval

submit_mint_approval itself performs:

- gateway config checks
- context-bound message_hash derivation
- prior instruction scanning
- guardian set membership checks
- quorum checks
- ProcessedBurnEntry creation
- recipient token account validation
- SPL Token mint_to CPI

Without an explicit compute budget instruction, test failures may come from runtime compute limits rather than gateway logic.

## Scope

Compute budget is not part of the protocol state.

Compute budget is not stored in GatewayConfig.

Compute budget does not affect authorization rules.

Compute budget only affects whether a heavy transaction has enough runtime compute to execute.

## Stage 2.5 transaction shape

The expected Stage 2.5 test transaction shape is:

1. ComputeBudgetProgram.setComputeUnitLimit
2. Ed25519 instruction for guardian A
3. optional interleaved non-Ed25519 instruction
4. Ed25519 instruction for guardian B
5. submit_mint_approval with SPL Token mint CPI

## Test policy

Stage 2.5 tests should use compute budget when executing the full gateway mint CPI path.

Tests should still verify protocol failures separately:

- wrong message_hash
- missing signature
- unknown guardian
- replay
- wrong mint
- wrong recipient token account
- mint CPI failure rollback

## Current conclusion

Stage 2.5 prerequisite 5 is closed for the prototype.

ComputeBudgetProgram.setComputeUnitLimit should be added to the client/test transaction for the full CPI path.

This is an execution strategy, not a protocol rule.

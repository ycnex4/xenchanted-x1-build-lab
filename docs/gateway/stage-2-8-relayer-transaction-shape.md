# Stage 2.8 Relayer Transaction Shape

This document defines the Stage 2.8 relayer / watcher transaction shape for the X1 direct mint gateway prototype.

Stage 2.8 is a design and transaction-shape stage.

It does not change the on-chain runtime.

It specifies how an external relayer should assemble the X1 transaction that calls the already-tested Stage 2.5 / Stage 2.6 gateway runtime path.

## Previous evidence

Stage 2.5 proved the live direct mint CPI path on X1 testnet:

    guardian signatures
    -> context-bound message_hash
    -> gateway validation
    -> replay protection
    -> ProcessedBurnEntry
    -> SPL Token mint_to CPI
    -> replay rejection after success

Stage 2.6 proved failure atomicity:

    no false ProcessedBurnEntry remains if mint_to CPI fails

Stage 2.7 cleaned runtime account hygiene without changing behavior.

## Goal

The relayer must construct a transaction that contains:

1. Compute budget instruction
2. One Ed25519 verification instruction per guardian approval
3. submit_mint_approval instruction
4. All required accounts
5. Correct instruction ordering
6. Correct retry behavior
7. No authority over protocol rules

The relayer is not trusted to decide economic outcomes.

The relayer only submits already-formed messages and guardian approvals to the gateway runtime.

## Non-goals

Stage 2.8 does not define:

- production watcher implementation
- production guardian key management
- final deployment authority model
- frontend gateway UI
- claim-based fallback
- fee policy
- governance
- live mainnet operations

## Relayer input

A relayer transaction requires the following inputs:

- canonical_event_key
- recipient X1 address
- minted_amount
- deadline_or_finality_block
- message_nonce
- guardian_set_version
- approved guardian public keys
- guardian signatures over the exact message_hash
- gateway_config PDA
- guardian_set PDA
- processed_burn PDA
- xxxl_mint account
- recipient token account
- mint_authority PDA
- SPL Token program id
- instructions sysvar
- payer
- system program

## Message hash

The relayer must compute the same context-bound message hash as the runtime.

The message hash binds:

- message type
- route id
- source chain id
- source token
- canonical event key
- recipient
- minted amount
- guardian set version
- deadline / finality block
- message nonce

The relayer must not sign or submit a message hash that was not derived from the full mint context.

## Instruction order

The transaction order must be:

    ComputeBudgetProgram.setComputeUnitLimit
    Ed25519 guardian approval instruction #1
    Ed25519 guardian approval instruction #2
    ...
    Ed25519 guardian approval instruction #N
    submit_mint_approval

The gateway runtime scans prior instructions.

Therefore all Ed25519 verification instructions must appear before submit_mint_approval.

## Compute budget

The compute budget instruction is client-side / relayer-side.

It is not part of the on-chain protocol state.

Current prototype tests use:

    ComputeBudgetProgram.setComputeUnitLimit({ units: 600_000 })

The relayer may tune this value later based on measured runtime needs, but must not make protocol behavior depend on an off-chain compute policy.

## Required accounts

submit_mint_approval requires:

- gateway_config
- guardian_set
- processed_burn
- xxxl_mint
- recipient_token_account
- mint_authority
- token_program
- payer
- instructions_sysvar
- system_program

The runtime enforces:

- gateway_config PDA
- processed_burn PDA
- expected xxxl_mint
- recipient token account owner
- recipient token account mint
- standard SPL Token program id
- mint_authority PDA
- instructions sysvar address

## Recipient token account policy

The recipient token account must already exist before submit_mint_approval.

Stage 2.8 relayer may prepare the recipient token account before the mint transaction, but account creation is outside submit_mint_approval.

For the first prototype, submit_mint_approval does not create an ATA.

## Retry behavior

Relayer retry must preserve idempotency.

Safe retry cases:

- transaction not confirmed
- blockhash expired before execution
- RPC timeout
- upload / send failure before final confirmation

Unsafe retry assumptions:

- relayer must not assume failure means no execution
- relayer must check processed_burn state before retrying if confirmation is ambiguous
- relayer must not submit a second different message for the same canonical_event_key

Replay protection is enforced by processed_burn PDA.

If processed_burn exists, the relayer should treat the event as already processed and stop.

## Failed transaction behavior

If submit_mint_approval fails before successful execution, no ProcessedBurnEntry should remain.

Stage 2.6 already proved this for:

- failed mint_to CPI
- wrong xxxl_mint
- wrong recipient token account mint
- wrong recipient token account owner

The relayer should surface this as a failed submission, not as a completed mint.

## Buffer / deployment issue distinction

Runtime deployment failures and relayer transaction failures are different classes.

Deployment buffer accounts may appear during program deploy failures.

Relayer mint submission must not create or depend on program deploy buffers.

Stage 2.8 treats deployment buffer handling as operational knowledge, not part of the mint transaction protocol.

## Relayer authority boundary

The relayer does not control:

- mint amount rules
- guardian quorum rules
- canonical event key rules
- replay protection
- mint authority PDA
- expected XXXL mint
- token account validation
- protocol state transitions

The relayer controls only transaction assembly and submission.

The protocol decides whether the transaction is valid.

## Minimal relayer algorithm

1. Read finalized Ethereum burn event.
2. Build canonical_event_key.
3. Build Stage 2 message context.
4. Compute message_hash.
5. Collect guardian approvals over message_hash.
6. Derive X1 accounts.
7. Ensure recipient token account exists.
8. Check whether processed_burn already exists.
9. Assemble transaction:
   - compute budget
   - guardian Ed25519 instructions
   - submit_mint_approval
10. Send transaction.
11. Confirm transaction.
12. If ambiguous, re-check processed_burn and recipient token balance.
13. Record result.

## Success criteria

Stage 2.8 is complete when the repository contains:

- documented relayer transaction shape
- account ordering
- retry policy
- processed_burn idempotency policy
- boundary between relayer and protocol
- clear next step toward a TypeScript relayer prototype

## Current conclusion

Stage 2.8 defines the off-chain transaction assembly boundary for the X1 direct mint gateway.

The relayer is an execution helper.

The protocol remains the source of truth.

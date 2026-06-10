# Stage 2 Direct Mint Prototype Start

This document opens the first Stage 2 runtime implementation direction after Stage 1.10 and Theo review refinements.

## Purpose

The first Stage 2 implementation path is a minimal direct-mint prototype.

It is not production gateway deployment.

It is not the full bridge.

It is not a frontend bridge flow.

It is not final guardian infrastructure.

## Confirmed inputs

The prototype starts after these milestones:

- EV-01 transaction-level atomicity confirmed on X1 testnet
- EV-02 account write rollback confirmed on X1 testnet
- Stage 1.10 X1 program instruction and PDA derivation design completed
- Stage 1.10 Theo review refinements completed

## First implementation direction

The first implementation direction is:

- direct mint first
- guardian verification inside submit_mint_approval
- gateway PDA signs token mint CPI
- relayer pays ProcessedBurnEntry rent
- ProcessedBurnEntry is permanent replay protection
- claim-based flow remains fallback, not first implementation

## Minimal prototype scope

The first prototype should include only:

1. gateway config account
2. guardian set account
3. processed burn account
4. mint authority PDA
5. initialize_gateway_config
6. initialize_guardian_set or set_guardian_set
7. submit_mint_approval direct mint path
8. replay rejection
9. invalid quorum rejection
10. mint CPI failure rollback test if token interface allows it

## Explicit non-goals

This prototype does not implement:

- production Ethereum watcher
- production relayer
- production guardian key management
- frontend bridge UX
- claim-based flow
- emergency governance beyond minimal pause if needed
- dynamic route/economic coefficient updates
- multi-chain support
- deployment authority finalization

## Implementation rule

Every runtime step must preserve the Stage 1 invariant:

verification succeeds before replay mark;
replay mark happens before or atomically with mint;
if mint fails, no replay mark remains.

## Next step

The next branch should add a minimal runtime skeleton or prototype test harness based on this boundary.

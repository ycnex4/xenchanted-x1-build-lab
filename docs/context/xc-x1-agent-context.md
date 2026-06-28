# XC / X1 Agent Context

This is the compact project context future Codex agents should read before making XC, X1, gateway, or XXXL changes.

## XC

xEnchanted Crypto, or XC, is an immutable NFT-driven algorithmic mining protocol inspired by XEN first-principles.

Public positioning:

> "XEN mines tokens. xEnchanted Crypto mines state."

XEN is the entry point: a user burns XEN and receives a Core NFT, which represents readable protocol state.

XC has no premine, no founder allocation, no admin mint, no whitelist, no upgrade proxy, and immutable rules.

Main XC assets:

- Core NFT
- Forged NFT
- Stake NFT
- XNTD ERC-20

Core NFT is minted by burning XEN. Redeeming Core or Forged NFTs mints XNTD and burns the NFT. Enchant combines same-level NFTs and increases level. Forge burns XNTD plus a current-epoch L1 sacrifice to create a Forged NFT. Stake burns the original NFT and mints a Stake NFT; redeeming Stake remints the original, with reward and penalty rules. Market v1 is for Core/Forged NFTs only; Stake NFTs are not market assets.

## X1 Direction

The X1 work extends XC state into X1.

The current repo is focused on X1 Build, XXXL, gateway architecture, and runtime safety. XNTD burned or locked on Ethereum can become input for X1-side state. XXXL is the X1-side derivative/output token concept connected to XNTD gateway work.

The gateway design maps deterministic Ethereum burn events into X1 mint authorization. Stage 1 proved the deterministic model. Stage 1.5 mapped the deterministic model to runtime concepts.

The current XXXL SVM runtime is scaffold-only and locked. The current safety posture is not deployable and not release-ready.

This context is not an unlock signal. It does not imply that the runtime is deployable, release-ready, or ready to unlock.

## Current Checkpoint

Current `main` / `origin/main` checkpoint after the latest completed boundary:

- Merge XXXL account contract review boundary
- `main` / `origin/main` @ `460ef2f746ec91b716895ab9c51d5f6b4eb0f21d`

The latest completed boundary documented the existing `consume_gateway_mint` 9-account contract. It was docs-only. Runtime source under `programs/xxxl-svm/src/` was not changed.

`ACCOUNT_CONTRACT_UNREVIEWED` remains active. Runtime remains scaffold-only, locked, unreleasable, and not deployable.

## Current Blockers

The current deployment blockers are:

1. `PLACEHOLDER_PROGRAM_ID`
2. `LIVE_ROUTE_DISABLED`
3. `SPL_CPI_EXECUTION_DISABLED`
4. `ACCOUNT_CONTRACT_UNREVIEWED`
5. `MOLLUSK_COVERAGE_INCOMPLETE`
6. `PRODUCTION_GUARDIAN_SET_UNSET`
7. `PRODUCTION_PROOF_LOG_UNSET`
8. `EXTERNAL_REVIEW_INCOMPLETE`

## Current Account Contract

The current `consume_gateway_mint` account contract has 9 accounts:

1. `mint_state`
2. `gateway_config`
3. `guardian_set`
4. `processed_event`
5. `recipient_balance`
6. `spl_token_mint`
7. `recipient_token_account`
8. `mint_authority_pda`
9. `token_program`

Writable accounts:

- `processed_event`
- `recipient_balance`
- `spl_token_mint`
- `recipient_token_account`

Readonly accounts:

- `mint_state`
- `gateway_config`
- `guardian_set`
- `mint_authority_pda`
- `token_program`

External signer accounts:

- none

## Next Likely Work

The next logical stage is account contract test gap closure.

This should not remove `ACCOUNT_CONTRACT_UNREVIEWED` unless a reviewed boundary explicitly allows it. The goal is to close the test matrix documented in `docs/xxxl/xxxl-account-contract-review-boundary.md`.

The goal is not to enable runtime execution.

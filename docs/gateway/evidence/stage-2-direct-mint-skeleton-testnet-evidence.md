# Stage 2 Direct Mint Gateway Skeleton Testnet Evidence

This document records the first Stage 2 direct mint gateway skeleton runtime evidence on X1 testnet.

## Scope

This evidence covers a minimal gateway runtime skeleton.

It is not production gateway deployment.

It is not full bridge implementation.

It does not include token mint CPI yet.

It does not include production Ethereum watcher, relayer, guardian infrastructure, frontend bridge UX, or final deployment authority policy.

## Prototype repository

Runtime prototype repository:

- `~/xenchanted-x1-lab/hello-x1`

Prototype branch:

- `stage-2-direct-mint-gateway-skeleton`

Local commits:

- `a66b92b Add Stage 2 direct mint gateway skeleton`
- `7d664e4 Add Stage 2 direct mint gateway skeleton test`

## Program

X1 testnet program id:

- `9tCJe4M1MJQtE1gDxNYNE75fNUGpSAKiX56rgUMR8984`

Deploy signature for this skeleton update:

- `3EbU65uWo3zwVGGo3KJ1rXsgDPVfsTUPKFaJo15WPu8F9yvKojs5rpQQP9S5Wsg5xf2fgigRvBDPnyEQY2rvR5cK`

## Runtime skeleton added

The skeleton added the following account types:

- `GatewayConfig`
- `GuardianSet`
- `ProcessedBurnEntry`

The skeleton added the following gateway instructions:

- `initialize_gateway_config`
- `set_guardian_set`
- `submit_mint_approval`

The skeleton retained the previous EV-01 / EV-02 rollback probe instructions.

## Test command

The following test command was run against X1 testnet after deploying the updated program:

    yarn ts-mocha -p ./tsconfig.json -t 1000000 tests/gateway_direct_mint_skeleton.ts

## Test result

The test passed:

    Stage 2 direct mint gateway skeleton
    ✔ initializes config, guardian set, processed burn, and rejects replay
    ✔ rejects unknown guardian approval

    2 passing

RPC returned temporary 429 retry messages during the run, but the test completed successfully.

## Evidence confirmed

This Stage 2 skeleton evidence confirms:

1. `initialize_gateway_config` works on X1 testnet.
2. `set_guardian_set` works on X1 testnet.
3. `submit_mint_approval` creates `ProcessedBurnEntry`.
4. Replay with the same `canonical_event_key` is rejected because the processed PDA already exists.
5. Unknown guardian approvals are rejected.
6. When unknown guardian approval fails, `ProcessedBurnEntry` is not created.

## What this does not prove

This evidence does not prove:

- real guardian cryptographic signature verification
- Ethereum event watcher correctness
- production relayer correctness
- token mint CPI correctness
- mint authority PDA signing correctness
- final XXXL mint integration
- full bridge economics
- production deployment readiness

## Current conclusion

Stage 2 has moved from design-only planning into a minimal X1 testnet runtime skeleton.

The skeleton validates the first direct-mint gateway account and replay-protection path:

- gateway config
- guardian set
- processed burn replay protection
- quorum-like guardian membership checking
- replay rejection
- failed invalid guardian path without processed burn creation

The next runtime step should be narrow and evidence-based.

Recommended next step:

- add real guardian signature verification or
- add token mint CPI prototype

The token mint CPI path should only be added after confirming the exact X1 token program interface and mint authority setup.

# Build v1 X1 runtime boundary checkpoint

## Branch

`build-v1-x1-runtime-boundary`

## Purpose

This checkpoint records the first production-boundary mapping for X1 Build v1.

The goal is to move from the TypeScript implementation lab toward an X1 runtime architecture without prematurely writing runtime code.

This checkpoint is intentionally docs-first.

## Main decision

The next Build stage is not frontend preview work.

The next Build stage is the X1 runtime boundary:

- what lives in X1 runtime state;
- what remains scanner/indexer/registrar infrastructure;
- which instructions are allowed;
- how replay and checkpoint protection should be mapped;
- how gateway-created Build differs from X1-native Build;
- how Build Identity stays separate from protocol accounting.

## Runtime boundary recorded

Document added:

- `docs/build/build-v1-x1-runtime-boundary.md`

## Key conclusions

### 1. Build State stores durable facts

Runtime Build State should store:

- owner;
- buildId;
- version;
- createdAt;
- updatedAt;
- ethereumIdentity;
- Build Identity metadata;
- historyBld;
- originBld;
- historyXbp;
- XNTD commitment facts;
- X1 fee checkpoint facts.

It should not store:

- public spendable BLD balance;
- public spendable XBP balance;
- live wallet balance;
- live token escrow balance;
- RPC status;
- frontend preview data.

### 2. ETH/XC gateway Build is not an empty shell

A Build created through the ETH/XC gateway must represent verified Ethereum/XC contribution history.

It must not be a clean empty object.

A clean `UNCOMMITTED` Build shell belongs to the X1-native creation path.

### 3. Gateway activation should use full-profile boundary

Gateway activation/update must not silently apply only one part of the Ethereum/XC profile.

The runtime boundary requires the full profile needed by the operation:

- Core redeem history;
- global XEN.burn history;
- XNTD commitment facts.

### 4. Runtime should prefer cumulative checkpoints over raw event lists

Real users may have large XC histories.

The runtime should not require thousands of raw source events inside one X1 transaction.

Preferred v1 direction:

- scanner/indexer verifies raw Ethereum/XC events off-chain;
- registrar/approval layer authorizes a cumulative profile checkpoint;
- runtime validates authority, monotonicity, identity, and replay;
- runtime stores accepted cumulative Build facts.

### 5. Replay protection must be runtime-mapped carefully

The lab uses replay sets.

Production runtime should avoid unbounded replay sets inside one Build account.

Candidate runtime strategies:

- marker accounts;
- checkpoint ids;
- monotonic source cursors;
- hybrid scanner replay + runtime checkpoint replay.

Preferred direction:

- off-chain event replay protection for raw events;
- on-chain checkpoint replay and monotonic cursor protection.

### 6. Build Identity is an owner-only metadata layer

Build Identity remains separate from accounting.

Identity updates must not change:

- historyBld;
- originBld;
- historyXbp;
- XNTD commitment;
- X1 fee contribution;
- replay protection.

## Proposed runtime instruction list

Conceptual instructions:

- `create_x1_native_build`
- `gateway_activate_build`
- `gateway_update_build`
- `update_build_identity`
- `apply_x1_fee_checkpoint`

These names are not final API names.

They define the intended responsibility boundaries.

## Non-goals

This checkpoint does not implement:

- X1 program code;
- live watcher service;
- deployment scripts;
- signer keys;
- production bridge execution;
- frontend UI;
- preview DTO;
- spendable BLD token;
- trustless Ethereum proof verification.

## Validation intent

After this checkpoint, validation should remain:

- `npm run typecheck`
- `npm test`
- `npm run build`

No RPC command is required for this docs-first stage.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content should be printed.

# Stage 1.5 Runtime Mapping Notes

## Purpose

Stage 1.5 documents how the completed Stage 1 Gateway deterministic model maps to a future X1 runtime design.

This is not a runtime implementation milestone.

Stage 1.5 does not add production keys, live RPC reads, deployed Ethereum contracts, deployed X1 programs, relayer runtime, watcher runtime, frontend gateway flow, production bridge governance, or deployment logic.

The goal is to preserve the boundary between the pure deterministic model and future runtime-specific concerns.

Stage 1 proved the bridge-to-mint logic as a deterministic model. Stage 1.5 explains what must remain true when that model is later translated into X1-side runtime accounts, instructions, storage, and operational boundaries.

## Stage 1 model baseline recap

Stage 1 Gateway is a pure deterministic model for minting XXXL from validated source-chain burn evidence.

The model covers:

- canonical message encoding
- domain separation
- canonical event key calculation
- X1 recipient hash calculation
- message hash calculation
- route validation
- burn evidence validation
- recipient validation
- amount validation
- Ed25519 guardian signature verification
- approval verification
- guardian quorum verification
- processed burn replay protection
- mint authorization
- mint core state mutation
- state-backed positive end-to-end mint flow
- state-backed negative end-to-end rejection matrix

The Stage 1 flow:

    Stage1GatewayState
    -> executeStage1MintCore()
    -> authorizeStage1Mint()
    -> verifyStage1GuardianQuorum()
    -> verifyStage1Approval()
    -> verifyStage1GatewayMintMessage()
    -> checkStage1BurnNotProcessed()
    -> markStage1BurnProcessed()
    -> mutate recipient balance and totalMinted

Core invariant:

    No XXXL mint state is mutated unless message validation, guardian quorum, and replay protection all pass.

A repeated canonicalEventKey must never mint twice.

## Runtime mapping principles

Future X1 runtime work should preserve the Stage 1 separation of concerns.

Verification layer:

    validates route, evidence, encoding, hashes, recipient, amount, signatures

Authorization layer:

    composes guardian quorum verification and replay protection

Mint core:

    mutates XXXL mint state only after authorization succeeds

Processed burn registry:

    prevents canonicalEventKey replay

Runtime layer:

    provides account storage, instruction atomicity, access boundaries, and operational integration

Stage 1.5 should be treated as a mapping checkpoint before any production runtime code is written.

## X1 account and storage mapping

The Stage 1 in-memory model must eventually map to persistent X1-side runtime state.

Likely runtime state categories:

- gateway config account
- guardian set account
- processed burn registry
- XXXL mint state
- optional accounting / audit state

Gateway config may include route configuration, accepted source chains, accepted source tokens, domain parameters, source chain weights, active guardian set reference, and quorum threshold.

Guardian set state may include guardian public keys, guardian set version, threshold, activation rules, and rotation metadata if rotation is supported.

Processed burn state must store canonical event keys or deterministic entries derived from them.

XXXL mint state must represent total minted amount and recipient balances or the equivalent X1 token mint accounting.

Important open question:

    Should processed burns be stored in a single registry account, sharded accounts, PDA-like accounts by canonicalEventKey, or another X1-native storage model?

The answer depends on X1 account constraints, storage cost, transaction limits, and indexing requirements.

## Processed burn atomicity

Stage 1 uses logical model steps:

    checkStage1BurnNotProcessed()
    markStage1BurnProcessed()
    mint state mutation

In the deterministic model this is safe because execution is synchronous and controlled by the test runtime.

In a real X1 runtime, the processed burn mark and mint state mutation must be atomic.

Required runtime invariant:

    A canonicalEventKey must be marked processed if and only if the corresponding XXXL mint succeeds.

Invalid runtime states to avoid:

- processed burn marked but mint failed
- mint succeeded but processed burn was not marked
- processed burn checked in one transaction and marked in another

Preferred runtime mapping:

    One instruction / transaction path:
      verify message
      verify quorum
      check processed burn
      mark processed burn
      mint XXXL
      commit all state changes atomically

The Stage 1 model proves check-before-mark logic at the model level. Runtime must preserve the same property at the account-write level.

## XXXL mint state mapping

Stage 1 treats XXXL minting as deterministic balance mutation:

    recipient balance += xxxlMintAmount
    totalMinted += xxxlMintAmount

For the Stage 1 gateway baseline, monetary mapping is intentionally simple:

    sourceChainWeightBps = 10000
    xxxlMintAmount = burnedAmount

This preserves the initial one-to-one Stage 1 rule:

    accepted XNTD burn evidence -> equivalent XXXL mint amount

Future runtime design must decide whether XXXL is a native X1 token, a program-controlled token, an account-based balance model, or a ledger entry later mirrored into a token contract or program.

Regardless of runtime representation:

- mint amount is derived from validated message data
- mint amount is not chosen by relayer
- mint amount is not chosen by guardian signatures alone
- mint amount is not mutable by off-chain operator preference

Any future source weighting, source expansion, or fee policy should be introduced as a separate explicitly documented milestone.

## Guardian set management

Stage 1 uses guardian public keys as deterministic test configuration.

Runtime must define how guardian sets are stored, selected, and rotated.

Open runtime questions:

- how is the active guardian set identified?
- can multiple guardian sets be valid during a transition window?
- does the signed message include guardian set version?
- who or what can rotate guardians?
- is rotation governed, admin-controlled, contract-controlled, or epoch-based?
- can guardian rotation exist without compromising immutable core mint rules?

Stage 1 only proves that a given guardian set and threshold can verify approvals deterministically.

Guardian signatures authorize that a source-chain burn event is accepted. They should not be able to arbitrarily choose recipient, amount, route, or monetary rules outside the signed canonical message.

If guardian rotation requires a managed layer, that managed layer should be treated as transport / verification infrastructure, not as mutable monetary core.

## Relayer and watcher boundaries

Stage 1 does not implement watchers or relayers.

Future runtime should preserve this boundary.

Watcher:

    observes source-chain events
    builds canonical evidence
    may submit data for guardian signing
    should not have mint authority by itself

Guardian:

    signs canonical gateway mint message after verifying evidence
    contributes to quorum
    should not mutate X1 state directly unless also acting as relayer

Relayer:

    submits already-approved message and approvals to X1
    pays fees / sends transaction
    should not be trusted for correctness

X1 gateway runtime:

    verifies message, approvals, quorum, replay state
    mints only if all checks pass

Core invariant:

    A malicious or mistaken relayer must not be able to mint XXXL without valid guardian quorum and valid canonical message data.

Relayer submission should be permissionless if practical, because correctness should come from verification, not from trusted submission.

## Recipient validation

Stage 1 currently validates the recipient as a 32-byte non-zero value.

This is enough for deterministic model testing, but runtime may need stronger validation.

Possible future X1 recipient checks:

- recipient must be a valid X1 account or public key
- recipient must not be the zero address / zero public key
- recipient encoding must be canonical
- recipient must match the recipient hash inside the signed message
- recipient may need curve / format validation if X1 account rules require it

Stage 1.5 records that X1 runtime should define the exact recipient format before production minting.

## Burn amount limits

Stage 1 validates:

    burnedAmount > 0
    xxxlMintAmount = burnedAmount

Stage 1 does not define practical min/max limits.

Production design may need additional limits for dust prevention, storage spam prevention, fee economics, source-chain event sanity, overflow safety, per-message maximums, per-route maximums, or per-epoch maximums.

Important distinction:

    A dust/spam minimum is a runtime policy.
    A source-chain weight or mint conversion rule is a monetary policy.
    A hard cap changes economic behavior and should not be added silently.

Stage 1.5 does not choose final limits. It only records that production runtime should make this explicit.

## Message expiry and TTL

Stage 1 includes deadlineOrFinalityBlock, but the baseline does not enforce expiry.

Runtime must decide whether approved messages expire.

Possible models:

- no expiry
- source finality only
- deadline timestamp
- finality block
- guardian-set-bound validity
- epoch-bound validity

Main risk without TTL:

    An old but valid signed message can be submitted much later, as long as it was never processed.

This may be acceptable if the source burn is permanent and replay protection is correct. It may be unacceptable if guardian sets, source-chain policies, or route rules change over time.

TTL is not a Stage 1 blocker, but production runtime must explicitly choose and document the validity model.

## Runtime failure behavior

Rejected runtime paths must not mutate mint state.

Stage 1 negative end-to-end tests already prove this at model level for:

- malformed message fields
- route mismatch
- invalid guardian signature
- unknown guardian approval
- quorum failure
- preprocessed canonicalEventKey replay

Runtime implementation must preserve the same behavior.

If verification fails:

    no mint
    no totalMinted change
    no recipient balance change
    no processed burn mark

If quorum fails:

    no mint
    no totalMinted change
    no recipient balance change
    no processed burn mark

If replay check fails:

    no mint
    no totalMinted change
    no recipient balance change
    no duplicate processed burn entry

## What remains out of scope

Stage 1.5 does not implement:

- real X1 account storage
- atomic X1 instruction execution
- deployed XXXL token runtime
- Ethereum contracts
- X1 programs
- relayer runtime
- watcher runtime
- frontend gateway flow
- deployment logic
- guardian operations
- guardian rotation implementation
- fee collection runtime
- production key management
- production bridge governance
- mainnet deployment
- X1 deployment

Stage 1.5 also does not change Stage 1 generated vectors or deterministic tests.

## Current conclusion

Stage 1 is closed as a deterministic gateway model baseline.

The next runtime-facing work should not rewrite Stage 1. It should map Stage 1 invariants into X1 runtime design while preserving the same separation of concerns:

    verification before authorization
    authorization before mint mutation
    processed burn replay protection before mint
    processed burn mark atomic with mint
    relayer untrusted
    guardian quorum required
    recipient and amount derived from canonical signed message

The most important runtime mapping items are:

- processed burn atomicity
- X1 account/storage layout
- XXXL mint state representation
- guardian set versioning and rotation model
- relayer/watcher boundary
- recipient format validation
- burn amount policy
- message expiry / TTL policy

Until those are documented and reviewed, Stage 2 runtime implementation should not begin.

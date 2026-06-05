# Authoritative XC Epoch Minimum Production Source Plan

## Branch

authoritative-xc-epoch-minimum-production-source-plan

## Purpose

This document defines production-source options for authoritative XC epoch minimum validation.

The deterministic validation chain is already complete in runtime/tests:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> app service
-> registrar handler
-> assertAuthoritativeXcEpochMinimum()
-> Build state

The remaining question is not how to validate.

The remaining question is where the production authoritative source comes from.

This is a plan-only milestone.

It does not change runtime code.

## Current completed state

The runtime can validate:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

when an XcEpochMinimumSource is provided.

Current source interface:

XcEpochMinimumSource {
  authoritativeEpochMinimum(lockEpoch: number): bigint | null;
}

Current deterministic helper:

createStaticXcEpochMinimumSource()

Current validation helper:

assertAuthoritativeXcEpochMinimum()

## Production source requirement

A production source must answer:

What was the authoritative XC Core L1 minimum for this lockEpoch?

It must not be derived from the user-submitted amountXntd.

It must not be derived from observedRequiredXntdLock itself.

It must come from an independent authoritative XC state source.

## Source option A: trusted integration source

A trusted integration service reads XC state and supplies the expected minimum.

Flow:

1. Integration service reads XC epoch / base nominal state.
2. Integration service constructs XcEpochMinimumSource.
3. appSubmitProof() receives the source.
4. Runtime validates observedRequiredXntdLock against the source.

Pros:

- simplest production-like path
- matches current runtime boundary
- easy to test and operate
- no snapshot migration required
- no on-chain verifier required

Cons:

- trust is placed in the integration service
- not fully trust-minimized
- needs operational monitoring

Recommended use:

- first production-like integration
- private beta
- controlled bridge / registrar testing

## Source option B: finalized Ethereum RPC / XC Lens read

The integration reads finalized XC state from Ethereum using XC Core / Lens.

Flow:

1. Read XC epoch state from finalized Ethereum block.
2. Compute / read currentBaseNominal or epoch minimum.
3. Create source from finalized state.
4. Submit proof with xcEpochMinimumSource.

Pros:

- source is grounded in Ethereum state
- closer to production truth
- can use existing XC Core / Lens views
- reduces arbitrary registrar input risk

Cons:

- needs RPC reliability
- needs finalized-block policy
- needs chain reorg handling
- needs ABI/address configuration
- needs error handling for unavailable RPC/state

Recommended use:

- first serious production integration after trusted source
- off-chain registrar / watcher service

## Source option C: checkpoint source

A checkpoint service records accepted XC epoch minimums.

Flow:

1. A checkpoint process periodically records epoch -> minimum.
2. Proof submission uses the checkpoint map.
3. Runtime validates against checkpointed values.

Pros:

- deterministic and reproducible
- can be audited after the fact
- avoids live RPC dependency during submission
- good for bridge / batch flows

Cons:

- checkpoint freshness must be managed
- checkpoint authority/trust model must be defined
- needs policy for stale or missing epochs

Recommended use:

- bridge-style integration
- production systems that prefer stable inputs over live reads

## Source option D: bridge-provided source

The bridge layer supplies authoritative XC epoch minimum as part of a broader cross-chain message.

Flow:

1. Bridge watcher observes XC / Ethereum state.
2. Bridge message includes epoch minimum.
3. Registrar validates observedRequiredXntdLock against bridge-provided source.

Pros:

- fits bridge architecture
- can bundle multiple cross-chain facts
- can be signed by bridge validators / guardians

Cons:

- inherits bridge trust assumptions
- bridge signer policy must be explicit
- not part of immutable core protocol

Recommended use:

- when X1 bridge infrastructure becomes the main transport layer

## Source option E: X1-native verified source

X1 program verifies XC epoch minimum through an on-chain or semi-on-chain mechanism.

Flow examples:

- verified checkpoint account
- light-client-like proof
- signed state root / attestation
- X1-native registrar state

Pros:

- strongest long-term direction
- reduces off-chain trust
- aligns with X1-native Build model

Cons:

- most complex
- likely requires new on-chain design
- may require proof formats, state roots, or verifier programs
- not necessary for the current deterministic runtime model

Recommended use:

- later production hardening
- after bridge / registrar model stabilizes

## Recommended first production-like path

Use Option A first:

trusted integration source

Then evolve to Option B or C:

- finalized Ethereum RPC / XC Lens read
- checkpoint source

Reason:

The current runtime already has the correct boundary:

appSubmitProof(..., xcEpochMinimumSource)

This means production can start by creating a reliable source outside the runtime and passing it in.

Do not persist the source in BuildApplicationState yet.

Do not add snapshot schema changes yet.

Do not introduce live RPC directly into core app state yet.

## Recommended near-term sequence

1. Keep deterministic source for unit/e2e tests.
2. Add a production-source adapter design document.
3. Define finalized block policy.
4. Define XC Core / Lens read fields.
5. Define failure behavior:
   - missing epoch
   - stale source
   - RPC unavailable
   - mismatched minimum
6. Add adapter tests with mocked XC state.
7. Only then decide whether to wire adapter into CLI / service runtime.

## Failure policy

If authoritative minimum is unavailable:

- reject proof submission
- do not mark registrar message as processed
- do not mark XNTD commitment event as used
- do not mutate Build lock state

If observedRequiredXntdLock mismatches authoritative source:

- reject proof submission
- preserve mutation safety
- report explicit mismatch error

Current runtime already supports these error categories:

- MissingAuthoritativeXcEpochMinimum
- MismatchedAuthoritativeXcEpochMinimum

## Non-goals

This plan does not implement:

- real Ethereum RPC reads
- XC Core / Lens ABI integration
- bridge signer logic
- X1 on-chain verification
- snapshot migration
- CLI integration
- persistent app-state source ownership

## Current conclusion

The deterministic validation chain is complete.

The next production-readiness decision is source ownership, not validation mechanics.

Recommended first path:

trusted integration source -> finalized Ethereum RPC / Lens read or checkpoint source

Long-term path:

bridge-provided or X1-native verified source

# Authoritative XC Epoch Minimum Production Source Adapter Design

## Branch

authoritative-xc-epoch-minimum-production-source-adapter-design

## Purpose

This document designs the next production-readiness layer for authoritative XC epoch minimum validation.

The deterministic validation chain is already complete:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> app service
-> registrar handler
-> assertAuthoritativeXcEpochMinimum()
-> Build state

The remaining production question is source construction:

How does a production integration create a trustworthy XcEpochMinimumSource?

This is a design-only milestone.

It does not change runtime code.

## Current completed boundary

Runtime currently accepts:

XcEpochMinimumSource {
  authoritativeEpochMinimum(lockEpoch: number): bigint | null;
}

If provided, the runtime validates:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The source is optional at integration boundaries.

This is intentional.

The core runtime should validate against a source, but should not yet own live Ethereum RPC, bridge state, or persistent source storage.

## Adapter goal

The adapter should turn external XC epoch state into a deterministic XcEpochMinimumSource.

Conceptual adapter responsibility:

external XC state
-> validated epoch minimum records
-> XcEpochMinimumSource

The adapter must not derive the required lock from:

- amountXntd
- observedRequiredXntdLock
- user-provided lock amount
- mutable Build state

The adapter must derive the epoch minimum from an independent XC state source.

## Recommended first adapter type

Start with a mocked / static production-shaped adapter.

Do not start with real RPC.

Reason:

- keeps tests deterministic
- validates source construction policy before network concerns
- avoids introducing RPC secrets / provider config
- avoids snapshot changes
- avoids CLI changes
- avoids ABI/address hardcoding too early

## Proposed adapter interface

A future adapter can expose:

interface XcEpochMinimumSourceAdapter {
  createSource(input: XcEpochMinimumSourceAdapterInput): XcEpochMinimumSource;
}

Input concept:

type XcEpochMinimumSourceAdapterInput = {
  epochs: readonly XcEpochMinimumRecord[];
  finalizedAt?: bigint;
  sourceName: string;
};

Record concept:

type XcEpochMinimumRecord = {
  lockEpoch: number;
  minimumXntd: bigint;
  observedAt: bigint;
  sourceChainId?: string;
  sourceBlockNumber?: bigint;
  sourceBlockHash?: string;
};

This is a design shape only.

Do not implement it in this milestone.

## Finalized block policy

For a real Ethereum / XC Lens read adapter, production should define a finalized block policy before implementation.

Questions to answer:

1. Which block tag is acceptable?
   - finalized
   - safe
   - explicit block number
   - confirmed block with N confirmations

2. What happens if finalized state is unavailable?

3. What happens if RPC nodes disagree?

4. What happens if the epoch boundary is near the current block?

5. Should the source use:
   - current epoch only
   - historical epoch map
   - current + recent epochs
   - checkpointed epoch records

Recommended initial policy:

- do not read latest
- use finalized or explicitly confirmed block
- reject missing data
- reject stale data when policy requires freshness
- prefer deterministic checkpoint records in tests

## XC Core / Lens read fields

A future Ethereum / XC Lens adapter needs to know exactly which fields define the epoch minimum.

Candidate fields:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- genesisTs
- halvingIntervalSec
- initialNominal
- epochAt(timestamp), if available
- currentBaseNominal(), if available
- current protocol parameters exposed through Lens

For XNTD lock validation, the key value is:

authoritativeEpochMinimum(lockEpoch)

In the current design this should equal the XC Core L1 nominal for the relevant epoch.

The adapter should make this explicit:

epoch -> Core L1 minimum nominal

## Epoch selection policy

The lock / relock payload contains:

- lockEpoch
- observedRequiredXntdLock

The adapter must answer the required minimum for lockEpoch.

Open production question:

Should lockEpoch be:

1. supplied by the observed event,
2. computed by the registrar from XC state,
3. cross-checked against event timestamp / block timestamp,
4. or derived from XC epochAt(timestamp)?

Recommended first policy:

- accept lockEpoch from the observed event / proof payload
- validate observedRequiredXntdLock against authoritativeEpochMinimum(lockEpoch)
- later add timestamp / block cross-checks if needed

## Failure behavior

If the adapter cannot produce an authoritative minimum:

- return null for that epoch
- runtime rejects with MissingAuthoritativeXcEpochMinimum

If the adapter produces a minimum and payload differs:

- runtime rejects with MismatchedAuthoritativeXcEpochMinimum

If adapter input is malformed:

- adapter creation should fail before proof submission
- do not create a source from invalid records

Malformed examples:

- negative epoch
- zero minimum
- duplicate epoch records with different minimums
- stale record when freshness is required
- missing source metadata when policy requires it

## Mocked adapter test strategy

Before real RPC, tests should cover a mocked adapter.

Recommended tests:

1. builds XcEpochMinimumSource from valid records
2. rejects duplicate epoch records with conflicting minimums
3. rejects zero minimum records
4. returns null for missing epoch
5. supports multiple epoch records
6. preserves deterministic behavior
7. does not read network
8. does not require secrets

The mocked adapter should be a production-shaped test boundary, not a final production integration.

## Security / trust model

The runtime validates consistency against a provided source.

It does not prove the source is correct.

Therefore production security depends on the source ownership model.

Trust levels:

1. Static test source
   - deterministic tests only

2. Trusted integration source
   - operational trust
   - acceptable for early controlled deployment

3. Finalized Ethereum RPC / Lens source
   - grounded in Ethereum state
   - depends on RPC/finality policy

4. Checkpoint source
   - deterministic and auditable
   - depends on checkpoint authority

5. Bridge-provided source
   - fits bridge model
   - inherits bridge signer assumptions

6. X1-native verified source
   - strongest long-term direction
   - highest implementation complexity

## Recommended implementation sequence

1. Keep current deterministic source as-is.
2. Add adapter design docs first.
3. Add mocked adapter implementation and tests.
4. Add source record validation.
5. Add adapter failure tests.
6. Add docs for finalized block policy.
7. Only later add real Ethereum RPC / Lens adapter.
8. Only later wire adapter into CLI/service runtime.

## Non-goals

This design milestone does not implement:

- adapter runtime code
- real Ethereum RPC
- XC Core ABI
- XC Lens ABI
- provider config
- private keys
- RPC URLs
- snapshot schema changes
- CLI integration
- bridge signer integration
- X1 on-chain verification

## Current conclusion

The next safe production-readiness step is a mocked production-shaped source adapter.

It should validate source records and produce an XcEpochMinimumSource without network access.

Real RPC should come later, after source policy and adapter tests are stable.

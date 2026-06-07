# Stage 1 burn amount policy

This document defines the Stage 1 burn amount policy for the XNTD-to-XXXL Gateway.

This is a design / readiness document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

Stage 1 Gateway converts Ethereum XNTD burn evidence into X1 XXXL mint approval.

This document defines the accepted burn amount policy.

The core rule is:

For Stage 1, XXXL mint amount equals the verified Ethereum XNTD burned amount.

## Source context

This document builds on:

- docs/gateway/stage-1-ethereum-burn-event-schema.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-test-vectors.md
- docs/gateway/stage-1-gateway-theo-review-notes.md
- docs/gateway/stage-1-gateway-pre-implementation-blockers.md
- docs/gateway/stage-1-gateway-hash-signature-recipient-decisions.md
- docs/gateway/stage-1-gateway-mandatory-source-block-fields.md
- docs/gateway/stage-1-x1-mint-core-immutability.md
- docs/gateway/stage-1-processed-burn-atomicity.md
- docs/gateway/stage-1-ethereum-finality-rule.md
- docs/gateway/stage-1-recipient-safety-policy.md

## Stage 1 amount rule

Stage 1 amount rule:

- burnedAmount must be greater than zero
- sourceChainWeightBps must equal 10000
- xxxlMintAmount must equal burnedAmount
- mintToken must be XXXL
- burnedAmount must match the amount emitted by the accepted Ethereum XNTD burn event

This is a full-weight Stage 1 conversion rule.

It does not create a price peg.

It does not mean XXXL is wrapped Ethereum XNTD.

It means Stage 1 uses a 1:1 accounting conversion from verified Ethereum XNTD burn evidence into X1-native XXXL mint amount.

## Zero amount policy

Stage 1 must reject zero burnedAmount.

Reason:

- zero burn has no economic meaning
- zero burn can create spam or meaningless gateway messages
- zero mint approval should not create processed registry entries
- zero amount can hide malformed event handling

Policy:

burnedAmount == 0 must be rejected.

## Minimum amount policy

Stage 1 does not define an arbitrary protocol minimum above zero at the requirement-definition layer.

Reason:

- arbitrary minimums can exclude small users
- arbitrary minimums create unnecessary policy surface
- the gateway should not silently alter the meaning of a valid burn
- spam control should be considered separately from monetary correctness

Frontend may warn users about very small burns if fees make the action economically irrational.

Frontend warning is not the same as protocol rejection.

Future implementation may add a minimum only if there is a clear security, runtime, spam, or UX reason.

If a minimum is added, it must be:

- explicit
- documented
- immutable or clearly separated from monetary discretion
- included in tests
- included in user-facing UI
- reviewed before implementation

## Maximum amount policy

Stage 1 does not define an arbitrary protocol maximum at the requirement-definition layer.

Reason:

- arbitrary maximums can distort verified burn-to-mint accounting
- maximums create additional policy surface
- Stage 1 sourceChainWeightBps already defines conversion
- overflow and runtime limits should be handled as technical safety checks, not discretionary monetary policy

Implementation must still ensure numeric safety:

- burnedAmount must fit the selected integer type
- xxxlMintAmount must fit the selected integer type
- encoding must be fixed-width and non-ambiguous
- overflow must be impossible or rejected
- mint core must reject amounts outside representable bounds

If a maximum is required due to X1 runtime, token mint, storage, compute, or security constraints, it must be documented explicitly before implementation.

## Amount source of truth

The source of truth for burnedAmount is the accepted Ethereum XNTD burn event.

Guardians must verify:

- burn transaction succeeded
- expected XNTD burn event exists
- burnedAmount is emitted by the expected event
- burnedAmount is greater than zero
- burnedAmount matches the canonical message field
- xxxlMintAmount equals burnedAmount
- sourceChainWeightBps equals 10000

Guardians must not choose burnedAmount manually.

Guardians must not choose xxxlMintAmount manually.

## X1 mint core amount verification

Before minting, X1 mint core must verify:

- burnedAmount > 0
- sourceChainWeightBps == 10000
- xxxlMintAmount == burnedAmount
- mintToken == XXXL
- encoded amount is canonical
- amount fits supported range
- no overflow is possible
- guardian signatures are over the exact amount fields

Mint core must reject:

- burnedAmount == 0
- xxxlMintAmount == 0
- xxxlMintAmount != burnedAmount
- sourceChainWeightBps != 10000
- amount encoded as decimal string
- amount encoded with ambiguous precision
- amount outside supported numeric range
- guardian-approved amount that does not match route rule

## No fee subtraction in Stage 1 amount

Stage 1 gateway amount policy does not subtract relayer fees from xxxlMintAmount.

If relayer fees exist later, they must be handled separately from the burn-to-mint amount rule.

Stage 1 route rule remains:

xxxlMintAmount = burnedAmount

No hidden fee, haircut, spread, premium, or multiplier should be applied by guardians or relayers.

## Frontend behavior

Frontend should show:

- burnedAmount
- expected xxxlMintAmount
- sourceChainWeightBps
- warning for zero amount
- optional warning for economically tiny burn amounts
- finality status
- recipient validation status

Frontend must not imply that Stage 1 guarantees market value parity between XNTD and XXXL.

Frontend wording should make clear:

- XNTD is burned on Ethereum
- XXXL is minted natively on X1
- Stage 1 uses full-weight accounting conversion
- XXXL is not wrapped XNTD

## Guardian rejection cases

Guardians must reject evidence if:

- burnedAmount is missing
- burnedAmount is zero
- burnedAmount is malformed
- burnedAmount is encoded ambiguously
- burnedAmount does not match the Ethereum burn event
- xxxlMintAmount does not equal burnedAmount
- sourceChainWeightBps does not equal 10000
- amount exceeds supported encoding / runtime bounds
- amount field appears to be decimal text instead of canonical integer encoding

## Relayer behavior

Relayers must not modify amount fields.

Relayers must not submit alternate xxxlMintAmount.

Relayers must not subtract fees from mint amount.

Relayers must not round amount.

Relayers must not convert decimals.

Relayers must submit the signed message exactly as approved.

Any amount mismatch must be rejected by X1 mint core.

## Processed registry implication

Processed records should store:

- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount

This allows audits to confirm:

- source burn amount
- Stage 1 conversion rule
- X1 mint amount
- no relayer or guardian amount manipulation

## Test vector implications

Future tests and vectors must include:

- valid positive burnedAmount
- zero burnedAmount rejected
- xxxlMintAmount equals burnedAmount
- xxxlMintAmount higher than burnedAmount rejected
- xxxlMintAmount lower than burnedAmount rejected
- sourceChainWeightBps not 10000 rejected
- amount encoded as decimal string rejected
- amount overflow / out-of-range rejected
- guardian-signed wrong amount rejected
- relayer-modified amount rejected
- processed record stores burnedAmount and xxxlMintAmount

## Current conclusion

Stage 1 burn amount policy is:

- reject zero burnedAmount
- do not define arbitrary min/max at the requirement-definition layer
- require burnedAmount to match the accepted Ethereum XNTD burn event
- require sourceChainWeightBps = 10000
- require xxxlMintAmount = burnedAmount
- reject any mismatch or ambiguous amount encoding

This closes the burn amount policy requirement-definition blocker.

Implementation should still not begin until exact cryptographic test vectors and exact X1 deployment authority model are documented.

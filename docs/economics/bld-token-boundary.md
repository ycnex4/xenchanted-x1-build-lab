# BLD token boundary

## Status

Current authoritative boundary decision.

## Decision

Spendable / transferable BLD is a token.

Build does not display, mirror, cache, or expose BLD token balance.

## Layer separation

Build State records protocol history and stable Build facts.

BLD token represents spendable / transferable BLD.

These are separate layers.

## Build State may store

- history_bld
- origin_bld
- history_xbp
- XNTD commitment facts
- X1 fee contribution facts
- Build Identity metadata
- replay protection state

## Build State must not store

- BLD token balance
- available_bld
- spendable_bld
- transferable_bld
- escrowed_bld
- wallet token balance
- token account state
- token allowance / approval state

## Build view

Build view must not show BLD token balance as part of Build State.

A UI may show wallet token balances elsewhere, but that is not Build State and not Build Identity.

## Relock boundary

Relock must not read Build.available_bld because Build.available_bld does not exist.

If a future relock operation requires spendable BLD availability, it must check the BLD token layer directly at operation time.

That check belongs to the operation / token / escrow layer, not to Build State.

## Origin BLD

origin_bld in Build State is not a token balance.

It records the Genesis Origin tier cap reached by the Build.

Any future token minting, claim, escrow, transfer, or burn mechanics must be specified in the BLD token layer.

## External projects

External projects should read:

- Build State for historical contribution and identity
- BLD token accounts for spendable / transferable BLD

They should not treat Build State as a wallet balance.

## Short rule

Build shows who the user is in protocol history.

BLD token shows what spendable BLD the wallet owns.

# XC protocol params build validation design

This document designs how X1 Build validation should consume `XcProtocolParamsSource`.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The project now has a reusable mocked/tested source for deployed xEnchanted Crypto protocol params:

    XcProtocolParamsSource

The source reads deployed XC Lens through:

    getProtocolParams()

It is separate from:

    XcEpochMinimumSource

The source exposes normalized protocol params:

- genesisTs
- halvingInterval
- xenBurnHalvingInterval
- currentEpoch
- nextHalvingTs
- initialNominal
- currentBaseNominal
- initialXenBurn
- currentXenBurnAmount
- enchantMultiplier
- maxLevel
- baseAprBpsNow
- bpsDenom
- earlyPenaltyBps
- maxWalletNfts

## Design goal

Define how future X1 Build validation should use XC protocol params as an authoritative source of current XC economic context.

The goal is to reduce hardcoded XC economic parameters in X1 Build validation.

The validation layer should consume protocol params from `XcProtocolParamsSource` through dependency injection.

The validation layer should remain deterministic and testable with mocked params.

The validation layer should not perform real RPC directly.

## Core principle

X1 Build validation should not guess current XC epoch/economic values.

It should receive them from a source of truth.

For deployed XC mainnet, the source of truth is:

    xEnchantedNFTLens.getProtocolParams()

For local tests, the source of truth should be mocked `XcProtocolParams`.

## Likely authoritative fields

The following fields should be treated as authoritative for future X1 Build validation design:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- halvingInterval
- xenBurnHalvingInterval
- nextHalvingTs
- genesisTs

These fields define the current XC epoch/economic context.

## Fields preserved but not primary

The following fields should still be preserved, but are not expected to be primary Build validation inputs at first:

- initialNominal
- initialXenBurn
- enchantMultiplier
- maxLevel
- baseAprBpsNow
- bpsDenom
- earlyPenaltyBps
- maxWalletNfts

They may be useful for UI, diagnostics, consistency checks, or future policy decisions.

## X1 Build validation scope

This design concerns the validation of XC-derived Build inputs.

It does not design:

- BLD token economics
- market mechanics
- native X1 Build creation through BLD burn
- bridge mechanics
- signer/watcher infrastructure
- real RPC scripts
- transaction sending
- wallet flows

Those remain separate milestones.

## Existing X1 Build requirements to preserve

Existing X1 Build design intent should remain:

- user must have XC history from Ethereum side
- Build creation/update should be tied to real protocol actions
- minimum XC requirements should adapt to XC epoch/economic context where appropriate
- validation should avoid hardcoding protocol economics when Lens values can be used
- validation should remain deterministic and replayable

## Current important XC/X1 relation

For X1 Build participation, the project has already discussed that Ethereum-side participation should not be based only on a simple Core mint/redeem path.

The design direction includes requiring meaningful XC protocol interaction, such as:

- Core NFT redeem history
- Forge NFT mint / XNTD burn path
- XNTD lock requirement tied to current epoch minimums

This design focuses only on how protocol params should feed that validation.

## Proposed validation context model

Future validation should introduce a context object similar to:

    XcProtocolParamsValidationContext

Recommended fields:

- protocolParams
- sourceLabel
- observedAtMs or observedBlock metadata if available later
- validationMode

The first implementation can keep this simple and pass only `protocolParams`.

## Proposed validation input

Future validation functions should accept already-loaded protocol params rather than calling RPC internally.

Example shape:

    validateXcBuildContext({
      proof,
      protocolParams
    })

or:

    createXcBuildValidationService({
      protocolParamsSource,
      proofStore
    })

The first implementation should likely avoid service complexity and start with pure functions.

## Recommended first design target

The first practical use should be a pure helper that derives current XC requirements from protocol params.

Recommended future file:

    src/model/xc-protocol-params-build-validation.ts

Recommended future tests:

    tests/xc-protocol-params-build-validation.test.ts

Possible exported functions:

- deriveCurrentXcBuildRequirements()
- validateXcBuildAgainstProtocolParams()

## Derived requirements

A future helper may derive:

- currentEpoch
- requiredBaseNominal
- requiredXenBurnAmount
- requiredXntdLockMinimum
- requiredForgeMinimum
- epochTiming metadata

The exact naming should be decided in implementation design.

## CurrentBaseNominal use

`currentBaseNominal` should be used as the basis for epoch-aware XC-denominated requirements.

Possible uses:

- minimum XNTD lock size
- current epoch base reference for Build activation
- proof sanity checks
- UI display of current required XC economic baseline

This avoids hardcoding the base nominal in X1 Build logic.

## CurrentXenBurnAmount use

`currentXenBurnAmount` should be used as the authoritative current XEN burn amount required for new Core L1 minting in the current XC epoch.

Possible uses:

- validating whether a Core mint/redeem proof belongs to expected current economic context
- UI display
- replay/snapshot checks
- documenting historical context for Build creation

This value should not be used as a user-entered secret or config.

## CurrentEpoch use

`currentEpoch` should be used to bind validation to the current XC epoch context.

Possible uses:

- epoch-aware requirement calculation
- relock/update rules
- preventing stale assumptions about previous epoch requirements
- display and audit logs

## Halving interval fields

`halvingInterval` and `xenBurnHalvingInterval` should be used as protocol metadata.

Possible uses:

- explain current epoch schedule
- validate expected next transition
- UI countdowns
- diagnostics

They should not be manually redefined in X1 Build logic unless needed as fallback constants in a separate offline/test mode.

## NextHalvingTs and genesisTs

`nextHalvingTs` and `genesisTs` should be used for timing context.

Possible uses:

- UI display
- checkpoint metadata
- consistency checks
- future snapshot/replay validation

Validation should avoid relying on local machine time where on-chain params are available.

## XNTD lock minimum design direction

The earlier X1 Build design direction says XNTD lock should be tied to the minimum nominal of the current epoch, not a fixed 100 XNTD.

Therefore future design should consider:

    requiredXntdLockMinimum = currentBaseNominal

or a multiplier over currentBaseNominal.

The exact multiplier should be decided in a separate validation rules milestone.

This document does not finalize that formula.

## Forge minimum design direction

XC Forge has min/max bounds derived from current base nominal.

Known XC economic direction:

- Forge minimum uses currentBaseNominal based logic
- Forge max uses currentBaseNominal based logic

For X1 Build validation, a future design may require evidence of Forge participation or XNTD burn consistent with current protocol rules.

This design does not implement that rule yet.

## Avoid overcoupling

The validation layer should not require all protocol params if a specific rule only needs a subset.

However, storing the full `XcProtocolParams` snapshot may be useful for auditability.

Recommended approach:

- accept full protocol params
- derive focused requirement objects
- store or log only sanitized non-secret values

## Snapshot / replay considerations

For replay and watcher systems, a Build validation may need to know which protocol params were used at the time of validation.

Future snapshots may include:

- currentEpoch
- currentBaseNominal
- currentXenBurnAmount
- nextHalvingTs
- source lens address
- chain id
- block number if available

The current source does not include block metadata.

Adding block metadata should be a separate milestone.

## Source trust boundary

`XcProtocolParamsSource` reads from deployed XC Lens.

It should be trusted only as a read source.

It does not prove user action by itself.

User action proof still comes from event replay / proof registrar / watcher proof pipelines.

Therefore protocol params source should be used to validate context, not to replace proof validation.

## Error policy

Future validation errors should be sanitized.

Allowed examples:

    Invalid XC protocol params build validation: missing currentBaseNominal
    Invalid XC build requirement: currentBaseNominal must be positive
    XC build validation failed: stale protocol params

Forbidden error content:

- RPC URL
- API key
- raw provider internals
- raw env
- transport config
- authorization headers

## Testing plan

Future implementation should use mocked protocol params only.

Test cases should cover:

1. derives current epoch from protocol params
2. derives current base nominal requirement
3. derives current XEN burn amount requirement
4. preserves currentBaseNominal as bigint
5. preserves currentXenBurnAmount as bigint
6. handles epoch 0
7. handles later epochs
8. rejects zero currentBaseNominal if invalid for rules
9. rejects zero currentXenBurnAmount if invalid for rules
10. does not call real RPC
11. does not import viem
12. does not read process.env
13. does not create public client
14. does not add wallet/transaction paths

## Non-goals for first implementation

The first implementation should not:

- call `XcProtocolParamsSource` directly from pure validation helpers
- execute real RPC
- add a script
- add dependencies
- add viem imports
- add process.env reads
- add bridge logic
- add transaction logic
- finalize all Build economic formulas

## Recommended next milestone after review

Recommended implementation design or direct implementation branch:

    xc-protocol-params-build-validation

But only after review.

Expected first implementation files:

- src/model/xc-protocol-params-build-validation.ts
- tests/xc-protocol-params-build-validation.test.ts
- src/index.ts

The implementation should be pure and mocked.

## Decision

Future X1 Build validation should consume `XcProtocolParams` as the authoritative XC economic context.

The first implementation should derive validation requirements from protocol params without real RPC, without new dependencies, and without changing the existing proof validation model.

This design is accepted for review.

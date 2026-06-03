# Genesis Origin BLD Notes

## Branch

genesis-origin-bld

## Purpose

This branch implements the Genesis Origin BLD claim model for the TypeScript MVP model layer.

Genesis Origin BLD is a one-time allocation based on historical BLD.

## Scope

Included:

- ClaimGenesisOriginBldInput type
- calculateGenesisOriginBld helper
- claimGenesisOriginBld transition
- tiered Genesis Origin allocation
- duplicate claim protection
- not-eligible protection
- GenesisOriginAlreadyClaimed error
- GenesisOriginNotEligible error
- tests for all tier thresholds
- tests proving unrelated layers do not change

Excluded:

- registrar GENESIS_ORIGIN message integration
- genesis_origin_claimed external registry
- signature validation
- Merkle proof logic
- bridge proof logic
- XEN Burn Power accounting
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Tier model

Genesis Origin BLD is calculated from historyBld:

- historyBld >= 1 -> originBld = 11
- historyBld >= 11 -> originBld = 22
- historyBld >= 121 -> originBld = 55
- historyBld >= 1111 -> originBld = 121

121 BLD is the maximum Genesis Origin cap.

## Implemented behavior

claimGenesisOriginBld:

1. rejects if originBld already exists
2. calculates tier from historyBld
3. rejects if historyBld is below eligibility threshold
4. sets originBld
5. adds originBld to availableBld
6. updates updatedAt from claimedAt

## Accounting behavior

Genesis Origin BLD does not increase historyBld.

Genesis Origin BLD increases availableBld because it is usable / spendable BLD.

## Failure behavior

If historyBld is 0:

- originBld must remain 0
- availableBld must remain unchanged
- updatedAt must remain unchanged

If claim is duplicated:

- originBld must remain unchanged
- availableBld must not increase again
- updatedAt must remain at the first successful claim timestamp

## Errors

Added BuildErrorCode values:

- GenesisOriginAlreadyClaimed
- GenesisOriginNotEligible

## Tests

Current Genesis Origin BLD tests verify:

- tiered Genesis Origin BLD calculation
- claims 11 originBld for historyBld >= 1
- claims 22 originBld for historyBld >= 11
- claims 55 originBld for historyBld >= 121
- claims 121 originBld for historyBld >= 1111
- rejects claim when historyBld is zero
- rejects duplicate Genesis Origin claim
- does not create XBP or unrelated accounting values

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 12 test files passed
- 66 tests passed

## Main invariant

Genesis Origin BLD is a one-time allocation derived from historyBld.

It must not create historyBld, XBP, XNTD commitment, or X1 fee contribution.

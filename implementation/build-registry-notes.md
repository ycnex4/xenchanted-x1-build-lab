# Canonical Build Registry Notes

## Branch

build-registry

## Purpose

This branch implements the canonical Build registry model for the TypeScript MVP model layer.

The registry prevents duplicate canonical Build creation before registrar or accounting logic is introduced.

## Scope

Included:

- BuildRegistry structure
- createEmptyBuildRegistry factory
- createRegisteredBuild helper
- duplicate buildId protection
- duplicate owner protection
- duplicate Ethereum identity protection
- tests proving registration does not create accounting value

Excluded:

- registrar messages
- processed_messages replay protection
- Core redeem accounting
- used_redeem_events
- XEN burn accounting
- used_xen_burn_events
- Genesis Origin BLD
- XNTD lock / unlock / relock
- X1 Fee Contribution checkpoints

## Registry fields

BuildRegistry contains:

- buildsById
- canonicalBuildByOwner
- canonicalBuildByEthereumIdentity

## Duplicate protection

createRegisteredBuild rejects:

- duplicate buildId
- duplicate owner
- duplicate Ethereum identity when Ethereum identity is provided

Builds without Ethereum identity can still be created by different owners.

## Errors

Added BuildErrorCode values:

- DuplicateBuildOwner
- DuplicateEthereumIdentity
- DuplicateBuildId

## Tests

Current registry tests verify:

- registers the first canonical Build for an owner
- rejects duplicate buildId
- rejects duplicate owner
- rejects duplicate Ethereum identity
- allows different owners without Ethereum identity
- does not create accounting value when registering a Build

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 4 test files passed
- 18 tests passed

## Main invariant

The registry only controls canonical Build uniqueness.

It must not create BLD, XBP, XNTD commitment, or X1 fee contribution value.

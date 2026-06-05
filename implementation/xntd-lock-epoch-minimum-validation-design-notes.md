# XNTD Lock Epoch Minimum Validation Design Notes

## Branch

required-xntd-lock-epoch-minimum-design

## Purpose

This milestone documents the intended production validation model for XNTD lock / relock required lock amounts.

It does not change runtime code.

## Problem

The current MVP runtime sets:

lockedXntd = amountXntd
requiredXntdLock = amountXntd

This is acceptable under the trusted registrar MVP assumption, but it is not production-complete.

If the registrar submits an amount below the real XC epoch minimum, the MVP model would accept that amount as both the locked amount and the required amount.

That could incorrectly mark a Build commitment as satisfied.

## Intended production rule

The intended production rule is:

requiredXntdLock = current epoch Core L1 nominal from xEnchanted Crypto

Production validation should require:

amountXntd > 0
requiredXntdLock > 0
amountXntd >= requiredXntdLock
requiredXntdLock == authoritativeEpochMinimum(lockEpoch)

## Design document

Added:

- docs/registrar/xntd-lock-epoch-minimum-validation.md

The document separates:

- actual locked amount
- required XNTD lock amount
- authoritative XC epoch minimum
- lockEpoch ordering
- production source of truth

## Linked documents

Updated:

- README.md
- docs/assumptions.md
- docs/registrar/xntd-lock-event-identity.md

These documents now link to the epoch minimum validation design.

## Scope boundary

This milestone does not change:

- lockXntd()
- relockXntd()
- registrar handlers
- proof payloads
- watcher candidates
- snapshot serialization
- CLI output
- tests

## Relationship to existing protections

Already implemented:

- processedMessages
- usedXntdCommitmentEvents
- monotonic lockEpoch guard

Epoch minimum validation is separate.

Replay and ordering protection prevents invalid event reuse or state regression.

Epoch minimum validation prevents under-locking.

## Recommended future implementation

Future runtime implementation should decide:

1. authoritative XC state source
2. whether requiredXntdLock is carried in payload or derived internally
3. how lockEpoch maps to epoch minimum
4. how finalized source context is represented
5. which tests prove under-lock rejection and correct epoch minimum acceptance

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed

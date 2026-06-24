# Gateway full-profile Build activation boundary

## Status

Current design boundary.

This document records the intended ETH/XC gateway behavior for creating or updating X1 Build records.

## Core idea

Build is a public history object.

The main purpose of Build is to let a participant bring their verified contribution history from XEN, xEnchanted Crypto, and X1 into one durable X1 record.

The ETH/XC gateway is therefore not a way to create an empty X1 object.

The ETH/XC gateway imports the participant's verified Ethereum/XC contribution profile into Build.

## Full-profile rule

Gateway activation or update must evaluate the full Ethereum/XC contribution profile for the Ethereum address.

The profile includes:

- Core redeem history -> `history_bld`
- global `XEN.burn` history -> `history_xbp`
- XNTD lock commitment -> `xntd_commitment_accepted`, `locked_xntd`, `required_xntd_lock`, `lock_epoch`

The gateway must not apply only one part of the profile while silently skipping the others.

## Zero history vs unchecked history

A verified zero is a valid scan result.

A verified zero Core redeem history is not sufficient for gateway Build activation.

An unchecked source is not allowed.

Examples:

- Core redeem scan completed and found zero history: valid scan result, but gateway Build activation is rejected because minimum Core redeem history is missing.
- XEN.burn scan completed and found zero history: valid result.
- Core redeem scan failed or was not performed: no gateway state transition.
- XEN.burn scan failed or was not performed: no gateway state transition.
- XNTD lock verification failed or was not performed for a new gateway Build: no gateway state transition.

## New Build through ETH/XC gateway

A new Build created through the ETH/XC gateway must not be empty.

For a new gateway-created Build, the gateway must:

1. verify the Ethereum address profile as one complete bundle;
2. scan Core redeem history;
3. scan global `XEN.burn` history;
4. verify accepted XNTD lock commitment;
5. require XNTD lock amount to satisfy the epoch minimum;
6. require minimum Core redeem history;
7. create the Build only with the verified profile facts applied.

For epoch `0`, the required XNTD lock minimum is `100000000`.

Minimum Core redeem history means existing `history_bld > 0` or at least one validated incoming Core redeem proof.

If the XNTD lock requirement is not satisfied, the gateway must not create the Build.

## Existing Build through ETH/XC gateway

For an existing Build, gateway update must also use a full-profile scan or a verified full-profile delta/checkpoint.

Gateway update may import newly verified contribution facts:

- additional Core redeem history;
- additional global `XEN.burn` history;
- accepted XNTD commitment facts if the Build was not yet committed;
- updated lock facts if a valid relock/update rule applies.

If the Build is already `COMMITTED`, temporary RPC, indexer, or gateway unavailability must not downgrade the Build.

Infrastructure failure means the new operation does not execute.

It does not mutate public Build status.

## Commitment rule

`COMMITTED` means the Build has an accepted historical XNTD commitment record.

It is a project-recognized contribution signal.

It is not a live RPC-derived status.

For a new gateway-created Build, accepted XNTD lock is mandatory.

For an existing Build that is not yet committed, gateway activation into `COMMITTED` requires accepted XNTD lock.

## X1-native Build distinction

X1-native Build creation may create a clean `UNCOMMITTED` Build shell.

That is not the ETH/XC gateway path.

A clean X1-native Build may exist as an X1 identity/history container without imported Ethereum/XC history.

A gateway-created Build must represent verified Ethereum/XC contribution history and must not be an empty shell.

## No partial import

The gateway must not verify only XNTD lock while skipping Core redeem and `XEN.burn` scans.

The gateway must scan Core redeem history, scan `XEN.burn` history, verify XNTD lock, and apply the complete verified profile bundle.

## Short rule

The ETH/XC gateway does not issue an empty Build.

The ETH/XC gateway imports the participant's verified contribution history.

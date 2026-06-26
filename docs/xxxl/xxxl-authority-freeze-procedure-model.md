# XXXL Authority Freeze Procedure Model

## Purpose

This document defines the first authority freeze procedure model for XXXL Program v1.

This is still planning / model work.

It is not production X1 runtime code.

It does not execute a live freeze.

## Core principle

Authority freeze is not discretionary guardian administration.

The policy is:

    Rules decide eligibility.
    Public timelock gives review window.
    Guardians attest and execute.
    Freeze removes upgrade and supply authority.

## Distinction from emergency freeze

Emergency freeze is a protective incident response action.

Authority freeze is a lifecycle transition where staged upgrade authority is removed or permanently frozen.

These are different procedures.

Emergency freeze can be a fast security response.

Authority freeze must be public, timelocked, prerequisite-bound, and final.

## Authority states

The model recognizes these authority states:

- STAGED_FINALIZATION
- FREEZE_PROPOSED
- FROZEN
- FREEZE_CANCELLED

A valid authority freeze proposal transitions:

    STAGED_FINALIZATION -> FROZEN

Invalid proposals preserve the current authority state.

## Mandatory prerequisites

Authority freeze requires:

- runtime schema complete
- transition semantics complete
- route policy complete
- incident policy complete
- deployment dry run accepted
- public disclosure ready
- freeze plan ready
- X1-native mechanics complete
- review completed

The key trust boundary is that authority freeze should not happen before deterministic X1-native mechanics are complete, reviewed, and documented.

## Guardian role

Guardians do not receive arbitrary freeze power.

Guardians act as threshold signers of a public, timelocked, prerequisite-bound procedure.

The model requires:

    authorityFreezeThreshold >= emergencyFreezeThreshold >= guardianQuorumThreshold

This prevents final authority freeze from being weaker than emergency freeze or ordinary guardian quorum.

## Timelock

Authority freeze requires a minimum public timelock of 7 days.

The proposal cannot execute before the timelock ends.

## Forbidden post-freeze capabilities

The freeze proposal must remove:

- program upgrade
- manual mint
- premine
- founder allocation
- hidden emission
- balance rewrite
- gateway bypass
- arbitrary mint path
- discretionary supply control

If any of these remain, the proposal is invalid.

## Post-freeze allowed actions

Only predeclared deterministic actions may remain available after freeze.

Candidate allowed actions:

- consume gateway mint
- route pause
- emergency freeze
- guardian rotation
- public notice
- post-mortem
- route retirement

These actions must be part of the already-defined protocol surface, not hidden upgrade authority.

## Evidence requirements

A proposal must include identifiers for:

- public disclosure
- freeze plan
- dry-run evidence
- review evidence

Missing evidence makes the proposal invalid.

## Non-goals

This stage does not implement:

- live freeze instruction
- production guardian signatures
- deployment scripts
- RPC usage
- secret handling

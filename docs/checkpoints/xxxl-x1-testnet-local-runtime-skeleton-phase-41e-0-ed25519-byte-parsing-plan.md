# XXXL X1 Testnet Local Runtime Skeleton — Phase 41E.0 Checkpoint

Date: 2026-07-02

## Phase

Phase 41E.0 — Ed25519 instruction byte parsing plan.

## Parent Checkpoint

`99ba836 Merge XXXL phase 41D3 structural prior lookup closure`

## Scope

Docs-only plan.

No runtime code is introduced.

## Purpose

Plan the next non-authorizing boundary after structural prior-instruction lookup:

- parse real Ed25519 instruction bytes;
- classify malformed layouts deterministically;
- extract non-authorizing parsed metadata;
- do not verify signatures;
- do not accept proof/evidence;
- do not count quorum;
- do not authorize execution.

## Required Guardrails

Future Phase 41E code must not gate on:

- `locates_prior_ed25519_instruction`.

Future Phase 41E code must gate on both:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

Future Phase 41E code must not trust Phase 41D3.2.3 descriptor booleans as validated evidence.

## Explicitly Closed

- Ed25519 cryptographic verification;
- proof acceptance;
- evidence acceptance;
- guardian quorum;
- authorization;
- replay writes;
- account mutation;
- CPI;
- mint;
- handler;
- live route.

## Review Requirement

External review is required before Phase 41E byte parsing code begins.

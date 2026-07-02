# XXXL X1 Testnet Local Runtime Skeleton — Phase 41F.0 Checkpoint

Date: 2026-07-02

## Phase

Phase 41F.0 — Ed25519 cryptographic verification plan.

## Parent Checkpoint

`2f759b7 Merge XXXL phase 41E offset table hardening acceptance record`

## Scope

Docs-only plan.

No runtime code is introduced.

## Purpose

Plan the next trust-sensitive boundary after Phase 41E byte parsing completion.

## Core Guardrail

Signature validity must remain separate from:

- proof acceptance;
- evidence acceptance;
- guardian validity;
- quorum;
- authorization;
- replay writes;
- mutation;
- CPI;
- mint;
- handler;
- live route.

## Preferred Split

Recommended future split:

- Phase 41F.1 — checked byte extraction boundary;
- Phase 41F.2 — Ed25519 signature verification boundary.

This split must be confirmed by external review.

## Review Requirement

External review is required before Phase 41F.1 begins.

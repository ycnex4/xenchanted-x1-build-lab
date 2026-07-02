# XXXL X1 Testnet Local Runtime Skeleton — Phase 41F.2 Checkpoint

Date: 2026-07-02

## Phase

Phase 41F.2 — Ed25519 signature verification boundary plan.

## Parent Checkpoint

`f5c9c7f Merge XXXL phase 41F checked extraction acceptance record`

## Scope

Docs-only plan.

No runtime code is introduced.

## Purpose

Plan the Ed25519 signature verification boundary after Phase 41F.1 checked byte extraction.

## Core Guardrail

Signature validity must remain separate from:

- proof acceptance;
- evidence acceptance;
- guardian validity;
- guardian set membership;
- quorum;
- authorization;
- replay writes;
- mutation;
- CPI;
- mint;
- handler;
- live route.

## Required Carry-Forward Notes

Before implementation, future work must address:

- `SAFETY_FLAGS` cumulative-vs-local semantics;
- Model A soundness documentation;
- self-reference binding preservation;
- program-id defense-in-depth re-check;
- status model attribution by verification model.

## Review Requirement

External review is required before Phase 41F.2 implementation begins.

# XXXL X1 Testnet Local Runtime Skeleton — Phase 41F.1 Checkpoint

Date: 2026-07-02

## Phase

Phase 41F.1 — checked Ed25519 byte extraction boundary.

## Parent Checkpoint

`e45869c Merge XXXL phase 41F Ed25519 verification plan acceptance record`

## Scope

Runtime-model boundary plus docs.

## Code

`programs/xxxl-svm/src/verifier/checked_ed25519_byte_extraction_boundary.rs`

Module registration:

`programs/xxxl-svm/src/verifier/mod.rs`

## Purpose

Extract parsed Ed25519 signature/public-key/message ranges through checked borrowed references only.

## Allowed

- checked extraction of signature bytes as `&[u8; 64]`;
- checked extraction of public key bytes as `&[u8; 32]`;
- checked extraction of message bytes as borrowed `&[u8]`;
- no attacker-sized message `Vec` copy.

## Forbidden

- cryptographic verification;
- native verification establishment;
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

## Review Requirement

External review is required before Phase 41F.2 begins.

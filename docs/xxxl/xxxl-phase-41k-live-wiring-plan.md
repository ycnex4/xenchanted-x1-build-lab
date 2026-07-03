# XXXL Phase 41K — Live-Wiring Plan

Date: 2026-07-03

Status: draft plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-live-wiring-plan`

Base checkpoint:

`03b1e4f Merge XXXL phase 41J replay protection implementation acceptance`

## Purpose

Phase 41K is the first planned transition from the accepted boundary model toward real X1 / SVM runtime wiring.

Until 41J, the verifier pipeline proves the logical safety chain:

`raw_payload_bytes -> internal 41I quorum authorization -> internal decode -> canonicalEventKey -> authoritative abstract processed-registry view -> eligibility / intent`

41K must plan how that logical model becomes real runtime wiring without breaking the accepted guarantees.

## High-Risk Status

41K is a high-risk gate.

It introduces surfaces that were intentionally forbidden through 41J:

- `AccountInfo`;
- Instructions sysvar loading;
- real guardian-set account loading;
- real processed-registry PDA loading;
- replay registry write;
- processed event marking;
- account mutation;
- CPI;
- SPL token mint;
- instruction handler;
- live route.

None of these may be enabled casually.

Each transition from logical boundary model to runtime behavior must be separately reviewed.

## Non-Goal of This Document

This document does not implement 41K.

This document does not authorize live execution.

This document does not select production addresses.

This document does not remove deployment blockers.

This document only defines the safe plan for future 41K implementation gates.

## Accepted Previous Guarantees

41K must preserve all accepted guarantees from earlier phases.

### 41F.1

Checked Ed25519 byte extraction from a prior instruction.

### 41F.2

Native SVM Ed25519 verification evidence is accepted only through the modeled native verifier boundary.

### 41G

Guardian signed message must equal `canonical_hash(raw_payload_bytes)`.

### 41H

Signer must be a member of the authoritative guardian set.

### 41H.1

Decoded payload binding must not be caller-supplied.

### 41H.2

Signed message bytes must be extracted from verified Ed25519 instruction bytes, not provided freely.

### 41I

Quorum authorization counts only successful, distinct, authoritative guardian approvals.

### 41J

Replay eligibility is derived only from:

`internal 41I over same raw_payload_bytes -> internal decode -> canonicalEventKey -> authoritative processed-registry view`

No external 41I result, free replay key, free decoded payload, or caller-supplied processed status is allowed.

## 41K Core Rule

41K must not weaken the chain:

`real Instructions sysvar -> checked extraction -> native Ed25519 verification -> payload hash binding -> guardian membership -> quorum -> same raw payload decode -> canonicalEventKey -> real processed-registry PDA -> atomic check-mark-mint`

Any implementation path that lets the caller inject, substitute, or desynchronize one of these elements is invalid.

## Required Sub-Gates

41K should be split into sub-gates.

### 41K.1 — Real Instructions Sysvar Loading Plan / Implementation

Goal:

Load the real Instructions sysvar and derive the prior Ed25519 instruction data from runtime state.

Must preserve:

- no caller-provided instruction bytes;
- no frontend-provided Ed25519 proof;
- no watcher-provided Ed25519 proof;
- no standalone verification evidence;
- no fabricated prior instruction entries.

Output:

A runtime-derived structure that can feed the accepted 41F.1 / 41F.2 boundaries.

### 41K.2 — Real Guardian-Set Account Loading

Goal:

Load the authoritative guardian set from a real program-controlled account or PDA.

Must preserve:

- no caller-supplied guardian set;
- no frontend-supplied guardian set;
- no watcher-supplied guardian set;
- guardian set id must match the payload and expected configured id;
- threshold and keys must come from the authoritative account.

Output:

An `AuthoritativeGuardianSetRef` constructed only from real program-controlled account data.

### 41K.3 — Real Processed-Registry PDA Loading

Goal:

Load the processed-event registry from a real program-controlled PDA.

Must preserve:

- no caller-supplied processed status;
- no frontend-supplied processed status;
- no watcher-supplied processed status;
- no empty fabricated registry view;
- no arbitrary account accepted as registry;
- registry PDA seeds and ownership must be checked.

Output:

An authoritative processed-registry view constructed only from real program-controlled PDA data.

### 41K.4 — Atomic Check-Mark-Mint Design

Goal:

Perform replay check, processed-event marking, and mint/action atomically.

Forbidden partial-commit windows:

- marked-but-not-minted;
- minted-but-not-marked;
- marked wrong event;
- minted for payload A while marking payload B;
- registry read from account A but write to account B;
- eligibility derived from one raw payload but mint performed from another.

Required:

The same decoded, quorum-authorized raw payload must drive:

- replay key;
- recipient;
- mint amount;
- target mint;
- processed-event mark;
- mint instruction.

### 41K.5 — Handler / CPI / Live Route

Goal:

Only after 41K.1–41K.4 are accepted, wire the instruction handler and CPI path.

Must remain separately reviewed:

- process instruction entrypoint;
- account order;
- signer seeds;
- SPL token mint authority;
- target mint account;
- recipient token account;
- failure behavior;
- replay behavior;
- route enablement.

## Mandatory Atomicity Rule

The future live instruction must be single-transaction atomic.

A successful transaction may end in exactly one valid state:

- source event is marked processed;
- corresponding XXXL mint/action is completed;
- all fields come from the same quorum-authorized raw payload.

A failed transaction must leave no processed mark and no mint/action.

## Forbidden 41K Design Patterns

41K must reject any design that:

- accepts external 41I results;
- accepts free signed message bytes;
- accepts free decoded payload;
- accepts free replay key;
- accepts free canonical event key;
- accepts caller-provided processed status;
- builds authoritative registry view from instruction data;
- builds authoritative guardian set from instruction data;
- lets watcher output become authority without on-chain verification;
- checks one registry account and writes another;
- authorizes payload A but mints payload B;
- marks event A but mints event B;
- writes processed state before all required checks are complete;
- performs mint before replay protection is secured.

## Required 41K Review Questions

Before any 41K implementation is accepted, reviewers must answer:

1. Are all runtime-derived inputs truly loaded from runtime state?
2. Is the Instructions sysvar path real and non-fabricated?
3. Is the guardian set loaded only from an authoritative program-controlled account?
4. Is the processed registry loaded only from an authoritative program-controlled PDA?
5. Is the same raw payload used for quorum, decode, replay key, recipient, amount, and mint?
6. Is replay protection enforced before mint?
7. Is processed marking atomic with mint?
8. Are all partial-commit windows closed?
9. Are handler/CPI/live route surfaces separately gated?
10. Does any caller-supplied field become authority?

## Current Plan Status

This is a plan document only.

No 41K code is implemented by this document.

No runtime account loading is enabled by this document.

No handler is added by this document.

No live route is enabled by this document.

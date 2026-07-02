# XXXL Phase 41D3.2.0 — Prior Enumeration / Checked Loading Plan

Date: 2026-07-02

## Status

Planning document only.

No runtime code is introduced in Phase 41D3.2.0.

## Parent Gate

Phase 41D3.1 was accepted by external review.

Accepted parent checkpoint:

`e52d8ac Merge XXXL phase 41D3 current index external acceptance record`

Accepted current-index boundary:

- `load_current_index_checked` is allowed;
- current index is ordering-only;
- `current_index == 0` is safe only if downstream prior lookup remains fail-closed;
- no prior instruction can exist before index `0`.

## Purpose

Define the minimum safe implementation plan for the next runtime sub-step:

Phase 41D3.2:

- prior-instruction enumeration;
- checked prior-instruction loading;
- prefiltering unrelated instructions;
- Phase 41C3 candidate descriptor construction;
- explicit same-index reject;
- explicit later-index reject;
- prior-index candidates only.

## Why Split This Step

The full accepted Phase 41D3 boundary is intentionally split because this is the first point where real runtime instruction loading will be introduced.

Primary risks to isolate:

- panic-safety around real runtime instruction loading;
- bounded prior-index enumeration;
- explicit same/later rejection;
- no accidental proof/evidence acceptance;
- no accidental authorization;
- no accidental mutation/CPI/mint behavior.

## Proposed Micro-Phases

### Phase 41D3.2.1 — Prior Index Range Enumeration Only

Allowed:

- accept a checked current instruction index from Phase 41D3.1 result;
- derive a bounded prior index range;
- enumerate only indexes strictly less than current index;
- `current_index == 0` produces an empty prior range;
- no instruction loading yet.

Forbidden:

- `load_instruction`;
- `load_instruction_at`;
- `load_instruction_at_checked`;
- raw sysvar byte parsing;
- Ed25519 candidate construction;
- Phase 41C3 descriptors;
- proof/evidence/auth/replay/CPI/mint.

Expected tests:

- current index `0` => empty prior range;
- current index `1` => `[0]`;
- current index `n` => `0..n`;
- same index is not included;
- later index is impossible from range construction;
- no panic / no unchecked indexing.

### Phase 41D3.2.2 — Checked Prior Instruction Loading

Allowed:

- call `load_instruction_at_checked` for indexes from the already-bounded prior range;
- map checked loading failures to deterministic rejection/skip behavior;
- preserve fail-closed result if loading fails.

Forbidden:

- `load_instruction`;
- unchecked instruction loading;
- raw sysvar byte parsing;
- Ed25519 cryptographic verification;
- evidence acceptance;
- quorum/auth/replay/CPI/mint.

Expected tests:

- checked load failure is handled without panic;
- no `unwrap`, `expect`, `panic!`, `unsafe`, unchecked indexing, unchecked slicing;
- loading is only attempted for prior indexes;
- no same/later index can be loaded through this boundary.

### Phase 41D3.2.3 — Prefilter + Phase 41C3 Descriptors

Allowed:

- prefilter unrelated instructions;
- identify Ed25519 program-id candidates structurally;
- construct Phase 41C3 candidate descriptors;
- feed descriptors into the already-existing Phase 41C3 ordering model;
- explicitly reject same-index and later-index candidates at runtime boundary;
- flip `locates_prior_ed25519_instruction: true`;
- flip `load_instruction_called: true` only if checked loading is used.

Forbidden:

- Ed25519 cryptographic verification;
- signature proof acceptance;
- verification evidence acceptance;
- guardian quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- runtime handler;
- live route unlock;
- deployment readiness claims.

Expected tests:

- unrelated instruction => discarded;
- wrong program id => descriptor marks non-Ed25519 candidate;
- structurally malformed candidate => descriptor rejects structurally;
- duplicate guardian evidence remains descriptor-level only;
- same-index candidate => explicit runtime reject;
- later-index candidate => explicit runtime reject;
- prior valid candidate => passed to Phase 41C3 model;
- ambiguous candidate set remains non-authorizing;
- no proof/evidence/auth is accepted.

## Required Safety Rules

### Ordering

All runtime lookup must obey:

- valid prior candidate index `< current_index`;
- same index `== current_index` is rejected;
- later index `> current_index` is rejected;
- `current_index == 0` yields no prior candidates.

### Loading

Only checked loading may be used:

- allowed: `load_instruction_at_checked`;
- forbidden: `load_instruction`;
- forbidden: unchecked loading;
- forbidden: raw sysvar byte parsing.

### Evidence Boundary

Phase 41D3.2 may locate and describe candidates, but must not accept evidence.

Candidate descriptor does not mean:

- proof is valid;
- signature is valid;
- guardian is authorized;
- quorum is reached;
- mint is authorized;
- replay registry may be updated;
- runtime state may mutate.

### Runtime Mutation Boundary

Phase 41D3.2 remains read-only.

It must not:

- mutate accounts;
- mark burns/events as processed;
- mint tokens;
- invoke CPI;
- call `invoke_signed`;
- open live route;
- claim deployment readiness.

## Safety Flags Expected After 41D3.2.3

Expected allowed flips only after code implementation and review:

- `locates_prior_ed25519_instruction: true`;
- `load_instruction_called: true`, only if `load_instruction_at_checked` is used.

Must remain false:

- raw parser flag;
- cryptographic verification;
- verification evidence acceptance;
- guardian quorum;
- authorization;
- replay protection writes;
- runtime mutation;
- CPI;
- mint execution;
- live route.

## Phase 41D3.2.0 Output

This document is only a plan and review target.

It does not implement:

- prior enumeration;
- checked instruction loading;
- prefilter;
- descriptors;
- same/later reject;
- any runtime behavior.

## Review Gate

Do not start Phase 41D3.2.1 code until this plan is reviewed and accepted.

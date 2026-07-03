# XXXL Phase 41I — Quorum Counting / Threshold Authorization Plan

Date: 2026-07-03

Status: planning only

Parent accepted main:

`7579c14 Merge XXXL phase 41H decoded payload binding hardening acceptance`

Parent accepted gate:

Phase 41H.1 — Decoded Payload Binding Hardening

## Purpose

Phase 41I is the next trust boundary after 41H.

41H proves that one verified signer is a member of the authoritative guardian set for one hash-bound payload.

41I must compose multiple 41H validations into a threshold quorum marker.

The purpose of 41I is:

- count unique validated guardian memberships;
- enforce the authoritative guardian-set threshold;
- reject duplicate guardians;
- reject insufficient quorum;
- reject mixed guardian sets;
- preserve payload binding across every counted guardian;
- output a quorum authorization marker only.

41I must not execute the mint.

## Current State Before 41I

The accepted chain is:

1. 41F.1 extracts checked Ed25519 byte slices.
2. 41F.2 establishes native SVM Ed25519 verification.
3. 41G binds raw payload bytes to the signed message.
4. 41H validates one verified signer as an authoritative guardian-set member.
5. 41H.1 hardens 41H so decoded payload fields come only from the same raw bytes hash-bound by 41G.

After 41H.1, a free caller-provided decoded payload is no longer accepted.

## Why Phase 35 Cannot Be Reused Directly

The old Phase 35 structural quorum verifier is useful as prior design material for:

- empty guardian set rejection;
- invalid threshold rejection;
- duplicate guardian rejection;
- duplicate approval rejection;
- unknown guardian rejection;
- quorum-not-reached rejection.

But Phase 35 must not be treated as a proof or authorization source.

Phase 35 operates over structural approval claims.

41I must operate over 41H-validated memberships, not caller-provided approval claims.

## Core Design Rule

41I must count only guardians that have passed 41H.

No caller-provided guardian approval claim may be counted directly.

No public key may be counted unless it is produced by a successful 41H membership validation.

No decoded payload may be supplied freely.

No hash-binding marker may be supplied freely.

## Preferred Implementation Model

Preferred model: 41I composes 41H internally.

41I should receive:

- one authoritative guardian set wrapper;
- one expected configured guardian set ID;
- one raw payload byte slice;
- one signed message byte slice;
- multiple verification attempts, each containing:
  - a 41F.2 native Ed25519 verification result;
  - a 41F.1 checked byte extraction result.

For each attempt, 41I calls 41H with the same:

- raw payload bytes;
- signed message bytes;
- expected configured guardian set ID;
- authoritative guardian set wrapper.

Then 41I counts only successful 41H outputs.

This model preserves the 41H.1 closure:

- the same raw payload bytes are used for every guardian;
- the same signed message bytes are used for every guardian;
- each counted guardian is individually verified through 41F.1, 41F.2, 41G, and 41H;
- no free `GuardianMembershipValidated` list can be forged by the caller.

## Alternative Model

Alternative model: 41H success markers could be extended with explicit payload context before 41I consumes them.

That would require adding fields such as:

- bound payload hash;
- signed message hash or equivalent binding context;
- guardian set ID;
- matched guardian public key.

Then 41I could safely consume a list of 41H success markers only if every marker carries the same payload context.

This model is not preferred for the first 41I implementation because it requires changing the 41H success type.

## Proposed 41I Success Marker

The future code boundary should return a marker similar to:

`GuardianQuorumAuthorizationEstablished`

It should include:

- status;
- guardian set ID;
- threshold;
- guardian count;
- unique validated guardian count;
- quorum reached;
- counted guardian public keys or indexes;
- local marker `quorum_counting_enabled = true`;
- local marker `authorization_enabled = true`.

All execution-related flags must remain false.

Required false flags:

- replay write enabled: false;
- processed event marking enabled: false;
- account mutation enabled: false;
- CPI enabled: false;
- invoke_signed enabled: false;
- SPL token mint_to enabled: false;
- process instruction handler added: false;
- live route enabled: false.

## Proposed Error Kinds

The future code boundary should reject:

- phase 41H validation failure;
- empty verification attempt list;
- empty guardian set;
- invalid threshold zero;
- threshold exceeds guardian count;
- duplicate guardian public key in guardian set;
- duplicate validated guardian;
- mixed guardian set ID;
- payload binding mismatch;
- quorum not reached.

If 41I composes 41H internally, payload binding mismatch should normally surface as a 41H `PayloadHashBindingNotEstablished` or related 41H error.

41I may wrap that as a 41H validation failure while preserving the inner 41H error.

## Counting Invariants

41I must enforce:

1. Every counted guardian passed 41H.
2. Every counted guardian belongs to the same authoritative guardian set.
3. Every counted guardian is bound to the same raw payload bytes.
4. Every counted guardian is bound to the same signed message bytes.
5. A guardian public key can be counted at most once.
6. Threshold zero is invalid.
7. Threshold greater than guardian count is invalid.
8. Unique validated guardian count must be greater than or equal to threshold.
9. No state can change on failure.
10. Success only produces a marker.

## Expected Tests

The future implementation should include tests for:

- valid 1-of-1 quorum;
- valid 2-of-3 quorum;
- threshold not reached;
- empty attempt list;
- duplicate validated guardian rejected;
- unknown guardian rejected through 41H;
- mixed guardian set rejected;
- payload substitution rejected through 41H/41G;
- one invalid 41F result causes fail-closed behavior;
- caller-supplied or unauthenticated guardian set still rejected through 41H;
- downstream execution flags remain false;
- forbidden runtime surfaces remain absent.

## Forbidden In 41I

41I must not add:

- runtime account loading;
- `AccountInfo`;
- sysvar loading;
- replay registry writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL token `mint_to`;
- instruction handler;
- live route;
- production program ID;
- production guardian account loading.

## Active Blockers Remain

No blocker is removed by 41I planning.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Review Questions

Reviewers should check:

1. Is 41I correctly scoped as quorum counting / threshold authorization only?
2. Is the preferred model, where 41I composes 41H internally, safer than consuming free 41H markers?
3. Are the same raw payload bytes and signed message bytes preserved across all counted guardians?
4. Are duplicate guardians rejected?
5. Is threshold enforcement sufficient?
6. Is quorum failure fail-closed?
7. Are 41H errors preserved sufficiently?
8. Are forbidden runtime surfaces excluded?
9. Are downstream mutation/CPI/mint/handler/live flags still false?
10. Is this plan sufficient before writing 41I code?

## Pre-Review Clarifications

These clarifications are part of the 41I planning boundary.

### Duplicate Signer Protection

41I must reject duplicate validated signers.

If the same guardian public key appears in more than one successful 41H result, that guardian may be counted only once.

Preferred behavior:

- reject duplicate validated guardian as an error;
- do not silently deduplicate;
- do not allow one guardian to satisfy multiple threshold slots.

This is separate from duplicate guardian keys inside the authoritative guardian set.

41H validates membership for one signer.

41I owns multi-signer uniqueness.

### Sequential Validation

41I must process every verification attempt through the same 41H validation path.

No optimization may skip:

- 41F.1 checked extraction;
- 41F.2 native Ed25519 verification result check;
- 41G payload hash binding;
- 41H guardian membership validation.

A guardian may be counted only after the full 41H path succeeds.

### Quorum Marker Shape

The future 41I success marker should be structurally similar to:

`GuardianQuorumAuthorizationEstablished`

Required fields:

- status;
- guardian set ID;
- guardian count;
- threshold;
- unique validated guardian count;
- counted guardian public keys or indexes;
- quorum reached;
- quorum counting enabled;
- authorization enabled.

All execution flags must remain false.

### 41I To 41J Transition

The 41I authorization marker does not activate runtime mutation.

41I does not mark processed events.

41I does not write replay state.

41I does not mint.

The next future gate after accepted 41I is 41J:

- replay protection;
- processed event marking;
- atomic state-transition planning.

41J must be reviewed separately.

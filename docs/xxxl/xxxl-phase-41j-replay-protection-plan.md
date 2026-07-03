# XXXL Phase 41J — Replay Protection / Processed Event Marking Plan


## Required Fixes Reconciliation

Audit Demon identified two blocking gaps in the initial 41J plan.

This base plan is amended by:

`docs/xxxl/xxxl-phase-41j-replay-protection-plan-fixes.md`

That fixes document supersedes any earlier wording implying that 41J may accept a standalone 41I result plus separate `raw_payload_bytes` without payload commitment.

Updated binding model:

41J must internally compose accepted 41I over the same `raw_payload_bytes` that is later decoded for `canonicalEventKey`.

Updated implementation scope:

41J is a non-mutating boundary model. It may return replay eligibility / processed-marking intent, but it must not perform runtime account loading or real processed-event marking.

The following remain forbidden in 41J boundary code:

- AccountInfo;
- sysvar loading;
- runtime account loading;
- real replay registry write;
- real processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL mint_to;
- handler;
- live route.
Date: 2026-07-03

Status: planning

Base main:

`63373c4 Merge XXXL phase 41I quorum authorization implementation acceptance`

## Purpose

Phase 41J introduces the replay-protection boundary after accepted Phase 41I quorum authorization.

The goal is to prevent the same Ethereum burn / canonical event from being authorized more than once.

41J must bind replay protection to the canonical payload, not to caller-provided claims.

## Upstream Preconditions

41J may only run after accepted 41I success.

Accepted 41I proof target:

Every counted guardian passed hardened 41H.2 and proved:

`41F-verified extracted message == canonical_hash(raw_payload_bytes)`

41I also established:

- shared `raw_payload_bytes`;
- shared expected guardian set ID;
- shared authoritative guardian set;
- successful distinct guardian count >= threshold;
- no free signed message;
- no free decoded payload;
- no free approval claims;
- no execution authority.

## 41J Core Invariant

A canonical event may be processed at most once.

The replay key is:

`canonicalEventKey`

The replay key must be derived from internally decoded `raw_payload_bytes`.

41J must not accept a caller-provided replay key as authority.

## Required Flow

The accepted 41J flow must be:

1. Receive accepted 41I quorum result.
2. Require 41I quorum success.
3. Receive the same `raw_payload_bytes` that was authorized by 41I.
4. Internally decode `raw_payload_bytes`.
5. Derive `canonicalEventKey` from decoded payload.
6. Read authoritative processed-event registry.
7. Fail if `canonicalEventKey` is already processed.
8. Only after successful replay check, allow processed-event marking.
9. Mark exactly that `canonicalEventKey` as processed.
10. Return a boundary result that proves replay protection was applied.

## Check-Before-Mark Rule

41J must preserve check-before-mark ordering.

Invalid order:

- mark first, then check;
- mark even if quorum failed;
- mark caller-provided key;
- mark different key than decoded payload key;
- mark without deriving from `raw_payload_bytes`.

Required order:

`41I success -> decode raw payload -> derive canonicalEventKey -> check not processed -> mark processed`

## Atomicity Requirement

In future runtime wiring, replay check and processed-event mark must be atomic in one transaction.

No future mint or state mutation may happen unless replay marking and downstream action are composed safely.

For this plan, no mint/CPI/live route is introduced.

## Registry Source

Processed-event registry must be authoritative.

Allowed future source:

- program-controlled processed-event account / PDA.

Forbidden sources:

- caller instruction data as authority;
- unauthenticated in-memory claims;
- frontend-provided processed status;
- watcher-provided processed status without on-chain verification.

## Marker Scope

41J may introduce replay-specific state marking only after review.

It must not introduce:

- token minting;
- CPI;
- `invoke_signed`;
- SPL `mint_to`;
- route execution;
- handler activation;
- live program deployment.

## Boundary Result Shape

The future 41J result should expose:

- phase;
- version;
- raw payload decoded internally;
- canonical event key derived internally;
- quorum authorization consumed;
- replay registry source;
- replay key already processed flag;
- replay check passed flag;
- processed event marking status;
- replay write scope;
- execution flags.

## Success Criteria

A 41J success means:

- accepted 41I quorum result was consumed;
- decoded payload came from the same raw payload;
- canonical event key was derived internally;
- processed registry was authoritative;
- event was not previously processed;
- exactly this canonical event key is eligible to be marked processed.

## Failure Criteria

41J must fail if:

- 41I result is not successful;
- 41I result does not establish logical quorum authorization;
- raw payload cannot be decoded;
- decoded canonicalEventKey is missing or malformed;
- registry source is not authoritative;
- canonicalEventKey is already processed;
- attempted mark key differs from decoded canonicalEventKey;
- any caller-supplied processed status is used as authority.

## Still Forbidden After 41J

Unless separately reviewed in later phases, the following remain forbidden:

- account mutation outside replay registry;
- CPI;
- `invoke_signed`;
- SPL token mint;
- instruction handler;
- live route;
- production program ID;
- production guardian account loading;
- target mint account mutation.

## Tests Required For 41J Code

Future 41J implementation must include tests for:

- accepted 41I result + unprocessed event succeeds;
- already processed canonicalEventKey is rejected;
- quorum failure cannot mark processed;
- caller-provided replay key cannot substitute decoded key;
- raw payload decode failure fails closed;
- processed registry from caller data is rejected;
- mark-before-check is impossible;
- mismatched mark key is rejected;
- replay write scope is limited to processed registry;
- CPI/mint/handler/live route flags remain false.

## Review Focus

Reviewers should verify:

- replay key comes only from internally decoded raw payload;
- processed registry authority is explicit;
- check-before-mark ordering is preserved;
- atomicity requirement is documented for future runtime wiring;
- no execution route is introduced;
- 41I logical quorum marker is consumed safely;
- no free decoded payload or free replay key exists.

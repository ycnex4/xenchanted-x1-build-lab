# XXXL Phase 41J — Replay Protection Plan Required Fixes

Date: 2026-07-03

Status: required plan fixes after Audit Demon review

Base plan commit:

`7f8a173 Document phase 41J replay protection plan`

## Verdict Context

Audit Demon verdict:

`REQUIRES FIXES`

Two blocking issues were identified:

1. `raw_payload_bytes` was not bound to the 41I-authorized payload.
2. The plan was ambiguous about whether 41J introduces real runtime account access and mutation.

This document fixes both issues and supersedes conflicting wording in the base 41J plan.

## Fix 1 — Bind 41J Raw Payload To 41I Authorization

The original plan said that 41J receives an accepted 41I quorum result and the same `raw_payload_bytes`.

That was insufficient because the accepted 41I result does not currently carry a payload commitment:

- no raw payload;
- no canonical event key;
- no payload hash.

Therefore a caller could theoretically pair:

`41I result for payload A`

with:

`raw_payload_bytes for payload B`

and then mark the replay key for B even though quorum authorized A.

## Accepted Fix 1 Model

41J must internally compose 41I over the same `raw_payload_bytes`.

41J must not accept a prebuilt 41I result as authority unless that result carries an explicit payload commitment and 41J verifies it.

For the current 41J boundary, use the safer model:

`41J composes 41I internally`

Required flow:

1. 41J receives guardian attempts, `raw_payload_bytes`, expected guardian set ID, authoritative guardian set, and abstract processed-registry view.
2. 41J calls accepted 41I internally with exactly that `raw_payload_bytes`.
3. 41I calls hardened 41H.2 per attempt with the same `raw_payload_bytes`.
4. Every counted guardian therefore proves:

   `41F-verified extracted message == canonical_hash(raw_payload_bytes)`

5. Only after internal 41I quorum succeeds may 41J decode that same `raw_payload_bytes`.
6. 41J derives `canonicalEventKey` from the internally decoded payload.
7. Replay eligibility is computed for that derived `canonicalEventKey`.

This restores the binding:

`quorum authorization -> same raw payload -> canonicalEventKey -> replay eligibility`

## Forbidden 41J Inputs

41J must not accept as authority:

- prebuilt 41I result without payload commitment;
- caller-provided replay key;
- caller-provided canonical event key;
- caller-provided decoded payload;
- caller-provided processed status;
- free signed message bytes;
- free guardian approval claims.

## Fix 2 — 41J Is Non-Mutating Boundary Model

The current 41J implementation phase must be a non-mutating boundary model.

41J code must not perform real runtime account access or real registry writes.

41J may model replay protection over an abstract processed-registry view and return eligibility / intent.

## Accepted 41J Code Scope

Allowed in 41J boundary code:

- compose accepted 41I internally;
- decode `raw_payload_bytes` internally;
- derive `canonicalEventKey` internally;
- check an abstract processed-registry view;
- reject already processed events;
- return replay eligibility / processed-marking intent;
- preserve audit fields.

Forbidden in 41J boundary code:

- `AccountInfo`;
- sysvar loading;
- runtime account loading;
- real account mutation;
- replay registry write;
- processed event marking write;
- CPI;
- `invoke_signed`;
- SPL token `mint_to`;
- instruction handler;
- live route;
- production program ID;
- production guardian account loading;
- target mint account mutation.

## Required Flags For 41J Boundary Code

The 41J boundary result may expose:

- `replay_check_passed: true`;
- `processed_marking_eligible: true`;
- `processed_marking_intent: true`.

But the following must remain false:

- `replay_write_enabled: false`;
- `processed_event_marking_enabled: false`;
- `account_mutation_enabled: false`;
- `runtime_account_loading_enabled: false`;
- `sysvar_loading_enabled: false`;
- `cpi_enabled: false`;
- `invoke_signed_enabled: false`;
- `spl_token_mint_to_enabled: false`;
- `process_instruction_handler_added: false`;
- `live_route_enabled: false`.

## Check-Before-Eligibility

Because 41J is currently non-mutating, the immediate boundary rule is:

`41I internal success -> decode same raw payload -> derive canonicalEventKey -> check abstract registry -> return eligibility`

Do not describe 41J boundary code as actually marking processed state.

Actual check-before-mark is deferred to a future live-wiring phase.

## Future Live-Wiring Atomicity

Future runtime wiring must perform replay check, processed-event mark, and downstream mint/action atomically.

Forbidden future failure windows:

- marked-but-not-minted;
- minted-but-not-marked;
- marked wrong event;
- minted for payload A while marking payload B.

Future live-wiring must be separately reviewed before enabling:

- AccountInfo;
- runtime account loading;
- registry write;
- processed event marking;
- CPI;
- mint;
- handler;
- live route.

## Replay Key Granularity

Replay uniqueness must use `canonicalEventKey`.

Reason:

`canonicalEventKey` is the canonical identity of the source burn event.

It represents the event being consumed.

`messageNonce` is not the replay identity. A message nonce may identify a guardian message or authorization envelope, but replay protection must prevent the same source event from being processed more than once.

Therefore 41J uses:

`canonicalEventKey`

not:

`messageNonce`

as the replay uniqueness key.

## Updated Review Target

After these fixes, reviewers should verify:

- 41J internally composes 41I over the same raw payload;
- no external 41I result can be paired with a different raw payload;
- canonicalEventKey is derived only after internal 41I success;
- 41J is non-mutating;
- AccountInfo/sysvar/runtime-account-loading are forbidden;
- replay writes and processed marking remain disabled;
- the boundary returns only replay eligibility / intent;
- future live-wiring is explicitly deferred.

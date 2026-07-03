# XXXL Phase 41I — Quorum Authorization Plan Resumption Addendum

Date: 2026-07-03

Status: resumed planning after accepted 41H.2

Base main:

`52d6a77 Merge XXXL phase 41H signed message binding hardening implementation acceptance`

## Reason

The original Phase 41I plan was blocked by Audit Demon because 41H did not yet prove that the Ed25519-verified message was the same message used by 41G as the payload hash.

That upstream gap is now closed by accepted Phase 41H.2.

Accepted 41H.2 proof target:

`guardian signed canonical_hash(raw_payload_bytes)`

## Resumed 41I Assumption

41I may now compose hardened 41H.2 internally.

Every counted guardian must pass 41H.2.

Therefore every counted guardian proves:

`41F-verified extracted message == canonical_hash(raw_payload_bytes)`

## Required 41I Rules

41I must enforce:

- same `raw_payload_bytes` for all attempts;
- same `expected_configured_guardian_set_id` for all attempts;
- same authoritative guardian set for all attempts;
- no caller-provided approval claims;
- no free decoded payload;
- no free signed message;
- no free 41G/41H markers;
- count only successful 41H.2 validations;
- dedup counted guardians by `matched_guardian_index` and public key;
- one guardian may satisfy only one threshold slot;
- failed attempts are not counted;
- failed attempt errors are preserved for auditability;
- quorum succeeds only if unique successful distinct guardians >= threshold.

## Failed Attempt Model

Accepted safer model:

`count-only-successful-distinct >= threshold`

Do not use all-or-nothing.

A failed attempt must not kill a valid M-of-N quorum, but it must be recorded in per-attempt audit output.

## Authorization Flag Caution

41I must not expand execution authority.

If 41I introduces a logical quorum marker, it remains non-executing.

No replay write, processed event marking, state mutation, CPI, mint, handler, or live route may be introduced.

Whether an `authorization_enabled` field may flip true must be explicitly reviewed in 41I review before code.

## Still Forbidden

41I must not add:

- replay registry writes;
- processed event marking;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint_to;
- instruction handler;
- live route;
- production program ID;
- production guardian account loading.

## Review Focus

Reviewers must check:

- 41I correctly composes accepted 41H.2;
- same payload / same guardian-set context is enforced;
- duplicate guardian counting is impossible;
- failed attempts are preserved but not counted;
- threshold is enforced from authoritative set;
- no free marker injection exists;
- no execution authority is introduced.

# XXXL Phase 41J — Replay Protection Plan Review Request

Date: 2026-07-03

Status: review request

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41j-replay-protection-plan`

Base main:

`63373c4 Merge XXXL phase 41I quorum authorization implementation acceptance`

Review document:

`docs/xxxl/xxxl-phase-41j-replay-protection-plan.md`

## Scope

Docs-only planning review.

No 41J code has been written.

## Review Target

Confirm whether the Phase 41J replay protection / processed event marking plan is safe before implementation.

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- 41I dependency correctly incorporated: yes/no
- canonicalEventKey derived from raw payload: yes/no
- free replay key rejected: yes/no
- free decoded payload rejected: yes/no
- authoritative processed registry model accepted: yes/no
- check-before-mark ordering accepted: yes/no
- atomicity requirement sufficient: yes/no
- processed marking scope sufficiently narrow: yes/no
- forbidden runtime surfaces absent: yes/no
- plan sufficient before 41J code: yes/no


## Required Fixes Update

Audit Demon initially returned:

`REQUIRES FIXES`

Blocking issues:

1. `raw_payload_bytes` was not bound to the 41I-authorized payload.
2. The plan was ambiguous about real runtime account access / mutation.

Fix document:

`docs/xxxl/xxxl-phase-41j-replay-protection-plan-fixes.md`

Updated review focus:

- 41J internally composes 41I over the same raw payload;
- no standalone 41I result can be paired with a different raw payload;
- 41J is non-mutating boundary code;
- AccountInfo/sysvar/runtime-account-loading are forbidden;
- replay write and processed marking remain disabled;
- output is replay eligibility / intent only;
- real registry write + atomic mint/live route are deferred to a separate future gate.

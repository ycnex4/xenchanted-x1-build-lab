# XXXL Phase 41I — Quorum Authorization Plan Review Request

Date: 2026-07-03

Status: review request

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41i-quorum-authorization-plan`

Parent accepted main:

`7579c14 Merge XXXL phase 41H decoded payload binding hardening acceptance`

Primary plan:

`docs/xxxl/xxxl-phase-41i-quorum-authorization-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41i-quorum-authorization-plan.md`

## Review Scope

This is a docs-only planning review.

No 41I code has been written.

No `.rs` files are changed.

Phase 41I is scoped to quorum counting and threshold authorization planning only.

## Proposed Direction

41I should compose 41H internally.

For every verification attempt, 41I should call 41H using the same:

- raw payload bytes;
- signed message bytes;
- expected configured guardian set ID;
- authoritative guardian set wrapper.

41I should count only successful 41H guardian membership validations.

41I must not count caller-provided approval claims.

41I must not accept a free decoded payload.

41I must not accept a free payload-binding marker.

## Intended Output

The future 41I code boundary should output only a quorum authorization marker.

It may set local 41I markers:

- quorum counting enabled: true;
- authorization enabled: true;
- quorum reached: true.

All execution-related markers must remain false:

- replay write enabled: false;
- processed event marking enabled: false;
- account mutation enabled: false;
- CPI enabled: false;
- invoke_signed enabled: false;
- SPL token mint_to enabled: false;
- instruction handler added: false;
- live route enabled: false.

## Main Questions

Please review:

1. Is 41I correctly scoped as quorum counting / threshold authorization only?
2. Is composing 41H internally the safest model?
3. Is it correct that 41I counts only successful 41H validations?
4. Is it correct to reject caller-provided approval claims as a direct input?
5. Is same raw payload + same signed message binding preserved across all counted guardians?
6. Is duplicate guardian rejection sufficient?
7. Is threshold enforcement sufficient?
8. Is quorum failure fail-closed?
9. Are 41H errors preserved sufficiently?
10. Are forbidden runtime surfaces excluded?
11. Is this plan sufficient before writing 41I code?

## Forbidden In This Phase

41I planning must not introduce:

- runtime account loading;
- AccountInfo;
- sysvar loading;
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

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- 41I scope correct: yes/no
- internal 41H composition preferred: yes/no
- caller-provided approvals excluded: yes/no
- same payload/message binding preserved: yes/no
- duplicate guardian rejection sufficient: yes/no
- threshold enforcement sufficient: yes/no
- fail-closed behavior sufficient: yes/no
- forbidden runtime surfaces absent: yes/no
- plan sufficient before code: yes/no

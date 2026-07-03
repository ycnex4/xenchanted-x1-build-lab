# XXXL Phase 41I — Resumed Quorum Authorization Plan Acceptance

Date: 2026-07-03

Status: accepted resumed plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41i-quorum-authorization-plan-resumed`

Accepted commits:

- `1f8050e Resume phase 41I quorum authorization plan after 41H hardening`
- `93fe61c Reconcile phase 41I plan with hardened 41H signed message model`

Base main:

`52d6a77 Merge XXXL phase 41H signed message binding hardening implementation acceptance`

## Final Verdict

Phase 41I resumed quorum authorization plan is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Plan sufficient before 41I code: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- 41H.2 dependency correctly incorporated: yes
- internal 41H.2 composition accepted: yes
- same raw payload enforced: yes
- same guardian set / expected ID enforced: yes
- duplicate guardian counting prevented: yes
- failed attempt model accepted: yes
- per-attempt errors preserved: yes
- authorization flag semantics safe: yes
- forbidden runtime surfaces absent: yes
- plan sufficient before 41I code: yes

## Note Resolution

Demon noted that the base 41I plan still contained stale free `signed_message_bytes` wording.

Resolved in commit:

`93fe61c Reconcile phase 41I plan with hardened 41H signed message model`

Accepted current model:

41I must not accept, thread, compare, or pass free signed message bytes.

41I composes hardened 41H.2 internally.

Every counted guardian must pass 41H.2 and prove:

`41F-verified extracted message == canonical_hash(raw_payload_bytes)`

## Accepted 41I Rules

41I must enforce:

- same `raw_payload_bytes` for all attempts;
- same authoritative guardian set for all attempts;
- same expected configured guardian set ID for all attempts;
- count only successful 41H.2 validations;
- dedup counted guardians by matched guardian index and public key;
- failed attempts are not counted;
- failed attempt errors are preserved;
- quorum succeeds only if unique successful distinct guardians >= threshold;
- no replay write, mutation, CPI, mint, handler, or live route.

## Next Step

After this acceptance is merged, Phase 41I code implementation may begin on a separate branch under high-risk audit.

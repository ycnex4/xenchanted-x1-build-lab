# XXXL Phase 41I — Quorum Authorization Boundary Implementation Acceptance

Date: 2026-07-03

Status: accepted implementation

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41i-quorum-authorization-implementation`

Accepted commit:

`9677edf Implement phase 41I quorum authorization boundary`

Base main:

`4d5955c Merge XXXL phase 41I resumed quorum authorization plan acceptance`

## Final Verdict

Phase 41I quorum authorization boundary implementation is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: none
- Code sufficient for 41I acceptance: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- same raw payload shared across all attempts: yes
- same expected guardian set ID shared across all attempts: yes
- same authoritative guardian set shared across all attempts: yes
- no stale signed_message_bytes threading: yes
- duplicate guardian cannot satisfy threshold twice: yes
- per-attempt errors preserved: yes
- authorization_enabled false safe: yes
- logical_quorum_authorization_established is non-executing: yes
- forbidden runtime surfaces absent: yes
- 41I quorum boundary accepted: yes

## Accepted Implementation

41I introduces a dedicated quorum authorization boundary:

`programs/xxxl-svm/src/verifier/quorum_authorization_boundary.rs`

The implementation composes hardened 41H.2 internally.

Each counted guardian must pass 41H.2 and therefore proves:

`41F-verified extracted message == canonical_hash(raw_payload_bytes)`

## Accepted 41I Properties

41I enforces:

- one shared `raw_payload_bytes` for all attempts;
- one shared `expected_configured_guardian_set_id` for all attempts;
- one shared authoritative `guardian_set` for all attempts;
- no free `signed_message_bytes`;
- no free decoded payload;
- no free guardian approval claims;
- count only successful 41H.2 validations;
- dedup by matched guardian index and guardian public key;
- duplicate guardian cannot satisfy threshold twice;
- failed attempts are preserved in per-attempt outcomes;
- failed attempts are not counted;
- failed attempts do not kill a valid M-of-N quorum;
- quorum succeeds only if successful distinct guardians >= threshold.

## Execution Boundary

41I does not introduce execution authority.

Accepted flags:

- `logical_quorum_authorization_established: true` only on success;
- `quorum_counting_enabled: true`;
- `authorization_enabled: false`;
- `replay_write_enabled: false`;
- `processed_event_marking_enabled: false`;
- `account_mutation_enabled: false`;
- `cpi_enabled: false`;
- `invoke_signed_enabled: false`;
- `spl_token_mint_to_enabled: false`;
- `process_instruction_handler_added: false`;
- `live_route_enabled: false`.

## Non-Blocking Note

`AuthoritativeGuardianSetRef::from_program_controlled_on_chain_source` was widened from private to `pub(crate)` so the 41I module and tests can construct authoritative wrappers.

This is accepted because:

- it is not public outside the crate;
- `wrapper_constructor_publicly_unrestricted` remains false;
- no production account-loading path exists yet;
- no live route exists yet.

Future live-wiring must ensure that authoritative wrappers are constructed only from real on-chain guardian-set data.

As the crate grows, consider one of:

- a test-only constructor;
- a construction token;
- a dedicated guardian-set account-loading factory.

This is deferred to the future live-wiring / account-loading audit.

## Tests

Accepted test results:

- focused `quorum_authorization_boundary` tests: OK;
- full `xxxl-svm` tests: OK;
- safety scan: OK.

## Next Step

After this acceptance is merged into `main`, Phase 41J may begin as a separate high-risk gate.

41J scope:

- replay protection;
- processed event marking;
- no account mutation beyond the explicitly reviewed replay boundary;
- no CPI;
- no mint;
- no handler;
- no live route unless separately reviewed.

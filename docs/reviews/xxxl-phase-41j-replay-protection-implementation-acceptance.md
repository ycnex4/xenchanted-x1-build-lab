# XXXL Phase 41J — Replay Protection Boundary Implementation Acceptance

Date: 2026-07-03

Status: accepted implementation

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41j-replay-protection-implementation`

Accepted commit:

- `2f78b49 Implement phase 41J replay protection boundary`

Base main:

`bdb7569 Merge XXXL phase 41J replay protection plan acceptance`

## Final Verdict

Phase 41J replay protection boundary implementation is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: 1
- Note: `caller_instruction_data` is redundant with `source == CallerInstructionData`, but harmless defense-in-depth.

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: 1
- Note: `pub(crate)` authoritative registry constructor is acceptable for the boundary model, but future 41K must ensure the real authoritative registry view is constructed only from a real on-chain processed-registry PDA.

## Accepted Implementation

41J implements a non-mutating replay eligibility boundary.

Accepted flow:

`raw_payload_bytes -> internal 41I quorum authorization -> internal raw payload decode -> canonicalEventKey -> authoritative abstract processed-registry view -> eligibility / intent`

## Verified Properties

- 41J composes 41I internally over the same `raw_payload_bytes`.
- 41J does not accept an external 41I result.
- `canonicalEventKey` is derived only from the internally decoded, internally authorized raw payload.
- Replay key is `canonicalEventKey`, not `messageNonce`.
- Caller-supplied registry views are rejected.
- Unauthenticated registry views are rejected.
- Already processed canonical event keys are rejected.
- Quorum failure cannot reach decode, replay key derivation, or eligibility.
- Payload substitution attack is rejected by internal 41I / 41H.2 binding.

## Mutation / Runtime Surfaces

All remain closed:

- `replay_write_enabled: false`
- `processed_event_marking_enabled: false`
- `account_mutation_enabled: false`
- `runtime_account_loading_enabled: false`
- `sysvar_loading_enabled: false`
- `cpi_enabled: false`
- `invoke_signed_enabled: false`
- `spl_token_mint_to_enabled: false`
- `process_instruction_handler_added: false`
- `live_route_enabled: false`

## Future Gate

41K live-wiring remains a separate high-risk gate.

Future 41K must separately review:

- real Instructions sysvar wiring;
- real guardian-set loading;
- real processed-registry PDA loading;
- authoritative registry-view construction from real PDA only;
- atomic check-mark-mint;
- handler wiring;
- CPI / mint;
- live route.

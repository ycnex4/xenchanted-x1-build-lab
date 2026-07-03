# XXXL Phase 41J — Replay Protection Plan Acceptance

Date: 2026-07-03

Status: accepted fixed plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41j-replay-protection-plan`

Accepted commits:

- `7f8a173 Document phase 41J replay protection plan`
- `0ab2603 Address phase 41J replay protection plan review fixes`

Base main:

`63373c4 Merge XXXL phase 41I quorum authorization implementation acceptance`

## Final Verdict

Phase 41J replay protection / processed event marking plan is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: none
- Fix 1 closed: yes
- Fix 2 closed: yes
- Plan sufficient before 41J code: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- Fix 1 closed, raw payload bound to internal 41I: yes
- Fix 2 closed, non-mutating model: yes
- no external 41I result can be paired with different raw payload: yes
- canonicalEventKey derived only after internal 41I success: yes
- free replay key rejected: yes
- free decoded payload rejected: yes
- AccountInfo/sysvar/runtime-account-loading forbidden: yes
- replay write and processed marking disabled: yes
- output is eligibility / intent only: yes
- future live-wiring explicitly deferred: yes
- plan sufficient before 41J code: yes

## Accepted 41J Model

41J must internally compose accepted 41I over the same `raw_payload_bytes`.

Accepted flow:

`raw_payload_bytes -> internal 41I quorum -> internal decode same raw_payload_bytes -> derive canonicalEventKey -> abstract processed-registry check -> replay eligibility / processed-marking intent`

41J must not accept a standalone 41I result plus separate raw payload as authority.

## Accepted Boundary Scope

41J is a non-mutating boundary model.

Allowed:

- compose accepted 41I internally;
- decode `raw_payload_bytes` internally;
- derive `canonicalEventKey` internally;
- check abstract processed-registry view;
- return replay eligibility / processed-marking intent.

Forbidden:

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

Required false flags:

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

## Non-Blocking Code Note

The future 41J code should model the abstract processed-registry view with authoritative-source provenance, similar to `AuthoritativeGuardianSetRef`.

The model should reject:

- caller-supplied processed registry;
- unauthenticated registry view;
- frontend-provided processed status;
- watcher-provided processed status without on-chain verification.

Real on-chain registry reading remains deferred to future live-wiring.

## Replay Key

Replay uniqueness uses:

`canonicalEventKey`

Reason:

`canonicalEventKey` identifies the source burn event being consumed.

`messageNonce` is not the replay identity.

## Future Gate

Real registry write, processed event marking, handler wiring, CPI/mint, and atomic check-mark-mint remain deferred to a separate high-risk live-wiring gate.

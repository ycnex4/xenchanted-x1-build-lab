# XXXL Phase 41E.2 Offset-Table Alias Hardening — External Acceptance

Date: 2026-07-02

Current main under review:

`43e6860 Merge XXXL phase 41E offset table alias hardening`

## Scope Accepted

Phase 41E.2 is accepted as parser hardening only.

Accepted scope:

- reject parsed signature/public-key/message ranges that alias the Ed25519 header/offset-table range `[0, 16)`;
- require all parsed ranges to start at or after `ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- fail deterministically with `ParsedRangeAliasesOffsetTable`;
- keep the boundary non-authorizing;
- introduce no cryptographic verification;
- introduce no proof/evidence/quorum/auth/replay/mutation/CPI/mint/handler/live route.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- offset-table alias hardening is correct;
- all three ranges must begin at offset `>= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- failure is deterministic and non-authorizing;
- all trust-sensitive flags remain false;
- Phase 41F.0 plan may begin after acceptance.

Theo confirmed Phase 41E is complete:

- 41E.0 — byte parsing plan;
- 41E.1 — byte parsing boundary;
- 41E.2 — offset-table alias hardening.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Offset-table alias hardening acceptable: yes.

Forbidden operations detected: no.

Trust-sensitive boundary drift: no.

Next phase allowed: yes.

Demon accepted:

- Phase 41E.2 precisely closes the Phase 41E.1 non-blocking note;
- threshold `>= 16` is correct;
- offset table is `[0,16)`;
- `offset == 16` is the first allowed data byte;
- the change only narrows parser acceptance;
- nothing new is accepted;
- no new loading, arithmetic, crypto, CPI, mutation, mint, handler, or live route was introduced;
- the new flag `rejects_offset_table_aliasing` is only a parser capability marker.

## Accepted Strict Rule

The parser now requires:

- `signature_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- `public_key_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`;
- `message_offset >= ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN`.

For the current single-signature layout:

- `ED25519_SINGLE_SIGNATURE_OFFSET_TABLE_LEN == 16`.

Therefore parsed ranges must not alias:

- Ed25519 header/offset-table bytes `[0,16)`.

## Accepted Failure Mode

If any parsed range starts inside `[0,16)`, the parser fails closed with:

- `ParsedRangeAliasesOffsetTable`.

This failure is deterministic and non-authorizing.

## Accepted Test Coverage

Phase 41E.2 adds tests for:

- signature range aliasing offset table;
- public key range aliasing offset table;
- message range aliasing offset table.

Each test confirms:

- `status == ParsedRangeAliasesOffsetTable`;
- `rejects_offset_table_aliasing == true`;
- `parses_ed25519_instruction_bytes == false`.

## Still Forbidden

The following remain forbidden after Phase 41E.2 acceptance:

- Ed25519 cryptographic verification;
- signature validity acceptance;
- guardian validity acceptance;
- cryptographic signature proof acceptance;
- verification evidence acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- process instruction handler;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41E.2.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41E is now complete.

Accepted Phase 41E pipeline:

- 41E.0 — Ed25519 byte parsing plan;
- 41E.1 — Ed25519 byte parsing code boundary;
- 41E.2 — offset-table alias hardening.

Next recommended phase:

- Phase 41F.0 — Ed25519 cryptographic verification plan.

Phase 41F.0 must be docs-only first.

Future 41F work must remain under a separate reviewed boundary and must not imply proof acceptance, guardian validity, quorum, authorization, replay writes, account mutation, CPI, mint, handler, or live route.

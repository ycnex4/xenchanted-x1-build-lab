# Phase 41K.6 B5.5 — Negative matrix and closure checklist

## Purpose

B5.5 records the negative matrix and closure checklist for the Phase 41K.6 watcher/relayer integration path.

B5.5 does not remove gates.

B5.5 does not introduce live RPC.

B5.5 does not sign, submit, simulate, spend SOL, or access private keys.

B5.5 is a closure-control checkpoint for B5.

## B5 implemented boundaries

B5 established the following boundaries:

- B5.1 watcher/relayer schema reconciliation inventory.
- B5.2 candidate payload v2 hash conversion.
- B5.3 quorum package boundary.
- B5.4 relayer submission package boundary.

## B5.1 coverage

B5.1 reconciled old Stage 2 watcher/relayer fields with the new Phase 41K.6 handler boundary.

Key decisions:

- guardianSetVersion becomes guardian_set_id bytes32.
- recipientBase58 must be clarified into recipient owner versus recipient token account.
- expectedMintedAmountOverride remains test-only unless later policy explicitly reintroduces it.
- sourceFinalityState remains watcher/finality metadata.
- operational ids remain non-payload-bound metadata.
- payload-bound fields must not drift after guardian signing.

## B5.2 coverage

B5.2 implemented deterministic payload v2 hash conversion.

Covered positive behavior:

- deterministic hash construction,
- sha256/hashv-compatible field concatenation,
- domain binding,
- amount encoded as u64 little-endian.

Covered negative and drift behavior:

- changing processed_event changes the payload hash,
- changing route_id changes the payload hash,
- changing mint changes the payload hash,
- changing recipient token account changes the payload hash,
- changing amount changes the payload hash,
- changing guardian_set_id changes the payload hash,
- malformed bytes32 fields are rejected,
- invalid u64 amounts are rejected.

B5.2 also confirms that watcher-only operational metadata does not change the payload hash.

## B5.3 coverage

B5.3 implemented the quorum package boundary.

Covered positive behavior:

- valid unique guardian quorum package is accepted,
- evidence is bound to payload_v2_hash,
- threshold is enforced,
- guardian_set_id is normalized,
- known guardians are checked.

Covered negative behavior:

- duplicate guardian evidence is rejected,
- unknown guardian evidence is rejected,
- insufficient quorum is rejected,
- signed message drift is rejected,
- invalid threshold is rejected,
- invalid source instruction index is rejected,
- malformed signature hex is rejected.

## B5.4 coverage

B5.4 implemented the no-send relayer submission package boundary.

Covered positive behavior:

- candidate converts to payload_v2_hash,
- quorum package validates against payload_v2_hash,
- relayer package records handler instruction boundary,
- no-send/no-sign/no-RPC/no-SOL/no-private-key boundary is explicit.

Covered negative behavior:

- empty eventId is rejected,
- empty journalId is rejected,
- guardian_set_id drift between candidate and quorum package is rejected,
- stale signatures are rejected when handler-bound candidate fields change after signing,
- relayer-only operational id changes do not alter payload hash.

## B5 negative matrix

| Failure class | Boundary | Expected result |
|---|---|---|
| malformed processed_event bytes32 | B5.2 | reject before package creation |
| malformed route_id bytes32 | B5.2 | reject before package creation |
| malformed mint bytes32 | B5.2 | reject before package creation |
| malformed recipient token account bytes32 | B5.2 | reject before package creation |
| malformed guardian_set_id bytes32 | B5.2 / B5.3 / B5.4 | reject before package creation |
| amount exceeds u64 | B5.2 | reject before package creation |
| payload-bound processed_event drift | B5.2 / B5.4 | payload hash changes; stale signatures reject |
| payload-bound route_id drift | B5.2 / B5.4 | payload hash changes; stale signatures reject |
| payload-bound mint drift | B5.2 / B5.4 | payload hash changes; stale signatures reject |
| payload-bound recipient token account drift | B5.2 / B5.4 | payload hash changes; stale signatures reject |
| payload-bound amount drift | B5.2 / B5.4 | payload hash changes; stale signatures reject |
| payload-bound guardian_set_id drift | B5.2 / B5.4 | payload hash changes or package mismatch rejects |
| duplicate guardian evidence | B5.3 | reject before relayer submission |
| unknown guardian evidence | B5.3 | reject before relayer submission |
| insufficient quorum | B5.3 | reject before relayer submission |
| signed message mismatch | B5.3 / B5.4 | reject before relayer submission |
| invalid threshold | B5.3 | reject before relayer submission |
| invalid source instruction index | B5.3 | reject before relayer submission |
| malformed signature | B5.3 | reject before relayer submission |
| empty eventId | B5.4 | reject before package creation |
| empty journalId | B5.4 | reject before package creation |
| relayer-only operational id change | B5.4 | allowed; payload hash unchanged |

## B5 relation to B3

B3 proves hostile live-gated handler rejection before mutation.

B5 catches many of the same failure classes earlier, before relayer submission.

This does not replace B3.

The handler remains the final authority.

## B5 relation to B4

B4 decided that the handler path remains gated.

B5 preserves that decision.

B5 does not activate production runtime execution.

B5 only prepares offline deterministic watcher/relayer package boundaries.

## B5 closure checklist

B5 is closed when all of the following are true:

- watcher boundary is documented,
- relayer boundary is documented,
- previous Stage 2 watcher/relayer evidence is reconciled,
- candidate schema is documented,
- payload v2 hash conversion is implemented,
- payload v2 hash conversion is tested,
- quorum package schema is implemented,
- quorum package schema is tested,
- relayer submission package schema is implemented,
- relayer submission package schema is tested,
- malformed package cases are rejected before relayer submission,
- no-send/no-sign/no-RPC/no-SOL/no-private-key boundary is preserved,
- full TypeScript tests remain green,
- full xxxl-svm lib tests remain green,
- B6 entry criteria are clear.

## B6 entry criteria

B6 may start only after B5 is merged to main.

B6 target:

Ethereum burn -> watcher observation -> guardian quorum -> relayer submission -> X1 testnet mint

B6 must include:

- explicit live/testnet opening boundary,
- no private key exposure,
- external signer or explicit operator wallet boundary,
- success rehearsal,
- replay rehearsal,
- hostile package rehearsal,
- confirmation/outcome observation,
- rollback or stop conditions for ambiguous results.

## B5.5 conclusion

B5.5 closes the B5 design and package-preparation layer.

The next valid checkpoint after B5 merge is:

B6 — X1 testnet deploy + end-to-end Ethereum burn -> X1 mint.

B6 must not be treated as automatic production activation.

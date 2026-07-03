# XXXL Phase 41K.3 — Processed-Registry Loading Plan Amendment 3

Date: 2026-07-03

Branch:

`stage-41k3-processed-registry-plan`

Applies to:

- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-1.md`
- `docs/xxxl/xxxl-phase-41k-3-processed-registry-loading-plan-amendment-2.md`

## Status

This amendment addresses Claude hostile audit of Amendment 2.

Claude verdict:

`REQUIRES FIXES`

Blocking issues raised:

1. canonical PDA bump handling was not explicit;
2. exact uninitialized expected PDA representation was not pinned and could allow lamport-dusting DoS;
3. `canonical_event_key` sufficiency as the only seed identity was asserted but not made normative;
4. 41K.4 atomic create/init/consume invariant needed to be promoted from forward note to required forward invariant.

This Amendment 3 resolves those issues at plan level.

Amendment 3 supersedes conflicting earlier wording.

## Decision 8 — Canonical PDA Derivation And Bump

41K.3 processed-event PDA derivation must use canonical Solana PDA derivation.

Seed bytes are fixed as:

- `b"xxxl"`;
- `b"processed-event"`;
- `canonical_event_key` as exactly 32 bytes.

The loader must derive:

`(expected_processed_event_pda, canonical_bump) = Pubkey::find_program_address(seeds, program_id)`

The loader must not trust any caller-supplied PDA bump.

If any bump is supplied in instruction data, account data, or test fixture metadata, it is informational only and must not drive derivation.

If a bump is stored or compared by the implementation, it must equal the internally derived canonical bump.

Any non-canonical PDA address for the same logical event must be rejected.

Required invariant:

`account.key == expected_processed_event_pda`

This check applies before any account data is trusted.

## Decision 9 — Exact Uninitialized Expected PDA Representation

41K.3 requires the expected processed-event PDA account to be supplied in the instruction account list.

A supplied account is classified as an accepted uninitialized expected PDA only if all of the following are true:

- `account.key == expected_processed_event_pda`;
- `account.is_signer == false`;
- `account.executable == false`;
- `account.owner == system_program::id()`;
- `account.data_len() == 0`.

Lamports are not part of the uninitialized classification.

Rationale:

Lamport dusting must not turn an otherwise valid unprocessed event into a permanent replay/mint DoS.

A system-owned expected PDA with empty data and nonzero lamports is still classified as uninitialized / unprocessed.

41K.4 / 41K.5 must handle this state safely during atomic account initialization, for example by allocate/assign/top-up or another reviewed mechanism.

If 41K.4 cannot safely initialize a lamport-dusted system-owned empty expected PDA, the design must be reopened before live mark implementation.

## Decision 10 — Invalid Account Classification

The loader must distinguish uninitialized from corrupted/invalid states.

Accepted uninitialized state:

- expected PDA key;
- system-program owner;
- empty data;
- not executable;
- not signer.

Invalid states include:

- expected PDA with XXXL owner and zero discriminator;
- expected PDA with XXXL owner and wrong discriminator;
- expected PDA with XXXL owner and unsupported version;
- expected PDA with XXXL owner and `consumed == false`;
- expected PDA with non-system, non-XXXL owner;
- supplied account whose key is not the expected PDA.

Invalid states must reject.

They must not be treated as unprocessed.

No account data is trusted before owner / PDA / data-length / discriminator / version checks appropriate to its state.

## Decision 11 — Canonical Event Key Sufficiency

`canonical_event_key` is accepted as the sole PDA seed identity for processed-event replay protection only under this invariant:

`canonical_event_key` is the canonical, collision-resistant identity of one source burn event after accepted payload binding.

It must be derived internally from the signed / quorum-authorized raw payload path.

It must not be caller-supplied independently.

It must identify the source event such that one source burn event can be processed at most once.

The processed-event PDA seed does not include `route_id` or `recipient` because replay protection is per canonical source event, not per route/recipient variant.

However, initialized processed-event account validation must still verify stored:

- `canonical_event_key`;
- `route_id`;
- `recipient`.

These checks are integrity / corruption checks for initialized account data.

They are not a substitute for canonical event identity.

If implementation review finds that current `canonical_event_key` derivation does not bind the accepted source event identity strongly enough, 41K.3 implementation must stop and reopen the earlier canonical-event-key / payload-binding design before live replay protection proceeds.

## Decision 12 — Required 41K.4 Atomicity Invariant

41K.4 / 41K.5 must not create a durable initialized processed-event PDA with `consumed == false`.

The live mark path must create / initialize / set consumed state atomically.

Required forward invariant:

`no durable initialized consumed=false processed-event PDA may be produced by any accepted runtime path`

If account creation or initialization fails, the whole operation must fail without leaving a valid initialized unconsumed processed-event PDA.

The existing helper:

`mark_processed_event_consumed(...)`

must not be accepted as live mark semantics unless it is proven compatible with this invariant, or replaced / wrapped by a reviewed atomic create/init/consume path.

If 41K.4 proves that atomic create/init/consume is not feasible under Solana runtime constraints, the 41K.3 lifecycle model must be reopened before live route implementation.

## Decision 13 — Option A Adapter Invariants

41K.3 keeps Amendment 2 Decision 6:

Option A adapter to existing 41J list-based `AuthoritativeProcessedRegistryViewRef`.

This is accepted only under these invariants:

1. One replay-eligibility check handles exactly one canonical event.
2. 41J uses the processed-registry list only for membership of the current internally derived `canonical_event_key`.
3. 41J does not rely on list-wide semantics such as batch processing, registry size, ordering, iteration, or audit completeness.
4. Invalid processed-event PDA states reject before adapter construction.
5. The adapter constructor is internal and type-enforced.

Adapter semantics remain:

- unprocessed -> empty processed list;
- processed -> one-item processed list containing `canonical_event_key`;
- invalid lifecycle state -> rejection before adapter construction.

If implementation review finds that 41J uses the list for anything beyond single-key membership for the current event, 41K.3 implementation must stop and either refine 41J or reopen the reconciliation decision.

## Decision 14 — Type-Enforcement Pattern

The future 41K.3 adapter must not be externally constructible.

The implementation should use a private / sealed / `pub(crate)` construction pattern such that only a successful 41K.3 processed-event PDA load can produce the runtime authoritative view.

Handler discipline is not sufficient.

Tests should prove that caller-supplied / unauthenticated processed-registry views remain rejected.

## Updated Required Tests For 41K.3 Code

Future 41K.3 implementation tests must include:

1. canonical bump derivation through `Pubkey::find_program_address`;
2. non-canonical PDA address rejected;
3. caller-supplied bump ignored / not trusted;
4. expected PDA with system owner, empty data, zero lamports classified as uninitialized / unprocessed;
5. expected PDA with system owner, empty data, nonzero lamports classified as uninitialized / unprocessed;
6. expected PDA with XXXL owner and zero discriminator rejected as invalid;
7. expected PDA with XXXL owner and wrong discriminator rejected as invalid;
8. expected PDA with non-system, non-XXXL owner rejected;
9. initialized `consumed == true` classified as processed / replay rejection;
10. initialized `consumed == false` rejected as invalid lifecycle state;
11. adapter unprocessed case produces empty list for 41J membership;
12. adapter processed case produces one-item list containing `canonical_event_key`;
13. invalid state cannot construct adapter;
14. 41J caller-supplied / unauthenticated registry tests remain passing;
15. no replay write enabled;
16. no processed marking enabled;
17. no account mutation enabled;
18. no CPI / mint / handler / live route enabled.

## Updated Acceptance Gate

The 41K.3 plan is acceptable only if reviewers agree that:

- canonical bump-only PDA derivation is required;
- caller-supplied bump is never trusted;
- uninitialized expected PDA representation is pinned as system-owned + empty data + expected key + non-signer + non-executable;
- lamports do not affect uninitialized classification;
- lamport dusting must not create permanent mint DoS;
- XXXL-owned zero/wrong discriminator is invalid, not unprocessed;
- `canonical_event_key` is sufficient only as canonical source-event identity from accepted payload binding;
- route_id / recipient checks are initialized-account integrity checks;
- 41K.4 must enforce atomic create/init/consume;
- Option A adapter assumes single-event membership-only 41J semantics;
- adapter construction is internal and type-enforced;
- all write / mark / mutation / CPI / mint / handler / live route surfaces remain disabled in 41K.3.

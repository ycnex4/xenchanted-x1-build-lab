# Registrar Trust Model Evolution

## 1. Purpose

This document describes the evolution path for the Ethereum Registrar trust model used by X1 Build.

The goal is to start with a simple MVP model while keeping a clear path toward stronger verification.

The registrar should never create value by opinion.

It should verify source events and prepare deterministic updates.

---

## 2. Current MVP model

For MVP, the registrar may be trusted infrastructure.

This means a trusted registrar authority can submit verified messages to the X1 Build Program.

The Build Program accepts registrar messages only from the approved trust path.

The registrar is responsible for verifying:

- Core redeem events
- XEN.burn(user, amount) calls
- XNTD lock / unlock / relock state
- required_xntd_lock from XC epoch state
- source transaction success
- source event uniqueness

---

## 3. Why trusted MVP is acceptable

A trusted registrar is acceptable for MVP because the design phase is still focused on:

- proving the accounting model
- validating Build state structure
- testing source event flows
- building readable user-facing state
- avoiding premature complexity

The MVP must still avoid arbitrary totals.

Even trusted messages must be source-based and replay-protected.

---

## 4. MVP limitations

The trusted registrar model has limitations:

- users must trust the registrar operator
- registrar downtime may delay updates
- registrar mistakes may affect Build state
- the trust path is not fully decentralized
- other projects may treat the data differently depending on their risk tolerance

These limitations should be documented clearly.

---

## 5. Mandatory protections even in MVP

Even with a trusted registrar, the X1 Build Program should enforce:

- processed_messages[message_id]
- used_redeem_events[redeem_key]
- used_xen_burn_events[xen_burn_key]
- genesis_origin_claimed[identity]
- canonical_build_by_identity[identity]

The registrar should not be able to replay the same source event twice.

The registrar should not be able to apply one source event to multiple Builds.

---

## 6. Stage 1: trusted registrar

## Description

Single trusted registrar authority submits messages.

## Pros

- simplest implementation
- fastest MVP
- easiest debugging
- clear operational responsibility

## Cons

- centralized trust
- operator error risk
- less suitable for high-value dependent projects

## Appropriate use

- early testing
- internal validation
- public prototype with clear disclosure
- low-risk contribution display

---

## 7. Stage 2: multi-signer registrar

## Description

Registrar messages require signatures from multiple approved watchers.

Example:

2-of-3 watchers

or:

3-of-5 watchers

## Pros

- reduces single-operator risk
- improves credibility
- allows independent watchers
- still simpler than full proof verification

## Cons

- more operational complexity
- signer coordination required
- signer rotation must be designed

## Required additions

- signer set account
- signature threshold
- signer rotation policy
- message domain separation
- replay protection across signer sets

---

## 8. Stage 3: Merkle root checkpoints

## Description

Indexers publish batches of verified source events.

A Merkle root is submitted on-chain.

Users or relayers submit proofs for individual events.

## Pros

- lower on-chain storage for large datasets
- stronger transparency
- public auditability
- source batches can be mirrored by independent parties

## Cons

- more complex user flow
- proof generation required
- batch timing matters
- root publishing still needs trust or governance model

## Appropriate use

- larger scale
- public datasets
- many source events
- stronger audit requirements

---

## 9. Stage 4: independent public indexers

## Description

Multiple independent indexers produce matching datasets.

The Build ecosystem can compare data across independent sources.

## Pros

- higher transparency
- easier community verification
- less dependence on one operator

## Cons

- requires indexer standards
- potential data disagreements
- needs dispute / reconciliation process

## Important note

Independent indexers improve transparency but do not automatically remove the need for an accepted on-chain update path.

---

## 10. Stage 5: direct proof verification

## Description

The X1 Build Program verifies source proofs directly, where technically possible.

## Pros

- strongest trust minimization
- less reliance on registrar operators
- best long-term credibility

## Cons

- may be technically complex
- cross-chain proof verification can be expensive
- may require bridge/proof infrastructure
- may not be available in early X1 environment

## Appropriate use

- mature protocol stage
- high-value dependent integrations
- when proof infrastructure is stable

---

## 11. Trust model and first principles

xEnchanted Crypto core protocol remains immutable and no-admin.

The registrar is not part of the immutable Ethereum core protocol.

It is an infrastructure layer for reflecting verified source history into X1 Build.

This distinction matters.

A managed registrar does not rewrite XC protocol rules.

It only affects how source events are reflected into X1 Build.

Users and projects should be able to evaluate registrar trust separately from XC core protocol trust.

---

## 12. Message domain separation

Every signed or verified message should include a domain separator.

Suggested domain fields:

- protocol_name
- environment
- x1_chain_id
- build_program_id
- registrar_version
- message_type

This prevents signatures or messages from being reused across environments or program versions.

---

## 13. Signer rotation

If multi-signer registrar is used, signer rotation must be explicit.

Rotation should define:

- who can rotate signers
- whether rotation has delay
- whether old signer sets remain valid for old messages
- how active signer set version is stored
- how emergency rotation works

For MVP, this may be deferred.

For production, it must be documented before launch.

---

## 14. Failure handling

Possible failures:

- registrar outage
- incorrect message
- duplicate submission
- source chain reorg
- indexer bug
- signer compromise

MVP handling:

- reject duplicates on-chain
- keep source event logs off-chain
- allow delayed resubmission
- document trusted operator responsibility

Future handling:

- multi-signer confirmation
- challenge windows
- root replacement policy
- public audit datasets

---

## 15. Main invariants

- Registrar must not send arbitrary totals.
- Every update must be source-based or checkpoint-based.
- Replay protection is mandatory.
- One source event can update one Build only once.
- Trust model should become stronger over time.
- Registrar trust is infrastructure trust, not XC core protocol control.

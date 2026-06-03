# XNTD Lock Proof Model

## 1. Purpose

This document describes possible proof and bridge models for reflecting XNTD lock state into X1 Build.

The goal is to define how X1 Build can trust that a user has locked XNTD on the Ethereum / XC side.

This is not implementation code.

---

## 2. Current MVP direction

For MVP, XNTD lock state is verified by the Ethereum Registrar.

The registrar sends verified messages to the X1 Build Program:

- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD

The X1 Build Program records the accepted lock state.

This is the simplest model for early implementation.

---

## 3. Why proof model matters

XNTD lock affects whether XC-derived commitment is active.

Rule:

xc_commitment_active =
  history_bld > 0
  AND locked_xntd >= required_xntd_lock

Because this status may influence how other X1 projects interpret a Build, lock verification must be clear.

---

## 4. What must be proven

A valid XNTD lock update should prove:

- Ethereum identity
- Build identity / mapping
- locked XNTD amount
- required lock amount
- XC epoch used for required lock
- source transaction success
- source contract
- source block / timestamp
- replay protection key

---

## 5. MVP model: trusted registrar

## Description

A trusted Ethereum Registrar verifies Ethereum-side lock state and submits a registrar message to X1.

## Pros

- simplest
- fast to build
- consistent with other MVP registrar messages
- avoids premature bridge complexity

## Cons

- users trust registrar infrastructure
- lock state updates may be delayed
- registrar mistakes are possible
- not fully trust-minimized

## MVP use

This is the recommended MVP model.

---

## 6. Model B: threshold-signed watchers

## Description

Multiple watchers independently verify lock state.

A lock message is accepted only if enough watchers sign it.

Example:

2-of-3

or:

3-of-5

## Pros

- reduces single-operator risk
- improves trust model
- still easier than full cross-chain proof verification

## Cons

- requires signer management
- signer rotation needed
- message domain separation required
- more operational complexity

---

## 7. Model C: Merkle lock checkpoints

## Description

An indexer publishes a Merkle root containing verified lock states or lock events.

Users or relayers submit Merkle proofs to X1 Build.

## Pros

- scalable
- auditable
- easier for public datasets
- reduces per-event trust if roots are publicly verified

## Cons

- more complex user / relayer flow
- root publishing still needs trust model
- proof generation required
- update timing matters

---

## 8. Model D: bridge-aware lock proof

## Description

A bridge or cross-chain proof system verifies Ethereum lock events directly or through bridge messages.

## Pros

- strongest long-term path if infrastructure is reliable
- less registrar trust
- clearer proof path

## Cons

- depends on bridge infrastructure
- may introduce guardian / multisig trust
- proof verification may be expensive
- not ideal for early MVP unless already available

---

## 9. Required lock amount proof

The required_xntd_lock is based on current XC epoch Core L1 nominal.

For MVP:

Ethereum Registrar calculates it from XC protocol state.

Future options:

- mirror XC epoch parameters to X1
- verify epoch state through proof
- include epoch parameters in signed registrar messages
- include Core contract view snapshots in public checkpoints

Important:

required_xntd_lock must not be arbitrary.

---

## 10. Lock state source

A lock source may be:

- Ethereum XNTD lock contract
- bridge escrow contract
- registrar-tracked lock account
- future X1 mirrored XNTD lock representation

The chosen source must be deterministic and auditable.

---

## 11. Unlock and relock proof

UNLOCK_XNTD must prove the new locked amount.

RELOCK_XNTD must prove:

- new locked amount
- new required lock amount
- new lock epoch
- relock source transaction
- that relock was accepted by the source lock system

The Build Program must additionally enforce:

available_bld >= history_bld

for relock.

---

## 12. Reorg considerations

Ethereum-side source events may be affected by chain reorgs.

MVP registrar should use confirmation depth before submitting messages.

Suggested policy:

- wait for enough confirmations
- record source block number
- record source block hash if available
- avoid submitting very fresh events

Exact confirmation depth can be decided later.

---

## 13. Failure cases

Possible failures:

- registrar submits wrong lock amount
- source transaction reorgs
- lock event is indexed twice
- unlock is missed
- relock is delayed
- required_xntd_lock is calculated from wrong epoch
- bridge proof is delayed or unavailable

Mitigations:

- replay protection
- source event keys
- public indexer logs
- delayed finality policy
- future multi-signer validation
- future public proof model

---

## 14. Recommended MVP decision

Use trusted Ethereum Registrar for MVP XNTD lock updates.

Do not build full lock proof / bridge verification in MVP.

Reason:

- accounting model is still being validated
- Build state structure comes first
- registrar message format already covers lock updates
- proof/bridge model can evolve later

---

## 15. Main invariants

- XNTD lock state must be source-based.
- required_xntd_lock must be based on XC epoch Core L1 nominal.
- XNTD lock does not create BLD.
- XNTD lock does not create XBP.
- Relock requires available_bld >= history_bld.
- MVP can use trusted registrar, but long-term path should reduce trust.

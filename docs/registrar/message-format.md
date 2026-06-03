# Ethereum Registrar Message Format

## 1. Purpose

This document defines the conceptual message format used by the Ethereum Registrar to send verified Ethereum-side updates to the X1 Build Program.

The goal is to keep every X1 Build update tied to a verified source event or deterministic state transition.

The registrar must not send arbitrary totals.

---

## 2. Main principles

Each message must be:

- source-based
- replay-protected
- deterministic
- tied to one Build
- tied to one event type
- verifiable by message_id

Main rule:

one source event -> one message -> one accounting action -> one Build

---

## 3. Common message fields

Every registrar message should include:

- message_id
- message_type
- source_chain_id
- source_contract
- source_tx_hash
- source_index
- ethereum_identity
- build_id
- created_at_block
- created_at_timestamp
- payload

## Field meanings

message_id:
Unique replay-protection key.

message_type:
Type of update being performed.

source_chain_id:
Ethereum chain id.

source_contract:
Contract address that produced or proves the source event.

source_tx_hash:
Ethereum transaction hash.

source_index:
Log index, trace index, or canonical source index.

ethereum_identity:
Ethereum-side user identity / address.

build_id:
Target X1 Build identifier.

payload:
Message-specific data.

---

## 4. message_id

The message_id should be deterministic.

Suggested format:

message_id = hash(
  source_chain_id,
  source_contract,
  message_type,
  source_tx_hash,
  source_index
)

The X1 Build Program should reject already processed message_id values.

---

## 5. Message types

Initial message types:

- CONNECT_XC_HISTORY
- ADD_CORE_REDEEM
- ADD_XEN_BURN_POWER
- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD

Future message types may be added later.

---

## 6. CONNECT_XC_HISTORY

Purpose:

Connect valid xEnchanted Crypto history to a Build.

This message may also grant Genesis Origin BLD if eligible.

## Payload

- tiered_origin_bld
- genesis_origin_eligible
- genesis_origin_claimed
- build_genesis_epoch

## Validation

- Ethereum identity must be canonical for the Build.
- Message must not be replayed.
- Genesis Origin BLD may be granted only once.
- Genesis Origin BLD may be granted only during Build Genesis Epoch.
- tiered_origin_bld must be derived from earned_bld tiers.

## Accounting

If eligible:

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

Must not increase:

earned_bld

---

## 7. ADD_CORE_REDEEM

Purpose:

Create earned BLD from a verified Core redeem event.

## Payload

- redeem_key
- token_id
- xen_burned
- normalized_bld
- owner_at_redeem
- core_contract

## Validation

- redeem_key must be unique.
- Core redeem event must be verified.
- normalized_bld must be derived from xen_burned.
- Same Core redeem must not be counted twice.

## Accounting

earned_bld += normalized_bld
available_bld += normalized_bld

---

## 8. ADD_XEN_BURN_POWER

Purpose:

Create XEN Burn Power from a verified XEN.burn(user, amount) call.

## Payload

- xen_burn_key
- user
- amount
- normalized_xbp
- xen_contract
- caller_or_source_contract
- trace_index

## Validation

- xen_burn_key must be unique.
- Source must be successful XEN.burn(user, amount).
- Zero-address transfers must not be counted.
- Attribution target is the user argument in XEN.burn(user, amount).

## Accounting

earned_xbp += normalized_xbp
available_xbp += normalized_xbp

---

## 9. LOCK_XNTD

Purpose:

Record XNTD lock state for XC commitment activation.

## Payload

- locked_xntd
- required_xntd_lock
- lock_epoch

## Validation

- Build must have earned_bld > 0.
- required_xntd_lock must be based on current XC epoch Core L1 nominal.
- Lock state must be verified.
- Lock must not create BLD or XBP.

## Accounting

locked_xntd = payload.locked_xntd
required_xntd_lock = payload.required_xntd_lock
lock_epoch = payload.lock_epoch

xc_commitment_active =
  earned_bld > 0
  AND locked_xntd >= required_xntd_lock

---

## 10. UNLOCK_XNTD

Purpose:

Record XNTD unlock state.

## Payload

- locked_xntd
- required_xntd_lock
- lock_epoch

## Validation

- Unlock state must be verified.
- Unlock must not modify BLD or XBP.

## Accounting

locked_xntd = payload.locked_xntd

If locked_xntd < required_xntd_lock:

xc_commitment_active = false

---

## 11. RELOCK_XNTD

Purpose:

Update XNTD commitment under a new XC epoch requirement.

## Payload

- new_locked_xntd
- new_required_xntd_lock
- new_lock_epoch

## Validation

Relock is allowed only if:

available_bld >= earned_bld

The new required lock must be based on current XC epoch Core L1 nominal.

Relock must not create BLD or XBP.

## Accounting

locked_xntd = new_locked_xntd
required_xntd_lock = new_required_xntd_lock
lock_epoch = new_lock_epoch

xc_commitment_active =
  earned_bld > 0
  AND locked_xntd >= required_xntd_lock

---

## 12. Replay protection

The X1 Build Program should store processed message IDs.

processed_messages[message_id] = true

If a message_id was already processed, the message must be rejected.

---

## 13. Canonical identity protection

The same Ethereum identity must not be able to create or update multiple canonical Builds with the same source history.

Suggested mapping:

canonical_build_by_identity[ethereum_identity] = build_id

If an Ethereum identity is already bound to another Build, conflicting updates must be rejected.

---

## 14. Main invariants

- Registrar messages must not send arbitrary totals.
- Every message must be tied to a source event or deterministic state transition.
- One message can be processed only once.
- One source event can update one Build only once.
- BLD, XBP, XNTD lock, and X1 Fee Contribution remain separate accounting layers.

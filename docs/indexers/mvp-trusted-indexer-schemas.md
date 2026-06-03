# MVP Trusted Indexer Schemas

## 1. Purpose

This document describes the conceptual data schemas for MVP trusted indexers used by X1 Build.

The goal is to define what data indexers collect, how they normalize it, and what they submit to the X1 Build Program.

This is not implementation code.

---

## 2. MVP trust model

For MVP, indexers may be trusted infrastructure.

They should not create arbitrary values by opinion.

They should only submit data derived from:

- verified Ethereum source events
- verified XEN.burn(user, amount) calls
- verified xEnchanted Crypto Core redeem events
- verified XNTD lock / unlock / relock state
- verified X1 transaction fee metadata

Future versions may replace or strengthen this with:

- threshold-signed watchers
- Merkle roots and proofs
- public datasets
- independent indexers
- on-chain proof verification

---

## 3. Common source event fields

Most indexed source events should include:

- source_chain_id
- source_contract
- source_tx_hash
- source_index
- block_number
- block_timestamp
- status
- indexed_at

## Field meanings

source_chain_id:
Chain where the source event happened.

source_contract:
Contract that emitted or contains the source action.

source_tx_hash:
Transaction hash.

source_index:
Log index, trace index, or canonical source index.

block_number:
Source chain block number.

block_timestamp:
Source chain block timestamp.

status:
Whether the source action was successful.

indexed_at:
Indexer timestamp.

---

## 4. Core redeem event schema

Used to create history_bld.

## Schema

- redeem_key
- source_chain_id
- core_contract
- token_id
- owner_at_redeem
- xen_burned
- normalized_bld
- source_tx_hash
- log_index
- block_number
- block_timestamp
- indexed_at
- applied_to_build

## redeem_key

redeem_key = hash(
  source_chain_id,
  core_contract,
  token_id,
  source_tx_hash,
  log_index
)

## Normalization

1 BLD = 100,000,000 XEN burned through redeemed Core history

normalized_bld = xen_burned / 100,000,000 XEN

Internal representation should support fractional BLD.

---

## 5. XEN burn event schema

Used to create XEN Burn Power.

## Schema

- xen_burn_key
- source_chain_id
- xen_contract
- user
- amount
- normalized_xbp
- caller_or_source_contract
- source_tx_hash
- trace_index
- block_number
- block_timestamp
- indexed_at
- applied_to_build

## xen_burn_key

xen_burn_key = hash(
  source_chain_id,
  xen_contract,
  source_tx_hash,
  trace_index,
  user,
  amount
)

## Canonical source

successful XEN.burn(user, amount)

Arbitrary zero-address transfers must not be counted.

## Normalization

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount)

normalized_xbp = amount / 100,000,000 XEN

Internal representation should support fractional XBP.

---

## 6. XNTD lock state schema

Used to update XNTD commitment state.

## Schema

- lock_event_key
- source_chain_id
- lock_contract
- ethereum_identity
- build_id
- locked_xntd
- required_xntd_lock
- lock_epoch
- action_type
- source_tx_hash
- log_index
- block_number
- block_timestamp
- indexed_at
- applied_to_build

## action_type

Allowed values:

- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD

## Validation intent

The indexer should verify that:

- lock state is real
- required_xntd_lock is based on the current XC epoch Core L1 nominal
- source transaction succeeded
- action_type matches the observed state transition

---

## 7. X1 fee contribution checkpoint schema

Used to update X1 Fee Contribution.

## Schema

- build_id
- x1_owner
- total_fee_paid
- total_tx_count
- counted_until_slot
- last_updated
- checkpoint_id
- checkpoint_authority
- indexed_at

## checkpoint_id

checkpoint_id = hash(
  x1_chain_id,
  x1_owner,
  counted_until_slot,
  total_fee_paid,
  total_tx_count
)

## Validation

The Build Program should accept only newer checkpoints:

counted_until_slot > stored_x1_fee_counted_until_slot

## Fee inclusion

total_fee_paid should include:

- base fee
- priority fee

The value should be stored in raw smallest X1 units.

---

## 8. Registrar message schema

The indexer / registrar may wrap source data into registrar messages.

## Common message fields

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

## message_id

message_id = hash(
  source_chain_id,
  source_contract,
  message_type,
  source_tx_hash,
  source_index
)

The Build Program must reject already processed message_id values.

---

## 9. Indexed event application status

Each off-chain indexed event may track whether it was already submitted or applied.

Suggested fields:

- applied_to_build
- applied_build_id
- applied_message_id
- applied_at
- apply_tx_signature
- apply_status

This helps indexer operators avoid duplicate submissions.

On-chain replay protection remains mandatory.

---

## 10. Batch submission model

For MVP, messages may be submitted one by one.

Later, batches may be used.

A batch may include:

- batch_id
- batch_type
- batch_source_range
- event_count
- message_count
- merkle_root, if used later
- signer / authority
- created_at

MVP does not require Merkle proofs.

---

## 11. Main invariants

- Indexers must not submit arbitrary totals except approved cumulative fee checkpoints.
- Core redeem events create history_bld only once.
- XEN burn events create XBP only once.
- XNTD lock messages update commitment state only.
- X1 fee checkpoints update fee contribution only.
- Every submitted message must be replay-protected.
- Off-chain convenience flags do not replace on-chain protection.

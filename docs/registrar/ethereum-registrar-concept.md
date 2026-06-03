# Ethereum Registrar Concept

## 1. Purpose

The Ethereum Registrar verifies Ethereum-side source data and prepares updates for X1 Build.

It should not send arbitrary totals.

It should work with verified source events and deterministic state transitions.

The registrar is responsible for Ethereum-side truth.

The X1 Build Program records accepted updates and exposes readable state.

---

## 2. Responsibilities

The Ethereum Registrar should verify and prepare:

- Core redeem events for BLD creation
- XEN.burn(user, amount) calls for XEN Burn Power
- XNTD lock / unlock / relock state
- required_xntd_lock based on the current XC epoch
- replay-protected update messages for X1

---

## 3. Core redeem verification

Core redeem is the only source of earned BLD.

When a Core NFT is redeemed, the registrar should extract:

- user / owner
- token_id
- xenBurned
- transaction hash
- log index
- block number
- chain id
- core contract address

The registrar should normalize xenBurned into BLD.

Display unit:

1 BLD = 100,000,000 XEN burned through redeemed Core history

The registrar must not count the same Core redeem more than once.

---

## 4. Core redeem key

Each Core redeem source must have a unique key.

redeem_key = hash(
  chain_id,
  core_contract,
  token_id,
  tx_hash,
  log_index
)

This key prevents one Core redeem from creating BLD more than once.

---

## 5. XEN Burn Power verification

XEN Burn Power comes from successful official XEN burn calls.

Canonical source:

successful XEN.burn(user, amount)

This includes:

- top-level calls to XEN.burn(user, amount)
- internal calls through projects that used the official XEN burn interface

It does not include arbitrary transfers to the zero address.

The registrar / indexer should extract:

- user
- amount
- transaction hash
- trace index / call index
- block number
- caller / source contract
- chain id
- XEN contract address

---

## 6. XEN burn key

Each XEN burn source must have a unique key.

xen_burn_key = hash(
  chain_id,
  xen_contract,
  tx_hash,
  trace_index,
  user,
  amount
)

This key prevents one XEN.burn(user, amount) call from creating XBP more than once.

---

## 7. XNTD lock requirement

XNTD lock is required for Build records that received earned BLD through Core redeem.

The required lock amount is tied to the minimum Core L1 nominal of the current XC epoch.

required_xntd_lock = current epoch Core L1 nominal

The registrar should determine this value from the current Ethereum-side XC protocol state at lock or relock time.

The X1 side does not need to calculate XC epoch economics itself.

---

## 8. XNTD lock / unlock / relock messages

The registrar should prepare messages for:

- lock_xntd
- unlock_xntd
- relock_xntd

Each message should include:

- user / identity
- build identifier
- locked_xntd
- required_xntd_lock
- lock_epoch
- source transaction data
- message_id

Relock is allowed only if the Build has enough available BLD:

available_bld >= history_bld

The registrar or X1 Build Program must enforce this rule depending on where the current available_bld state is checked.

---

## 9. Genesis Origin BLD

Genesis Origin BLD is a one-time tiered allocation.

It is based on history_bld at the first valid xEnchanted Crypto history connection during the Build Genesis Epoch.

Allocation tiers:

- history_bld >= 1     -> origin_bld = 11
- history_bld >= 11    -> origin_bld = 22
- history_bld >= 121   -> origin_bld = 55
- history_bld >= 1111  -> origin_bld = 121

121 BLD is the maximum Genesis Origin cap, not the default allocation.

It may be granted only when a Build first connects valid xEnchanted Crypto history during the Build Genesis Epoch.

It must not be granted:

- on update
- on relock
- outside Build Genesis Epoch
- more than once per canonical Ethereum/XC identity

When granted:

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

It must not increase history_bld.

---

## 10. Message replay protection

Every registrar message sent to X1 should have a unique message_id.

message_id = hash(
  source_chain_id,
  source_contract,
  event_type,
  tx_hash,
  log_index_or_trace_index
)

The X1 Build Program should reject already processed messages.

---

## 11. Canonical identity

The registrar should help enforce a canonical relationship between an Ethereum/XC identity and a Build.

This prevents the same Ethereum-side history from being used to create or update multiple canonical Builds.

Suggested protection:

canonical_build_by_identity[identity] = build_id

The exact identity model can be finalized later.

---

## 12. Trust model

For MVP, the registrar may be trusted.

Later versions may use:

- threshold-signed watchers
- Merkle roots and proofs
- independent public indexers

The initial goal is to avoid arbitrary totals and keep every update tied to source events.

---

## 13. Main rule

Ethereum Registrar does not create value by opinion.

It only verifies source events and prepares deterministic updates.

One source event can update one Build only once.



# PDA and Account Layout

## 1. Purpose

This document describes the conceptual PDA and account layout for the X1 Build Program.

It is not implementation code.

The goal is to define which persistent accounts may be needed, what each account stores, and which program instructions may read or update them.

---

## 2. Main account categories

The Build Program may need the following account categories:

- BuildState account
- Identity binding account
- Processed message account / map
- Used Core redeem event account / map
- Used XEN burn event account / map
- Genesis Origin claim account / map
- Fee checkpoint state
- Program config / authority state

The exact account model may be optimized later.

---

## 3. BuildState account

## Purpose

Stores the main readable Build state.

## Suggested PDA seed

build_state = PDA(
  "build",
  build_id
)

Alternative:

build_state = PDA(
  "build",
  owner
)

## Stored fields

- owner
- build_id
- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp
- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at
- created_at
- updated_at
- version

## Updated by

- create_build
- create_build_with_bld_burn
- process_registrar_message
- update_fee_checkpoint
- burn_bld
- transfer_bld, if implemented inside Build Program
- burn_xbp, if implemented

## Notes

BuildState is the primary reader-facing account.

Other X1 projects should be able to read this account or a derived view from it.

---

## 4. Identity binding account

## Purpose

Prevents one canonical identity from being bound to multiple canonical Builds.

## Suggested PDA seed

identity_binding = PDA(
  "identity",
  identity_kind,
  identity_hash
)

## Stored fields

- identity_kind
- identity_hash
- build_id
- bound_at
- source

## identity_kind examples

- x1_owner
- ethereum_identity
- xc_identity

## Updated by

- create_build
- connect_xc_history
- process_registrar_message

## Validation

If identity_binding already exists for a different build_id, conflicting updates must be rejected.

---

## 5. Processed message account / map

## Purpose

Prevents replay of registrar messages.

## Suggested PDA seed

processed_message = PDA(
  "processed_message",
  message_id
)

## Stored fields

- message_id
- message_type
- build_id
- processed_at
- source_chain_id
- source_tx_hash

## Updated by

- process_registrar_message

## Validation

If processed_message already exists, reject the message.

---

## 6. Used Core redeem event account / map

## Purpose

Prevents the same Core redeem event from creating BLD more than once.

## Suggested PDA seed

used_redeem_event = PDA(
  "redeem_event",
  redeem_key
)

## Stored fields

- redeem_key
- build_id
- token_id
- normalized_bld
- source_chain_id
- source_tx_hash
- source_index
- processed_at

## Updated by

- ADD_CORE_REDEEM through process_registrar_message

## Validation

If used_redeem_event already exists, reject the update.

---

## 7. Used XEN burn event account / map

## Purpose

Prevents the same XEN.burn(user, amount) call from creating XBP more than once.

## Suggested PDA seed

used_xen_burn_event = PDA(
  "xen_burn_event",
  xen_burn_key
)

## Stored fields

- xen_burn_key
- build_id
- user
- amount
- normalized_xbp
- source_chain_id
- source_tx_hash
- trace_index
- processed_at

## Updated by

- ADD_XEN_BURN_POWER through process_registrar_message

## Validation

If used_xen_burn_event already exists, reject the update.

---

## 8. Genesis Origin claim account / map

## Purpose

Prevents repeated Genesis Origin BLD allocation for the same canonical Ethereum/XC identity.

## Suggested PDA seed

genesis_origin_claim = PDA(
  "genesis_origin",
  identity_hash
)

## Stored fields

- identity_hash
- build_id
- tiered_origin_bld
- history_bld_at_claim
- claimed_at
- build_genesis_epoch

## Updated by

- CONNECT_XC_HISTORY through process_registrar_message

## Validation

If genesis_origin_claim already exists, reject a new Genesis Origin allocation for that identity.

---

## 9. Fee checkpoint state

## Purpose

Stores the latest accepted X1 Fee Contribution checkpoint for a Build.

This may be stored directly inside BuildState, but a separate account can be used if needed.

## Suggested PDA seed

fee_checkpoint = PDA(
  "fee_checkpoint",
  build_id
)

## Stored fields

- build_id
- total_fee_paid
- tx_count
- counted_until_slot
- last_updated
- checkpoint_authority

## Updated by

- update_fee_checkpoint

## Validation

New checkpoint must satisfy:

counted_until_slot > stored_counted_until_slot

## Notes

For v1, storing these fields directly in BuildState may be simpler.

A separate account is useful if fee data grows or if multiple fee-related views are added later.

---

## 10. Program config / authority state

## Purpose

Stores program-level configuration and approved authorities.

## Suggested PDA seed

program_config = PDA(
  "config"
)

## Stored fields

- version
- registrar_authority
- fee_indexer_authority
- build_genesis_epoch_start
- build_genesis_epoch_end
- paused_flags, if any
- authority_model

## Important design note

The final authority model must be designed carefully.

xEnchanted Crypto core protocol is immutable and no-admin.

The X1 Build Program may still need explicit authorities for registrar/indexer messages in MVP.

Those authorities should be documented as infrastructure trust, not as arbitrary protocol control.

---

## 11. Account ownership and mutability

## BuildState

Mutable through valid program instructions only.

## Source protection accounts

Append-only.

Once created, they should not be modified except possibly for metadata corrections if explicitly allowed.

## Program config

Mutable only according to the chosen authority model.

For MVP, this may be controlled by a trusted authority.

Future versions may move to threshold-signers or immutable config after stabilization.

---

## 12. Rent / account growth considerations

Creating one PDA per processed event is clean and simple, but may become expensive at scale.

Potential alternatives:

- store processed event hashes in compressed structures
- use Merkle roots and proofs
- use batched registrar checkpoints
- keep detailed event data off-chain and store only message hashes on-chain

For MVP, explicit PDAs are easier to reason about.

Optimization can come later.

---

## 13. Reader accounts

Other X1 projects should primarily read:

- BuildState
- FeeContributionView, if separate
- identity binding, if needed

They should not need to read:

- processed messages
- used redeem events
- used XEN burn events
- internal registrar data

---

## 14. Main invariants

- One BuildState per canonical Build.
- One canonical identity should map to one Build.
- One registrar message can be processed only once.
- One Core redeem event can create BLD only once.
- One XEN burn event can create XBP only once.
- Genesis Origin BLD can be claimed only once per canonical Ethereum/XC identity.
- Fee checkpoints must move forward by slot.
- Reader-facing data should remain simple and stable.


# Build Program Instruction Layout

## 1. Purpose

This document describes the conceptual instruction layout for the X1 Build Program.

It is not implementation code.

The goal is to define the main instructions, their purpose, the fields they update, and the validation rules they must enforce.

---

## 2. Main principles

The Build Program should enforce state transitions, replay protection, and accounting separation.

Main accounting rule:

BLD, XBP, XNTD lock, and X1 Fee Contribution must remain separate layers.

The program must not accept arbitrary totals unless the update path is explicitly checkpoint-based.

---

## 3. create_build

## Purpose

Create a canonical Build account / object for a user.

## Inputs

- owner
- build_id / build_mint

## Updates

- owner
- build_id / build_mint
- created_at
- updated_at
- version

## Validation

- Build must not already exist for the same canonical identity.
- Build identifier must be unique.
- No contribution fields are created by this instruction alone.

---

## 4. create_build_with_bld_burn

## Purpose

Create an active Build in X1 by burning 11 BLD.

## Inputs

- owner
- amount_bld_to_burn

## Requirement

amount_bld_to_burn = 11 BLD

## Updates

- owner
- build_id / build_mint
- created_at
- updated_at
- version

## Validation

- User must burn exactly 11 BLD through the approved path.
- Burn must be final before Build creation is accepted.
- Build must not already exist for the same canonical identity.
- This instruction must not create earned_bld.
- This instruction must not create XEN burn history.

---

## 5. process_registrar_message

## Purpose

Process a verified Ethereum Registrar message.

This is the common entry point for Ethereum-side updates.

## Inputs

- message_id
- message_type
- source_chain_id
- source_contract
- source_tx_hash
- source_index
- ethereum_identity
- build_id
- payload

## Updates

Depends on message_type.

Possible message types:

- CONNECT_XC_HISTORY
- ADD_CORE_REDEEM
- ADD_XEN_BURN_POWER
- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD

## Validation

- message_id must not be already processed.
- message must come from approved registrar / trust path.
- ethereum_identity must be canonical for the target Build.
- message_type must be supported.
- payload must match message_type.
- One processed message can produce only one deterministic state transition.

## Replay protection

After successful processing:

processed_messages[message_id] = true

---

## 6. connect_xc_history

## Purpose

Connect valid xEnchanted Crypto history to a Build.

This may be handled directly or as a message_type inside process_registrar_message.

## Inputs

- ethereum_identity
- build_id
- tiered_origin_bld
- genesis_origin_eligible

## Updates

If eligible:

- origin_bld
- available_bld
- updated_at

## Validation

- Genesis Origin BLD may be granted only once.
- Genesis Origin BLD may be granted only during Build Genesis Epoch.
- tiered_origin_bld must match the earned_bld tier.
- origin_bld must not increase earned_bld.

## Accounting

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

---

## 7. add_core_redeem

## Purpose

Create earned BLD from a verified Core redeem event.

This may be handled as ADD_CORE_REDEEM inside process_registrar_message.

## Inputs

- redeem_key
- xen_burned
- normalized_bld
- token_id
- source data

## Updates

- earned_bld
- available_bld
- updated_at

## Validation

- redeem_key must be unique.
- normalized_bld must be derived from xen_burned.
- Source event must be verified by Ethereum Registrar.
- Same Core redeem must not be counted twice.

## Accounting

earned_bld += normalized_bld
available_bld += normalized_bld

---

## 8. add_xen_burn_power

## Purpose

Create XEN Burn Power from a verified XEN.burn(user, amount) call.

This may be handled as ADD_XEN_BURN_POWER inside process_registrar_message.

## Inputs

- xen_burn_key
- user
- amount
- normalized_xbp
- source data

## Updates

- earned_xbp
- available_xbp
- updated_at

## Validation

- xen_burn_key must be unique.
- Source must be successful XEN.burn(user, amount).
- Zero-address transfers must not be counted.
- XBP must not create BLD.

## Accounting

earned_xbp += normalized_xbp
available_xbp += normalized_xbp

---

## 9. lock_xntd

## Purpose

Record XNTD lock state and activate XC commitment if requirements are satisfied.

This may be handled as LOCK_XNTD inside process_registrar_message.

## Inputs

- locked_xntd
- required_xntd_lock
- lock_epoch

## Updates

- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- updated_at

## Validation

- Build must have earned_bld > 0.
- required_xntd_lock must be based on current XC epoch Core L1 nominal.
- Lock state must be verified.
- Lock must not create BLD or XBP.

## Activation

xc_commitment_active =
  earned_bld > 0
  AND locked_xntd >= required_xntd_lock

---

## 10. unlock_xntd

## Purpose

Record updated XNTD lock state after unlock.

This may be handled as UNLOCK_XNTD inside process_registrar_message.

## Inputs

- locked_xntd
- required_xntd_lock
- lock_epoch

## Updates

- locked_xntd
- xc_commitment_active
- updated_at

## Validation

- Unlock state must be verified.
- Unlock must not modify BLD or XBP.

## Deactivation

If locked_xntd < required_xntd_lock:

xc_commitment_active = false

---

## 11. relock_xntd

## Purpose

Update XNTD commitment under a new XC epoch requirement.

This may be handled as RELOCK_XNTD inside process_registrar_message.

## Inputs

- new_locked_xntd
- new_required_xntd_lock
- new_lock_epoch

## Updates

- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- updated_at

## Validation

Relock is allowed only if:

available_bld >= earned_bld

The new required lock must be based on current XC epoch Core L1 nominal.

Relock must not create BLD or XBP.

---

## 12. update_fee_checkpoint

## Purpose

Update X1 Fee Contribution from an approved X1 Fee Indexer checkpoint.

## Inputs

- total_fee_paid
- total_tx_count
- counted_until_slot
- last_updated

## Updates

- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at
- updated_at

## Validation

- Checkpoint must come from approved indexer / trust path.
- counted_until_slot must be greater than stored x1_fee_counted_until_slot.
- Fee values must be cumulative totals.
- This instruction must not create BLD or XBP.

## Accounting

x1_fee_contribution = total_fee_paid
x1_tx_count = total_tx_count
x1_fee_counted_until_slot = counted_until_slot
last_fee_update_at = last_updated

---

## 13. burn_bld

## Purpose

Burn available BLD through an approved Build Program path.

This can be used for Build creation or future mechanics.

## Inputs

- owner
- amount
- burn_reason

## Updates

- available_bld
- updated_at

## Validation

- available_bld must be greater than or equal to amount.
- Burn must not decrease earned_bld.
- Burn must not create XBP.
- Burn reason must be supported.

---

## 14. transfer_bld

## Purpose

Transfer available BLD if BLD is implemented as transferable state or tokenized balance.

## Inputs

- sender
- recipient
- amount

## Updates

- sender.available_bld
- recipient.available_bld
- updated_at for both relevant records if applicable

## Validation

- sender.available_bld must be greater than or equal to amount.
- Transfer must not decrease sender.earned_bld.
- Transfer must not increase recipient.earned_bld.
- Transfer must not create new BLD supply.

## Note

The exact transfer model is not finalized.

This instruction may be replaced by a separate token program model.

---

## 15. burn_xbp

## Purpose

Burn or use available XBP through an approved future mechanic.

## Inputs

- owner
- amount
- burn_reason

## Updates

- available_xbp
- updated_at

## Validation

- available_xbp must be greater than or equal to amount.
- Burn must not decrease earned_xbp.
- Burn must not create BLD.
- Burn reason must be supported.

## Note

The exact XBP use model is not finalized.

---

## 16. read_build_view

## Purpose

Expose readable Build data for users and other X1 projects.

## Output

- owner
- build_id
- earned_bld
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

---

## 17. read_fee_contribution_view

## Purpose

Expose X1 Fee Contribution data in a minimal reader format.

## Output

- total_fee_paid
- tx_count
- counted_until_slot
- last_updated

---

## 18. Main invariants

- create_build alone does not create contribution fields.
- create_build_with_bld_burn does not create earned_bld.
- Registrar messages must be replay-protected.
- Core redeem is the only source of earned_bld.
- XEN.burn(user, amount) is the only source of earned_xbp.
- X1 Fee Contribution comes only from approved cumulative checkpoints.
- XNTD lock does not create BLD or XBP.
- available_bld may change through allowed use, burn, transfer, or origin/earned updates.
- earned_bld remains historical.
- available_bld must be greater than or equal to earned_bld for relock.

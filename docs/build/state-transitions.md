# X1 Build v1 State Transitions

## 1. Purpose

This document describes the main state transitions for X1 Build v1.

The goal is to define which actions can change BuildState, which fields they update, and which validation rules must be enforced.

Main rule:

Each field must be updated only through its own valid source path.

---

## 2. create_build

Creates a canonical Build object for a user.

## Inputs

- owner
- build_mint / build_id

## Updates

- owner
- build_mint / build_id
- created_at
- updated_at
- version

## Validation

- The identity must not already have a canonical Build.
- The Build identifier must be unique.
- No contribution fields are created by this transition alone.

## Notes

Build creation alone does not create earned_bld, earned_xbp, or X1 Fee Contribution.

---

## 3. create_build_with_bld_burn

Creates an active Build in X1 by burning BLD.

## Inputs

- owner
- amount_bld_to_burn

## Requirement

amount_bld_to_burn = 11 BLD

## Updates

- owner
- build_mint / build_id
- created_at
- updated_at
- version

## Validation

- User must burn exactly 11 BLD through the approved Build Program path.
- The identity must not already have a canonical Build.
- The burn must be final before Build creation is accepted.

## Notes

This transition does not create fake earned_bld.

It does not create fake XEN burn history.

It does not require XNTD lock or relock.

---

## 4. connect_xc_history

Connects valid xEnchanted Crypto history to an existing or newly created Build.

## Inputs

- Ethereum identity
- Build identifier
- proof/message from Ethereum registrar

## Updates

- updated_at
- optional Genesis Origin BLD if eligible

## Validation

- Ethereum identity must be canonical for this Build.
- Source message must not be replayed.
- Genesis Origin BLD may be granted only once during Build Genesis Epoch.
- Connecting history alone must not create earned_bld unless a valid Core redeem source event is included.

## Genesis Origin rule

If eligible:

origin_bld += 121
available_bld += 121

origin_bld must not increase earned_bld.

---

## 5. add_core_redeem

Adds earned BLD from a verified Core redeem event.

## Inputs

- redeem_key
- xen_burned
- token_id
- Ethereum source data

## Updates

- earned_bld
- available_bld
- updated_at

## Validation

- redeem_key must be unique.
- Core redeem event must be verified by Ethereum registrar.
- BLD amount must be normalized from Core.xenBurned.
- The same Core redeem must not be counted more than once.

## Accounting

earned_bld += normalized(Core.xenBurned)
available_bld += normalized(Core.xenBurned)

## Normalization

1 BLD = 100,000,000 XEN burned through redeemed Core history.

Internal storage should support fractional BLD.

---

## 6. add_xen_burn_power

Adds XEN Burn Power from verified global XEN burn calls.

## Inputs

- xen_burn_key
- user
- amount
- Ethereum source data

## Updates

- earned_xbp
- available_xbp
- updated_at

## Validation

- xen_burn_key must be unique.
- Source must be a successful XEN.burn(user, amount) call.
- Arbitrary transfers to the zero address must not be counted.
- XBP must not be mixed with BLD.

## Accounting

earned_xbp += normalized(amount)
available_xbp += normalized(amount)

## Normalization

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount).

---

## 7. lock_xntd

Locks XNTD to activate the XC commitment layer.

## Inputs

- amount
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
- Required lock must be determined from the current XC epoch minimum Core L1 nominal.
- Actual locked amount must match the lock state.
- Lock must not create BLD or XBP.

## Activation rule

xc_commitment_active =
  earned_bld > 0
  AND locked_xntd >= required_xntd_lock

---

## 8. unlock_xntd

Unlocks XNTD and may deactivate the XC commitment layer.

## Inputs

- amount_to_unlock

## Updates

- locked_xntd
- xc_commitment_active
- updated_at

## Validation

- Unlock must not make locked_xntd negative.
- Unlock must not modify earned_bld, available_bld, earned_xbp, or available_xbp.

## Deactivation rule

If locked_xntd < required_xntd_lock:

xc_commitment_active = false

---

## 9. relock_xntd

Updates XNTD lock under a new epoch requirement.

## Inputs

- new_required_xntd_lock
- new_lock_epoch
- new_locked_xntd

## Updates

- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- updated_at

## Validation

Relock is allowed only if:

available_bld >= earned_bld

The new required lock must be determined from the current XC epoch minimum Core L1 nominal.

Relock must not create BLD or XBP.

## Notes

If available_bld < earned_bld, the user must restore available_bld before relock.

---

## 10. update_fee_checkpoint

Updates X1 Fee Contribution from an indexer checkpoint.

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

Accept only newer checkpoints:

counted_until_slot > x1_fee_counted_until_slot

Checkpoint must come from the approved X1 Fee Indexer / checkpoint path.

## Accounting

x1_fee_contribution = total_fee_paid
x1_tx_count = total_tx_count
x1_fee_counted_until_slot = counted_until_slot

## Notes

This tracks fee payment activity, not necessarily user activity.

Sponsored transactions credit the fee payer, not necessarily the signer.

---

## 11. transfer_or_use_available_bld

Changes available BLD through an allowed mechanism.

## Inputs

- amount
- action_type

## Updates

- available_bld
- updated_at

## Validation

- available_bld must not become negative.
- earned_bld must not decrease.
- origin_bld must not be changed unless the action specifically consumes available origin balance through accounting rules.

## Notes

Selling, transferring, burning, or using available BLD affects available_bld only.

Historical earned_bld remains unchanged.

---

## 12. transfer_or_use_available_xbp

Changes available XBP through an allowed mechanism.

## Inputs

- amount
- action_type

## Updates

- available_xbp
- updated_at

## Validation

- available_xbp must not become negative.
- earned_xbp must not decrease.
- XBP must not be converted into BLD by this transition.

---

## 13. Main invariants

- earned_bld is created only from verified Core redeem history.
- origin_bld is created only through Genesis Origin allocation.
- earned_xbp is created only from verified XEN.burn(user, amount) calls.
- X1 Fee Contribution is updated only through cumulative checkpoints.
- XNTD lock does not create BLD or XBP.
- BLD and XBP are separate accounting layers.
- One source event can update one Build only once.
- Build updates must be append-only by verified source events or valid checkpoints.

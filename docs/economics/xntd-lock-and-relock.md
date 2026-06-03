# XNTD Lock and Relock

## 1. Purpose

This document describes the conceptual XNTD lock and relock model for X1 Build.

XNTD lock is a commitment layer for Builds that receive history_bld through xEnchanted Crypto Core redeem history.

The goal is to reduce the simple extraction path:

mint Core L1
-> redeem Core L1
-> receive XNTD
-> create / update Build
-> sell all XNTD

XNTD lock does not create BLD.

XNTD lock does not create XBP.

XNTD lock is a commitment condition, not a contribution source.

---

## 2. When XNTD lock is required

XNTD lock is required only for Builds that receive history_bld.

A Build has XC-derived history if:

history_bld > 0

If a Build is created in X1 through burn 11 BLD and does not receive history_bld, XNTD lock is not required.

---

## 3. Required lock amount

The required lock amount is tied to the minimum Core L1 nominal of the current XC epoch.

Rule:

required_xntd_lock = current epoch Core L1 nominal

This makes the lock adaptive to XC epoch economics.

It avoids a fixed lock amount that becomes too high or too low across epochs.

---

## 4. Source of required lock value

The required lock amount should be determined by the Ethereum Registrar using Ethereum-side xEnchanted Crypto protocol state.

The X1 Build Program should not independently calculate XC epoch economics unless the required protocol parameters are mirrored safely.

For MVP:

Ethereum Registrar determines:

- current XC epoch
- current Core L1 nominal
- required_xntd_lock

Then sends this data through a verified registrar message.

---

## 5. Lock activation rule

The XC commitment layer is active only if:

xc_commitment_active =
  history_bld > 0
  AND locked_xntd >= required_xntd_lock

If history_bld = 0, the Build does not need XC commitment activation.

If locked_xntd is below required_xntd_lock, the XC commitment layer is inactive.

---

## 6. Lock message

A LOCK_XNTD registrar message should include:

- build_id
- ethereum_identity
- locked_xntd
- required_xntd_lock
- lock_epoch
- source transaction data
- message_id

The message should update:

- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- updated_at

---

## 7. Unlock

Unlocking reduces or removes locked XNTD.

An unlock may deactivate the XC commitment layer.

If after unlock:

locked_xntd < required_xntd_lock

then:

xc_commitment_active = false

Unlock must not modify:

- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp
- x1_fee_contribution

---

## 8. Relock

Relock updates the XNTD commitment under a new XC epoch requirement.

This is useful when the XC epoch changes and the minimum Core L1 nominal changes.

Relock may allow a user to adjust locked XNTD to the new epoch requirement.

---

## 9. Relock integrity rule

Relock is allowed only if:

available_bld >= history_bld

This protects the integrity of the Build.

Meaning:

If a user sold, transferred, burned, or used too much available BLD, they must restore available_bld before relock.

---

## 10. Why relock requires available_bld >= history_bld

history_bld records historical contribution.

available_bld is the user's current usable BLD balance.

If a user monetized part of available BLD, relock should not allow them to reduce or update XNTD commitment while their available balance is below their historical Build level.

This creates a clear choice:

- keep enough available BLD and relock normally
- sell/use BLD and delay relock until available BLD is restored

---

## 11. Relock message

A RELOCK_XNTD registrar message should include:

- build_id
- ethereum_identity
- new_locked_xntd
- new_required_xntd_lock
- new_lock_epoch
- source transaction data
- message_id

The message should update:

- locked_xntd
- required_xntd_lock
- lock_epoch
- xc_commitment_active
- updated_at

Relock must not create:

- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp

---

## 12. Example

Initial state:

history_bld = 100
available_bld = 122
locked_xntd = 100
required_xntd_lock = 100
xc_commitment_active = true

User sells 30 BLD:

history_bld = 100
available_bld = 92

Epoch changes and user wants relock.

Relock check:

available_bld >= history_bld

92 >= 100 is false.

Relock is not allowed.

User restores 8 BLD:

available_bld = 100

Relock check:

100 >= 100 is true.

Relock is allowed.

---

## 13. Native X1 Build path

A Build created through burn 11 BLD does not need XNTD lock by default.

Reason:

This path does not mint XNTD through Core redeem.

It does not create the Core redeem -> XNTD sell pressure that the lock is designed to reduce.

If the same Build later connects XC history and receives history_bld, then XNTD lock rules apply.

---

## 14. Lock is not contribution

XNTD lock should not be interpreted as contribution by itself.

It is a commitment condition.

It may make XC-derived history active or inactive, but it does not create new BLD or XBP.

Other projects may choose to read locked_xntd or xc_commitment_active, but the Build model should keep it separate from contribution fields.

---

## 15. Implementation direction

For MVP, XNTD lock state can be verified on Ethereum side by the registrar and reflected into X1 Build through registrar messages.

Future options:

- direct bridge-aware lock proof
- threshold-signed registrar messages
- Merkle proofs of lock state
- X1-side mirrored XNTD representation

The MVP should prioritize clarity and replay protection.

---

## 16. Main invariants

- XNTD lock is required only for Builds with history_bld > 0.
- required_xntd_lock equals current epoch Core L1 nominal.
- XNTD lock does not create BLD.
- XNTD lock does not create XBP.
- Unlock may deactivate xc_commitment_active.
- Relock requires available_bld >= history_bld.
- Native Build creation through burn 11 BLD does not require XNTD lock unless XC history is later connected.

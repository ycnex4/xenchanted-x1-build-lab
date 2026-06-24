<!--
Superseded note:
This document contains pre-cleanup Build balance terminology such as available_bld,
earned_xbp, available_xbp, one-time Genesis Origin claim, or relock-by-available_bld.
For the current authoritative Build State model, use:

- docs/build/build-state-history-identity-model.md
- docs/build/build-v1-spec.md
- docs/checkpoints/build-state-history-identity-cleanup.md

Current model:
Build State stores durable public history, not live spendable balances.
Build Identity stores owner-controlled name/logo metadata.
Future spendable BLD belongs to a separate BLD asset / ledger / escrow layer.
-->

# Program Authority Model

## 1. Purpose

This document describes the conceptual authority model for the X1 Build Program.

The goal is to separate protocol-style user state from infrastructure trust needed for MVP registrar and indexer updates.

This is not implementation code.

---

## 2. Core distinction

xEnchanted Crypto core protocol is immutable and no-admin.

X1 Build is a separate X1-side system that reflects verified contribution layers.

Because X1 Build depends on cross-chain data and indexer data, MVP may require controlled infrastructure authorities.

This does not mean that those authorities control the Ethereum xEnchanted Crypto core protocol.

They only control which verified external updates are accepted by the X1 Build Program.

---

## 3. Authority categories

The X1 Build Program may need several authority categories:

- registrar_authority
- fee_indexer_authority
- config_authority
- emergency_authority, optional
- upgrade_authority, if the program is upgradeable during MVP

Each authority must have a clearly limited role.

---

## 4. registrar_authority

## Purpose

Submits Ethereum Registrar messages.

## Allowed actions

- CONNECT_XC_HISTORY
- ADD_CORE_REDEEM
- ADD_XEN_BURN_POWER
- LOCK_XNTD
- UNLOCK_XNTD
- RELOCK_XNTD

## Must not do

registrar_authority must not:

- arbitrarily set Build totals
- bypass replay protection
- create history_bld without Core redeem source
- create XBP without XEN.burn(user, amount) source
- modify X1 Fee Contribution
- change program config unless explicitly allowed

---

## 5. fee_indexer_authority

## Purpose

Submits X1 Fee Contribution checkpoints.

## Allowed actions

- update_fee_checkpoint

## Must not do

fee_indexer_authority must not:

- create history_bld
- create available_bld
- create origin_bld
- create XBP
- modify XNTD lock state
- modify registrar data

---

## 6. config_authority

## Purpose

Manages program-level configuration during MVP.

Possible configuration fields:

- registrar_authority
- fee_indexer_authority
- build_genesis_epoch_start
- build_genesis_epoch_end
- signer set, if multi-signer model is added
- registrar version
- accepted message domains

## Important

config_authority is the most sensitive authority.

Its power should be minimized.

For production, this authority should either be:

- removed
- time-locked
- multi-sig controlled
- replaced with immutable config
- replaced with threshold governance / signer set rules

---

## 7. emergency_authority

## Purpose

Optional emergency authority for MVP infrastructure risk.

Possible powers:

- pause registrar message processing
- pause fee checkpoint processing
- prevent new external updates during incident response

## Must not do

emergency_authority must not:

- edit existing Build history
- mint BLD
- mint XBP
- delete source protection records
- rewrite contribution state

## Design preference

Avoid emergency authority if possible.

If used, keep it narrow and document it clearly as infrastructure protection.

---

## 8. upgrade_authority

## Purpose

If the X1 Build Program is upgradeable during MVP, upgrade_authority controls program upgrades.

## Risk

This is the strongest authority.

An upgradeable program can change rules.

This is not equivalent to immutable/no-admin protocol behavior.

## MVP position

Upgradeability may be acceptable during early design and testing if disclosed.

Before production, the project should decide whether to:

- freeze / remove upgrade authority
- move to governed upgrade process
- deploy a final immutable version
- keep upgradeability but clearly label Build as infrastructure-managed

---

## 9. Authority minimization principle

Every authority should be minimized.

A good authority model answers:

- What can this authority do?
- What can it not do?
- Which fields can it modify?
- Which fields are protected by source-event rules?
- Can this authority be removed later?
- What happens if the authority is compromised?

---

## 10. Protected fields

These fields should not be arbitrarily set by any authority:

- history_bld
- available_bld
- origin_bld
- earned_xbp
- available_xbp
- locked_xntd
- required_xntd_lock
- x1_fee_contribution

They may change only through valid instructions and validated source paths.

---

## 11. Source protection is stronger than authority

Even trusted authorities must pass replay and source checks.

Required protections:

- processed_messages[message_id]
- used_redeem_events[redeem_key]
- used_xen_burn_events[xen_burn_key]
- genesis_origin_claimed[identity]
- canonical_build_by_identity[identity]

An authority should not be able to apply the same source event twice.

---

## 12. MVP authority model

Suggested MVP model:

- registrar_authority: trusted signer
- fee_indexer_authority: trusted signer
- config_authority: deployer / controlled signer
- emergency_authority: optional
- upgrade_authority: allowed only during early MVP, if needed

This should be explicitly documented as MVP infrastructure trust.

---

## 13. Production direction

Possible production direction:

- registrar messages require threshold signatures
- fee checkpoints require trusted signer set or public root
- config authority is removed or time-locked
- upgrade authority is removed or heavily constrained
- emergency authority is removed or limited to pausing external updates only

The long-term goal is to reduce discretionary authority.

---

## 14. Relation to first principles

The first-principles commitment applies strictly to the immutable xEnchanted Crypto core protocol.

X1 Build is a derived X1-side system.

It can use infrastructure authorities during MVP as long as:

- this is disclosed
- authorities cannot rewrite XC core rules
- contribution fields remain source-based
- arbitrary totals are not accepted
- trust model evolution is documented

---

## 15. Main invariants

- Authorities must be limited by role.
- Registrar authority cannot bypass source rules.
- Fee indexer authority cannot create BLD or XBP.
- Config authority should be minimized.
- Emergency authority, if used, should pause only external updates.
- Upgrade authority is a production-risk decision.
- Build state should remain source-based, not opinion-based.

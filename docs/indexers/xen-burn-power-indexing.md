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

# XEN Burn Power Indexing

## 1. Purpose

This document describes how to index Global XEN Burn Power for X1 Build.

XEN Burn Power represents verified global XEN burn participation.

It is separate from BLD.

BLD comes only from redeemed Core NFT history.

XEN Burn Power comes from verified XEN.burn(user, amount) calls.

---

## 2. Main rule

Only official XEN burn calls should be counted.

Canonical source:

successful XEN.burn(user, amount)

Arbitrary transfers to the zero address must not be counted as XEN Burn Power.

This keeps the metric tied to the official XEN burn interface.

---

## 3. What should be indexed

The indexer should detect successful calls to:

XEN.burn(address user, uint256 amount)

The indexer should support:

- top-level calls directly to the XEN contract
- internal calls through projects that call the official XEN burn function

---

## 4. Top-level calls

For MVP, the indexer may start with top-level successful transactions where:

to = XEN contract
selector = burn(address,uint256)
status = success

The indexer extracts:

- user
- amount
- tx_hash
- block_number
- chain_id
- XEN contract address

This is simpler but may miss burns made through other contracts.

---

## 5. Internal calls / traces

For production, the indexer should read traces and detect any successful internal call where:

target = XEN contract
selector = burn(address,uint256)
status = success

This allows the system to count burns made through other projects that use the official XEN burn interface.

The indexer extracts:

- user
- amount
- tx_hash
- trace_index / call_index
- block_number
- caller / source_contract
- chain_id
- XEN contract address

---

## 6. XEN burn key

Each indexed burn call must have a unique key.

xen_burn_key = hash(
  chain_id,
  xen_contract,
  tx_hash,
  trace_index,
  user,
  amount
)

This prevents one XEN.burn(user, amount) call from creating XBP more than once.

For top-level calls, trace_index may be set to a canonical value or replaced with the top-level call index.

---

## 7. Normalization

XEN Burn Power should use the same denominator as BLD.

Display unit:

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount)

Accounting:

earned_xbp += normalized(amount)
available_xbp += normalized(amount)

Internal storage should support fractional XBP.

---

## 8. Fields updated

A valid XEN burn source updates:

- earned_xbp
- available_xbp
- updated_at

It must not update:

- history_bld
- available_bld
- origin_bld
- locked_xntd
- x1_fee_contribution

---

## 9. Indexer database fields

Suggested off-chain record:

- xen_burn_key
- chain_id
- xen_contract
- user
- amount
- normalized_xbp
- tx_hash
- trace_index / call_index
- block_number
- caller / source_contract
- status
- indexed_at
- applied_to_build

---

## 10. Update model

The indexer should not send arbitrary totals.

It should send new verified source events or batches of source events.

Each source event must be deduplicated by xen_burn_key.

The Build Program or registrar path should reject already used xen_burn_key values.

---

## 11. Attribution

The XEN Burn Power belongs to the user argument passed into:

XEN.burn(user, amount)

This is different from tx sender and different from caller.

If a project contract calls XEN.burn(user, amount), the contribution should be attributed to user, not necessarily to the project contract.

---

## 12. Trust model

MVP may use a trusted indexer.

Future versions may use:

- multiple watchers
- threshold-signed source batches
- public indexed datasets
- Merkle roots and proofs

The critical requirement is that each update remains tied to unique source calls.

---

## 13. Main invariants

- XBP is created only from successful XEN.burn(user, amount) calls.
- Zero-address transfers are not enough.
- XBP does not create BLD.
- BLD does not create XBP.
- One XEN burn source can update one Build only once.
- The user argument inside XEN.burn(user, amount) is the attribution target.


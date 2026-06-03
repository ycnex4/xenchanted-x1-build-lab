# X1 Fee Contribution

## 1. Purpose

X1 Fee Contribution records how much network fee an address has paid on X1 as transaction fee payer.

The goal is to represent real network-level contribution through paid transaction fees.

This metric is not limited to one project or one protocol action.

If an address pays fees to use X1, that address contributes value to the X1 network.

Other X1 projects may interpret this value independently.

---

## 2. What is counted

Tracked value:

x1_fee_contribution =
sum of all network fees paid by an address as transaction fee payer on X1

Important distinction:

fee payer != signer

The contribution is credited to the address that actually paid the network fee.

If a user signs a transaction but another address pays the fee, the fee contribution belongs to the fee payer.

---

## 3. Scope

The metric should count all confirmed X1 transactions where:

fee_payer = user address

This is intentional.

The metric should represent general network contribution, not only participation in one application.

---

## 4. Technical model

In a Solana-like environment, the actual finalized transaction fee is not reliably readable by a program during instruction execution.

The finalized fee is part of transaction metadata / runtime accounting after confirmation.

Therefore, the correct model is indexer-based.

Flow:

1. X1 transaction is confirmed.
2. Indexer reads transaction metadata.
3. Indexer extracts:
   - fee payer
   - fee paid
   - transaction signature
   - slot / block
4. Indexer aggregates fee data per fee payer.
5. A periodic checkpoint is submitted.
6. On-chain state records the new cumulative value.

---

## 5. Fees included

The metric should include:

- base fee
- priority fee

Priority fee is also a real fee paid by the fee payer and should not be excluded.

---

## 6. Storage

On-chain state should store raw smallest X1 units.

Display normalization should happen off-chain or in the UI.

Suggested type:

u64

---

## 7. Minimal fields

- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at

Where:

x1_fee_contribution =
total network fees paid by this address as fee payer

x1_tx_count =
number of counted transactions where this address was fee payer

x1_fee_counted_until_slot =
latest slot included in the accounting checkpoint

last_fee_update_at =
timestamp / slot of last update

---

## 8. Cumulative checkpoint model

Instead of submitting every transaction individually, use cumulative checkpoints.

Example checkpoint:

address: user
total_fee_paid: 123456789
total_tx_count: 842
counted_until_slot: 10000000

The update should be accepted only if:

new_counted_until_slot > previous_counted_until_slot

This avoids one on-chain record per transaction and keeps updates cheaper.

---

## 9. Replay and double-counting protection

With cumulative checkpoints, the basic protection is:

only accept newer checkpoints

If:

new_counted_until_slot <= stored_counted_until_slot

then reject the update.

If a per-transaction model is ever used, each transaction would need a unique key:

fee_event_key = hash(
  x1_chain_id,
  tx_signature,
  fee_payer,
  fee_amount
)

For scalability, cumulative checkpoints are preferred.

---

## 10. Trust model

## MVP

Trusted indexer.

This is acceptable for MVP because the metric is informational / contribution-tracking infrastructure, not direct financial settlement.

## Future options

- threshold-signed checkpoints
- Merkle root + user proof
- independent public indexers

A stronger trust model can be added when other projects begin using the metric for more sensitive decisions.

---

## 11. Sponsored transaction behavior

This metric tracks fee payment activity, not user activity.

Known characteristic:

- If fee_payer = user, contribution belongs to user.
- If fee_payer = relayer, contribution belongs to relayer.
- If fee_payer = dApp, contribution belongs to dApp.

This is correct for this metric because it follows runtime-level fee payer attribution.

---

## 12. Reader interface

Other X1 projects should be able to read:

FeeContributionView:
- total_fee_paid
- tx_count
- counted_until_slot
- last_updated

This makes the metric usable as ecosystem infrastructure.

---

## 13. Main rule

X1 Fee Contribution records fee payment activity.

It should remain a separate raw cumulative metric.

It should not be merged into a universal score by default.

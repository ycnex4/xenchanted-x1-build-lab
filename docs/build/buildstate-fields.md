# BuildState v1 Fields

## 1. Identity fields

| Field | Meaning | Updated by | Notes |
|---|---|---|---|
| owner | X1 address that owns the Build | Build Program | Main owner / holder of the Build |
| build_mint / build_id | Unique Build identifier | Build Program | NFT-like Build object identifier |
| created_at | Creation timestamp / slot | Build Program | Set once |
| updated_at | Last state update timestamp / slot | Build Program | Updated on any valid state change |
| version | State version | Build Program | Allows future migrations / extensions |

---

## 2. BLD fields

| Field | Meaning | Source | Updated by | Validation |
|---|---|---|---|---|
| history_bld | Historical BLD earned from redeemed Core NFT history | Core redeem events | Ethereum Registrar -> Build Program | Only from unique Core redeem source events |
| available_bld | Current available BLD | Earned BLD, Origin BLD, transfers, burns, uses | Build Program | Cannot be changed by arbitrary totals |
| origin_bld | Genesis Origin allocation | First valid XC history connection during Build Genesis Epoch | Ethereum Registrar -> Build Program | Granted once only as tiered allocation, not counted as earned BLD |

## Rules

Core redeem:

history_bld += normalized(Core.xenBurned)
available_bld += normalized(Core.xenBurned)

Genesis Origin allocation:

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

origin_bld must not increase history_bld.

Display unit:

1 BLD = 100,000,000 XEN burned through redeemed Core history

Internal storage should support fractional BLD.

---

## 3. XEN Burn Power fields

| Field | Meaning | Source | Updated by | Validation |
|---|---|---|---|---|
| earned_xbp | Historical global XEN Burn Power | Successful XEN.burn(user, amount) calls | Ethereum Registrar / Indexer -> Build Program | Only from unique XEN burn source events |
| available_xbp | Current available XEN Burn Power | Earned XBP and future allowed mechanics | Build Program | Must not be mixed with BLD |

## Rules

XEN.burn(user, amount):

earned_xbp += normalized(amount)
available_xbp += normalized(amount)

Display unit:

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount)

XBP and BLD are separate accounting layers.

---

## 4. XNTD commitment fields

| Field | Meaning | Source | Updated by | Validation |
|---|---|---|---|---|
| locked_xntd | Amount of XNTD currently locked | XNTD lock / unlock / relock action | Build Program / Registrar path | Must match actual lock state |
| required_xntd_lock | Required lock amount for current commitment | Current XC epoch minimum Core L1 nominal | Ethereum Registrar -> Build Program | Determined by registrar at lock/relock time |
| lock_epoch | Epoch under which current lock was made | XC epoch | Ethereum Registrar -> Build Program | Updated on lock/relock |
| xc_commitment_active | Whether XC-derived commitment is active | Derived field | Build Program | history_bld > 0 AND locked_xntd >= required_xntd_lock |

## Rules

xc_commitment_active =
  history_bld > 0
  AND locked_xntd >= required_xntd_lock

Relock allowed only if:

available_bld >= history_bld

Relock should not create new BLD or XBP.

Relock only updates:

locked_xntd
required_xntd_lock
lock_epoch
xc_commitment_active
updated_at

---

## 5. X1 Fee Contribution fields

| Field | Meaning | Source | Updated by | Validation |
|---|---|---|---|---|
| x1_fee_contribution | Total network fees paid by owner as X1 fee payer | X1 transaction metadata | X1 Fee Indexer -> Build Program | Cumulative checkpoint only |
| x1_tx_count | Number of counted X1 transactions where owner was fee payer | X1 transaction metadata | X1 Fee Indexer -> Build Program | Must come from checkpoint |
| x1_fee_counted_until_slot | Latest X1 slot included in fee accounting | X1 transaction metadata | X1 Fee Indexer -> Build Program | New checkpoint slot must be greater |
| last_fee_update_at | Last fee contribution update timestamp / slot | Build Program | Updated with checkpoint | Informational |

## Rules

Only accept checkpoint if:

new_counted_until_slot > x1_fee_counted_until_slot

x1_fee_contribution = cumulative total fees paid by address as fee payer.

This includes:

base fee
priority fee

This tracks fee payment activity, not necessarily user activity.

---

## 6. Source protection fields / maps

These may live in registrar/indexer state, Build Program state, or both depending on final architecture.

| Key | Purpose |
|---|---|
| used_redeem_events[redeem_key] | Prevents one Core redeem from creating BLD more than once |
| used_xen_burn_events[xen_burn_key] | Prevents one XEN burn call from creating XBP more than once |
| processed_messages[message_id] | Prevents replayed bridge / registrar messages |
| genesis_origin_claimed[identity] | Prevents repeated Genesis Origin BLD allocation |
| canonical_build_by_identity[identity] | Prevents creating multiple canonical Builds from the same identity |

## Core redeem key

redeem_key = hash(
  chain_id,
  core_contract,
  token_id,
  tx_hash,
  log_index
)

## XEN burn key

xen_burn_key = hash(
  chain_id,
  xen_contract,
  tx_hash,
  trace_index,
  user,
  amount
)

## Message key

message_id = hash(
  source_chain_id,
  source_contract,
  event_type,
  tx_hash,
  log_index_or_trace_index
)

---

## 7. Reader interface

Other X1 projects should be able to read Build data without needing to understand the full history.

Suggested readable output:

BuildView:
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

FeeContributionView:
- total_fee_paid
- tx_count
- counted_until_slot
- last_updated

---

## 8. Minimal v1 state summary

Minimal BuildState v1:

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

Main rule:

Each field must be updated only by its own valid source path.

BLD does not come from XBP.
XBP does not come from BLD.
Fee Contribution does not create BLD or XBP.
XNTD lock does not create BLD or XBP.



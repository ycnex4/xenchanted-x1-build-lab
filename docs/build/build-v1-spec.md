# X1 Build v1 Spec

## 1. Purpose

X1 Build is a voluntary NFT-like user object in X1 that records independent verified contribution layers:

1. Core redeem contribution in xEnchanted Crypto.
2. Global XEN Burn Power.
3. XNTD commitment.
4. X1 network fee contribution.
5. Build creation in X1 through BLD burn.

The goal is not to merge all values into one universal score.

The goal is to expose separate, readable, verifiable fields that other X1 projects may interpret independently.

---

## 2. Core principles

### 2.1 One Build, multiple verified layers

A user should have one canonical Build.

The Build may receive new verified data from different sources over time.

Build is one object with appendable verified layers.

### 2.2 No arbitrary totals

The Build must not accept arbitrary totals such as:

user has 100 BLD
user has 500 XBP

It should accept only verified source events, valid state transitions, or trusted checkpoints.

### 2.3 Independent accounting

The main fields must not be mixed:

BLD != XEN Burn Power != X1 Fee Contribution != XNTD Lock

Each field has its own source, meaning, and update rules.

---

## 3. BLD

### 3.1 Meaning

BLD is the normalized Build unit derived from redeemed Core NFT history.

It represents contribution through xEnchanted Crypto.

### 3.2 Source

Only redeemed Core NFT history creates earned BLD.

Core redeem
-> read Core.xenBurned
-> normalize
-> earned_bld

Actions that do not create earned BLD:

- Forge mint
- Stake redeem
- Enchant
- Market activity
- XNTD lock
- XNTD burn
- Global XEN burn outside XC
- X1 fee contribution

### 3.3 Normalization

BLD should not be equal 1:1 to raw XEN amount.

Display unit:

1 BLD = 100,000,000 XEN burned through redeemed Core history

Internal storage should support fractional values for future epochs.

Examples:

100,000,000 XEN burn -> 1 BLD
50,000,000 XEN burn  -> 0.5 BLD
25,000,000 XEN burn  -> 0.25 BLD

### 3.4 Fields

- earned_bld
- available_bld
- origin_bld

### 3.5 earned_bld

earned_bld is the historical BLD earned through redeemed Core NFT history.

It does not decrease when the user sells, spends, or burns available BLD.

### 3.6 available_bld

available_bld is the currently available BLD.

It may change through allowed mechanics such as sale, transfer, purchase, burn, or other future actions.

### 3.7 origin_bld

origin_bld is the Genesis Origin allocation.

It is not earned BLD.

It exists to seed Build creation in X1 through BLD burn.

---

## 4. Genesis Origin BLD

### 4.1 Amount

121 BLD

Symbolism:

11 x 11 = 121

Since Build creation in X1 requires burning 11 BLD, one Genesis Origin allocation can theoretically support 11 new Builds.

### 4.2 Eligibility

Genesis Origin BLD is granted only once when a Build first connects valid xEnchanted Crypto history during the Build Genesis Epoch.

It does not matter whether the Build already existed in X1 before or is created together with this connection.

### 4.3 Restrictions

Genesis Origin BLD is not granted:

- on update
- on relock
- outside Build Genesis Epoch
- more than once per canonical Ethereum/XC identity

### 4.4 Accounting

When Genesis Origin BLD is granted:

origin_bld += 121
available_bld += 121

It must not increase:

earned_bld

---

## 5. Build creation in X1 through BLD burn

A user without XEN/XC history may create an active Build in X1 through BLD burn.

### 5.1 Requirement

burn 11 BLD

### 5.2 Result

The Build is created and becomes active.

The user does not receive fake earned BLD or fake XEN burn history.

### 5.3 No relock

Build creation through BLD burn does not require XNTD lock or relock, because this path does not create the risk:

Core redeem -> XNTD dump

---

## 6. XEN Burn Power

### 6.1 Meaning

XEN Burn Power represents verified global XEN burn participation.

It is not compensation.
It is not a debt.
It is not a guaranteed reward.

It is historical participation power that may be used by X1 mechanics.

### 6.2 Source

Canonical source:

successful XEN.burn(user, amount)

This includes:

- top-level calls to XEN.burn(user, amount)
- internal calls through projects that used the official XEN burn interface

It does not mean arbitrary transfers to the zero address.

### 6.3 Normalization

Use the same denominator:

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount)

### 6.4 Fields

- earned_xbp
- available_xbp

### 6.5 earned_xbp

earned_xbp is the historical XEN Burn Power derived from verified XEN burn calls.

### 6.6 available_xbp

available_xbp is the currently available XEN Burn Power.

The exact mechanics for transfer, use, or burn can be defined separately.

---

## 7. XNTD lock

### 7.1 Purpose

XNTD lock is an activation / commitment requirement for Build records that received earned BLD through Core redeem.

It exists to reduce the simple path:

mint Core L1
-> redeem Core L1
-> receive XNTD
-> get Build
-> sell all XNTD

### 7.2 Required lock amount

The required lock amount is tied to the minimum Core L1 nominal of the current XC epoch.

required_xntd_lock = current epoch Core L1 nominal

### 7.3 XC commitment activation

xc_commitment_active =
  earned_bld > 0
  AND locked_xntd >= required_xntd_lock

### 7.4 Relock

When a new epoch begins, the user may relock under the new requirement.

Relock is allowed only if:

available_bld >= earned_bld

This means the user must preserve or restore the earned BLD amount before reducing or updating XNTD commitment.

---

## 8. X1 Fee Contribution

### 8.1 Meaning

X1 Fee Contribution records how much network fee an address has paid on X1 as transaction fee payer.

It tracks fee payment activity, not necessarily user activity.

### 8.2 Scope

All confirmed X1 transactions where:

fee_payer = user address

### 8.3 Accounting

The metric should include:

- base fee
- priority fee

### 8.4 Storage

On-chain state should store raw smallest X1 units.

Display normalization should happen off-chain or in the UI.

### 8.5 Fields

- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at

### 8.6 Update model

Use indexer-based cumulative checkpoints.

The Build Program should not try to calculate finalized fee inside the same transaction.

Checkpoint example:

address: user
total_fee_paid: 123456789
total_tx_count: 842
counted_until_slot: 10000000

Accept only newer checkpoints:

new_counted_until_slot > previous_counted_until_slot

### 8.7 Known characteristic

This metric tracks fee payment activity, not user activity.

If a sponsored transaction uses a relayer or dApp as fee payer, the X1 Fee Contribution belongs to the relayer or dApp, not necessarily to the user who signed or initiated the action.

### 8.8 Reader interface

Other X1 projects should be able to read:

- total_fee_paid
- tx_count
- counted_until_slot
- last_updated

---

## 9. Source event protection

### 9.1 Core redeem key

redeem_key = hash(
  chain_id,
  core_contract,
  token_id,
  tx_hash,
  log_index
)

### 9.2 XEN burn key

xen_burn_key = hash(
  chain_id,
  xen_contract,
  tx_hash,
  trace_index,
  user,
  amount
)

### 9.3 Fee checkpoint protection

new_counted_until_slot > previous_counted_until_slot

### 9.4 Main rule

one source event -> one accounting action -> one Build

---

## 10. Minimal BuildState v1 fields

- owner
- build_mint / build_id
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

## 11. Short definition

X1 Build is a voluntary NFT-like user object that records independent verified contribution layers: Core redeem contribution, global XEN Burn Power, XNTD commitment, and X1 fee contribution.

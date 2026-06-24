# X1 Build v1 Spec

## 1. Purpose

X1 Build is a voluntary NFT-like user object in X1 that records independent verified contribution layers:

1. Core redeem contribution in xEnchanted Crypto.
2. Global XEN Burn Power.
3. XNTD commitment.
4. X1 network fee contribution.
5. Owner-controlled Build Identity metadata.

The goal is not to merge all values into one universal score.

The goal is to expose separate, readable, verifiable fields that other X1 projects may interpret independently.

## 2. Core principles

### 2.1 Build State stores history, not live balances

Build State stores durable protocol facts.

It must not store a public live spendable BLD balance.

It must not store a public live spendable XBP balance.

Spendable / transferable BLD is a separate token. Build State and Build view do not display, mirror, or cache BLD token balance.

### 2.2 One Build, multiple verified layers

A user should have one canonical Build.

The Build may receive new verified data from different sources over time.

Build is one object with appendable verified layers.

### 2.3 No arbitrary totals

The Build must not accept arbitrary totals such as:

    user has 100 BLD
    user has 500 XBP

It should accept only verified source events, valid state transitions, or trusted checkpoints.

### 2.4 Independent accounting

The main fields must not be mixed:

    BLD history != XEN Burn Power != X1 Fee Contribution != XNTD Lock != Build Identity

Each field has its own source, meaning, and update rules.

## 3. Build State

### 3.1 Minimal BuildState v1 fields

Identity / ownership:

- owner
- build_mint / build_id
- ethereum_identity
- created_at
- updated_at
- version

Build Identity display metadata:

- build_name
- logo_uri
- metadata_updated_at

Historical contribution facts:

- history_bld
- origin_bld
- history_xbp

Stable XNTD commitment facts:

- locked_xntd
- required_xntd_lock
- lock_epoch
- xntd_commitment_accepted

X1 fee checkpoint facts:

- x1_fee_contribution
- x1_tx_count
- x1_fee_counted_until_slot
- last_fee_update_at

### 3.2 Fields intentionally not in BuildState

BuildState v1 does not contain:

- public spendable BLD balance
- public spendable XBP balance
- live wallet balance
- live token escrow balance
- external runtime RPC status
- public UNKNOWN commitment status

These values belong outside BuildState.

## 4. BLD history

### 4.1 Meaning

`history_bld` is the normalized Build unit derived from redeemed Core NFT history.

It represents historical contribution through xEnchanted Crypto.

It is non-decreasing.

It must not decrease when a user sells, spends, burns, transfers, or otherwise uses any future spendable BLD asset.

### 4.2 Source

Only redeemed Core NFT history creates `history_bld`.

Canonical flow:

    Core redeem
      -> read Core.xenBurned
      -> normalize
      -> history_bld

Actions that do not create `history_bld`:

- Forge mint
- Stake redeem
- Enchant
- Market activity
- XNTD lock
- XNTD burn
- Global XEN burn outside XC
- X1 fee contribution
- Build Identity update
- future spendable BLD transfer

### 4.3 Normalization

Display unit:

    1 BLD = 100,000,000 XEN burned through redeemed Core history

Internal storage should support future precision decisions.

## 5. Genesis Origin BLD

### 5.1 Meaning

`origin_bld` is Genesis Origin BLD.

It is not earned BLD.

It is not history.

It exists to record the Genesis Origin tier reached by the Build.

### 5.2 Tier caps

Genesis Origin BLD is tiered by `history_bld`.

Eligible cap:

- history_bld >= 1 -> origin_bld cap = 11
- history_bld >= 11 -> origin_bld cap = 22
- history_bld >= 121 -> origin_bld cap = 55
- history_bld >= 1111 -> origin_bld cap = 121

121 BLD is the maximum Genesis Origin cap, not the default allocation.

### 5.3 Upgrade-by-delta rule

Genesis Origin is not a one-time static claim.

It is an upgrade-to-cap model.

If the current origin tier is lower than the eligible tier, the Build may upgrade `origin_bld` to the eligible cap.

Examples:

    origin_bld = 0,  history_bld = 1     -> origin_bld becomes 11
    origin_bld = 11, history_bld = 11    -> origin_bld becomes 22
    origin_bld = 22, history_bld = 121   -> origin_bld becomes 55
    origin_bld = 55, history_bld = 1111  -> origin_bld becomes 121

The conceptual delta is:

    delta_origin_bld = eligible_origin_bld - current_origin_bld

The stored field is the cap reached:

    origin_bld = eligible_origin_bld

### 5.4 Accounting

Genesis Origin upgrade changes:

    origin_bld
    updated_at

It must not change:

    history_bld
    history_xbp
    locked_xntd
    required_xntd_lock
    lock_epoch
    x1_fee_contribution
    Build Identity

It does not mint or expose a public spendable BLD balance inside BuildState.

## 6. XEN Burn Power

### 6.1 Meaning

XEN Burn Power represents verified global XEN burn participation.

It is not compensation.

It is not a debt.

It is not a guaranteed reward.

It is historical participation power that may be interpreted by X1 mechanics.

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

### 6.4 Field

BuildState stores:

    history_xbp

`history_xbp` is historical and non-decreasing.

BuildState v1 does not store a public spendable XBP balance.

## 7. XNTD lock

### 7.1 Purpose

XNTD lock is an activation / commitment requirement for Build records that received BLD history through Core redeem.

It exists to reduce the simple path:

    mint Core L1
      -> redeem Core L1
      -> receive XNTD
      -> get Build history
      -> sell all XNTD

### 7.2 Required lock amount

The required lock amount is tied to the minimum Core L1 nominal of the relevant XC epoch.

Conceptual rule:

    required_xntd_lock = observed epoch Core L1 minimum nominal

### 7.3 Stored lock facts

BuildState stores stable lock facts:

- locked_xntd
- required_xntd_lock
- lock_epoch
- xntd_commitment_accepted

These are historical/state facts from accepted operations.

### 7.4 Public commitment status

Public Build view derives commitment status from stored facts only.

Allowed public status values:

- COMMITTED
- UNCOMMITTED

Allowed public reasons:

- NO_HISTORY
- NO_COMMITMENT
- COMMITMENT_INSUFFICIENT
- COMMITMENT_ACCEPTED

BuildState does not expose UNKNOWN as public state.

If current live epoch / RPC / external context is unavailable, that is an operation-level validation or infrastructure concern, not public Build state.

### 7.5 Relock boundary

Relock must not read a public BuildState spendable balance.

Future relock rules that require actual BLD availability must check the external BLD asset / ledger / escrow layer at operation time.

BuildState itself records only stable Build facts.

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

## 9. Build Identity

### 9.1 Meaning

Build Identity is owner-controlled display metadata for a Build.

It is separate from protocol accounting.

### 9.2 Fields

- build_name
- logo_uri
- metadata_updated_at

### 9.3 Rules

Build Identity:

- may be empty
- may be updated by the Build owner
- does not require globally unique names
- stores logo as URI, not raw file
- does not change contribution accounting
- does not change lock accounting
- does not change fee contribution
- does not change replay protection

### 9.4 Non-goals

Build Identity is not Build Actor.

Build Identity is not permission to act on behalf of the Build.

Build Actor remains a future active layer.

## 10. Source event protection

### 10.1 Core redeem key

    redeem_key = hash(
      chain_id,
      core_contract,
      token_id,
      tx_hash,
      log_index
    )

### 10.2 XEN burn key

    xen_burn_key = hash(
      chain_id,
      xen_contract,
      tx_hash,
      trace_index,
      user,
      amount
    )

### 10.3 Fee checkpoint protection

    new_counted_until_slot > previous_counted_until_slot

### 10.4 Main rule

    one source event -> one accounting action -> one Build

## 11. Short definition

X1 Build is a voluntary NFT-like user object that records independent verified contribution layers: Core redeem contribution history, global XEN Burn Power history, XNTD commitment facts, X1 fee contribution checkpoints, and owner-controlled Build Identity metadata.

## Gateway full-profile activation boundary

ETH/XC gateway Build activation is full-profile based.

The gateway must evaluate Core redeem history, global `XEN.burn` history, and XNTD lock commitment together as one verified Ethereum/XC contribution profile.

A new Build created through the ETH/XC gateway must not be empty. It requires accepted XNTD lock according to the epoch minimum. For epoch `0`, the required XNTD lock minimum is `100000000`.

A gateway transition must not silently skip Core redeem or `XEN.burn` history scans. A verified zero is a valid scan result, but gateway Build activation requires minimum Core redeem history: existing `history_bld > 0` or at least one validated incoming Core redeem proof. An unchecked source is not allowed.

X1-native Build creation remains separate and may create a clean `UNCOMMITTED` Build shell.

## Gateway preview is display-only

Gateway profile preview is a read-only UX helper.

Preview data is used only to show the participant the currently observed ETH/XC profile before Build creation or activation.

Preview data must not be persisted into Build state, registry state, registrar replay sets, or contribution replay sets.

Preview does not reserve eligibility and does not create any protocol commitment.

Only gateway activation stores verified contribution facts, and activation must validate the submitted full-profile bundle again before mutating state.

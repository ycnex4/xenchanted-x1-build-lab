# Current Design Checkpoint

Status: draft / design phase.

This repository tracks the current X1 Build design before implementation.

## Current decision

Create documentation first. Do not start code until the core spec, BuildState fields, state transitions, registrar model, indexer models, and economic assumptions are reviewed.

## Documents created

### Build

- docs/build/terminology.md
- docs/build/build-v1-spec.md
- docs/build/buildstate-fields.md
- docs/build/state-transitions.md
- docs/build/program-instruction-layout.md
- docs/build/pda-account-layout.md
- docs/build/program-authority-model.md
- docs/build/build-reader-interface.md

### Registrar

- docs/registrar/ethereum-registrar-concept.md
- docs/registrar/message-format.md
- docs/registrar/trust-model-evolution.md

### Indexers

- docs/indexers/x1-fee-contribution.md
- docs/indexers/xen-burn-power-indexing.md
- docs/indexers/mvp-trusted-indexer-schemas.md

### Economics

- docs/economics/bld-origin-and-native-entry.md
- docs/economics/bld-transfer-and-burn-mechanics.md
- docs/economics/xntd-lock-and-relock.md
- docs/economics/bld-tokenization-decision.md
- docs/economics/xntd-lock-proof-model.md
- docs/economics/post-mvp-bld-composability.md

### Checkpoints

- docs/checkpoints/current-design-checkpoint.md
- docs/checkpoints/mvp-implementation-sequence.md
- docs/checkpoints/documentation-consistency-review.md
- docs/checkpoints/implementation-branch-plan.md
- docs/checkpoints/implementation-risk-checklist.md
- docs/checkpoints/final-pre-implementation-checkpoint.md

## Current model summary

X1 Build is a voluntary NFT-like user object in X1 that records independent verified contribution layers.

The main layers are:

- BLD from redeemed Core NFT history
- XEN Burn Power from verified XEN.burn(user, amount) calls
- XNTD commitment through lock / relock
- X1 Fee Contribution from network fees paid by an address as X1 fee payer
- Build creation in X1 through burn 11 BLD

These layers must remain separate.

BLD does not come from XBP.
XBP does not come from BLD.
X1 Fee Contribution does not create BLD or XBP.
XNTD lock does not create BLD or XBP.

## BLD

BLD is the normalized Build unit derived from redeemed Core NFT history.

Display unit:

1 BLD = 100,000,000 XEN burned through redeemed Core history

Fields:

- history_bld
- available_bld
- origin_bld

history_bld is historical and does not decrease when available BLD is sold, spent, burned, or transferred.

## Genesis Origin BLD

Genesis Origin BLD is a one-time tiered allocation.

It is based on history_bld at the first valid xEnchanted Crypto history connection during the Build Genesis Epoch.

Allocation tiers:

- history_bld >= 1     -> origin_bld = 11
- history_bld >= 11    -> origin_bld = 22
- history_bld >= 121   -> origin_bld = 55
- history_bld >= 1111  -> origin_bld = 121

121 BLD is the maximum Genesis Origin cap, not the default allocation.

Genesis Origin BLD is not earned BLD.

When granted:

origin_bld += tiered_origin_bld
available_bld += tiered_origin_bld

It must not increase history_bld.

## Build creation in X1 through BLD burn

A user without XEN/XC history may create an active Build in X1 by burning BLD.

Requirement:

burn 11 BLD

This does not create fake history_bld or fake XEN burn history.

This path does not require XNTD lock or relock.

## XEN Burn Power

XEN Burn Power comes from verified official XEN burn calls.

Canonical source:

successful XEN.burn(user, amount)

Arbitrary transfers to the zero address are not counted.

Display unit:

1 XBP = 100,000,000 XEN burned through XEN.burn(user, amount)

## XNTD lock / relock

XNTD lock is a commitment requirement for Build records that received earned BLD through Core redeem.

Required lock:

required_xntd_lock = current epoch Core L1 nominal

Commitment activation:

xc_commitment_active =
  history_bld > 0
  AND locked_xntd >= required_xntd_lock

Relock is allowed only if:

available_bld >= history_bld

## X1 Fee Contribution

X1 Fee Contribution records cumulative network fees paid by an address as fee payer on X1.

It tracks fee payment activity, not necessarily user activity.

It should include:

- base fee
- priority fee

MVP model:

- trusted indexer
- cumulative checkpoints
- raw smallest X1 units on-chain
- display normalization off-chain / UI
- reader interface for other X1 projects

## Source protection

Main rule:

one source event -> one accounting action -> one Build

Protection keys / maps:

- used_redeem_events[redeem_key]
- used_xen_burn_events[xen_burn_key]
- processed_messages[message_id]
- genesis_origin_claimed[identity]
- canonical_build_by_identity[identity]

## Current next steps

Potential next documents / design areas:

1. First implementation milestone definition.
2. Run documentation consistency checklist.
3. Prepare scaffold branch.
4. Implementation test matrix.
5. Post-MVP integration policy.
6. Start build-mvp-scaffold branch.



















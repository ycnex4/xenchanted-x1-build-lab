# X1 Build v1 Runtime Boundary

## Status

Design boundary.

This document defines the intended production boundary for X1 Build v1 before runtime implementation.

It is not runtime code.

It is not a deployment plan.

It is not a frontend preview document.

It records what must live in X1 runtime state, what must stay off-chain, and which invariants must be preserved when moving from the TypeScript lab model toward production architecture.

## Purpose

X1 Build v1 is a durable public history object.

Its job is to record verified contribution facts from independent layers:

- xEnchanted Crypto Core redeem history;
- global XEN burn participation history;
- XNTD commitment facts;
- X1 fee contribution checkpoints;
- owner-controlled Build Identity metadata.

The runtime must preserve these facts as stable Build state.

The runtime must not become:

- a frontend preview cache;
- a live wallet balance mirror;
- a spendable BLD ledger;
- a spendable XBP ledger;
- an Ethereum log scanner;
- a bridge execution engine;
- a general reputation score calculator.

## Current model being mapped

The TypeScript lab model already separates:

- Build state;
- Build registry;
- proof objects;
- proof submission;
- registrar handlers;
- source event replay protection;
- snapshot persistence;
- CLI inspection.

The runtime boundary maps this model into production responsibilities.

The main question is not "how should the UI show Build?"

The main question is:

Which state transitions must X1 runtime accept, reject, and store?

## Core runtime principles

### 1. Build State stores durable facts

Build State stores facts that should remain meaningful even if:

- RPC providers are down;
- an indexer is behind;
- frontend UI changes;
- wallet balances change;
- future spendable BLD exists elsewhere;
- gateway infrastructure is temporarily unavailable.

### 2. Build State does not store live spendable balances

Build State must not store:

- public spendable BLD balance;
- public spendable XBP balance;
- live XNTD wallet balance;
- live escrow balance;
- live RPC status;
- temporary gateway preview state.

Spendable / transferable BLD, if introduced, belongs to a separate token, ledger, escrow, or asset layer.

### 3. One Build, multiple verified layers

A user should have one canonical Build.

The Build may receive verified data from different sources over time.

Those sources must remain independently readable.

The runtime must not collapse all contribution layers into one arbitrary score.

### 4. No partial gateway import

ETH/XC gateway activation must not apply only one part of the participant profile while silently skipping the rest.

Gateway activation must evaluate the complete Ethereum/XC profile boundary required for that operation.

For a gateway-created Build, an empty shell is not valid.

### 5. X1-native Build is a separate path

X1-native Build creation may create a clean Build shell.

ETH/XC gateway Build activation must import verified Ethereum/XC contribution history.

These are different creation paths and must remain distinct in runtime rules.

## Runtime components

## 1. On-chain / X1 runtime state

The following state belongs in X1 runtime or its production-equivalent persistent state.

### 1.1 Build Account

Canonical Build object.

Required fields:

- owner;
- buildId;
- version;
- createdAt;
- updatedAt;
- ethereumIdentity;
- buildName;
- logoUri;
- metadataUpdatedAt;
- historyBld;
- originBld;
- historyXbp;
- lockedXntd;
- requiredXntdLock;
- lockEpoch;
- xntdCommitmentAccepted;
- x1FeeContribution;
- x1TxCount;
- x1FeeCountedUntilSlot;
- lastFeeUpdateAt.

Runtime rules:

- `historyBld` is non-decreasing.
- `historyXbp` is non-decreasing.
- `originBld` follows upgrade-to-cap rules.
- Build Identity updates do not change accounting.
- XNTD commitment facts are changed only by accepted commitment transitions.
- X1 fee contribution checkpoints must be monotonic by counted slot.
- `updatedAt` changes only on accepted state transitions.

### 1.2 Build Registry / canonical indexes

The runtime needs canonical uniqueness checks.

Required uniqueness:

- one Build per buildId;
- one canonical Build per owner;
- one canonical Build per Ethereum identity when an Ethereum identity is attached.

Possible account/index forms:

- BuildById;
- BuildByOwner;
- BuildByEthereumIdentity.

The exact storage mechanism may depend on X1 runtime capabilities.

The invariant does not depend on the storage mechanism.

### 1.3 Registrar Configuration

The runtime must know which authority is allowed to submit accepted profile checkpoints or proof bundles.

Minimum fields:

- registrarAuthority or guardian set root;
- threshold / quorum policy if multi-signer;
- schemaVersion;
- active flag;
- updatedAt.

Open decision:

- single registrar authority for v1 MVP;
- threshold guardian approval model;
- staged migration from single authority to threshold.

The runtime boundary must support future hardening without changing Build State semantics.

### 1.4 Replay / checkpoint protection state

The TypeScript lab uses replay sets.

A production runtime must avoid unbounded account growth inside one Build account.

Therefore replay protection must be mapped into one or more bounded or separately-addressed structures.

Candidate strategies:

1. Per-source-event marker accounts.
2. Cumulative profile checkpoint accounts.
3. Monotonic source cursor checkpoints.
4. Hybrid model: off-chain event replay plus on-chain cumulative checkpoint replay.

For v1 runtime planning, the preferred production direction is:

- off-chain scanner/indexer verifies raw source events;
- registrar submits a cumulative profile checkpoint;
- runtime stores accepted cumulative facts and monotonic cursors;
- runtime prevents replay of the same checkpoint;
- runtime rejects non-monotonic or regressive updates.

This avoids forcing thousands of individual Ethereum/XC source events into one X1 transaction.

## 2. Off-chain components

The following components must not be implemented as Build State itself.

### 2.1 Ethereum / XC scanner

Responsible for:

- scanning Core redeem history;
- scanning global XEN.burn history;
- detecting XNTD commitment events;
- applying Ethereum finality policy;
- deduplicating source events;
- producing watcher candidates;
- maintaining scanner audit data.

The scanner does not mutate Build State directly.

### 2.2 Proof / checkpoint builder

Responsible for converting scanner output into one of:

- proof objects;
- cumulative profile checkpoints;
- registrar messages;
- activation bundles.

The builder must preserve source identity and replay keys.

### 2.3 Registrar / approval layer

Responsible for deciding which proof/checkpoint bundle is accepted for runtime submission.

In v1, this may be operator-controlled.

In later stages, this may become threshold-guardian controlled or more trust-minimized.

### 2.4 Frontend / API layer

Responsible for:

- displaying Build state;
- displaying wallet state;
- explaining requirements;
- preparing user actions;
- showing pending infrastructure states.

The frontend must not define protocol truth.

Frontend preview data must never be treated as Build State.

## Runtime instructions

This section defines the intended instruction boundary.

Names are conceptual and may be adapted to the final runtime framework.

## 1. create_x1_native_build

Creates a clean X1-native Build shell.

Allowed when:

- owner has no existing Build;
- buildId is unused;
- optional ethereumIdentity is either empty or not already attached elsewhere;
- initial Build Identity metadata satisfies length and URI limits.

Effects:

- creates Build Account;
- creates registry/index entries;
- sets contribution fields to zero;
- sets XNTD commitment as uncommitted;
- stores optional Build Identity metadata.

Must not:

- create historyBld;
- create historyXbp;
- create originBld;
- accept XNTD commitment;
- import Ethereum/XC profile;
- pretend gateway activation happened.

## 2. gateway_activate_build

Creates a Build through verified Ethereum/XC profile import.

Allowed when:

- Build does not already exist for owner/buildId;
- Ethereum identity is not attached to another Build;
- full Ethereum/XC profile boundary is satisfied;
- minimum Core redeem history is present;
- accepted XNTD commitment satisfies required epoch minimum;
- required scans/checkpoints are complete;
- registrar/guardian authorization is valid.

Effects:

- creates Build Account;
- creates registry/index entries;
- applies verified Core redeem history;
- applies verified XEN burn history;
- applies accepted XNTD commitment facts;
- applies Genesis Origin upgrade derived from historyBld;
- records accepted checkpoint/replay protection;
- emits or records activation event if runtime supports it.

Must be atomic:

- if any required part fails, no Build is created;
- no partial history is applied;
- no checkpoint marker is consumed;
- no registry entry is created.

## 3. gateway_update_build

Updates an existing Build with newly verified Ethereum/XC profile facts.

Allowed when:

- Build exists;
- owner matches;
- Ethereum identity matches;
- checkpoint/proof is valid;
- checkpoint/proof is newer than previously accepted source cursor;
- update does not decrease durable history;
- update does not downgrade accepted commitment due to infrastructure failure.

Effects may include:

- increase historyBld;
- increase historyXbp;
- upgrade originBld to a higher eligible cap;
- accept or update XNTD commitment facts according to valid rules;
- record accepted checkpoint/replay protection;
- update updatedAt.

Must not:

- change owner;
- change buildId;
- attach a different Ethereum identity;
- decrease historyBld;
- decrease historyXbp;
- downgrade public commitment because RPC/indexer is unavailable;
- mutate Build Identity unless explicitly part of a separate owner-authorized instruction.

## 4. update_build_identity

Updates owner-controlled display metadata.

Allowed when:

- Build exists;
- signer is Build owner;
- buildName and logoUri satisfy limits;
- URI policy is satisfied.

Effects:

- updates buildName;
- updates logoUri;
- updates metadataUpdatedAt;
- updates updatedAt if metadata changes.

Must not change:

- historyBld;
- originBld;
- historyXbp;
- lockedXntd;
- requiredXntdLock;
- lockEpoch;
- xntdCommitmentAccepted;
- x1FeeContribution;
- replay protection;
- registry identity mapping.

## 5. apply_x1_fee_checkpoint

Applies a verified X1 fee contribution checkpoint.

Allowed when:

- Build exists;
- checkpoint authority is valid;
- countedUntilSlot is greater than previous x1FeeCountedUntilSlot;
- contribution values are non-negative and monotonic.

Effects:

- updates x1FeeContribution;
- updates x1TxCount;
- updates x1FeeCountedUntilSlot;
- updates lastFeeUpdateAt;
- updates updatedAt.

Must not:

- change Ethereum/XC history;
- change XNTD commitment;
- change Build Identity;
- change owner;
- change registry identity mapping.

## Gateway activation model

## Full-profile checkpoint direction

A gateway activation should not require submitting every raw source event to X1 runtime.

The likely production model is cumulative full-profile checkpoint activation.

A profile checkpoint should include:

- buildId;
- owner;
- ethereumIdentity;
- schemaVersion;
- checkpointId;
- source chain id;
- source contract identifiers;
- finality reference;
- Core redeem scan cursor;
- XEN.burn scan cursor;
- XNTD commitment reference;
- cumulative historyBld;
- cumulative historyXbp;
- lockedXntd;
- requiredXntdLock;
- lockEpoch;
- xntdCommitmentAccepted;
- registrar/guardian approval data.

Runtime verifies:

- authority / quorum;
- schemaVersion;
- identity match;
- checkpoint replay;
- monotonic source cursor;
- non-decreasing cumulative facts;
- minimum Core redeem history for gateway-created Build;
- accepted XNTD commitment for gateway-created Build;
- no duplicate owner/buildId/Ethereum identity.

Runtime does not verify every Ethereum log directly in v1 unless a later proof system makes that practical.

## Direct proof bundle vs cumulative checkpoint

The lab supports proof objects.

Runtime may support direct proof bundles only if proof count is bounded and practical.

However, real users may have large histories.

Therefore the runtime boundary should not require raw per-event proof submission as the only production path.

Preferred v1 direction:

- raw source event replay protection happens in scanner/indexer/audit layer;
- runtime receives cumulative checkpoints;
- runtime stores monotonic checkpoint state;
- runtime can be audited against scanner snapshots.

## Atomicity requirement

For gateway activation:

- create Build;
- apply history;
- apply commitment;
- apply origin upgrade;
- write replay/checkpoint marker;
- create registry indexes;

must succeed or fail as one operation.

If X1 transaction limits make this impossible in one instruction, staged activation may be introduced only if pending state is clearly separated from public Build State.

Pending activation state must not be displayed as a completed Build.

## Staged activation fallback

If needed later, staged activation can use a temporary Activation Session.

Rules:

- Activation Session is not Build State.
- It does not create public contribution history.
- It does not create a usable Build.
- It expires or can be cancelled.
- Finalize instruction must validate the full bundle again.
- Only finalize mutates Build State.

This is a fallback, not the preferred simple model.

## Build Identity runtime limits

Build Identity is owner-controlled display metadata.

Runtime should define limits before implementation:

- max buildName length;
- max logoUri length;
- allowed URI schemes;
- whether empty strings normalize to null;
- whether metadata can be changed before any contribution;
- whether metadata update emits an event/log;
- whether metadata update charges only normal network fees.

Initial recommended direction:

- buildName may be null or non-empty string within max length;
- logoUri may be null or URI string within max length;
- empty string normalizes to null;
- owner may update identity regardless of commitment status;
- identity update has no accounting effects.

## Genesis Origin runtime rule

Genesis Origin is upgrade-to-cap based on historyBld.

The runtime must not mint or expose spendable BLD inside Build State.

When historyBld increases, runtime may update originBld to the eligible cap.

Eligible caps:

- historyBld >= 1 -> originBld cap = 11;
- historyBld >= 11 -> originBld cap = 22;
- historyBld >= 121 -> originBld cap = 55;
- historyBld >= 1111 -> originBld cap = 121.

Runtime invariant:

- originBld never decreases;
- originBld never exceeds eligible cap;
- originBld update does not change spendable balances.

## XNTD commitment runtime rule

XNTD commitment is a stable accepted fact.

For gateway-created Build:

- accepted XNTD commitment is mandatory;
- required amount is tied to observed XC epoch minimum;
- lock facts are stored as durable state;
- commitment is not a live RPC-derived status.

For existing Build:

- accepted commitment should not be downgraded by infrastructure failure;
- a future relock/update rule may update lock facts;
- relock availability checks must not read Build.availableBld because that field does not exist.

## X1 fee contribution runtime rule

X1 fee contribution should use cumulative checkpoints.

Runtime must not calculate all historical fee activity inside the same user transaction.

Checkpoint must be monotonic:

- new counted slot > previous counted slot;
- contribution does not decrease;
- transaction count does not decrease.

If a relayer pays fees, the fee contribution belongs to the fee payer, not necessarily the user who initiated the action.

## Non-goals for this stage

This stage does not implement:

- X1 program code;
- deployment scripts;
- production signer keys;
- live watcher service;
- trustless Ethereum proof verification;
- bridge execution;
- spendable BLD token;
- marketplace mechanics;
- frontend UI;
- preview DTO;
- public marketing copy.

## Open questions before implementation

1. Should v1 use single registrar authority or threshold approval?
2. What exact signing scheme should X1 runtime verify?
3. What account/address derivation scheme is available and safest on X1?
4. What are practical account size limits?
5. What are practical instruction/account limits?
6. Should gateway activation use direct checkpoint only, or allow bounded direct proof bundle too?
7. What exact source cursors are needed for Core redeem and XEN.burn scans?
8. How should source checkpoint audit data be published?
9. What max length should Build Identity fields use?
10. What event/log format should runtime emit?
11. Should x1 fee checkpoint authority be the same registrar or a separate authority?
12. What should be immutable at v1 deployment time?
13. What can be rotated, and through what governance/safety process?

## Short rule

Build v1 runtime stores durable verified Build history.

It does not scan Ethereum.

It does not mirror wallet balances.

It does not store preview state.

It does not expose spendable BLD or XBP balances.

It accepts only valid, authorized, monotonic state transitions.

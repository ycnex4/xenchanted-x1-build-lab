# Stage 1.7 X1 Account / Storage Layout Design

## Purpose

Stage 1.7 documents the future X1-side account and storage layout questions for the gateway model before any runtime implementation begins.

This is a design-only milestone.

Stage 1.7 does not implement:

- X1 gateway runtime code
- deployed X1 programs
- deployed Ethereum contracts
- production keys
- production guardian operations
- relayer runtime
- watcher runtime
- frontend flow
- token deployment
- emergency pause implementation
- account allocation scripts
- Stage 2 runtime code

The goal is to define the storage categories, account responsibilities, versioning boundaries, replay-protection layout, guardian references, source coefficient storage, mint-state representation, and atomicity requirements that future X1 runtime code must preserve.

Stage 1 proved deterministic gateway verification and mint authorization in a pure model.

Stage 1.5 mapped the deterministic model to future runtime concerns.

Stage 1.6 defined the guardian set management design boundary.

Stage 1.7 focuses on where future X1 runtime state should live and how it should be separated.

## Baseline from previous stages

Stage 1 Gateway proved the deterministic model for:

- canonical message encoding
- domain separation
- message hash calculation
- canonical event key calculation
- recipient hash calculation
- route validation
- evidence validation
- recipient validation
- amount validation
- guardian signature verification
- guardian quorum verification
- processed burn replay protection
- mint authorization
- mint core state mutation
- positive state-backed end-to-end flow
- negative state-backed rejection matrix

Stage 1.5 added runtime mapping requirements:

- processed burn mark must be atomic with mint
- relayer is untrusted
- guardian quorum is required
- recipient and amount are derived from canonical signed message
- runtime must define account/storage layout before implementation
- CPI / cross-program atomicity must preserve all-or-nothing semantics

Stage 1.6 added guardian-specific design requirements:

- guardian set identity should be explicit
- guardian set versioning should be explicit
- signed messages should bind to guardian set identity or version
- guardian compromise and recovery must be handled as runtime / transport safety
- emergency pause must not become mutable monetary control
- guardian set version must not bypass canonicalEventKey replay protection

Stage 1.7 builds on these conclusions.

## Storage design principles

The future X1 runtime should separate storage by responsibility.

The storage model should make it clear which account answers which question.

Core design principles:

- route rules should not be mixed with recipient balances
- guardian set state should not be mixed with processed burn replay entries
- processed burn replay protection should not be mixed with mutable transport policy
- source coefficients should be explicit and versioned
- pause state should be narrowly scoped
- mint state should be changed only after authorization succeeds
- audit fields should not create alternative mint authority
- account layout should support deterministic verification without off-chain assumptions

A good storage layout should make the runtime easy to reason about:

    What route is accepted?
    Which source chain is accepted?
    Which source token is accepted?
    Which coefficient applies?
    Which guardian set is valid?
    Was this burn already processed?
    What amount should be minted?
    Which recipient receives it?
    Did the mint and replay mark commit atomically?

## Proposed storage categories

Future X1 runtime may need these storage categories:

- gateway config state
- route / source-chain config state
- source coefficient state
- guardian set reference state
- processed burn registry
- processed burn shard / entry accounts
- mint state
- recipient balance or token account state
- pause / emergency state
- versioning / config epoch state
- indexing / audit state

Stage 1.7 does not choose a final X1 account mechanism.

It defines responsibilities that future X1 runtime accounts should preserve.

## Gateway config state

Gateway config state represents the top-level gateway rules.

It may include:

- gateway version
- active route config version
- active guardian set version or reference
- accepted source chain registry reference
- accepted source token registry reference
- source coefficient registry reference
- processed burn registry reference
- mint state reference
- pause state reference
- domain separator or domain config reference
- message schema version
- runtime config version

The gateway config should answer:

- what gateway version is active?
- what message schema is accepted?
- which route config is active?
- which guardian set is active or accepted?
- which source coefficient rules are active?
- which pause rules are active?
- which processed burn registry must be used?

The gateway config should not contain:

- arbitrary recipient balances
- raw guardian signatures
- unbounded processed burn entries
- private keys
- production secrets
- mutable monetary overrides hidden outside protocol rules

Important principle:

    Gateway config chooses which rules are active.
    Gateway config should not allow arbitrary minting outside those rules.

## Route and source-chain config

Route config defines which source events may be accepted.

A route may include:

- source chain id
- source chain name or identifier
- source token identifier
- destination network identifier
- destination mint identifier
- route version
- route status
- required finality policy
- accepted message schema version
- coefficient reference
- guardian set reference or allowed guardian set versions
- pause scope

The route config should answer:

- is this source chain accepted?
- is this source token accepted?
- is this destination accepted?
- which coefficient applies to this route?
- which guardian set may approve this route?
- is the route paused?
- which finality policy applies?
- which schema version applies?

Route config must be explicit because future X1-side model may account for multiple networks with different weights.

Core framing:

    Different source networks may be accepted.
    Different source networks may have different coefficients.
    The coefficient reflects the weight of the source.

Ethereum-side XC is expected to be the primary source.

Sidechains can be additional sources with reduced coefficients.

The runtime storage model must make this hierarchy explicit rather than implicit.

## Source coefficient state

Source coefficient state defines how accepted source events are converted into X1-side result amounts.

A coefficient entry may include:

- source chain id
- source token id
- route version
- coefficient numerator
- coefficient denominator
- coefficient bps
- activation timestamp/block
- expiration timestamp/block
- coefficient version
- status
- audit label

The coefficient model should answer:

- what coefficient applies to this source?
- when did it become active?
- is it still active?
- which route uses it?
- is it part of the signed message context?
- does message validity depend on coefficient version at signing time or submission time?

Important design question:

    Should the coefficient version be part of the signed message?

If coefficient version is not included in the signed message or domain, an old message could be submitted after coefficient changes and be interpreted under different rules.

Safer design direction:

    The signed message must bind to the route version and/or coefficient version that determines the coefficient.

The mint core must use the coefficient version from the signed message, not the currently active config coefficient at submission time.

Coefficient changes apply only to messages signed after activation.

This preserves deterministic interpretation.

Important principle:

    Validators confirm events.
    Protocol rules apply coefficients.

Validators should not choose coefficients per message.

## Guardian set reference state

Stage 1.6 defines guardian set management.

Stage 1.7 maps that design into storage responsibilities.

Gateway / route config may need to store:

- active guardianSetVersion
- accepted guardian set versions
- guardian set account references
- revoked guardian set versions
- guardian set activation time
- guardian set expiration time
- route-specific guardian set binding
- emergency guardian set status
- guardian set hash

Guardian set storage should answer:

- which guardian set is active?
- which guardian set signed this message?
- is that guardian set accepted for this route?
- is that guardian set expired?
- is that guardian set revoked?
- does the signature threshold match the stored guardian set?
- is guardian set rotation in progress?

Important principle:

    Guardian set version helps verify signatures.
    Guardian set version must not permit duplicate minting of the same source burn.

The replay-protection anchor remains canonicalEventKey.

## Processed burn registry

The processed burn registry prevents replay.

It answers one core question:

    Has this canonicalEventKey already been used?

The registry may be represented as:

- a single registry account
- sharded registry accounts
- per-event entry accounts
- PDA-like deterministic event accounts
- source-chain-specific registries
- route-specific registries

The correct model depends on X1 storage constraints.

The registry must support:

- deterministic lookup by canonicalEventKey
- atomic mark with mint
- rejection of duplicate canonicalEventKey
- no duplicate processed entry for the same event
- auditability of processed events
- scalability under many source events

The registry should not allow:

- deleting processed entries to enable replay
- marking processed without successful mint
- minting without marking processed
- changing canonicalEventKey identity after processing
- bypassing replay protection by changing guardianSetVersion
- bypassing replay protection by changing route version

Important principle:

    The same source burn event must not be mintable twice.

This remains true across guardian rotations, coefficient changes, route changes, and runtime upgrades.

The processed burn registry is global across all routes.

A canonicalEventKey processed under any route must not be processable under any other route, guardian set, coefficient version, or runtime upgrade.

## Processed burn sharding

A single unbounded processed burn registry may not be practical.

Possible sharding strategies:

### By canonicalEventKey hash prefix

Processed burn entries are distributed by hash prefix.

Advantages:

- deterministic
- even distribution
- avoids route-specific replay bypass

Disadvantages:

- requires shard derivation logic
- may complicate indexing

### By source chain

Each source chain has its own processed burn registry.

Advantages:

- easy to reason about per-source activity
- supports source-specific scaling

Disadvantages:

- canonicalEventKey must include source chain strongly
- route changes must not allow replay across source registries

### By route

Each route has its own processed burn registry.

Advantages:

- aligns with route config
- simpler route-specific accounting

Disadvantages:

- dangerous if the same source event can be interpreted under multiple routes
- route migration must not enable replay

### Per-event deterministic account

Each canonicalEventKey maps to its own processed entry account.

Advantages:

- strong replay identity
- simple lookup
- easy atomic creation if runtime supports it

Disadvantages:

- many accounts
- storage cost
- rent/account management
- indexing load

Preferred design direction:

    Processed burn identity should be derived from canonicalEventKey, not from mutable route or guardian config alone.

This prevents replay through config changes.

## Processed burn entry fields

A processed burn entry may store:

- canonicalEventKey
- source chain id
- source token id
- source tx hash or event reference
- source log index or event nonce
- recipient hash
- X1 recipient
- burned amount
- minted amount
- route version
- coefficient version
- guardianSetVersion used for verification
- processed timestamp/block
- mint state reference
- status

Minimal storage may only need canonicalEventKey.

Richer storage improves auditability.

Open design question:

    What is the minimum storage needed for safety, and what extra storage is useful for auditability?

Important principle:

    Extra audit fields must not become alternative authorization rules.

The canonicalEventKey remains the replay anchor.

## Mint state

Mint state represents the X1-side result of accepted source events.

It may include:

- total minted amount
- per-recipient balances
- token mint reference
- mint authority reference
- mint program reference
- supply cap if any
- per-route minted totals
- per-source-chain minted totals
- per-coefficient-version minted totals
- accounting totals for audit

Stage 1 model uses simple balance mutation:

    recipient balance += xxxlMintAmount
    totalMinted += xxxlMintAmount

Future X1 runtime may represent mint state as:

- native X1 token mint
- custom token program
- account-based ledger
- token account balance
- program-controlled balance registry

Stage 1.7 does not choose the token representation.

It records that mint state mutation must happen only after authorization succeeds.

## Recipient balance / token account state

Recipient state depends on the chosen X1 token model.

Possible models:

- direct balance mapping in gateway-owned storage
- token accounts controlled by a token program
- mint-to-recipient through CPI
- claimable balance account
- receipt entry later converted into token balance

Questions to resolve:

- does the gateway mint directly to recipient?
- does the recipient need a pre-existing token account?
- can the gateway create a recipient account?
- who pays storage/account creation cost?
- what happens if recipient account creation fails?
- can mint and processed mark remain atomic if recipient account creation is required?
- does the runtime support mint-to or claim-based flow more safely?

Atomicity requirement:

    If recipient balance cannot be updated, processed burn must not be marked.

If processed burn is marked, the recipient result must be created.

## Pause / emergency state

Pause state may be needed for runtime safety.

Pause state may include:

- global pause flag
- route-specific pause flag
- source-chain-specific pause flag
- guardian-set-specific pause flag
- message-schema-specific pause flag
- activation timestamp/block
- pause reason code
- pause authority reference
- unpause rules
- expiration time, if pause is time-bounded

Pause should be scoped narrowly.

Possible pause scopes:

- pause all gateway submissions
- pause one source chain
- pause one route
- pause one guardian set version
- pause one message schema version
- pause one coefficient version

Pause should not allow:

- arbitrary mint
- arbitrary burn
- arbitrary balance change
- processed burn deletion
- replay override
- hidden coefficient changes
- changing monetary rules without explicit versioning

Pause prevents new mints, but must never modify processed burn registry entries, recipient balances, or totalMinted.

Pause does not undo past mints.

Pause does not enable replay of previously processed events.

Core framing:

    Pause protects runtime safety.
    Pause does not own monetary rules.

## Versioning and config binding

Versioning is central to safe runtime storage.

Potential versions:

- gateway version
- message schema version
- route version
- coefficient version
- guardianSetVersion
- finality policy version
- pause epoch
- runtime config version

Design questions:

- which versions are included in the signed message?
- which versions are derived from gateway config at submission time?
- which versions are stored in processed burn entries?
- which version changes invalidate pending messages?
- which version changes only affect new messages?
- can multiple versions be accepted during transition?
- how are deprecated versions rejected?

Safer design direction:

    Any version that changes message interpretation should be bound to the signed message or domain.

This includes at least:

- route/config version
- guardianSetVersion
- coefficient version if it affects minted amount
- message schema version

## Message verification context

Runtime verification needs a complete context.

Verification context may include:

- gateway config
- route config
- source coefficient entry
- guardian set account
- processed burn registry or shard
- pause state
- recipient account or mint target
- message schema version
- domain separator
- finality policy

The verifier should not rely on hidden off-chain context.

The runtime must be able to reconstruct:

- expected domain
- expected route
- expected coefficient
- expected guardian set
- expected recipient hash
- expected mint amount
- expected canonicalEventKey
- expected replay status

Important principle:

    Same input message + same runtime config version should produce same verification result.

If runtime config changes, message validity rules must be explicit.

## Account-write atomicity

Stage 1.5 defined the atomicity rule:

    A canonicalEventKey must be marked processed if and only if the corresponding mint succeeds.

Stage 1.7 maps this into account writes.

The future runtime instruction should avoid split-phase execution:

- check processed in one transaction
- mark processed in another transaction
- mint in a third transaction

Preferred flow:

    load gateway config
    load route config
    load coefficient entry
    load guardian set
    load pause state
    derive processed burn entry
    verify message
    verify signatures
    check replay
    compute mint amount
    mark processed
    mint / credit recipient
    commit all changes atomically

Invalid states:

- processed entry created but mint failed
- mint succeeded but processed entry missing
- recipient balance changed but totalMinted not updated
- totalMinted updated but recipient balance missing
- replay mark created under one route and mint under another
- CPI mint succeeded but processed mark failed
- processed mark succeeded but CPI mint failed
- processed burn entry created for canonicalEventKey A, but mint credited to the recipient intended for canonicalEventKey B

## CPI / cross-program implications

If minting uses a separate token program, future runtime may require CPI-like calls.

CPI design must preserve all-or-nothing behavior.

Questions:

- does the gateway instruction call token mint program?
- does token mint happen before or after processed mark?
- can the runtime roll back all account writes if CPI fails?
- can the token program reject recipient account state?
- can mint authority be constrained to gateway program only?
- can external program behavior create partial state?
- how are token program upgrades handled, if any?

Required invariant:

    Processed mark and mint result must commit together or fail together.

If this cannot be guaranteed with direct minting, a claim-based model may be safer.

## Claim-based alternative

A claim-based model separates accepted evidence from token receipt.

Flow:

    verify message
    check replay
    mark processed
    create claim account
    user later claims/mints result

Advantages:

- avoids some recipient account creation failures
- can simplify relayer submission
- user can handle destination token account setup
- may reduce CPI risk during gateway submission

Disadvantages:

- processed mark exists before final user token receipt
- claim account becomes new state to protect
- claim must be non-transferable or carefully designed
- user experience is more complex
- claim redemption must be replay-safe

If claim-based flow is used, Stage 1.5 atomicity must be reframed:

    processed burn marked if and only if claim result is created
    claim result redeemable exactly once
    final token mint protected by claim replay rules

Stage 1.7 does not choose direct mint vs claim.

It records the tradeoff.

## Indexing and audit state

Indexing / audit state can help users and explorers understand gateway activity.

Possible audit totals:

- total minted
- total minted by source chain
- total minted by route
- total minted by coefficient version
- total processed burns
- total processed by guardianSetVersion
- total rejected attempts, if stored
- total paused intervals, if stored

Audit fields should be derived or append-only where possible.

Audit fields should not be required for authorization unless explicitly part of the protocol rule.

Important principle:

    Audit state explains what happened.
    Authorization state decides what may happen.

Mixing the two can create unnecessary attack surface.

## Out-of-scope items

Stage 1.7 does not implement:

- actual X1 accounts
- account allocation
- account rent/storage pricing
- X1 instruction code
- token program integration
- claim program
- guardian set runtime
- pause runtime
- relayer
- watcher
- frontend
- production deployment
- production key management

Stage 1.7 also does not modify Stage 1 generated vectors.

## Current conclusion

Stage 1.7 defines the storage responsibilities that should be resolved before Stage 2 runtime implementation.

The most important storage decisions are:

- gateway config layout
- route/source-chain config layout
- source coefficient storage
- guardian set references
- processed burn registry layout
- processed burn sharding model
- processed burn entry fields
- mint state representation
- recipient balance / token account model
- pause state layout
- versioning and config binding
- message verification context
- account-write atomicity
- CPI / cross-program implications
- direct mint vs claim-based flow
- indexing and audit state

Stage 2 runtime implementation should not begin until these account/storage responsibilities are reviewed and stable enough to avoid hardcoding unstable assumptions.

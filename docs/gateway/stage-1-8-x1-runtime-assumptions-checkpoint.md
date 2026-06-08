# Stage 1.8 X1 Runtime Assumptions Checkpoint

## Purpose

Stage 1.8 documents the runtime assumptions that must be clarified before Stage 2 gateway runtime implementation begins.

This is a design-only checkpoint.

Stage 1.8 does not implement:

- X1 gateway runtime code
- deployed X1 programs
- deployed Ethereum contracts
- production keys
- production guardian operations
- relayer runtime
- watcher runtime
- frontend flow
- token deployment
- direct mint flow
- claim-based flow
- pause runtime
- account allocation scripts

The goal is to list the X1 execution guarantees, account behavior, token program assumptions, upgrade authority assumptions, CPI / cross-program assumptions, rent / storage assumptions, and direct mint vs claim-based decision criteria that must be reviewed before implementation.

Stage 1.7 defined account and storage responsibilities.

Stage 1.8 defines what must be known about the runtime environment before those storage responsibilities can safely become code.

## Baseline from previous stages

Stage 1 proved deterministic gateway verification and mint authorization in a pure model.

Stage 1.5 mapped the deterministic model to future runtime concerns.

Stage 1.6 defined guardian set management design.

Stage 1.7 defined the future X1-side account / storage layout responsibilities.

Stage 1.7 review refinements added:

- processed burn registry is global across all routes
- canonicalEventKey processed under any route must not be processable under any other route
- signed message must bind to route version and/or coefficient version
- mint core must use the coefficient version from the signed message
- coefficient changes apply only to messages signed after activation
- pause prevents new mints but must not modify processed registry entries, recipient balances, or totalMinted
- pause must not enable replay
- additional invalid atomicity state for canonicalEventKey / recipient mismatch

Stage 1.8 starts from these constraints.

## Core question

The core Stage 1.8 question is:

    What must be true about X1 runtime behavior before Stage 2 can safely implement the gateway?

The answer is not one assumption.

It is a set of assumptions across:

- transaction atomicity
- CPI / cross-program execution
- token mint behavior
- account creation
- rent / storage cost
- upgrade authority
- pause authority
- replay protection persistence
- account derivation
- finality expectations
- indexing and observability
- failure behavior

Stage 2 should not begin until these assumptions are documented clearly enough to avoid hidden runtime-dependent decisions.

## Runtime assumption categories

Future Stage 2 planning should confirm assumptions in these categories:

- transaction atomicity
- CPI atomicity
- account write rollback
- token program interface
- token mint authority
- recipient token account behavior
- account creation flow
- rent / storage model
- processed burn registry persistence
- deterministic account derivation
- program upgradeability
- pause authority
- guardian set account behavior
- route / coefficient version binding
- finality and source event confirmation
- event indexing / auditability
- error and failure semantics

## Transaction atomicity

The gateway requires all-or-nothing behavior.

Required invariant:

    Processed mark and mint or claim result must commit together or fail together.

Stage 2 must confirm:

- whether one transaction can update all required gateway accounts atomically
- whether all account writes roll back if any instruction step fails
- whether CPI failures roll back prior writes in the same transaction
- whether token mint failures roll back processed burn writes
- whether processed burn account creation can be atomic with mint or claim creation
- whether totalMinted and recipient balance can be updated atomically
- whether account creation failures roll back all gateway writes

If X1 transaction atomicity is strong, direct mint may be viable.

If atomicity is weak or unclear, claim-based flow may be safer.

## CPI / cross-program assumptions

If gateway minting uses a token program, the gateway may need CPI-like cross-program calls.

Stage 2 must confirm:

- whether CPI calls are atomic with caller state changes
- whether CPI failures revert all caller account writes
- whether CPI success can be followed by caller failure
- whether caller failure after CPI success rolls back token mint state
- whether the token program can reject recipient accounts
- whether token mint authority can be constrained to the gateway program
- whether token program upgrades can change mint semantics
- whether CPI depth or compute limits affect gateway flow
- whether multiple CPI calls can be safely composed in one transaction

Required invariant:

    CPI must not create partial mint state if processed burn marking fails.

If this cannot be guaranteed, Stage 2 should not implement direct mint without a claim-based fallback.

## Account write rollback

Stage 2 must confirm what happens when a runtime instruction fails after some account writes.

Questions:

- are all writes rolled back on failure?
- are created accounts removed on failure?
- are rent or fees consumed on failure?
- can a partially initialized account remain after failure?
- can a processed burn entry be created but not fully initialized?
- can a token mint occur while gateway state rolls back?
- can totalMinted update while recipient mint fails?

Invalid states must remain impossible:

- processed entry created but mint failed
- mint succeeded but processed entry missing
- recipient balance changed but totalMinted not updated
- totalMinted updated but recipient balance missing
- CPI mint succeeded but processed mark failed
- processed mark succeeded but CPI mint failed
- processed burn entry created for canonicalEventKey A, but mint credited to the recipient intended for canonicalEventKey B

## Token program interface

Stage 2 must identify the token interface before choosing direct mint or claim-based flow.

Questions:

- what token standard will represent the X1-side result?
- does the token program support mint-to-recipient?
- does the token program require a pre-existing recipient token account?
- can the gateway create a token account for the recipient?
- can the recipient create the token account later?
- does mint require signer authority, program authority, PDA-like authority, or another mechanism?
- can mint authority be immutable or program-bound?
- can token decimals and supply accounting be fixed at deployment?
- can token mint state be frozen or paused?
- can token program upgrades affect existing mint state?

Stage 1.8 does not choose a token standard.

It records that token interface assumptions are prerequisite for Stage 2.

## Mint authority assumptions

The gateway must not become arbitrary mint authority.

The mint authority model must preserve protocol rules.

Questions:

- who or what has mint authority?
- can mint authority be held by a program-derived authority?
- can mint authority be revoked or permanently bound?
- can an upgrade authority change mint logic?
- can any admin mint outside gateway verification?
- can pause authority mint?
- can guardian authority mint?
- can route config authority mint?
- can token program authority mint?

Required principle:

    Only successful protocol verification may create new X1-side mint result.

Guardian quorum authorizes evidence acceptance.

Guardian quorum does not own monetary rules.

Pause authority protects runtime safety.

Pause authority does not own monetary rules.

## Recipient token account assumptions

Recipient account behavior affects direct mint vs claim-based design.

Questions:

- must a recipient token account exist before mint?
- who creates it?
- who pays storage or rent?
- what happens if it does not exist?
- can the gateway create it during the same transaction?
- can relayer create it?
- can user create it later?
- can creation fail due to insufficient balance?
- can account creation failure block processed burn marking?
- can malicious recipient account state break mint?
- can recipient account ownership be verified deterministically?

Direct mint is simpler only if recipient token account behavior is predictable.

If recipient account creation is uncertain, claim-based flow may be safer.

## Account creation and rent model

Stage 2 must understand X1 account creation and rent/storage costs.

Questions:

- are accounts rent-exempt?
- is rent paid once or continuously?
- who pays for processed burn entry accounts?
- who pays for recipient token accounts?
- who pays for claim accounts?
- can rent be reclaimed?
- can rent reclamation delete important replay-protection state?
- can processed burn entries ever be closed?
- can claim accounts be closed after redemption?
- can storage pressure create denial-of-service risks?
- can attacker submit many invalid messages to force account allocation attempts?

Important principle:

    Replay-protection state must not disappear in a way that enables replay.

Processed burn entries should be permanent, or their replay-protection effect must remain permanent.

## Processed burn registry persistence

Stage 1.7 requires a global processed burn registry across all routes.

Stage 2 must confirm how this registry persists.

Questions:

- is processed burn registry stored in one account, shards, or per-event accounts?
- can processed entries be deleted?
- can processed entries be reinitialized?
- can program upgrade reset the registry?
- can rent cleanup remove processed entries?
- can migration preserve processed state?
- can route migration preserve global replay protection?
- can guardian set rotation preserve global replay protection?
- can coefficient changes preserve global replay protection?

Required principle:

    A canonicalEventKey processed once must remain unprocessable forever.

This must hold across route changes, coefficient changes, guardian set changes, pause/unpause, runtime upgrades, and migrations.

## Deterministic account derivation

Stage 2 should use deterministic account derivation where possible.

Questions:

- can processed burn entries be derived from canonicalEventKey?
- can route config accounts be derived from route id?
- can coefficient entries be derived from source chain and version?
- can guardian set accounts be derived from guardianSetVersion?
- can claim accounts be derived from canonicalEventKey?
- can recipient claim accounts be derived without ambiguity?
- can malicious users create conflicting accounts before the gateway?
- can account derivation include domain separation?
- can account derivation include gateway program id?

Important principle:

    Account identity should follow protocol identity.

For processed burn entries, protocol identity is canonicalEventKey.

For guardian set, protocol identity is guardianSetVersion or guardianSetHash.

For source coefficient, protocol identity is routeVersion and/or coefficientVersion.

## Program upgradeability

Stage 2 must decide the upgrade authority model.

Open questions:

- should the gateway program be immutable?
- should upgrade authority be burned?
- should upgrade authority be timelocked?
- should upgrade authority be controlled by governance, multisig, or guardians?
- can upgrade authority reinitialize processed burn registry?
- can upgrade authority change mint rules?
- can upgrade authority change coefficient interpretation?
- can upgrade authority bypass replay protection?
- can upgrade authority change pause behavior?
- can upgrade authority migrate state safely?

First-principles direction:

    Core monetary rules should not depend on hidden mutable control.

If program upgradeability exists, it must not silently break replay protection or mint rules.

Possible safe directions:

- burned upgrade authority after deployment
- staged deployment with review period before authority burn
- timelocked upgrade path for early gateway layer only
- separate immutable mint core from managed transport layer
- explicit migration rules that cannot reprocess canonicalEventKey entries

Stage 1.8 does not choose final upgrade model.

It records that upgradeability is a blocker assumption for Stage 2 design.

## Pause authority assumptions

Pause protects runtime safety.

Pause must not own monetary rules.

Questions:

- who can pause?
- who can unpause?
- can pause be global?
- can pause be route-specific?
- can pause be source-chain-specific?
- can pause be guardian-set-specific?
- can pause expire automatically?
- can unpause require timelock?
- can pause authority be separate from guardian quorum?
- can pause authority be separate from upgrade authority?
- can pause authority delete or edit processed entries?
- can pause authority change balances?
- can pause authority change totalMinted?
- can pause authority change coefficients?

Required principle:

    Pause prevents new mints.
    Pause does not undo past mints.
    Pause does not enable replay.
    Pause does not modify processed burn registry entries, recipient balances, or totalMinted.

## Guardian set runtime assumptions

Stage 1.6 defined guardian set design.

Stage 2 must confirm runtime storage and verification assumptions.

Questions:

- how is guardian set account stored?
- how is guardianSetVersion represented?
- can multiple guardian set versions be accepted during transition?
- can old guardian set versions expire?
- can old signed messages remain valid after rotation?
- what happens to pending messages during rotation?
- can compromised guardian set be revoked?
- does revocation affect already processed entries?
- can guardian set account be upgraded or rewritten?
- can guardian set changes affect canonicalEventKey replay protection?

Required principle:

    Guardian set changes must not enable replay.

Guardian set version verifies signatures.

canonicalEventKey prevents duplicate processing.

## Route and coefficient runtime assumptions

Stage 1.7 refined coefficient version binding.

Stage 2 must confirm:

- whether signed message includes routeVersion
- whether signed message includes coefficientVersion
- whether routeVersion determines coefficientVersion
- whether coefficientVersion determines mint amount
- whether coefficient changes invalidate pending messages
- whether coefficient changes apply only after activation
- whether old coefficient versions remain accepted for previously signed messages
- whether route config can be paused without changing coefficient
- whether route migration preserves global replay protection

Required principle:

    Mint amount must be derived from the route or coefficient version bound to the signed message.

The mint core must not use a different coefficient because config changed between signing and submission.

## Source chain identity assumptions

Multi-source accounting requires stable source identity.

Questions:

- how is sourceChainId represented?
- is sourceChainId the EVM chain id?
- is sourceChainId a canonical internal id?
- what happens if a source chain hard-forks?
- what happens if source chain id changes?
- what happens if a sidechain reconfigures consensus?
- does canonicalEventKey use sourceChainId at event time?
- can old processed entries remain valid after source reconfiguration?

Required principle:

    sourceChainId in canonicalEventKey should use the canonical chain identifier at the time of the burn event.

Future chain reconfiguration should not invalidate already processed burn entries.

Future chain reconfiguration should not allow already processed burn entries to be processed again.

## Source coefficient criteria

Stage 1.7 defines Ethereum-side XC as primary source and sidechains as additional reduced-coefficient sources.

Stage 2 should define criteria for coefficient assignment.

Possible criteria:

- validator set security
- economic finality
- bridge risk
- source chain maturity
- event availability
- reorg risk
- data quality
- watcher reliability
- community value
- protocol alignment
- historical participation value

Important principle:

    Coefficient reflects the weight of the source.

Ethereum-side XC is primary.

Sidechains are additional sources with reduced coefficients.

Stage 1.8 does not set final coefficient values.

It records that coefficient criteria should be explicit before production runtime.

## Finality assumptions

Source event finality must be defined before runtime.

Questions:

- how many confirmations are required for Ethereum-side events?
- how many confirmations are required for sidechain events?
- does each route have a separate finality policy?
- can finality policy change?
- is finality policy versioned?
- is finality policy included in signed message context?
- what happens if a reorg happens after guardian signing?
- what happens if a reorg happens after X1 mint?
- can guardian evidence be revoked after mint?
- can processed burn entry be reversed?

Required principle:

    X1 mint should be based on finalized source evidence according to route-specific finality policy.

Past processed entries should not be deleted as normal reorg handling.

If finality assumptions are weak, source coefficient should reflect that risk or source should not be accepted.

## Direct mint decision criteria

Direct mint is preferred if:

- transaction atomicity is strong
- CPI atomicity is strong
- token program interface is stable
- recipient token account can be created or verified safely
- account creation failure rolls back all gateway writes
- mint authority can be bound to gateway rules
- processed burn mark and token mint can commit atomically
- user experience benefit is significant
- runtime upgradeability cannot break replay protection

Direct mint should not be chosen if:

- CPI can create partial state
- token mint can succeed while processed mark fails
- processed mark can succeed while token mint fails
- recipient account creation is unreliable
- token program can be upgraded to change mint semantics unexpectedly
- mint authority can be used outside gateway verification
- replay registry persistence is uncertain

## Claim-based decision criteria

Claim-based flow is preferred if:

- recipient token account creation is uncertain
- CPI atomicity is uncertain
- relayer should not pay recipient account setup costs
- user should control final token receipt
- gateway should only create a verified claim result
- token mint should happen in a separate user-controlled transaction
- direct mint creates too many failure modes

Claim-based flow should not be chosen blindly.

It introduces new state:

- claim accounts
- claim replay protection
- claim redemption status
- claim recipient binding
- claim expiration policy, if any
- claim storage/rent responsibility

Required principle:

    If claim-based flow is used, claim creation must be atomic with processed burn marking.

Claim redemption must be possible exactly once.

Claim redemption must not allow recipient substitution unless explicitly designed.

## Fallback decision rule

If X1 runtime guarantees are not clear enough, Stage 2 should not implement production direct mint.

Safe fallback direction:

    Prefer claim-based design when direct mint atomicity depends on unverified CPI or account-creation assumptions.

However, if X1 runtime confirms strong all-or-nothing CPI and reliable recipient account handling, direct mint remains the simpler and cleaner UX.

Stage 2 should document the chosen assumption set before implementation.

## Minimum questions before Stage 2

Before Stage 2 begins, these questions should have explicit answers:

1. Are transaction writes fully atomic?

2. Are CPI calls fully atomic with caller state?

3. Can token mint succeed while gateway state fails?

4. Can gateway state succeed while token mint fails?

5. Does the token program require pre-existing recipient accounts?

6. Who pays for recipient token account creation?

7. Who pays for processed burn or claim storage?

8. Can processed burn entries ever be deleted or closed?

9. Can upgrade authority change gateway mint rules?

10. Can upgrade authority reinitialize processed burn registry?

11. Will upgrade authority be burned, timelocked, or retained?

12. Who can pause?

13. What does pause stop?

14. Can pause modify any historical state?

15. Is canonicalEventKey global across all routes in runtime implementation?

16. Is routeVersion and/or coefficientVersion included in signed message?

17. Does mint core use coefficient version from signed message?

18. What happens to pending messages after guardian rotation?

19. What source finality policy applies per route?

20. What is the source coefficient criteria for each accepted source?

21. How is sourceChainId represented in canonicalEventKey?

22. What happens after source chain reconfiguration?

23. What indexing is needed for auditability?

24. What events/logs are emitted for successful processing?

25. What errors are emitted for rejection?

## Out-of-scope items

Stage 1.8 does not implement:

- runtime instruction handlers
- token mint
- claim accounts
- direct mint flow
- relayer
- watcher
- production guardian service
- production signer management
- source coefficient values
- finality policy values
- pause authority keys
- upgrade authority keys
- deployment scripts
- frontend flows

Stage 1.8 also does not modify Stage 1 vectors.

## Current conclusion

Stage 1.8 defines the runtime assumptions that must be reviewed before Stage 2 gateway runtime implementation.

The highest-priority assumptions are:

- transaction atomicity
- CPI atomicity
- token program interface
- recipient token account behavior
- account creation and rent model
- processed burn registry persistence
- program upgradeability
- pause authority boundaries
- route and coefficient version binding
- source chain identity
- finality policy
- direct mint vs claim-based decision criteria

Stage 2 should not begin until these assumptions are answered or explicitly marked as implementation blockers.

If runtime assumptions are strong, direct mint may be the cleanest flow.

If runtime assumptions are uncertain, claim-based flow is the safer fallback.

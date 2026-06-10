# Stage 1.10 Theo Review Refinements

This document records Theo's review of the Stage 1.10 X1 Program Instruction and PDA Derivation Design.

Stage 1.10 was reviewed after EV-01 and EV-02 were confirmed by X1 testnet runtime evidence.

Theo's overall conclusion:

- Stage 1.10 is the strongest pre-implementation gateway design document so far.
- The document is concrete enough to move toward runtime implementation after a small set of mandatory decisions.
- The remaining blocker is not atomic rollback anymore.
- The remaining blocker is program design finalization, especially rent/payment and authority decisions.

## Confirmed strengths

Theo confirmed these parts of Stage 1.10 as strong:

1. Instruction set

The six candidate instructions are appropriate:

- initialize_gateway_config
- set_guardian_set
- submit_mint_approval
- claim_xxxl
- emergency_pause
- emergency_unpause

No unnecessary mutable configuration instructions were added.

In particular, the absence of set_route_config, set_coefficient, and set_mint_authority is correct for an immutable Stage 1-style model.

2. PDA seeds

The PDA seeds are concrete and domain-specific.

The processed burn seed is especially important:

    processed_burn = [b"processed", canonical_event_key]

This is the correct replay-protection anchor.

The mint authority seed is also concrete:

    mint_authority = [b"mint_authority"]

3. Failure atomicity matrix

The failure atomicity matrix is one of the strongest parts of the document.

Each failure case has a concrete required result.

There is no ambiguous depends-on-context language.

4. Guardian verification direction

Theo confirmed that Option A is the cleaner first design:

- guardian signatures are verified inside submit_mint_approval
- replay protection and mint or claim creation happen in the same atomic flow

Option B, where approvals are stored first and consumed later, creates extra state and additional race, cleanup, and partial-approval complexity.

## Required refinements before Stage 2 runtime implementation

### 1. GuardianSet account size must be fixed at creation time

The Stage 1.10 design used GuardianSet with a dynamic Vec-like guardian list.

Theo noted that on SVM-style accounts, account size must be fixed at creation time.

Refinement:

- GuardianSet account size is fixed when the GuardianSet PDA is created.
- The size is derived from guardian_count.
- The account does not resize later.
- If a future guardian set has a different number of guardians, a new GuardianSet PDA is created with an incremented guardian_set_version.
- Old GuardianSet PDAs remain historical records unless a later cleanup policy is explicitly defined.

Implementation implication:

Runtime code must compute deterministic account space at GuardianSet creation time.

### 2. Pause scope must be explicit

The original GatewayConfig only had:

    paused: bool

Theo noted that pause behavior must not accidentally block recovery actions.

Refinement:

- pause affects submit_mint_approval.
- pause affects claim_xxxl if claim-based flow is enabled.
- pause does not block emergency_unpause.
- pause should not block guardian set recovery or rotation if guardian rotation is supported.
- pause must not modify economics, route weights, mint amounts, or canonical event keys.

Possible future representation:

    pause_scope: u8

Candidate values:

- 0 = none
- 1 = mint_and_claim_only
- 2 = all_non_recovery_actions

For the first implementation, the preferred interpretation is mint_and_claim_only.

### 3. Claim cleanup must be explicit

ProcessedBurnEntry accounts should remain permanent because they are replay protection.

ClaimEntry accounts are different.

If claim-based flow is used, claim accounts may lock rent after claim completion.

Refinement:

- ProcessedBurnEntry is permanent by design.
- ClaimEntry cleanup is an open design decision unless the claim-based path is selected.
- A future close_claim instruction may be added as post-MVP cleanup.
- If claim-based flow is chosen for first implementation, claim rent recovery must be decided before code.

For direct mint first implementation, claim cleanup can remain a fallback-path decision.

### 4. Rent payer model must be decided before code

Theo identified rent payment as the main remaining operational blocker.

Every account creation path must identify a payer.

Candidate rent payer decisions:

- GatewayConfig: deployer / initializer
- GuardianSet: deployer or guardian operator
- RouteConfig: deployer / initializer
- ProcessedBurnEntry: relayer payer account
- ClaimEntry: relayer payer account, unless claim-based UX later assigns this to the claimant
- MintAuthority PDA: no separate data account unless the implementation creates one

Decision for first implementation:

- submit_mint_approval includes a mutable payer account.
- The relayer pays rent for ProcessedBurnEntry creation.
- If direct mint is used, there is no ClaimEntry in the first path.
- The relayer's operational cost must include one permanent ProcessedBurnEntry rent payment per processed burn.

Reason:

The gateway program should not need to be prefunded for per-burn account creation during the first implementation.

## Mandatory decisions before the first X1 gateway implementation branch

Theo identified four blocker-level decisions that must be made before runtime code begins.

### Decision 1: first implementation path

Decision:

Direct mint is the first implementation path.

Rationale:

- EV-01 and EV-02 confirmed X1 rollback behavior.
- Direct mint is closest to the Stage 1 deterministic burn-to-mint model.
- It avoids claim lifecycle and cleanup complexity.
- Claim-based flow remains a fallback if compute budget, transaction size, account setup, or token CPI limitations make direct mint unsafe.

### Decision 2: guardian verification boundary

Decision:

Guardian verification happens inside submit_mint_approval.

This corresponds to Option A from Stage 1.10.

submit_mint_approval should verify:

- canonical message hash
- guardian public key membership
- guardian signature validity
- quorum threshold
- route binding
- recipient binding
- amount calculation
- finality/deadline requirements
- replay status

Rationale:

This keeps verification, replay marking, and minting within one atomic transaction.

### Decision 3: mint authority model

Decision:

The gateway program uses a gateway-controlled mint authority PDA for token mint CPI.

Candidate seed:

    mint_authority = [b"mint_authority"]

The gateway program signs the mint CPI with PDA signer seeds.

Rationale:

- simplest first implementation
- explicit authority boundary
- compatible with direct mint
- keeps mint authority inside the gateway program's controlled execution path

Open implementation detail:

The exact X1 token program interface still needs to be confirmed in runtime code.

### Decision 4: rent payer for ProcessedBurnEntry

Decision:

The relayer pays rent for ProcessedBurnEntry creation through a payer account passed to submit_mint_approval.

Rationale:

- each accepted burn event creates one permanent replay-protection account
- gateway program does not need to hold operational funds
- relayer cost is explicit and proportional to processed gateway events

Implication:

submit_mint_approval must include payer as an account.

If the transaction fails, EV-01 / EV-02 rollback evidence supports that the account creation and rent debit should not remain partially applied in the tested failure model.

## Should-have decisions for a clean Stage 2 branch

### Pause scope

First implementation should define pause as affecting only mint/claim flows.

Pause must not prevent recovery or unpause.

### GuardianSet sizing

GuardianSet account size is deterministic at creation time and does not change later.

A different guardian count requires a new GuardianSet PDA version.

## Updated Stage 1.10 conclusion

EV-01 and EV-02 removed the atomic rollback blocker.

Stage 1.10 plus these refinements remove the main program-design ambiguity needed before a first implementation branch.

The recommended first Stage 2 implementation branch should be:

- direct mint first
- guardian verification inside submit_mint_approval
- gateway PDA mint authority
- relayer-paid ProcessedBurnEntry rent
- permanent ProcessedBurnEntry replay protection
- claim-based flow retained as fallback, not first implementation

Stage 2 runtime implementation should still begin as a minimal prototype branch, not as production gateway deployment.

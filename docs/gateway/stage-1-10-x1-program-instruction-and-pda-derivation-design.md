# Stage 1.10 X1 Program Instruction and PDA Derivation Design

Stage 1.10 documents the final gateway program design layer before any X1 runtime implementation begins.

This stage exists because EV-01 and EV-02 have now been confirmed by an X1 testnet runtime probe:

- EV-01 transaction-level atomicity: confirmed by X1 testnet probe
- EV-02 account write rollback: confirmed by X1 testnet probe

That evidence confirms that failed instructions roll back account writes in the tested X1 runtime scenario.

However, it does not define the gateway program itself.

Stage 1.10 therefore defines the instruction boundaries, PDA derivation model, signer verification boundary, CPI/mint authority model, processed burn registry shape, and failure atomicity rules that must be stable before Stage 2 runtime implementation.

## What Stage 1.10 is not

Stage 1.10 does not implement:

- X1 gateway runtime code
- production bridge logic
- deployed production X1 programs
- Ethereum contracts
- production guardian keys
- relayer runtime
- watcher runtime
- frontend gateway flow
- XXXL production mint authority
- production emergency controls

It is a design-only milestone.

## Why this stage is required

The project now has evidence that X1 can support the atomic rollback behavior required by the gateway model.

But runtime implementation should still not begin until the program-level design answers these questions:

- Which instructions exist?
- Which accounts does each instruction read or write?
- Which PDA seeds are canonical?
- Which authority signs or verifies each instruction?
- Where guardian quorum verification happens?
- Whether minting is direct or claim-based?
- Which actions must happen in one atomic transaction?
- Which failure cases must leave zero state changes?

Without these answers, runtime code would hardcode unstable assumptions.

## Baseline from previous stages

Stage 1 defined the deterministic burn-to-mint model.

Stage 1.5 mapped the deterministic model to future X1 runtime concerns.

Stage 1.6 documented guardian set management design.

Stage 1.7 documented account and storage layout responsibilities.

Stage 1.8 documented X1 runtime assumptions.

Stage 1.9 documented readiness for Stage 2 planning.

Stage 2.0 through Stage 2.8 explored runtime planning, candidate direct mint and claim-based flows, risk review, evidence plans, and prototype boundaries.

The EV-01 / EV-02 X1 testnet probe then confirmed the required rollback behavior for a failed instruction scenario.

Stage 1.10 now narrows the design from planning-level architecture into concrete program instruction and PDA derivation design.

## Candidate instruction set

### initialize_gateway_config

Purpose:

- create the gateway configuration PDA
- set immutable or controlled parameters needed by the program
- define the initial guardian set or guardian set pointer
- define route configuration pointers
- define mint authority assumptions

Candidate accounts:

- gateway_config PDA
- payer / initializer
- system program
- optional guardian_set PDA
- optional route_config PDA
- optional mint authority PDA

Open decisions:

- whether initializer authority is burned after initialization
- whether guardian set is stored directly in gateway_config or separate guardian_set PDA
- whether route weights are immutable or versioned

### set_guardian_set

Purpose:

- rotate or update guardian set if on-chain guardian rotation is allowed

Candidate accounts:

- gateway_config PDA
- current guardian_set PDA
- new guardian_set PDA
- authority account or quorum proof accounts
- payer if new storage is created

Open decisions:

- whether guardian rotation is allowed at all
- whether rotation is governed by existing guardian quorum
- whether rotation is disabled after finalization
- whether guardian_set_version is strictly monotonic

### submit_mint_approval

Purpose:

- verify a burn-to-mint message
- verify guardian quorum or approval evidence
- mark the canonical burn as processed
- either mint XXXL directly or create a claim

Candidate accounts:

- gateway_config PDA
- guardian_set PDA
- route_config PDA
- processed_burn PDA
- recipient token account or claim PDA
- mint account
- mint_authority PDA
- token program
- system program
- payer / relayer

Atomicity requirement:

- if verification fails, no processed burn entry is created
- if quorum fails, no processed burn entry is created
- if recipient validation fails, no processed burn entry is created
- if mint CPI fails, no processed burn entry remains
- if claim creation fails, no processed burn entry remains

### claim_xxxl

Purpose:

- allow recipient to claim XXXL from a previously created claim

Only needed if the claim-based model is chosen.

Candidate accounts:

- claim PDA
- gateway_config PDA
- recipient signer
- recipient token account
- mint account
- mint_authority PDA
- token program

Atomicity requirement:

- if recipient signature fails, claim remains unchanged
- if mint CPI fails, claim remains unclaimed
- if claim close fails, minted state must not be inconsistent

### emergency_pause

Purpose:

- pause new submit_mint_approval or claim_xxxl flows if emergency controls are allowed

Candidate accounts:

- gateway_config PDA
- pause authority or guardian quorum evidence

Open decisions:

- whether emergency pause exists
- whether it can pause direct mint, claim, or both
- whether it can ever modify economics
- whether it can be permanently disabled

### emergency_unpause

Purpose:

- unpause the gateway after emergency review

Candidate accounts:

- gateway_config PDA
- pause authority or guardian quorum evidence

Open decisions:

- whether unpause authority is the same as pause authority
- whether unpause requires stronger quorum than pause

## PDA derivation model

PDA derivation should be deterministic, stable, and tied to canonical message fields.

### gateway_config

Seed:

    [b"gateway"]

Purpose:

- singleton gateway configuration
- stores active route version pointer
- stores guardian set version pointer
- stores pause state if pause is allowed
- stores mint authority metadata if applicable

### guardian_set

Seed:

    [b"guardian_set", guardian_set_version_le_bytes]

Purpose:

- stores guardian public keys
- stores quorum threshold
- stores activation timestamp or slot if needed
- stores deactivation metadata if rotation is supported

### route_config

Seed:

    [b"route", route_id]

Purpose:

- stores source chain id
- source token
- mint token
- source chain weight bps
- active/inactive state
- route version

### processed_burn

Seed:

    [b"processed", canonical_event_key]

Purpose:

- replay protection
- proves canonical burn event has already been consumed
- prevents duplicate mint or duplicate claim

### claim

Seed:

    [b"claim", canonical_event_key]

Purpose:

- stores claimable mint amount if claim-based flow is used
- binds recipient
- binds route version and guardian set version
- prevents anyone else from claiming the same burn

### mint_authority

Seed:

    [b"mint_authority"]

Purpose:

- PDA authority used by the gateway program to mint XXXL, if the gateway program owns mint authority

Open decision:

- whether the gateway program should hold mint authority directly
- whether a separate immutable mint authority program should exist
- whether mint authority can be revoked or frozen after deployment

## Candidate account structs

These structs are design targets, not final runtime code.

### GatewayConfig

    #[account]
    pub struct GatewayConfig {
        pub schema_version: u16,
        pub gateway_version: u64,
        pub active_guardian_set_version: u64,
        pub active_route_version: u64,
        pub paused: bool,
        pub bump: u8,
    }

### GuardianSet

    #[account]
    pub struct GuardianSet {
        pub version: u64,
        pub threshold: u16,
        pub guardian_count: u16,
        pub guardians: Vec<[u8; 32]>,
        pub active: bool,
        pub created_at: i64,
        pub deactivated_at: Option<i64>,
    }

### RouteConfig

    #[account]
    pub struct RouteConfig {
        pub route_id: [u8; 32],
        pub version: u64,
        pub source_chain_id: u64,
        pub source_token: [u8; 32],
        pub mint_token: [u8; 32],
        pub source_chain_weight_bps: u16,
        pub active: bool,
    }

### ProcessedBurnEntry

    #[account]
    pub struct ProcessedBurnEntry {
        pub canonical_event_key: [u8; 32],
        pub guardian_set_version: u64,
        pub route_version: u64,
        pub minted_amount: u64,
        pub processed_at: i64,
    }

### ClaimEntry

    #[account]
    pub struct ClaimEntry {
        pub canonical_event_key: [u8; 32],
        pub recipient: [u8; 32],
        pub guardian_set_version: u64,
        pub route_version: u64,
        pub amount: u64,
        pub claimed: bool,
        pub created_at: i64,
    }

## Signer and verification boundaries

The gateway design must choose exactly where guardian verification happens.

### Option A: guardian signatures verified inside submit_mint_approval

The instruction receives the canonical message and guardian signatures.

The program verifies:

- canonical message hash
- guardian public key membership
- signature validity
- threshold
- route binding
- recipient binding
- amount calculation
- deadline/finality block

Benefits:

- one instruction can fully validate and mint/claim
- replay protection can be marked atomically with mint/claim

Risks:

- compute budget may be high
- signature verification may require specific X1 runtime support
- transaction size may become limiting

### Option B: guardian approvals are submitted before mint/claim

Separate instructions store guardian approvals.

Then submit_mint_approval consumes stored approvals.

Benefits:

- lower per-instruction compute
- easier to inspect approvals

Risks:

- more state accounts
- more replay/state complexity
- more failure cases
- more cleanup rules

### Current preferred boundary

The preferred first design should remain single-transaction where possible:

- verify message
- verify quorum
- mark processed burn
- mint directly or create claim

This keeps the Stage 1 invariant closest to the deterministic model.

If compute or transaction size prevents this, the claim-based or staged-approval design becomes the fallback.

## CPI and mint authority design

If direct mint is used, submit_mint_approval must invoke token mint CPI.

Required atomic rule:

    verification + processed burn mark + mint CPI must be in one transaction

If mint CPI fails, processed_burn must not remain created.

EV-01 / EV-02 testnet evidence supports the assumption that account writes are rolled back when the instruction fails.

Still open before implementation:

- exact X1 token program interface
- mint authority account type
- PDA signer format for mint authority
- whether mint authority can be immutable
- whether XXXL mint already exists or is created by deployment flow

## Direct mint vs claim-based boundary

### Direct mint

submit_mint_approval performs:

1. message verification
2. guardian verification
3. processed_burn PDA creation
4. XXXL mint CPI to recipient

Pros:

- closest to burn-to-mint model
- one successful transaction completes the bridge mint
- no separate user claim step

Cons:

- higher compute and transaction size pressure
- recipient token account must be ready or created
- mint CPI failure rolls back full transaction

### Claim-based

submit_mint_approval performs:

1. message verification
2. guardian verification
3. processed_burn PDA creation
4. claim PDA creation

claim_xxxl later performs mint CPI.

Pros:

- separates verification from user claim
- lower complexity in submit path
- recipient can claim when account setup is ready

Cons:

- weaker immediate mint semantics
- more account state
- claim lifecycle and cleanup required
- processed burn and claim consistency must be enforced

## Failure atomicity matrix

| Failure point | Required result |
| --- | --- |
| invalid message encoding | no processed burn, no claim, no mint |
| invalid route | no processed burn, no claim, no mint |
| invalid source chain | no processed burn, no claim, no mint |
| invalid token | no processed burn, no claim, no mint |
| invalid recipient | no processed burn, no claim, no mint |
| guardian signature invalid | no processed burn, no claim, no mint |
| guardian quorum failure | no processed burn, no claim, no mint |
| replayed canonicalEventKey | no duplicate claim, no duplicate mint |
| mint CPI failure | no processed burn remains in direct mint flow |
| claim creation failure | no processed burn remains in claim-based flow |
| claim mint failure | claim remains unclaimed |
| pause active | no processed burn, no claim, no mint |

## Stage 1.10 implementation blocker list

Before Stage 2 runtime implementation begins, these decisions must be stable:

1. final instruction set
2. direct mint vs claim-based first implementation path
3. exact PDA seeds
4. exact account struct fields and fixed-size constraints
5. guardian verification mechanism
6. token mint CPI interface
7. mint authority model
8. emergency pause existence and authority
9. processed burn account creation and replay behavior
10. error code and event/logging model
11. compute and transaction-size constraints for the selected path

## Current conclusion

EV-01 and EV-02 removed the atomic rollback blocker.

They do not remove the program design blocker.

Stage 1.10 should be treated as the final instruction/PDA design milestone before any Stage 2 runtime implementation branch.

Stage 2 runtime code should not begin until this design is reviewed and the open decisions are either resolved or explicitly marked as implementation blockers.

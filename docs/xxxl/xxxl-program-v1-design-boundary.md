# XXXL Program v1 Design Boundary

## Purpose

This document defines the first design boundary for the canonical XXXL program on X1.

XXXL is the canonical X1-native token layer for the xEnchanted / X1 gateway path.

In the Genesis Phase, XXXL is minted only from verified Ethereum XNTD gateway events.

This document is intentionally boundary-first. It does not define production deployment parameters, live X1 runtime accounts, final X1-native emission mechanics, or xDex listing mechanics.

## Main separation

XXXL and Build are separate protocol layers.

XXXL:

- represents the canonical X1-native token
- is initially minted only through verified Ethereum XNTD gateway events
- must not require a Build to receive minted XXXL
- must not read Build state to authorize gateway minting
- must not derive supply rights from Build activation or Build identity

Build:

- represents separate history / identity / contribution state
- uses confirmed historical actions
- does not derive authority from the current XXXL balance
- does not gate the basic XNTD transfer to X1 path

This separation is required so that users can move XNTD to X1 before creating or activating Build.

## Genesis Phase

The XXXL Genesis Phase starts gateway-only.

Rules:

- XXXL minting is authorized only by verified Ethereum XNTD gateway events.
- There is no manual mint.
- There is no premine.
- There is no founder allocation.
- There is no hidden emission path.
- There is no X1-native mint path in the first phase.
- The controller must not expose arbitrary supply control.

A valid gateway mint must be traceable to a unique Ethereum-side XNTD event that passed the gateway authorization rules.

## Upgradeability boundary

The canonical XXXL program may be temporarily upgradeable during staged protocol finalization.

This temporary upgradeability exists only because the final X1-native emission mechanics are not yet fully implemented.

It must not be interpreted as:

- admin mint authority
- discretionary supply control
- founder allocation authority
- hidden emission authority
- permission to rewrite existing user balances
- permission to bypass gateway authorization

Allowed future upgrades may only add deterministic user-action protocol mechanics.

The expected future direction is X1-native Core / Forge / Stake-like mechanics.

After the planned X1-native emission mechanics are implemented, documented, tested, reviewed, and publicly explained, the upgrade authority must be removed, frozen, or otherwise made permanently unable to change the program.

## Finalization covenant

The public covenant is:

1. XXXL starts gateway-only.
2. No manual mint.
3. No premine.
4. No founder allocation.
5. Future upgrades may only add deterministic user-action protocol mechanics.
6. Future X1-native emission must be Core / Forge / Stake-like logic, not admin mint.
7. All new mechanics must be documented, tested, discussed with the community, and publicly explained.
8. After X1-native emission mechanics are complete, upgrade authority must be removed / frozen.

## Gateway-only mint authorization

A gateway mint authorization must include enough deterministic source data to prove that the mint came from a unique Ethereum-side XNTD event.

At the model boundary, the authorization must bind at least:

- route id
- source chain id
- source token
- source sender
- source burn / lock transaction hash
- source event index
- source block number
- source block hash
- canonical event key
- X1 recipient
- XXXL mint amount
- mint token
- finality / approval evidence

The exact Stage 1 gateway message schema and guardian approval logic remain the source of truth for Ethereum-to-X1 authorization.

The XXXL program boundary consumes only valid gateway mint authorizations.

## Stage 1 authorization consumer boundary

The XXXL Program v1 gateway entry must consume successful Stage 1 gateway mint authorization.

This means the canonical gateway path is not:

    arbitrary local mint request -> XXXL mint

It is:

    verified Stage 1 gateway message -> successful Stage 1 mint authorization -> XXXL mint

Stage 1 remains responsible for message verification, route binding, source binding, guardian quorum, and source replay protection.

XXXL remains responsible for local consumed-event protection and the Genesis Phase supply invariant.

## Replay protection

Every accepted gateway mint must mark its canonical event key as processed.

Replay rule:

- if canonical event key was already processed, mint must fail
- failed mint must not mutate supply
- failed mint must not mark new replay state
- successful mint and processed-event mark must be atomic at runtime

## Supply invariant

During the Genesis Phase:

    XXXL total supply = sum(valid accepted gateway mint amounts)

No other operation may increase total supply.

Manual supply changes are forbidden.

Future deterministic X1-native emission may extend the supply invariant, but only by adding explicit user-action mechanics.

## Genesis supply invariant hardening

The Genesis Phase supply invariant is strengthened as:

    XXXL total supply = sum(Stage 1 authorized gateway mint amounts consumed exactly once)

This explicitly excludes manual mint, premine, founder allocation, admin mint, hidden emission, Build-derived supply rights, and current-balance-derived supply rights.

Rejected transitions must not mutate supply or replay state.

## X1 runtime mapping summary

The future X1 runtime must preserve the deterministic model at the account and instruction level.

Required runtime objects include:

- XXXL mint state
- gateway configuration state
- guardian set state
- processed event state
- recipient balance state

The canonical Genesis Phase instruction is:

    consume_gateway_mint

This instruction must atomically verify the gateway authorization, mint XXXL, update recipient balance, update total supply, and mark the canonical event key consumed.

If any check fails, supply and replay state must remain unchanged.

## Runtime mapping direction

The future X1 runtime mapping must preserve these model-level properties:

- check before mark
- mint and replay mark in one atomic state transition
- no state change on invalid authorization
- no state change on replay
- no state change on wrong route
- no state change on wrong mint token
- no state change on zero amount
- no Build dependency for gateway mint
- no manual supply control

## Deployment readiness and xDex planning summary

The first deployment-ready scope is the gateway-only Genesis Phase.

Before deployment, the project must have final route configuration, guardian policy, replay storage design, authority model, upgradeability disclosure, freeze plan, and public Genesis Phase explanation.

xDex listing may happen before full Build launch because XXXL and Build are separate layers:

- XXXL is transferable token state
- Build is non-transferable history / identity / contribution state
- Build does not derive rights from current XXXL balance
- XNTD transfer to X1 does not require Build activation

The listing must not imply guaranteed price, liquidity, rewards, Build allocation, or final emission schedule.

## Out of scope for this stage

This stage does not implement:

- live X1 deployment
- production upgrade authority removal
- final X1-native Core / Forge / Stake-like emission
- xDex listing integration
- frontend changes
- watcher runtime
- guardian key management changes
- production signer rotation
- RPC smoke scripts

## Review questions

1. Is the gateway-only Genesis Phase boundary clear enough?
2. Is the temporary upgradeability covenant strict enough?
3. Is the Build / XXXL separation explicit enough?
4. Is the supply invariant strong enough for the Genesis Phase?
5. Which X1 runtime constraints must be proven before deployment readiness?

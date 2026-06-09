# Stage 2.5 Gateway Risk Review

## Purpose

Stage 2.5 defines a gateway risk review for the future X1-side gateway.

This is a planning-only checkpoint.

It does not implement runtime code, deploy an X1 program, create production mint authority, enable real cross-chain minting, or approve a final gateway architecture.

The purpose is to collect the highest-risk unresolved assumptions from the Stage 2 runtime planning work before any implementation begins.

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 2.0 created the gateway runtime planning outline.

Stage 2.1 mapped runtime assumption dependencies.

Stage 2.2 described the direct mint candidate.

Stage 2.3 described the claim-based candidate.

Stage 2.4 compared direct mint and claim-based architecture.

Stage 2.5 now turns those findings into a risk review.

The goal is not to choose the architecture immediately.

The goal is to identify what can break the design if implemented too early.

## Risk review scope

This review covers:

- X1 runtime atomicity
- CPI/token mint behavior
- processed burn registry durability
- canonicalEventKey immutability
- guardian quorum and signature verification
- route and coefficient version binding
- source chain finality and fork handling
- pause and upgrade boundaries
- storage and rent risks
- indexing and event observability
- failure recovery without admin intervention
- implementation blockers

This review does not cover:

- production deployment
- final contract/program code
- frontend implementation
- token listing
- market strategy
- final guardian set selection
- legal or operational governance
- live bridge operation

## Risk severity model

This document uses three severity levels.

| Severity | Meaning |
| --- | --- |
| Critical | A failure can create invalid minting, permanent loss of claim/redeem rights, replay, broken accounting, or a need for forbidden manual intervention. |
| High | A failure can block users, create ambiguous protocol state, weaken auditability, or force complex migration before launch. |
| Medium | A failure can degrade UX, indexing, monitoring, or operational clarity, but should not directly create invalid minting if core protections hold. |

A risk marked Critical should block runtime implementation until resolved.

## Risk status model

| Status | Meaning |
| --- | --- |
| Open | Not yet resolved or not yet verified against concrete X1 runtime behavior. |
| Needs evidence | The expected design is known, but must be proven with X1 documentation, prototype tests, or runtime experiments. |
| Mitigated by design | The design contains a mitigation, but implementation must later prove it. |
| Accepted planning constraint | The risk is intentionally carried as a documented planning constraint, not implemented. |

Most risks in this checkpoint remain Open or Needs evidence.

That is expected.

Stage 2 is still planning.

## Risk register

| ID | Risk | Severity | Status | Implementation impact |
| --- | --- | --- | --- | --- |
| R-01 | X1 transaction atomicity is not confirmed. | Critical | Needs evidence | Blocks both direct mint and claim-based flow. |
| R-02 | CPI/token mint behavior does not roll back safely on failure. | Critical | Needs evidence | Blocks direct mint and claim redemption. |
| R-03 | Processed burn registry can become inconsistent with mint or claim state. | Critical | Open | Blocks all gateway implementation. |
| R-04 | canonicalEventKey derivation changes after launch. | Critical | Open | Can allow replay, duplicate claims, or invalid rejection. |
| R-05 | Route/coefficient version is not bound into signed messages. | Critical | Mitigated by design | Must be proven in implementation. |
| R-06 | Guardian quorum accepts malformed, duplicate, stale, or wrong-domain signatures. | Critical | Mitigated by design | Blocks gateway security if not proven. |
| R-07 | Source chain fork handling is undefined. | Critical | Open | Blocks source routes until finality rules are defined. |
| R-08 | Pause behavior can reinterpret or delete existing state. | High | Open | Blocks safe emergency design. |
| R-09 | Upgrade behavior can reinterpret old messages, claims, or coefficients. | Critical | Open | Blocks upgradeable runtime design unless constrained. |
| R-10 | Claim account lifecycle weakens replay protection. | Critical | Mitigated by design | Blocks claim-based flow unless ProcessedBurnEntry remains authoritative. |
| R-11 | Claim ownership rules are ambiguous. | High | Open | Blocks claim-based flow. |
| R-12 | Recipient token account handling is ambiguous. | High | Open | Blocks direct mint and claim redemption UX/security. |
| R-13 | Storage/rent requirements create stuck or unbounded state. | High | Open | Blocks claim-based flow until storage policy is defined. |
| R-14 | Compute budget is too small for signature verification and state writes. | High | Needs evidence | May force architecture changes. |
| R-15 | Transaction size is too small for messages, signatures, and required accounts. | High | Needs evidence | May force batching or claim-based flow. |
| R-16 | Events/logs/indexing are insufficient for audit and claim discovery. | Medium | Open | Affects observability and claim UX. |
| R-17 | Relayer assumptions accidentally become trust assumptions. | High | Mitigated by design | Must ensure relayer cannot choose recipient, amount, or route. |
| R-18 | Mint authority model requires admin control. | Critical | Open | Blocks first-principles compatibility. |
| R-19 | Manual recovery path is required after partial failure. | Critical | Open | Violates no-admin protocol expectations. |
| R-20 | Migration path changes protocol meaning. | Critical | Open | Blocks upgrade/migration design. |

## R-01: X1 transaction atomicity

Risk:

The gateway depends on transaction-level atomicity.

Direct mint requires processed mark, token mint, and accounting to commit together or fail together.

Claim-based flow requires processed mark and claim creation to commit together, then claim redemption and mint to commit together.

Why it matters:

If atomicity is weak, the protocol can enter invalid states:

- source event marked processed but no token minted
- token minted but source event not marked processed
- claim created without processed entry
- processed entry created without claim
- claim marked redeemed without mint
- mint completed while claim remains redeemable

Severity:

Critical.

Required evidence:

- X1 runtime documentation
- minimal prototype tests
- explicit rollback behavior under failure
- tests for failed CPI/token mint
- tests for failed account creation
- tests for failed accounting update

Implementation gate:

No gateway implementation until transaction atomicity is confirmed.

## R-02: CPI/token mint rollback behavior

Risk:

The gateway may call a token program or token mint interface.

If that call fails after gateway state changes, rollback behavior must be guaranteed.

Why it matters:

Direct mint and claim redemption both depend on token mint side effects.

A failed token CPI must not leave processed, claim, accounting, or redeemed state mutated.

Severity:

Critical.

Required evidence:

- token program interface details
- failure behavior tests
- account write rollback tests
- mint authority behavior tests

Implementation gate:

No direct mint or claim redemption implementation until token CPI rollback behavior is proven.

## R-03: Processed burn registry durability

Risk:

ProcessedBurnEntry is the main replay protection layer.

If it is not durable, deterministic, and globally scoped, the same source event can be reused.

Why it matters:

Replay protection must survive:

- route changes
- coefficient changes
- guardian set changes
- pause/unpause
- upgrades
- claim closure
- source route deactivation

Severity:

Critical.

Required design rule:

ProcessedBurnEntry must be the authoritative replay barrier.

ClaimAccount must never replace it.

Closing or redeeming claims must not delete replay protection.

Implementation gate:

No gateway implementation until processed burn registry persistence and derivation are defined.

## R-04: canonicalEventKey immutability

Risk:

If canonicalEventKey derivation changes, the same source event may map to different keys.

Why it matters:

This can cause:

- duplicate minting
- duplicate claim creation
- invalid rejection of valid events
- incompatible historical state
- unsafe migration

Severity:

Critical.

Required design rule:

canonicalEventKey derivation must be immutable for each source route version.

If derivation ever changes, the version boundary must prevent reinterpretation.

Implementation gate:

No implementation until canonicalEventKey fields and versioning are frozen for the candidate route.

## R-05: Route and coefficient version binding

Risk:

A message verified under one route or coefficient version may be replayed or reinterpreted under another.

Why it matters:

This can change mint amount or eligibility after the source event has occurred.

Severity:

Critical.

Mitigation:

Messages must bind:

- source route
- route version
- coefficient version
- source chain identity
- domain separator
- guardian set version if required
- canonicalEventKey inputs

Implementation gate:

No implementation until route/coefficient version binding is included in test vectors.

## R-06: Guardian quorum and signature verification

Risk:

Guardian verification may accept bad signatures, wrong-domain signatures, duplicate guardians, stale guardian sets, or malformed messages.

Why it matters:

Guardians confirm source evidence.

If verification is weak, attackers can mint or create claims from invalid evidence.

Severity:

Critical.

Required design rule:

Guardians confirm only source evidence.

They do not choose recipient, amount, route, coefficient, replay status, or protocol outcome.

Implementation gate:

No implementation until signature verification has negative matrix tests.

## R-07: Source chain finality and forks

Risk:

A source event may be observed on a fork or before finality.

Why it matters:

A mint or claim based on a non-canonical source event can create invalid X1-side supply.

Severity:

Critical.

Required design rule:

Each source route must define finality.

Source fork handling must be explicit before route activation.

Implementation gate:

No source route implementation until finality and fork policy are documented.

## R-08: Pause behavior

Risk:

Pause behavior may be too broad, too narrow, or state-mutating.

Why it matters:

Pause must protect the system without changing protocol meaning.

Dangerous pause behavior includes:

- deleting claims
- deleting processed entries
- changing coefficients retroactively
- changing redemption rules
- allowing admin recovery mints
- trapping valid claims indefinitely without policy

Severity:

High.

Implementation gate:

No pause implementation until separate rules are defined for claim creation, claim redemption, direct mint, route activation, and replay state.

## R-09: Upgrade boundaries

Risk:

If runtime upgradeability exists, upgrades may reinterpret old messages, claims, or replay entries.

Why it matters:

The user wants the core protocol logic to remain first-principles compatible.

Upgradeability can become a hidden admin path if not tightly constrained.

Severity:

Critical.

Required design rule:

Upgrades must not silently change:

- canonicalEventKey meaning
- route/coefficient interpretation
- claim ownership
- redemption rules
- mint authority
- replay protection
- historical message interpretation

Implementation gate:

No upgradeable runtime design until upgrade boundaries and migration rules are documented.

## R-10: Claim lifecycle and replay protection

Risk:

Claim-based flow introduces claim accounts that may be pending, redeemed, closed, expired, or migrated.

If claim lifecycle is confused with replay protection, replay safety can break.

Why it matters:

A closed claim must not allow the same source event to create a new claim.

Severity:

Critical.

Required design rule:

ProcessedBurnEntry must remain even if a claim is redeemed or closed.

Implementation gate:

No claim-based implementation until claim lifecycle and replay state are clearly separated.

## R-11: Claim ownership ambiguity

Risk:

The claim-based model may not clearly define who can redeem a claim.

Why it matters:

Ambiguous ownership can allow wrong-recipient redemption or stuck claims.

Severity:

High.

Required design questions:

- must the recipient sign?
- can a relayer redeem for the recipient?
- can claims be transferred?
- can claims be delegated?
- can recipient token account differ from recipient identity?
- can claims expire?
- can claims be cancelled?

Implementation gate:

No claim-based implementation until ownership rules are final.

## R-12: Recipient token account handling

Risk:

The recipient token account may not exist, may be wrong, may be controlled by another identity, or may require creation.

Why it matters:

Direct mint must solve this during the mint transaction.

Claim-based flow can defer it, but redemption must still enforce safe ownership.

Severity:

High.

Implementation gate:

No implementation until recipient token account rules are defined for both candidates.

## R-13: Storage and rent

Risk:

Claim-based flow introduces additional accounts and possible long-lived pending claims.

Why it matters:

Unbounded storage can create economic or operational pressure.

Closing claims can help, but must not weaken auditability or replay protection.

Severity:

High.

Implementation gate:

No claim-based implementation until storage/rent policy is documented.

## R-14: Compute budget

Risk:

Gateway verification may exceed runtime compute limits.

Why it matters:

Signature verification, message reconstruction, account loading, replay checks, and token minting may be too heavy in one transaction.

Severity:

High.

Implementation gate:

No implementation until compute budget is measured with realistic message/signature/account counts.

## R-15: Transaction size limits

Risk:

Gateway messages may exceed transaction size limits.

Why it matters:

Messages may include source evidence, signatures, account references, guardian data, and route data.

Severity:

High.

Implementation gate:

No implementation until transaction size is measured with realistic worst-case payloads.

## R-16: Indexing and observability

Risk:

Events/logs may be insufficient for users and watchers to audit gateway state.

Why it matters:

Claim-based flow especially requires claim discovery.

Direct mint also requires audit visibility for source event to X1 result mapping.

Severity:

Medium.

Implementation gate:

No production gateway plan until event/indexing requirements are defined.

## R-17: Relayer trust creep

Risk:

Relayers may accidentally become trusted actors if they can influence message fields.

Why it matters:

The relayer must only submit data.

It must not choose:

- recipient
- amount
- route
- coefficient
- canonicalEventKey
- guardian set
- replay state

Severity:

High.

Implementation gate:

No implementation until relayer input is strictly separated from signed message content.

## R-18: Mint authority model

Risk:

The X1-side token mint authority may require a mutable/admin-controlled actor.

Why it matters:

The protocol philosophy requires immutable/no-admin core rules.

If mint authority is discretionary, the gateway becomes admin-mint-like.

Severity:

Critical.

Implementation gate:

No implementation until mint authority can be made compatible with deterministic protocol rules.

## R-19: Manual recovery requirement

Risk:

A failure mode may require manual admin recovery.

Why it matters:

Manual recovery can violate the first-principles/no-admin model.

Severity:

Critical.

Required design rule:

The system should avoid states that require admin intervention.

If recovery is needed, it must be deterministic and protocol-defined.

Implementation gate:

No implementation until partial-failure recovery is eliminated or protocol-defined.

## R-20: Migration changes protocol meaning

Risk:

Migration may change the meaning of historical claims, processed entries, messages, routes, or coefficients.

Why it matters:

Historical protocol state must not be reinterpreted.

Severity:

Critical.

Implementation gate:

No migration path unless it preserves historical meaning and replay safety.

## Cross-risk conclusions

The highest-risk dependency cluster is:

- transaction atomicity
- CPI/token mint rollback
- processed burn registry durability
- canonicalEventKey immutability
- source finality/fork handling
- mint authority model

These risks block both direct mint and claim-based implementation.

Direct mint adds concentrated risk around:

- one-transaction atomicity
- recipient token account handling
- compute/transaction size pressure

Claim-based flow adds distributed risk around:

- claim lifecycle
- claim ownership
- storage/rent
- indexing/discovery
- migration compatibility

## Current recommendation

Do not implement gateway runtime behavior yet.

The next step should be X1 runtime evidence collection or prototype-only experiments focused on the highest-risk assumptions.

The gateway architecture can continue to be refined, but production-like implementation should remain blocked until the critical risks have evidence-backed answers.

If evidence confirms strong atomicity, safe token mint rollback, deterministic recipient account handling, and acceptable compute/transaction limits, direct mint should remain the preferred candidate.

If direct mint constraints fail but claim creation/redemption can be proven safe, claim-based flow remains the fallback candidate.

If neither risk cluster is resolved, neither candidate should be implemented.

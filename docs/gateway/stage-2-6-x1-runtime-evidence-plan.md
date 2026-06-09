# Stage 2.6 X1 Runtime Evidence Collection Plan

## Purpose

Stage 2.6 defines the evidence collection plan for unresolved X1 runtime assumptions.

This is a planning-only checkpoint.

It does not implement runtime code, deploy an X1 program, create production mint authority, enable real cross-chain minting, or approve a final gateway architecture.

The purpose is to define which X1 runtime facts must be proven before direct mint, claim-based flow, or any production-like gateway implementation can begin.

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 2.5 identified the highest-risk unresolved gateway assumptions.

The critical risk cluster is:

- X1 transaction atomicity
- CPI/token mint rollback behavior
- processed burn registry durability
- canonicalEventKey immutability
- source finality and fork handling
- mint authority model

Stage 2.6 turns those risks into an evidence plan.

The goal is not to build the gateway.

The goal is to define what must be verified, how it can be verified, and what each result unlocks or keeps blocked.

## Evidence categories

Evidence can come from:

1. Official X1 runtime documentation.

2. Minimal local prototype experiments.

3. Devnet/testnet prototype experiments.

4. Runtime failure tests.

5. Token program interface tests.

6. Account/storage persistence tests.

7. Transaction size and compute measurements.

8. Event/log/indexing observations.

Evidence should be recorded in repository documentation before it affects architecture choice.

No evidence should be treated as valid if it only exists as informal memory or chat discussion.

## Evidence quality levels

| Level | Meaning |
| --- | --- |
| E0 | No evidence. Assumption remains unconfirmed. |
| E1 | Documentation evidence only. Useful, but not enough for implementation. |
| E2 | Minimal prototype evidence. Confirms basic behavior in controlled conditions. |
| E3 | Negative/failure prototype evidence. Confirms rollback and rejection behavior. |
| E4 | Integration-like evidence. Confirms behavior with realistic message/account/signature size. |
| E5 | Production-readiness evidence. Includes repeatable tests, documented limits, and review. |

Gateway implementation should not begin from E0/E1.

For critical assumptions, E3 is the minimum planning target before implementation planning can become credible.

## Evidence register

| ID | Runtime fact to verify | Risk source | Required minimum | Blocks |
| --- | --- | --- | --- | --- |
| EV-01 | Transaction-level atomicity. | R-01 | E3 | Direct mint, claim creation, claim redemption. |
| EV-02 | Account write rollback on failed instruction. | R-01, R-03 | E3 | All gateway state writes. |
| EV-03 | CPI/token mint rollback on failure. | R-02 | E3 | Direct mint, claim redemption. |
| EV-04 | Token mint authority model. | R-18 | E2/E3 | X1-side XXXL minting path. |
| EV-05 | Deterministic account derivation. | R-03, R-10 | E2 | Processed entries, claims, config accounts. |
| EV-06 | Processed burn registry persistence. | R-03 | E3 | Replay protection. |
| EV-07 | Maximum transaction size. | R-15 | E4 | Message and signature format. |
| EV-08 | Compute budget for verification path. | R-14 | E4 | Direct mint vs claim-based selection. |
| EV-09 | Event/log availability and indexing. | R-16 | E2/E3 | Auditability and claim discovery. |
| EV-10 | Token account creation/handling rules. | R-12 | E3 | Recipient delivery flow. |
| EV-11 | Pause state behavior. | R-08 | E2/E3 | Emergency boundaries. |
| EV-12 | Upgrade/migration constraints. | R-09, R-20 | E1/E2 | Runtime governance boundary. |
| EV-13 | Source route finality model. | R-07 | E1/E2 | Source route activation. |
| EV-14 | Source fork handling model. | R-07 | E1/E2 | Canonical source event acceptance. |
| EV-15 | Guardian signature verification limits. | R-06, R-14, R-15 | E4 | Quorum design and message format. |
| EV-16 | Rent/storage behavior for long-lived accounts. | R-13 | E2/E3 | Claim-based flow. |

## EV-01: Transaction-level atomicity

Question:

If a gateway instruction performs multiple state changes and one later step fails, does the entire transaction roll back?

Why it matters:

Direct mint requires the processed burn mark, token mint, and accounting update to commit together or fail together.

Claim-based flow requires processed burn mark and claim creation to commit together.

Minimum evidence:

- documentation evidence describing atomic transaction behavior
- prototype that writes multiple accounts and intentionally fails after partial writes
- proof that all writes roll back after failure
- repeatable test command

Required level:

E3.

Unlocks:

- direct mint planning can continue
- claim creation planning can continue
- claim redemption planning can continue

Without evidence:

- no gateway runtime implementation

## EV-02: Account write rollback

Question:

Are account writes rolled back when an instruction fails after mutating account data?

Why it matters:

ProcessedBurnEntry, ClaimAccount, MintState, PauseState, and VersionState all depend on safe rollback.

Minimum evidence:

- prototype account write
- forced failure after write
- post-failure state inspection
- repeatable negative test

Required level:

E3.

Unlocks:

- replay registry implementation planning
- claim account implementation planning
- mint accounting implementation planning

Without evidence:

- no persistent gateway state writes

## EV-03: CPI/token mint rollback

Question:

If the gateway invokes the token mint interface and the invocation fails, are gateway-side writes rolled back?

Why it matters:

Token mint failure must not leave processed, claim, accounting, or redeemed state mutated.

Minimum evidence:

- prototype CPI/token mint call
- forced mint failure
- failed token account handling case
- post-failure state inspection
- repeatable negative test

Required level:

E3.

Unlocks:

- direct mint candidate can remain preferred
- claim redemption can be planned credibly

Without evidence:

- no minting path implementation

## EV-04: Token mint authority model

Question:

Can the X1-side token mint authority be controlled by deterministic protocol logic instead of discretionary admin control?

Why it matters:

The protocol must remain compatible with immutable/no-admin first-principles rules.

Minimum evidence:

- documentation of token mint authority model
- prototype showing gateway-authorized mint path
- proof that arbitrary admin mint is not required for normal flow

Required level:

E2 for architecture planning.

E3 before implementation planning.

Unlocks:

- XXXL mint path design
- direct mint and claim redemption feasibility

Without evidence:

- X1-side token minting remains blocked

## EV-05: Deterministic account derivation

Question:

Can processed entries, claim accounts, and config/version accounts be derived deterministically?

Why it matters:

Replay protection and claim identity must not depend on relayer-selected addresses.

Minimum evidence:

- documentation or prototype for deterministic account derivation
- collision behavior
- address derivation from canonicalEventKey
- version/domain separation behavior

Required level:

E2.

Unlocks:

- processed burn registry planning
- claim account planning

Without evidence:

- claim-based flow remains blocked
- replay registry design remains incomplete

## EV-06: Processed burn registry persistence

Question:

Can the processed burn registry persist indefinitely and remain authoritative across route changes, claim closure, and upgrades?

Why it matters:

Replay protection is the central safety layer.

Minimum evidence:

- prototype persistent ProcessedBurnEntry
- duplicate event rejection
- rejection after claim redemption/closure if claim-based prototype exists
- route/coefficient version boundary tests

Required level:

E3.

Unlocks:

- replay protection implementation planning

Without evidence:

- no gateway implementation

## EV-07: Maximum transaction size

Question:

Can a realistic gateway message fit within transaction size limits?

Why it matters:

Gateway messages may include route data, source identity, recipient, amount, canonicalEventKey inputs, guardian signatures, and required accounts.

Minimum evidence:

- documentation of transaction size limits
- measured maximum realistic gateway payload
- guardian quorum payload measurement
- direct mint vs claim creation comparison

Required level:

E4.

Unlocks:

- message format decisions
- guardian quorum size decisions
- direct mint vs claim-based choice

Without evidence:

- transaction format remains provisional

## EV-08: Compute budget

Question:

Can signature verification, message reconstruction, replay checks, state writes, and token minting fit within compute budget?

Why it matters:

Direct mint may concentrate too much work in one transaction.

Claim-based flow may split compute across two transactions.

Minimum evidence:

- documentation of compute limits
- prototype measurement for verification-only flow
- prototype measurement for direct mint-like flow
- prototype measurement for claim creation-like flow
- prototype measurement for claim redemption-like flow

Required level:

E4.

Unlocks:

- architecture choice gate
- direct mint feasibility
- claim-based fallback feasibility

Without evidence:

- implementation would be premature

## EV-09: Event/log availability and indexing

Question:

Can the runtime emit or expose enough observable data for users, watchers, and indexers?

Why it matters:

Direct mint requires auditability.

Claim-based flow requires claim discovery.

Minimum evidence:

- documentation of event/log model
- prototype event emission
- observed fields and indexing behavior
- test showing claim discovery if claim-based flow remains active

Required level:

E2/E3.

Unlocks:

- frontend/indexing planning
- claim UX planning

Without evidence:

- claim-based UX remains weak

## EV-10: Token account creation and handling

Question:

How are recipient token accounts created, validated, and used?

Why it matters:

Direct mint must deliver to a safe recipient token account.

Claim-based redemption must not allow unauthorized recipient substitution.

Minimum evidence:

- documentation of token account ownership rules
- prototype valid recipient token account mint
- wrong recipient rejection
- missing account behavior
- account creation behavior if supported

Required level:

E3.

Unlocks:

- direct mint recipient design
- claim redemption recipient design

Without evidence:

- token delivery rules remain blocked

## EV-11: Pause state behavior

Question:

Can pause state block new risky actions without changing historical state?

Why it matters:

Pause must not become hidden admin control or state reinterpretation.

Minimum evidence:

- prototype pause flag behavior
- paused direct mint rejection
- paused claim creation rejection
- selected paused redemption behavior
- proof that processed entries and claims are not deleted or reinterpreted

Required level:

E2/E3.

Unlocks:

- emergency boundary design

Without evidence:

- pause design remains conceptual

## EV-12: Upgrade and migration constraints

Question:

Can upgrades be constrained so they do not reinterpret historical state?

Why it matters:

Upgrades can violate protocol meaning if old messages, claims, coefficients, or replay entries are reinterpreted.

Minimum evidence:

- documentation of upgrade model
- explanation of immutable or upgradeable boundaries
- migration constraint proposal
- historical state compatibility plan if upgradeability exists

Required level:

E1/E2 for planning.

Unlocks:

- governance boundary documentation

Without evidence:

- upgradeable runtime should be treated as high-risk

## EV-13: Source route finality model

Question:

What finality threshold is required for each source route before a source event can be accepted?

Why it matters:

A non-final source event can disappear after fork/reorg.

Minimum evidence:

- source route finality rule
- watcher finality policy
- guardian signing policy
- documentation per source chain

Required level:

E1/E2.

Unlocks:

- source route activation planning

Without evidence:

- no source route should be treated as production-ready

## EV-14: Source fork handling model

Question:

How does the gateway handle competing source histories?

Why it matters:

The gateway must not mint from a non-canonical fork or accept the same logical event twice.

Minimum evidence:

- fork-aware canonicalEventKey policy
- guardian fork rejection policy
- source route finality rule
- replay behavior under fork ambiguity

Required level:

E1/E2.

Unlocks:

- source route risk classification

Without evidence:

- source route remains blocked

## EV-15: Guardian signature verification limits

Question:

How many guardian signatures can be verified within transaction and compute limits?

Why it matters:

Guardian quorum size affects security and feasibility.

Minimum evidence:

- signature verification prototype
- duplicate guardian rejection
- wrong-domain signature rejection
- stale guardian set rejection
- compute measurement
- transaction size measurement

Required level:

E4.

Unlocks:

- guardian threshold design
- message format design

Without evidence:

- guardian quorum remains provisional

## EV-16: Rent and storage behavior

Question:

What is the storage/rent behavior for long-lived gateway accounts and claim accounts?

Why it matters:

Claim-based flow may create unredeemed claims that persist indefinitely.

Minimum evidence:

- storage account cost model
- long-lived account behavior
- closure behavior
- proof that closing claim does not affect ProcessedBurnEntry
- policy for who pays account creation costs

Required level:

E2/E3.

Unlocks:

- claim-based flow feasibility

Without evidence:

- claim-based flow remains blocked

## Evidence recording format

Each evidence item should be recorded in a dedicated document or section with:

- evidence ID
- date
- branch
- source of evidence
- commands used
- observed result
- pass/fail status
- affected risks
- affected architecture candidates
- remaining uncertainty
- conclusion

Evidence should not be mixed into runtime implementation commits.

Evidence-first commits should remain documentation/prototype-only until the gate is satisfied.

## Prototype-only rules

Prototype-only experiments must follow these rules:

- no production deployment
- no production mint authority
- no live cross-chain minting
- no frontend production flow
- no irreversible user-facing assumptions
- no secret output in logs
- no private key printing
- no RPC key printing
- no admin-like recovery assumptions
- no architecture choice until evidence is documented

Prototype code, if added later, should be clearly separated from production code.

## Architecture impact matrix

| Evidence result | Direct mint impact | Claim-based impact |
| --- | --- | --- |
| Strong transaction/CPI rollback confirmed | Direct mint remains preferred. | Claim-based remains fallback. |
| Transaction atomicity confirmed but token CPI unclear | Direct mint blocked. | Claim redemption blocked. |
| Transaction size too small for direct mint | Direct mint weakened. | Claim-based may become preferred. |
| Compute too high for direct mint | Direct mint weakened. | Claim-based may become preferred. |
| Claim storage/rent unacceptable | No impact. | Claim-based weakened or blocked. |
| Recipient token account handling unsafe | Direct mint blocked. | Claim redemption blocked until solved. |
| Mint authority requires admin discretion | Both blocked. | Both blocked. |
| Processed registry persistence unclear | Both blocked. | Both blocked. |
| Source finality/forks undefined | Source route blocked. | Source route blocked. |

## Current conclusion

Stage 2 should remain in planning and evidence collection.

The repository now has enough design context to know what must not be implemented prematurely.

The next useful step is to collect X1 runtime evidence or create clearly isolated prototype-only experiments for the highest-risk assumptions.

The first evidence targets should be:

1. transaction atomicity
2. account write rollback
3. CPI/token mint rollback
4. token mint authority model
5. deterministic account derivation
6. processed burn registry persistence

Only after those are supported by evidence should the architecture choice between direct mint and claim-based flow become an implementation planning topic.

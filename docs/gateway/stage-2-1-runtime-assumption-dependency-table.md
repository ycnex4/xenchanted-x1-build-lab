# Stage 2.1 Runtime Assumption Dependency Table

## Purpose

Stage 2.1 defines the runtime assumption dependency table for future X1-side gateway planning.

This is a planning-only checkpoint.

It does not implement runtime code, deploy an X1 program, create production mint authority, enable real cross-chain minting, or choose final direct mint / claim-based architecture.

The purpose is to compare each known X1 runtime assumption against both candidate gateway tracks:

- direct mint
- claim-based flow

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 2.0 defined the gateway runtime planning outline.

It identified the next required document as a runtime assumption dependency table comparing direct mint and claim-based flow against concrete X1 runtime facts.

Stage 2.1 provides that table.

The table is intentionally conservative.

Unknown runtime behavior is treated as a blocker for implementation until confirmed.

## Status legend

| Status | Meaning |
| --- | --- |
| Confirmed | The assumption is verified against concrete X1 runtime facts. |
| Unknown | The assumption is not yet verified. |
| Blocker | The assumption must be resolved before implementation. |
| Planning OK | The assumption may remain unresolved during planning, but not during implementation. |
| Production later | The assumption can be refined after prototype planning but before production. |

## Blocker level legend

| Level | Meaning |
| --- | --- |
| Critical | Blocks both direct mint and claim-based implementation. |
| High | Blocks one candidate architecture or creates major safety risk. |
| Medium | Does not block planning but must be resolved before production. |
| Low | Documentation, UX, indexing, or operational refinement. |

## Dependency table

| Runtime assumption | Current status | Direct mint impact | Claim-based impact | Blocker level | Evidence needed | Planned resolution |
| --- | --- | --- | --- | --- | --- | --- |
| Transaction atomicity | Unknown | Direct mint requires processed mark and mint result to commit together or fail together. | Claim creation must commit atomically with processed mark. | Critical | X1 runtime documentation or minimal runtime test proving rollback behavior. | Verify before any implementation. |
| CPI atomicity | Unknown | Direct mint depends on CPI mint being atomic with caller state. | Claim-based flow can reduce first-step CPI reliance, but claim redemption still needs safe mint semantics. | Critical for direct mint, High for claim-based | X1 token CPI behavior test or official runtime confirmation. | Resolve before selecting direct mint. |
| Account write rollback | Unknown | Failed mint must not leave processed entry behind. | Failed claim creation must not leave processed entry behind. | Critical | Runtime test showing state writes rollback on failure. | Required before implementation. |
| Token program interface | Unknown | Direct mint must call a reliable token mint interface. | Claim redemption must call a reliable token mint interface. | Critical | Token program spec, mint instruction interface, authority model. | Required before implementation. |
| Token mint authority model | Unknown | Gateway program must safely control or invoke mint authority. | Claim redemption must safely control or invoke mint authority. | Critical | X1 token authority model and ownership rules. | Required before implementation. |
| Recipient token account creation | Unknown | Direct mint needs clear recipient token account handling. | Claim-based flow can defer recipient account setup to claim step. | High | X1 account creation rules and payer/rent behavior. | Direct mint blocked until clear. |
| Rent/storage model | Unknown | Processed entries and account creation require storage funding rules. | Claim accounts add additional storage/rent complexity. | Critical | X1 rent/storage documentation or test. | Required before implementation. |
| Processed burn registry persistence | Unknown | Registry must be permanent enough to prevent replay forever. | Same requirement; claim accounts do not replace processed registry. | Critical | Persistence guarantee for processed entries. | Required before implementation. |
| Deterministic account derivation | Unknown | Needed for processed entries and predictable account constraints. | Needed for processed entries and claim accounts. | High | X1 PDA/account derivation rules or equivalent. | Resolve before account layout design. |
| Program upgradeability | Unknown | Upgrade must not change replay or mint semantics silently. | Same risk; also must preserve claim account semantics. | Critical | X1 upgrade model and authority controls. | Resolve before implementation gate. |
| Pause authority | Unknown | Pause should block new direct mints without changing history. | Pause rules must define whether claim redemption is blocked or allowed. | High | X1 authority model and intended pause policy. | Define before instruction design. |
| Guardian set account behavior | Unknown | Runtime must load and verify guardian set/version. | Same requirement for claim creation. | High | Account format and signature verification feasibility. | Resolve during account layout planning. |
| Route/coefficient version binding | Unknown | Direct mint amount must use signed route/coefficient version. | Claim amount must use signed route/coefficient version and persist it. | Critical | Message format and config-version account plan. | Required before implementation. |
| Source chain identity | Unknown | Direct mint must reject wrong source route or domain. | Same requirement for claim creation. | High | Chain/domain/source route encoding rules. | Define in runtime message design. |
| Source chain finality | Unknown | Direct mint should not process unstable source events. | Claim creation should not process unstable source events. | High | Finality policy per source route. | Define before production; planning can proceed. |
| Source chain fork handling | Unknown | Must not allow same burn on competing forks to mint twice. | Must not allow same burn on competing forks to create multiple claims. | Critical | Fork-disambiguation strategy and canonicalEventKey fields. | Required before implementation. |
| canonicalEventKey derivation immutability | Unknown | Replay protection depends on stable derivation. | Same requirement; claim accounts also depend on it. | Critical | Frozen derivation spec or migration plan. | Required before implementation. |
| Compute budget | Unknown | Direct mint may fail if verification plus mint exceeds budget. | Claim-based first step may fit more easily; redemption has separate cost. | High | Compute budget measurements or runtime constraints. | Benchmark before architecture choice. |
| Transaction size limits | Unknown | Direct mint may require many accounts/signatures in one transaction. | Claim-based flow may split size across steps. | High | Runtime transaction size limits and account list limits. | Measure before architecture choice. |
| Event/log/error support | Unknown | Needed for auditability and relayer/watcher monitoring. | Needed for claim indexing and user support. | Medium | Runtime event/log/error support. | Resolve before production. |
| Account indexing support | Unknown | Helps audits but not core safety if state is queryable. | More important for claim discovery. | Medium | Indexing/query strategy. | Resolve before production UX. |
| Migration support | Unknown | Needed only if account layout or derivation changes. | More complex with claim accounts. | High | Upgrade/migration strategy or no-migration commitment. | Resolve before implementation readiness gate. |

## Direct mint dependency summary

Direct mint is the simplest candidate only if the runtime can prove strong atomicity.

Direct mint should remain blocked while any of these remain unknown:

- transaction atomicity
- CPI atomicity
- account write rollback
- token program interface
- token mint authority model
- processed burn registry persistence
- route/coefficient version binding
- source chain fork handling
- canonicalEventKey derivation immutability
- compute budget
- transaction size limits

Direct mint should not be selected if the processed mark and mint result can diverge.

## Claim-based dependency summary

Claim-based flow may be safer when CPI or recipient account creation assumptions are unclear.

However, claim-based flow is not automatically safe.

It introduces new dependencies:

- permanent claim account storage
- deterministic claim account derivation
- claim ownership rules
- claim redemption replay protection
- claim indexing
- claim rent/storage payer
- claim lifecycle policy

Claim-based flow should remain blocked while any of these remain unknown:

- transaction atomicity for processed mark plus claim creation
- account write rollback
- processed burn registry persistence
- claim account persistence
- claim ownership rules
- claim redemption mint authority
- canonicalEventKey derivation immutability
- source chain fork handling

## Candidate preference rule

The architecture choice must be evidence-driven.

If X1 provides strong atomic CPI minting, clear recipient token account handling, and manageable compute/transaction limits, direct mint may be preferred for simplicity.

If X1 CPI/account creation behavior remains unclear, claim-based flow should be preferred for planning, but only after claim semantics are fully specified.

If both candidate tracks depend on unresolved critical assumptions, implementation must not begin.

## Minimum evidence before implementation

Before implementation begins, Stage 2 must have evidence for:

1. Transaction rollback behavior.
2. CPI mint behavior.
3. Token program interface.
4. Mint authority model.
5. Processed burn registry persistence.
6. Account derivation model.
7. Rent/storage payer model.
8. Route/coefficient version binding.
9. canonicalEventKey derivation immutability.
10. Source chain fork handling.
11. Compute budget.
12. Transaction size limits.

## Planning outputs required after Stage 2.1

Stage 2.1 does not choose the final architecture.

It prepares the next planning documents:

1. Direct mint candidate runtime design.
2. Claim-based candidate runtime design.
3. Candidate account layout.
4. Candidate instruction layout.
5. Runtime test plan.
6. Implementation readiness gate.

## Current conclusion

Stage 2.1 makes the dependency surface explicit.

Direct mint is clean but requires stronger runtime guarantees.

Claim-based flow may be safer under uncertainty but adds claim-state complexity.

Both tracks remain planning candidates until concrete X1 runtime facts are confirmed.

The next useful step is to draft the direct mint candidate runtime design and the claim-based candidate runtime design as separate comparable planning documents.

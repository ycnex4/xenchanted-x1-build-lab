# Phase 41K.6 B1 — Guardian Quorum Authorization Plan

Status: planning checkpoint
Branch: stage-41k6-guardian-quorum-authorization
Base: main after Phase 41K.5 D3 merge

## Purpose

B1 closes the blocker identified after Phase 41K.5 D2 and D3.

The live consume_gateway_mint mark+mint path proves transport, atomicity, and rollback safety, but it does not yet prove that only guardian-authorized events can enter the mint path.

B1 must integrate guardian quorum and signature authorization into the same atomic production path before the production mint gate can ever be opened.

## Required ordering

Guardian authorization must happen before any state mutation.

Required order:

1. decode ConsumeGatewayMint
2. validate runtime accounts, route, mint, recipient, and processed_event unprocessed
3. verify guardian quorum and signature evidence
4. build execution plan
5. mark_processed_event_atomic
6. witness check
7. guarded SPL mint_to CPI

Rejected order:

1. mark_processed_event_atomic
2. verify guardian quorum
3. mint

Reason: failed authorization should leave processed_event untouched without relying on rollback.

## Current post-D3 state

Already completed:

- Phase 41K.5 D1 and D1.5: atomic mark+mint boundary and SBF harness.
- Phase 41K.5 D2: real production ConsumeGatewayMint path can reach mark+mint under dangerous test gate.
- Phase 41K.5 D3: 12 negative and failure-mode tests merged to main.
- Default production mint gate remains closed.

D3 proves:

- SPL Token CPI failure after processed-event mark rolls back the full transaction.
- Already-consumed processed event is rejected.
- Wrong PDA, account, authority, program, and input cases fail safely.
- Closed-gate default path still returns CpiBoundaryNotReady / 0x8.

## Existing verifier components

The repo already contains model and runtime boundary components for B1.

1. Instructions sysvar live wiring boundary.

- Loads prior Ed25519 precompile instructions from the real instructions sysvar.
- Rejects caller-provided or frontend-provided evidence.
- Currently documents that full handler integration is not yet enabled.

2. Guardian set account loading boundary.

- Loads program-controlled guardian set account.
- Checks PDA, owner, readonly and non-signer flags, discriminator, schema version, active status, threshold, guardian count, and duplicate guardian keys.
- Supports the current fixed maximum guardian count.

3. Ed25519 evidence parsing, extraction, and verification boundaries.

- Parses prior Ed25519 instruction data.
- Extracts signature, public-key, and message byte ranges.
- Establishes native Ed25519 verification evidence from prior precompile instruction execution.

4. Guardian membership boundary.

- Validates that the verified signer public key belongs to the authoritative on-chain guardian set.

5. Guardian quorum authorization boundary.

- Counts only successful membership validations.
- Deduplicates by guardian index and public key.
- Preserves failed attempt errors.
- Enforces threshold.
- Currently remains logical/model-level and does not yet authorize handler execution.

## Layout issue to resolve

Current ConsumeGatewayMint instruction layout is fixed:

- instruction length: 208
- account meta count: 11
- version: 2

The live Ed25519 proof path needs access to the instructions sysvar account. That means B1 cannot be a hidden internal change only. It needs one explicit account-contract decision.

Recommended decision:

Create a B1/V3 consume_gateway_mint runtime layout under a dangerous test gate.

V3 should add:

- account 11: instructions sysvar account, readonly, non-signer

V3 should preserve all existing V2 semantics where possible, but V2 should remain fail-closed and scaffold-safe until B1 is fully proven.

Alternative rejected for now:

Trying to squeeze B1 into the current V2 / 11-account layout.

Reason: relying on caller-provided guardian proof bytes would violate the existing verifier boundary design. B1 should use real prior Ed25519 precompile instructions and the real instructions sysvar.

## Proposed B1 slice plan

### B1A — Layout and account-contract decision

Goal:

- Introduce a guarded B1/V3 account contract or feature-gated extension.
- Add instructions sysvar as required readonly account.
- Keep default production build closed.
- Preserve old V2 tests unless explicitly updated under B1 feature.

Tests:

- Missing instructions sysvar rejects before mark.
- Wrong instructions sysvar account rejects before mark.
- Instructions sysvar writable rejects before mark.
- Instructions sysvar signer rejects before mark.

### B1B — Guardian set authoritative loading in handler path

Goal:

- Replace or supplement ad-hoc GuardianSetAccountView usage with load_phase_41k_2_guardian_set_account_info.
- Build AuthoritativeGuardianSetRef only from program-controlled on-chain data.
- Do not accept caller-provided guardian lists.

Tests:

- Wrong guardian set PDA rejects before mark.
- Inactive or deprecated guardian set rejects before mark.
- Threshold zero rejects before mark.
- Threshold above count rejects before mark.
- Duplicate guardian key rejects before mark.

### B1C — Prior Ed25519 evidence scan in handler path

Goal:

- Use instructions sysvar live wiring boundary.
- Scan strictly prior Ed25519 precompile instructions only.
- Reject current, future, non-Ed25519, or self-referential evidence.
- Extract signer public keys and message hashes from native verified Ed25519 precompile instructions.

Tests:

- No prior Ed25519 instructions rejects before mark.
- Non-Ed25519 prior instruction ignored or rejected.
- Ed25519 instruction after current instruction not accepted.
- Malformed Ed25519 instruction data rejects before mark.

### B1D — Payload binding

Goal:

- Bind Ed25519 message hash to the canonical guardian payload.
- Bind route id, guardian set id, target mint, canonical event key, recipient, amount, source chain id, and source chain weight.
- Avoid trusting a caller-provided hash that is not recomputed from canonical payload.

Open design question:

Current runtime instruction contains a compact mint payload, while earlier guardian payload model includes richer Ethereum source proof fields. B1 must decide whether runtime validates full raw guardian payload or a compact runtime authorization payload derived from the existing ConsumeGatewayMint args.

Recommended conservative direction:

- Do not invent a looser payload.
- First document exact B1 payload binding before writing the authorization gate.

### B1E — Quorum authorization gate before mark

Goal:

- Establish guardian quorum authorization before execution plan and before mark_processed_event_atomic.
- On failure, return before any mutable account changes.
- On success, continue into the already-proven D2/D3 atomic mark+mint path.

Tests:

- Valid quorum reaches mark+mint under dangerous test gate.
- Invalid signature rejects before mark.
- Unknown guardian rejects before mark.
- Threshold not met rejects before mark.
- Duplicate guardian signature counted once.
- Valid M-of-N succeeds even with extra invalid attempts.
- Same signatures over different payload fail.

## Non-goals for B1

B1 does not open the production gate by default.

B1 does not deploy.

B1 does not remove the dangerous test gate.

B1 does not solve watcher or relayer production infrastructure.

B1 does not weaken D3 atomicity assumptions.

## Completion criteria

B1 can be considered complete only when:

- Guardian quorum and signature authorization is wired into the same handler path before mark_processed_event_atomic.
- Valid quorum success path passes under dangerous test gate.
- Invalid quorum, unknown guardian, threshold-not-met, duplicate, and wrong-payload negative tests pass.
- All failures before authorization leave processed_event, SPL mint, recipient token account, recipient balance, and rent payer unchanged.
- Default production build remains closed-gate.
- D2 and D3 tests still pass.

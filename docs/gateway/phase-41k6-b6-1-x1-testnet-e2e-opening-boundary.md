# Phase 41K.6 B6.1 — X1 testnet E2E opening boundary

## Purpose

B6.1 opens the X1 testnet end-to-end rehearsal track after B5 external review closure.

B6.1 is an opening boundary and simulation/dry-run specification.

B6.1 does not deploy.

B6.1 does not submit transactions.

B6.1 does not sign transactions.

B6.1 does not spend SOL.

B6.1 does not access private keys.

B6.1 does not remove the B1C7 compile_error guard.

B6.1 does not weaken the B1C7 feature gate.

B6.1 does not open production gates.

B6.1 does not activate production runtime execution.

## Current main checkpoint

B5 external review closure is merged on main:

77eac0b Merge phase 41K.6 B5 external review closure

B5 is closed with external review confirmation:

- Claude: ACCEPT WITH NOTES, mandatory Q9.1 parity note confirmed closed.
- Theo: APPROVE WITH NOTES, B5 architecturally closed.

## B6.1 opening decision

B6.1 may begin as a no-send/no-sign/no-key/no-SOL/no-submit opening boundary.

B6.1 does not mean testnet submission is allowed.

B6.1 does not mean production activation is allowed.

B6.1 does not mean any compile-time guard may be removed.

B6.1 does not mean any dangerous feature may be enabled.

B6.1 only defines the safe boundary for later B6 work.

## B6.1 required safety boundary

B6.1 must preserve:

- no deploy,
- no transaction submit,
- no signing,
- no SOL spend,
- no private-key access,
- no keypair file printing,
- no seed phrase handling,
- no committed secrets,
- no production activation,
- no B1C7 compile_error removal,
- no B1C7 feature gate weakening,
- no automatic testnet or production gate opening.

## Gate policy

The B1C7 compile_error guard remains intact.

The B1C7 feature gate remains intact.

Any testnet or production-like gate opening remains a separate deliberate operator/project decision.

Gate opening is not an automatic consequence of B6.1.

Gate opening is not an automatic consequence of any later B6 engineering step.

## B6.1 allowed work

B6.1 may document and prepare:

- testnet readiness inventory shape,
- no-send dry-run package flow,
- B5 package input assumptions,
- read-only account inventory requirements,
- read-only processed_event registry check requirements,
- read-only guardian set inventory requirements,
- external signer or operator approval boundary for later phases,
- stop conditions,
- evidence capture requirements,
- B6.1 to B6.2 progression.

## B6.1 forbidden work

B6.1 forbids:

- live transaction submit,
- transaction signing,
- private key handling,
- keypair file loading,
- seed phrase handling,
- SOL spend,
- deploy,
- production activation,
- compile_error guard removal,
- feature gate weakening,
- dangerous feature enabling,
- processed_event marking,
- SPL Token MintTo execution,
- any state mutation on X1 testnet.

## Testnet RPC policy

B6.1 may define future read-only RPC requirements.

B6.1 itself must not require live RPC.

If a later B6 step introduces testnet RPC calls, those calls must be explicitly read-only until a later submit boundary is opened.

Read-only means:

- fetch program id,
- fetch account owner,
- fetch account data,
- fetch mint metadata,
- fetch token account metadata,
- fetch guardian set account metadata,
- fetch processed_event account state,
- simulate locally or prepare no-send packages.

Read-only does not include:

- sendTransaction,
- simulateTransaction with signer side effects,
- requestAirdrop,
- createAccount,
- allocate,
- assign,
- transfer,
- mintTo,
- mark processed_event,
- any instruction submission.

## B5 package inheritance

B6.1 consumes B5 package shape as input.

B6.1 does not redefine B5 fields.

The B5 no-send relayer package remains the source of truth for:

- candidate,
- payload_v2_hash,
- quorum package,
- relayer submission package,
- processed_event,
- route_id,
- mint,
- recipient token account,
- amount,
- guardian_set_id,
- prior evidence instruction count.

## Guardian signature policy

B6.1 does not use live guardian keys.

B6.1 does not request live guardian signatures.

B6.1 may reference mock, fixture, or pre-generated evidence only.

Any transition to live guardian signing requires a later explicit boundary.

## Processed event policy

B6.1 may define read-only processed_event checks.

B6.1 must not mark processed_event.

B6.1 must not allocate processed_event accounts.

B6.1 must not assign processed_event accounts.

B6.1 must not transfer rent for processed_event accounts.

Any processed_event mutation requires a later explicit submit boundary.

## B6 phase progression

B6.1 — opening boundary and simulation/dry-run specification.

B6.2 — testnet readiness inventory.

B6.3 — no-send dry-run package rehearsal.

B6.4 — external signer / operator approval boundary.

B6.5 — explicit testnet submit rehearsal boundary.

B6.6 — outcome observation.

B6.7 — B6 closure.

Only B6.5 or later may discuss actual submission, and only after a separate explicit opening decision.

## B6.1 closure requirements

B6.1 is closed when:

- opening boundary is documented,
- no-send/no-sign/no-key/no-SOL/no-submit boundary is explicit,
- B1C7 compile_error guard preservation is explicit,
- B1C7 feature gate preservation is explicit,
- gate opening policy is explicit,
- simulation/dry-run-only scope is explicit,
- B5 package inheritance is explicit,
- read-only RPC policy is explicit,
- processed_event read-only policy is explicit,
- guardian signature policy is explicit,
- B6 phase progression is explicit,
- full TypeScript tests remain green,
- full xxxl-svm lib tests remain green.

## Updated checkpoint list

✅ B1: guardian quorum authorization

✅ B2: valid quorum live-gated success test

✅ B3: hostile live-gated matrix

✅ B4: activation gate decision / production-readiness boundary

✅ B5: watcher/relayer integration path

✅ B5 external review closure

👉 B6.1: X1 testnet E2E opening boundary

⏭ B6.2: testnet readiness inventory

⏭ B6.3: no-send dry-run package rehearsal

⏭ B6.4: external signer / operator approval boundary

⏭ B6.5: explicit testnet submit rehearsal boundary

⏭ B6.6: outcome observation

⏭ B6.7: B6 closure

## B6.2 testnet readiness inventory

B6.2 defines the testnet readiness inventory required before no-send dry-run package rehearsal.

B6.2 is documented in:

docs/gateway/phase-41k6-b6-2-testnet-readiness-inventory.md

B6.2 remains no-send, no-sign, no-key, no-SOL, no-submit, and no-gate-removal.

B6.2 does not require live RPC.

B6.2 does not mutate X1 testnet state.

B6.2 preserves the B1C7 compile_error guard and feature gate.

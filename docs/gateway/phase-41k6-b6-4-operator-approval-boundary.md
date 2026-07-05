# Phase 41K.6 B6.4 — External signer / operator approval boundary

## Purpose

B6.4 defines the approval boundary required before any later testnet submission rehearsal can be considered.

B6.4 does not approve submission.

B6.4 does not approve signing.

B6.4 does not approve SOL spend.

B6.4 does not approve private-key handling.

B6.4 does not approve deploy.

B6.4 does not remove the B1C7 compile_error guard.

B6.4 does not weaken the B1C7 feature gate.

B6.4 does not open production gates.

B6.4 only defines what must be explicitly decided before a later B6.5 submit boundary may be opened.

## Current main checkpoint

B6.3 no-send dry-run package rehearsal is merged on main:

e4d228e Merge phase 41K.6 B6.3 no-send dry-run package rehearsal

B6.3 proved that the B5 relayer package shape can be rehearsed without RPC, signing, submission, SOL spend, private keys, or gate removal.

## B6.4 scope

B6.4 answers:

What approvals and boundaries must exist before moving from no-send rehearsal toward any explicit testnet submit rehearsal?

B6.4 does not perform that move.

B6.4 is a decision-boundary document.

## Boundary summary

B6.4 preserves:

- no deploy,
- no submit,
- no signing,
- no SOL spend,
- no private-key access,
- no keypair-file loading,
- no seed phrase handling,
- no live guardian key usage,
- no request for live guardian signatures,
- no processed_event mutation,
- no SPL mint mutation,
- no B1C7 compile_error removal,
- no B1C7 feature gate weakening,
- no production activation.

## Approval classes

B6.4 separates approval into five classes:

1. Read-only inventory approval.
2. No-send package approval.
3. Testnet signing approval.
4. Testnet submit approval.
5. Production or production-like activation approval.

Only the first two classes are compatible with B6.4.

The last three classes remain closed in B6.4.

## 1. Read-only inventory approval

Read-only inventory approval means the project may inspect or record non-mutating testnet state.

Allowed examples:

- program id lookup,
- account owner lookup,
- mint metadata lookup,
- token account metadata lookup,
- guardian set metadata lookup,
- processed_event account state lookup,
- route descriptor lookup.

Forbidden examples:

- sendTransaction,
- requestAirdrop,
- transfer,
- createAccount,
- allocate,
- assign,
- initializeAccount,
- initializeMint,
- mintTo,
- mark processed_event.

Read-only inventory approval does not imply testnet submit approval.

## 2. No-send package approval

No-send package approval means the project may prepare a package that could theoretically become a transaction later, while keeping it unsubmitted and unsigned.

Allowed examples:

- assemble B5 relayer package,
- assemble dry-run instruction boundary,
- derive expected accounts,
- compute payload hash,
- compute PDA addresses,
- record expected instruction order,
- record expected evidence instruction count.

Forbidden examples:

- signing,
- broadcasting,
- paying fees,
- loading keypair files,
- requesting live guardian signatures,
- modifying X1 testnet state.

No-send package approval does not imply signing approval.

No-send package approval does not imply submit approval.

## 3. Testnet signing approval

Testnet signing approval remains closed in B6.4.

Opening testnet signing requires a later explicit decision.

That later decision must define:

- signing actor,
- wallet boundary,
- key custody boundary,
- allowed environment,
- allowed transaction class,
- expiry or one-time-use policy,
- log redaction policy,
- stop conditions,
- rollback or abort procedure.

B6.4 does not collect signatures.

B6.4 does not request signatures.

B6.4 does not load signing keys.

B6.4 does not define any key material.

## 4. Testnet submit approval

Testnet submit approval remains closed in B6.4.

Opening testnet submit requires a later explicit B6.5 boundary.

That later boundary must define:

- exact network,
- exact program id,
- exact mint,
- exact route id,
- exact guardian set id,
- exact recipient token account,
- exact amount,
- exact processed_event account,
- exact fee payer boundary,
- exact transaction intent,
- exact success criteria,
- exact failure criteria,
- exact observation plan.

B6.4 does not submit.

B6.4 does not authorize submission.

## 5. Production or production-like activation approval

Production or production-like activation approval remains closed in B6.4.

Production or production-like activation is not an automatic consequence of B6.4.

Production or production-like activation is not an automatic consequence of B6.5.

Production or production-like activation remains a separate deliberate operator/project decision.

## Role vocabulary

B6.4 uses generic role names only:

- project operator,
- relayer operator,
- guardian signer,
- reviewer,
- observer.

B6.4 does not assign personal names.

B6.4 does not create formal governance roles.

B6.4 does not imply that any external person has accepted a protocol role.

## Required B6.5 preconditions

Before B6.5 may open, the project must have an explicit written decision covering:

- testnet-only scope,
- no production activation,
- B1C7 gate handling,
- whether any testnet-only feature gate change is proposed,
- whether signing is allowed,
- whether submission is allowed,
- whether SOL spend is allowed,
- fee payer boundary,
- guardian evidence boundary,
- route boundary,
- mint boundary,
- processed_event mutation boundary,
- abort conditions,
- observation conditions.

If any of these are unknown, B6.5 must not open.

## B1C7 gate policy

The B1C7 compile_error guard remains intact in B6.4.

The B1C7 feature gate remains intact in B6.4.

Any proposed gate change must be documented in a later explicit boundary.

No gate change may be hidden inside a technical refactor.

No gate change may be treated as automatic engineering progress.

## Key and secret policy

B6.4 forbids:

- private keys,
- seed phrases,
- mnemonic phrases,
- keypair-file loading,
- keypair-file paths,
- secret-key logging,
- committed secrets,
- live guardian signing,
- fee-payer signing.

Any future key usage requires a later explicit boundary and must not expose secrets in repo, logs, prompts, or documentation.

## Stop conditions

B6.4 requires a stop if any later step attempts:

- signing before explicit signing approval,
- submission before explicit submit approval,
- SOL spend before explicit SOL approval,
- private-key handling before explicit key boundary,
- gate removal before explicit gate decision,
- production-like activation before explicit operator/project decision,
- route drift from B5 package shape,
- mint drift from B5 package shape,
- guardian set drift from B5 package shape,
- processed_event mutation before explicit submit boundary,
- ambiguous network or RPC result,
- unclear operator responsibility.

## Evidence required for later B6.5 opening

A later B6.5 opening decision must preserve:

- current main commit,
- B6.1 boundary reference,
- B6.2 inventory reference,
- B6.3 no-send package reference,
- B6.4 approval boundary reference,
- exact package hash,
- exact payload hash,
- exact account list,
- exact allowed action list,
- exact forbidden action list,
- explicit statement that production activation is not included.

## B6.4 closure requirements

B6.4 is closed when:

- approval classes are documented,
- signing boundary remains closed,
- submit boundary remains closed,
- SOL boundary remains closed,
- key boundary remains closed,
- production activation boundary remains closed,
- B1C7 gate policy remains explicit,
- stop conditions are documented,
- B6.5 preconditions are documented,
- documentation diff check passes.

## B6.5 entry criteria

B6.5 may only open after B6.4 is merged.

B6.5 target:

explicit testnet submit rehearsal boundary.

B6.5 must begin with a separate written decision.

B6.5 must not be treated as automatic.

B6.5 must still exclude production activation unless a separate later production boundary is opened.

## Updated checkpoint list

✅ B1: guardian quorum authorization

✅ B2: valid quorum live-gated success test

✅ B3: hostile live-gated matrix

✅ B4: activation gate decision / production-readiness boundary

✅ B5: watcher/relayer integration path

✅ B5 external review closure

✅ B6.1: X1 testnet E2E opening boundary

✅ B6.2: testnet readiness inventory

✅ B6.3: no-send dry-run package rehearsal

👉 B6.4: external signer / operator approval boundary

⏭ B6.5: explicit testnet submit rehearsal boundary

⏭ B6.6: outcome observation

⏭ B6.7: B6 closure

## B6.5 pre-submit decision packet

B6.5 requires a separate explicit decision before any testnet submit rehearsal can open.

The pre-submit decision packet is documented in:

docs/gateway/phase-41k6-b6-5-pre-submit-decision-packet.md

Current status:

B6.5 remains closed.

Signing, submission, SOL spend, private-key handling, deploy, B1C7 gate removal, feature gate weakening, and production activation remain not approved.

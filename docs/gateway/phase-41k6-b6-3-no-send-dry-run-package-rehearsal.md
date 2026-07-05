# Phase 41K.6 B6.3 — No-send dry-run package rehearsal

## Purpose

B6.3 rehearses the B5 relayer submission package shape using B6 readiness inventory assumptions.

B6.3 remains no-send.

B6.3 does not deploy.

B6.3 does not submit transactions.

B6.3 does not sign transactions.

B6.3 does not spend SOL.

B6.3 does not access private keys.

B6.3 does not perform live RPC.

B6.3 does not remove the B1C7 compile_error guard.

B6.3 does not weaken the B1C7 feature gate.

B6.3 does not open production gates.

## Current main checkpoint

B6.2 testnet readiness inventory is merged on main:

4fd2121 Merge phase 41K.6 B6.2 testnet readiness inventory

## B6.3 implementation

B6.3 adds a focused TypeScript rehearsal test:

tests/phase41k6_b6_no_send_dry_run_package_rehearsal.test.ts

The test uses the existing B5 package builders:

- buildPhase41K6GatewayMintPayloadV2
- buildPhase41K6RelayerSubmissionPackage

B6.3 does not introduce a new package model.

B6.3 consumes the B5 package shape as-is.

## Canonical dry-run package

The dry-run package uses the known-answer payload vector:

- processed_event: [0xB2; 32]
- route_id: [0x41; 32]
- mint: [0x51; 32]
- recipient token account: [0x61; 32]
- amount: 1_234_567_890
- guardian_set_id: [0xC7; 32]

Expected payload hash:

0x56a318440e188d864052b8518f41deb7e4f998a975e3b6e19ca63815535ec77d

## Rehearsed behavior

B6.3 verifies:

- B5 relayer submission package can be assembled from readiness-style values,
- payload_v2_hash matches the cross-language known-answer vector,
- handler instruction boundary is preserved,
- quorum package remains structurally valid,
- no-send boundary flags are preserved,
- handler-bound field drift after evidence preparation is rejected,
- operational ids remain outside payload hash binding.

## No-send boundary

B6.3 explicitly preserves:

- no live RPC,
- no signing,
- no submit,
- no SOL spend,
- no private keys.

## B6.3 closure requirements

B6.3 is closed when:

- dry-run package rehearsal test is added,
- focused TypeScript test passes,
- TypeScript typecheck passes,
- documentation diff check passes,
- no live RPC/signing/submit/SOL/private-key path is introduced.

## B6.4 entry criteria

B6.4 may start after B6.3 is merged.

B6.4 target:

external signer / operator approval boundary.

B6.4 must remain no-send, no-sign, no-key, no-SOL, no-submit, and no-gate-removal unless a later explicit boundary says otherwise.

# Production Guardian Set Instantiation Plan

Status: COMPLETE after this package is merged.

Package:

- production-guardian-set-instantiation-plan

Approval:

- APPROVE_PRODUCTION_GUARDIAN_SET_INSTANTIATION_PLAN_NO_ACTIVATION

Scope:

- PLANNING_DOCUMENTATION_ONLY
- NO_SOURCE_CHANGE
- NO_GUARDIAN_SET_INSTANTIATION
- NO_BLOCKER_REMOVAL
- NO_ACTIVATION

## Purpose

This package defines the minimum production guardian set model required before requesting a concrete source/config package.

This package is intentionally bounded. It must not create an endless planning chain. Its direct exit condition is:

- request separate approval for `production-guardian-set-instantiation-source-change`.

## Current Preserved Runtime State

| Item | Status |
| --- | --- |
| ExternalReviewIncomplete | REMOVED |
| PlaceholderProgramId | REMOVED |
| LiveRouteDisabled | ACTIVE |
| SplCpiExecutionDisabled | ACTIVE |
| ProductionGuardianSetUnset | ACTIVE |
| ProductionProofLogUnset | ACTIVE |
| runtime_deployable | false |
| predeploy_gate | blocked |
| activation_authorized | false |
| deploy_authorized | false |
| rpc_mutation_authorized | false |
| route_enablement_authorized | false |
| spl_cpi_enablement_authorized | false |
| guardian_set_instantiation_authorized | false |
| blocker_removal_authorized | false |

## 1. Guardian Set Model

### Version

The production guardian set must be versioned.

Recommended field:

- `guardian_set_version: u64`

Initial production version:

- `1`

Versioning is required so future proof messages can bind to the intended guardian set and avoid ambiguity after rotation or replacement.

### Public Key Format

Only public keys / public addresses may be documented or committed.

Forbidden:

- private keys;
- keypair JSON files;
- seed phrases;
- mnemonic material;
- wallet export files.

Recommended guardian public key format:

- canonical Solana public key / Ed25519 public key representation;
- one guardian public key per entry;
- deterministic ordering by canonical byte representation or explicitly documented order.

The next source/config package must provide the exact production public guardian list as public evidence.

### Threshold

The guardian set must define:

- `guardian_count`;
- `threshold`;
- `threshold > 0`;
- `threshold <= guardian_count`.

Threshold selection is not finalized by this planning package. The next source/config package must record the exact threshold and justify it against the selected guardian count.

### Quorum Rule

A gateway message is authorized only if:

1. every counted signature belongs to a known guardian in the active guardian set;
2. duplicate guardians are counted once at most;
3. the number of unique valid guardian signatures is greater than or equal to `threshold`;
4. the signed domain and message digest match the gateway verification domain;
5. the guardian set version in the signed message matches the active guardian set version, if version binding is part of the final message format.

### Duplicate Signer Rule

Duplicate signatures from the same guardian must not increase quorum weight.

Required behavior:

- duplicate guardian detected;
- count once or reject deterministically;
- next source package must choose and test the exact behavior.

Recommended behavior:

- reject duplicate guardian signatures explicitly.

### Unknown Signer Rule

A signature from an unknown guardian must not count toward threshold.

Recommended behavior:

- reject the authorization attempt if any supplied signer is unknown.

Alternative allowed behavior:

- ignore unknown signers if and only if tests prove quorum cannot be inflated.

The next source package must choose one behavior and test it.

### Domain Separation

Guardian signatures must be bound to a specific signing domain.

Minimum required domain components:

- project/protocol identifier;
- target network / chain domain;
- gateway route identifier;
- guardian set version;
- canonical message digest.

The goal is to prevent signatures intended for another route, chain, environment, or guardian set version from authorizing a production mint.

### Rotation / Immutability Policy

Initial production recommendation:

- guardian set is immutable after initialization unless a separately approved guardian-rotation package is created.

Rotation must not be part of this package.

Future rotation, if needed, requires:

- separate planning/review package;
- separate source/config package;
- explicit evidence of old guardian set, new guardian set, threshold, abort rules, and no private key material.

## 2. Storage / Binding Model

### Options Considered

| Option | Description | Assessment |
| --- | --- | --- |
| Source constants | Guardian list hardcoded in source | simple but requires rebuild/redeploy for any change |
| Program-owned PDA/config account | Guardian set stored in a program-owned account | preferred production model |
| External public policy only | Guardian list lives only in docs/off-chain policy | insufficient as runtime source of truth |

### Recommended Production Model

Recommended model:

- program-owned PDA/config account for production guardian set.

Rationale:

- runtime can verify the active guardian set from account data;
- account owner can be checked;
- initialized/version fields can be checked;
- source does not need to hardcode every guardian public key forever;
- future rotation can be handled only through separately approved logic.

### PDA / Account Expectations

The next source package must define exact seeds. Recommended seed shape:

- `b"xxxl"`
- `b"guardian-set"`
- `b"production"`
- version seed or encoded version field

The next source package must decide whether version is part of the PDA seeds or only account data.

Account owner expectation:

- guardian set account must be owned by the XXXL SVM program.

Required fields:

- initialized flag;
- guardian set version;
- threshold;
- guardian count;
- guardian public key list;
- optional authority/freeze flag if the source model needs it;
- reserved bytes only if justified for future compatibility.

### Initialization Policy

The guardian set account must not be considered production-ready unless:

- account exists;
- account owner matches program ID;
- layout deserializes successfully;
- initialized flag is true;
- version is expected;
- threshold is valid;
- guardian list is non-empty;
- no duplicate guardians exist.

## 3. Verification Boundary

The guardian set must be consumed by the gateway authorization / verifier path that checks guardian quorum for burn-to-mint gateway messages.

The next source package must document:

- which function reads guardian set state;
- which function validates guardian signatures;
- how guardian set version is bound to the message;
- how duplicate signers are handled;
- how unknown signers are handled;
- how insufficient quorum is reported;
- which tests prove the locked/default state still remains blocked outside the intended guardian path.

### Resolution Condition for ProductionGuardianSetUnset

`ProductionGuardianSetUnset` may be considered ready for removal only after a separate source/config package proves all of the following:

1. production guardian set model exists in source/config;
2. guardian set account or binding model is implemented;
3. guardian set can be validated deterministically;
4. invalid guardian configurations fail safely;
5. tests cover valid quorum and rejection cases;
6. evidence records public guardian keys, threshold, PDA/account derivation, and no private key material;
7. route enablement remains blocked;
8. SPL CPI enablement remains blocked;
9. activation remains unauthorized.

This planning package does not remove the blocker.

## 4. Hard Abort Rules

Any future guardian set source/config package or live action must abort if any of the following is true:

| Condition | Required Result |
| --- | --- |
| Guardian PDA derivation mismatch | Abort |
| Guardian account owner mismatch | Abort |
| Guardian account missing | Abort |
| Guardian account not initialized | Abort |
| Guardian layout/serialization mismatch | Abort |
| Guardian set version mismatch | Abort |
| Empty guardian list | Abort |
| Duplicate guardian public keys | Abort |
| threshold = 0 | Abort |
| threshold > guardian_count | Abort |
| Missing signer | Abort |
| Unknown signer | Abort or deterministic reject per source design |
| Duplicate signer attempts to inflate quorum | Abort/reject |
| Signature/message domain mismatch | Abort |
| Program ID mismatch | Abort |
| ProgramData mismatch | Abort |
| Upgrade authority mismatch | Abort |
| RPC instability or inconsistent account reads | Abort |
| Any unrelated blocker unexpectedly removed | Abort |
| activation_authorized != false before exact GO | Abort |
| route_enablement_authorized != false before exact GO | Abort |
| spl_cpi_enablement_authorized != false before exact GO | Abort |

## 5. Next Package Shape

The direct follow-up package must be:

- `production-guardian-set-instantiation-source-change`

Purpose:

- implement or bind the production guardian set model defined by this plan;
- prove guardian set validation behavior;
- keep live route disabled;
- keep SPL CPI execution disabled;
- keep activation unauthorized.

### Expected Allowed Areas

The next source package may request changes to:

- guardian set account/model source files;
- gateway verifier / authorization boundary files;
- deployment status / blocker evidence logic only if resolving `ProductionGuardianSetUnset` is explicitly approved;
- tests for guardian set validation;
- docs/gateway/current-runtime-state.md;
- docs/gateway/current-execution-plan.md;
- docs/gateway/evidence/production-guardian-set-instantiation-source-change/.

Exact files must be listed in the next approval request.

### Expected Forbidden Actions

The next source package must still forbid:

- activation;
- deploy;
- upgrade;
- RPC mutation;
- live route enablement;
- SPL CPI enablement;
- proof log instantiation;
- unrelated blocker removal;
- private key or keypair material;
- cleanup outside approved scope.

### Required Tests / Evidence

The next source package must provide evidence for:

- guardian public key list;
- threshold;
- guardian set version;
- PDA/account derivation;
- account owner expectations;
- layout/serialization;
- valid quorum acceptance;
- insufficient quorum rejection;
- duplicate guardian rejection;
- unknown guardian rejection;
- wrong domain rejection;
- wrong guardian set version rejection, if version is message-bound;
- no private key/keypair material;
- source mutation limited to approved files;
- route/CPI/activation still blocked.

### Exit Condition

The next package may close only if it produces an actionable result:

- `ProductionGuardianSetUnset` remains ACTIVE with explanation, or
- `ProductionGuardianSetUnset` is resolved/removed only if explicitly approved and proven by source/tests/evidence.

No exact activation GO may be requested from this planning package alone.

# Blocker D.1 — State initialization design planning

Status:

BLOCKER_D_OPEN_STATE_INITIALIZATION_DESIGN_PLANNING_ONLY_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_D_NOT_CLOSED

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker D.1 opens the state initialization design track.

D.1 is planning-only.

It does not initialize any account.

It does not change runtime code.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate any network.

## Why D follows C

Blocker A is closed narrowly for the current upgrade authority model.

Blocker C is closed narrowly for the B1C7 handler boundary / invariant review.

The next blocker is state initialization because the handler boundary is not useful without a reviewed account initialization model.

## Current repo-grounded state facts

The runtime source currently defines fixed account lengths:

- MINT_STATE_ACCOUNT_LEN = 176
- GATEWAY_CONFIG_ACCOUNT_LEN = 256
- GUARDIAN_SET_ACCOUNT_LEN = 320
- PROCESSED_EVENT_ACCOUNT_LEN = 144
- RECIPIENT_BALANCE_ACCOUNT_LEN = 144

The runtime source also defines fixed discriminators for:

- MintState
- GatewayConfig
- GuardianSet
- ProcessedEvent
- RecipientBalance

The source exposes read-only account views for those layouts.

## Current PDA facts

The current PDA inventory records one explicit PDA derivation:

- name: gateway_mint_authority
- seeds: xxxl / gateway-mint-authority / v1
- depends_on_program_id: true
- purpose: expected SPL Token mint authority for gateway-backed XXXL minting

## Initial state categories

D.1 separates state into categories:

1. Long-lived protocol state
   - MintState
   - GatewayConfig
   - GuardianSet

2. Derived authority state
   - gateway_mint_authority PDA

3. Per-event replay state
   - ProcessedEvent

4. Per-recipient accounting state
   - RecipientBalance

5. SPL token state
   - SPL mint
   - recipient token accounts

D.1 does not close SPL mint authority architecture. That remains Blocker E.

## ProcessedEvent boundary

D.1 records that the legacy pre-41K.4 processed-event helper is not a live initialization model.

The live replay-protection model must preserve the Phase 41K.4 boundary:

SystemOwnedEmpty/SystemOwnedOrProgramOwnedPda -> InitializedConsumed

ProcessedEvent must not be treated as an already-initialized mutable registry unless a later reviewed D step explicitly proves that design.

## D design questions

Blocker D must answer:

- Which accounts are created once during protocol initialization?
- Which accounts are derived PDAs?
- Which accounts are created lazily per burn/event?
- Which accounts are created lazily per recipient?
- Which accounts require rent payer signatures?
- Which accounts must be readonly during ConsumeGatewayMint?
- Which accounts must be writable during ConsumeGatewayMint?
- Which account owners are expected before initialization?
- Which account owners are expected after initialization?
- Which discriminators and layout versions are written at initialization?
- Which fields are immutable after initialization?
- Which fields can change only through gateway-authorized consumption?
- Which fields must never be admin mutable?

## D closure requirements

Before Blocker D can close, the repo must record:

- exact initialization account list
- exact PDA derivation list
- exact account size list
- exact discriminator list
- exact initializer authority model
- exact rent payer model
- exact one-time initialization guard
- exact reinitialization rejection rule
- exact processed-event creation / marking model
- exact recipient-balance creation / initialization model
- exact relationship to SPL mint authority architecture in Blocker E
- exact no-admin-mint / no-admin-balance-write boundary
- exact evidence that D closure does not approve execution

## Non-closure statement

D.1 does not close Blocker D.

D.1 does not approve:

- state initialization execution
- account creation
- PDA creation
- SPL mint setup
- SPL CPI minting
- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- guardian package construction
- transaction submit
- mutation
- production activation

## Result

Current status:

BLOCKER_D_OPEN_STATE_INITIALIZATION_DESIGN_PLANNING_ONLY_NO_RPC_NO_EXECUTION

Current decision:

BLOCKER_D_NOT_CLOSED

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker D.2 — repo-grounded state layout and PDA inventory.

D.2 should inspect tracked repository code only.

D.2 must not initialize state, call RPC, use testnet, sign, deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate.

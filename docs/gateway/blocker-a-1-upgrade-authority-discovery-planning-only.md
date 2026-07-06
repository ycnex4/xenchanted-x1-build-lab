# Blocker A.1 — Upgrade authority discovery planning-only

Status:

BLOCKER_A_OPEN_DISCOVERY_PLANNING_ONLY_NO_RPC_NO_KEYS_NO_EXECUTION

Current decision:

BLOCKER_A_NOT_CLOSED

NO-GO REMAINS FOR TESTNET_RPC_LIVE_RPC_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

## Purpose

Blocker A.1 defines what must be discovered and proven before Blocker A can be closed.

Blocker A is about upgrade authority.

The goal is to establish a safe evidence path for proving the upgrade authority status of the deployed or future deployed program without accidentally performing RPC actions, signing, upgrades, or network submits during planning.

## Planning-only boundary

This step does not call RPC.

This step does not use testnet.

This step does not use live RPC.

This step does not use keys.

This step does not sign.

This step does not run solana program show.

This step does not run solana program deploy.

This step does not run solana program set-upgrade-authority.

This step does not inspect live ProgramData.

This step does not modify state.

This step does not submit to any network.

## What Blocker A must ultimately answer

Blocker A must answer:

- What is the intended upgrade authority model?
- Is the program intended to remain upgradeable during test phases?
- If upgradeable, who or what controls the upgrade authority?
- If immutable, what evidence proves the upgrade authority is none?
- What exact program id is being evaluated?
- What exact cluster or runtime is being evaluated?
- What exact evidence proves the ProgramData upgrade authority state?
- What evidence is local-only planning evidence versus live network evidence?

## Candidate closure states

Blocker A can close only into one explicit state:

1. UPGRADE_AUTHORITY_NONE_CONFIRMED

   The program is immutable for the evaluated deployment and ProgramData shows no upgrade authority.

2. UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

   The program remains upgradeable for an explicitly bounded test phase, with the authority model documented and accepted.

3. UPGRADE_AUTHORITY_PRESENT_AND_NOT_ACCEPTED

   The program remains upgradeable and the authority model is not accepted; Blocker A remains effectively blocking.

No implicit or ambiguous closure is allowed.

## Evidence required before closure

Future Blocker A closure requires evidence for:

- exact program id
- exact network or local runtime
- exact command used for discovery
- exact RPC URL or explicit local validator URL, if discovery uses RPC
- ProgramData address
- upgrade authority value
- whether upgrade authority is none or present
- timestamp or block/slot context where applicable
- who requested the check
- whether the check is local-only, testnet, or mainnet
- whether signing was used
- whether any mutation command was executed

## Commands forbidden in A.1

The following remain forbidden in A.1:

- solana program deploy
- solana program write-buffer
- solana program set-upgrade-authority
- solana program close
- solana program upgrade
- any command using a real keypair
- any command using testnet or live RPC
- any command that signs
- any command that mutates network or local validator state

## Allowed future discovery categories

Future discovery may be separated into:

- A.2 repo-only discovery: inspect repo configs and docs without RPC
- A.3 local-only dry discovery: use local disposable validator only if needed
- A.4 live evidence GO form: explicit user approval before any testnet or live RPC read
- A.5 live read-only evidence: read-only ProgramData inspection only after explicit GO
- A.R Theo review: external review before closure

This A.1 record does not approve any of those actions.

## Relationship to closed Blocker H

Blocker H is already closed only for local-validator health dry-run.

Blocker H closure does not approve Blocker A live discovery.

Blocker H closure does not approve program load, state init, SPL setup, signing, upgrade, or submit.

## Result

Blocker A.1 creates the planning boundary for upgrade authority discovery.

Blocker A remains open.

Current status:

BLOCKER_A_OPEN_DISCOVERY_PLANNING_ONLY_NO_RPC_NO_KEYS_NO_EXECUTION

Current decision:

BLOCKER_A_NOT_CLOSED

NO-GO REMAINS FOR TESTNET_RPC_LIVE_RPC_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

## Next safe step

The next safe step is Blocker A.2 repo-only upgrade authority source discovery.

A.2 should inspect repository files only and must not call RPC or use keys.

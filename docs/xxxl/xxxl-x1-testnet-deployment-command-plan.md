# XXXL X1 Testnet Deployment Command Plan

Status: Completed
Branch: `stage-xxxl-x1-testnet-deployment-command-plan`
Base: `6597891 Add X1 testnet deployment readiness preflight`

## Purpose

This document records the planned command sequence for a future X1 testnet deployment stage.

This is a command plan only.

This is not a deployment record.

This is not execution evidence.

This is not a mainnet readiness record.

This is not a production readiness record.

No RPC was used by this stage.

No program was deployed by this stage.

No transaction was submitted by this stage.

No SOL was spent by this stage.

No private key material is recorded.

No keypair contents are recorded.

No deployment blocker is removed.

## Selected X1 Testnet Program ID

Selected public X1 testnet Program ID candidate:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Local ignored keypair path:

- `.local-keys/xxxl-x1-testnet-program-keypair.json`

The local keypair file exists locally.

The local keypair is ignored through `.git/info/exclude`.

The local public key was checked and matches the selected Program ID candidate.

The keypair contents are not recorded.

The keypair contents are not committed.

## Verified PDA Fixture

PDA name:

- `gateway_mint_authority`

PDA derivation kind:

- `GatewayMintAuthority`

Seeds:

- `xxxl`
- `gateway-mint-authority`
- `v1`

Verified PDA:

- `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`

Verified bump:

- `252`

## Planned Network

Planned X1 testnet RPC:

- `https://rpc.testnet.x1.xyz`

The RPC endpoint must be re-confirmed before actual execution.

This plan must not be treated as proof that the endpoint is currently live.

This plan must not be treated as proof that the endpoint accepts deployment transactions.

## Future Deployment Command Template

The future deployment execution stage should use a command sequence equivalent to the following.

The commands are documented here as a plan only.

They are not executed in this stage.

Step 1 — confirm clean working tree:

- `git switch main`
- `git pull --ff-only origin main`
- `git status --short --untracked-files=all`

Step 2 — confirm local program keypair identity without printing keypair contents:

- `solana-keygen pubkey .local-keys/xxxl-x1-testnet-program-keypair.json`

Expected public key:

- `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

Step 3 — confirm keypair remains ignored:

- `git check-ignore -v .local-keys/xxxl-x1-testnet-program-keypair.json`

Step 4 — confirm no secret files are tracked or untracked:

- search tracked and untracked file names for `.local-keys`, `keypair`, `.json`, and `.env`
- fail the deployment execution stage if any unexpected secret-like file appears

Step 5 — run pre-deploy tests:

- `cd programs/xxxl-svm`
- `cargo test --lib`
- `cargo test pda_fixture_verification --lib -- --nocapture`
- `XXXL_TESTNET_PROGRAM_ID_CANDIDATE=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my cargo test x1_testnet_program_id_candidate_pda_dry_run --lib -- --ignored --nocapture`

Expected test evidence from the latest preflight:

- default Rust library tests: `201 passed`, `0 failed`, `1 ignored`
- PDA fixture verification tests: `6 passed`, `0 failed`
- selected candidate PDA dry-run: `1 passed`, `0 failed`

Step 6 — build the SVM/Solana program artifact:

- build command must be confirmed against the local toolchain before execution
- expected command family: Solana/SBF program build command
- the exact produced `.so` path must be recorded in the execution stage

Step 7 — deploy to X1 testnet using the selected program keypair:

- deployment command must be confirmed against the installed Solana/X1-compatible toolchain before execution
- expected command family: `solana program deploy`
- expected RPC: `https://rpc.testnet.x1.xyz`
- expected program keypair: `.local-keys/xxxl-x1-testnet-program-keypair.json`
- expected deployed Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`

A future deployment command may look like this, but must be checked before execution:

- `solana program deploy <PROGRAM_SO_PATH> --program-id .local-keys/xxxl-x1-testnet-program-keypair.json --url https://rpc.testnet.x1.xyz`

Step 8 — verify deployed Program ID:

- read deployment output
- confirm deployed Program ID equals `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- record transaction signature if produced
- record explorer or RPC evidence if available
- do not record private key material

Step 9 — verify program account:

- query program account through X1 testnet RPC
- confirm the account exists
- confirm the account is executable if the RPC/tooling exposes that field
- confirm account address equals the selected Program ID

Step 10 — record execution evidence:

- command used
- RPC used
- deployed Program ID
- deployment transaction signature
- post-deploy account verification
- SOL spent if measurable
- local git commit
- toolchain versions if available

## Commands Forbidden In This Planning Stage

This planning stage must not execute:

- `solana program deploy`
- any command that sends a transaction
- any command that contacts RPC for deployment
- any command that transfers SOL
- any command that prints keypair contents
- any command that commits `.local-keys`
- any command that commits `.env`
- any command that commits a private key, seed phrase, mnemonic, or signer secret

## Blocker Status

No blocker is removed.

No blocker is transitioned.

`PLACEHOLDER_PROGRAM_ID` remains active.

Remaining active blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Important Boundary

Even after a future successful X1 testnet deployment, the runtime must remain locked unless separate stages explicitly and safely transition blockers.

A testnet deployment alone does not enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian set
- production proof log
- external review closure
- production release readiness

## Required Future Work

Future stages must still record:

1. X1 testnet deployment execution evidence
2. X1 testnet deployed Program ID verification evidence
3. X1 testnet SPL mint authority transfer plan
4. X1 testnet SPL mint authority verification evidence
5. Program ID readiness model update
6. placeholder Program ID blocker transition

The blocker transition must happen only after the required evidence exists.

## Result

The X1 testnet deployment command plan was recorded.

The selected Program ID was re-confirmed against the local public key.

The keypair remains ignored.

No RPC was used.

No program was deployed.

No transaction was submitted.

No SOL was spent.

No secret material was recorded.

No blocker is removed.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

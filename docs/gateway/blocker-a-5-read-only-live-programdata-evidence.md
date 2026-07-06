# Blocker A.5 — Read-only live ProgramData evidence

Status:

READ_ONLY_PROGRAMDATA_EVIDENCE_MATCHED_EXPECTED_AUTHORITY

Current decision:

BLOCKER_A_NOT_CLOSED_PENDING_REVIEW

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker A.5 records read-only live ProgramData evidence for the selected A.3 authority model.

A.5 is the first approved live X1 testnet read-only RPC evidence step for Blocker A.

A.5 does not close Blocker A by itself.

A.5 does not approve signing, upgrade, initialization, SPL setup, guardian package construction, submit, or mutation.

## Approved GO phrase

I approve Blocker A.5 read-only live ProgramData evidence only, scoped to X1 testnet RPC https://rpc.testnet.x1.xyz, program id D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my, ProgramData account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T, and expected observed upgrade authority DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc, using read-only ProgramData inspection only, with no signing, no keypair use, no deploy, no write-buffer, no set-upgrade-authority, no close, no upgrade, no state initialization, no SPL setup, no guardian package construction, and no submit or mutation.

## Scope executed

- read-only RPC inspection only
- rpc_url: https://rpc.testnet.x1.xyz
- program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- expected_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- expected_observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- signing_used: false
- keypair_use: false
- mutation_executed: false
- deploy_executed: false
- write_buffer_executed: false
- set_upgrade_authority_executed: false
- close_executed: false
- upgrade_executed: false
- state_initialization_executed: false
- spl_setup_executed: false
- guardian_package_construction_executed: false
- submit_executed: false

## Commands executed

Read-only command 1:

solana program show D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my --url https://rpc.testnet.x1.xyz

Exit code: 0

Read-only command 2:

solana account D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my --url https://rpc.testnet.x1.xyz --output json

Exit code: 0

No solana config set command was executed.

No command requiring signing was executed.

## Evidence files

- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/metadata.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/solana-version.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-show.command.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-show.stdout.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-show.stderr.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-show.exit-code.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-account.command.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-account.stdout.json
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-account.stderr.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-account.exit-code.txt

## Observed ProgramData evidence

- observed_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- programdata_matches_expected: true
- authority_matches_expected: true

## Observed program account evidence

- program_account_query_exit_code: 0
- program_account_owner: NOT_PARSED
- program_account_executable: NOT_PARSED

## Raw stdout preview

program-show stdout:

```text
Program Id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
Authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
Last Deployed In Slot: 169365249
Data Length: 38584 (0x96b8) bytes
Balance: 0.26974872 SOL
```

program-show stderr:

```text
EMPTY
```

## Interpretation

The read-only ProgramData evidence matches the repo-grounded expected ProgramData account and expected observed upgrade authority.

This supports, but does not by itself close, the A.3 selected authority model:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

Blocker A still requires review before closure.

## Non-closure statement

A.5 does not close Blocker A.

A.5 records read-only evidence only.

A.5 does not approve:

- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- state initialization
- SPL setup
- guardian package construction
- submit
- mutation

## Result

Current status:

READ_ONLY_PROGRAMDATA_EVIDENCE_MATCHED_EXPECTED_AUTHORITY

Current decision:

BLOCKER_A_NOT_CLOSED_PENDING_REVIEW

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

A.5R review package for Theo / closure assessment

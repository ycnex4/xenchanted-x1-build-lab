# Blocker A.4 — Read-only live ProgramData evidence GO form

Status:

BLOCKER_A_OPEN_READ_ONLY_PROGRAMDATA_EVIDENCE_GO_FORM_DEFINED_NO_RPC_NO_EXECUTION

Current decision:

A5_READ_ONLY_LIVE_PROGRAMDATA_EVIDENCE_REQUIRES_EXPLICIT_SCOPED_GO

NO-GO REMAINS_FOR_TESTNET_RPC_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

## Purpose

Blocker A.4 defines the explicit GO form required before A.5 may perform read-only live ProgramData evidence collection.

A.4 does not collect live evidence.

A.4 does not call RPC.

A.4 does not use testnet.

A.4 does not use live RPC.

A.4 does not use keys.

A.4 does not sign.

A.4 does not inspect live ProgramData.

A.4 does not run solana program show.

A.4 does not deploy, upgrade, initialize state, configure SPL, construct guardian packages, or submit to any network.

## A.4 boundary

A.4 is a GO-form definition only.

A.4 records the exact future A.5 read-only scope and the explicit approval phrase that must be provided before A.5 can run.

Generic approval is not sufficient.

Examples that are not sufficient:

- continue
- go ahead
- check it
- run it
- use testnet

## Repo-grounded target for future A.5

Future A.5 target:

- network: X1 testnet
- rpc_url: https://rpc.testnet.x1.xyz
- program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- expected_observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

These values are carried forward from B6.38/A.2/A.3 as public identifiers only.

They are not signing material.

They do not prove fresh live ProgramData state until A.5 runs under explicit scoped GO.

## Future A.5 allowed read-only commands

Future A.5 may use only read-only inspection commands equivalent to:

solana program show "$PROGRAM_ID" --url "$RPC_URL"

Optional read-only account inspection may be used only if needed:

solana account "$PROGRAMDATA_ACCOUNT" --url "$RPC_URL" --output json

A.5 must use explicit --url.

A.5 must not change solana config.

A.5 must not rely on implicit default cluster.

A.5 must not require or print keypair paths.

A.5 must not sign.

## Future A.5 must record

Future A.5 must record:

- exact command used
- exact RPC URL
- exact program id
- observed ProgramData account
- observed upgrade authority
- whether observed ProgramData matches expected ProgramData
- whether observed authority matches expected authority
- whether signing was used
- whether mutation was executed
- command exit status
- raw read-only evidence output path

## Forbidden in A.5 unless separately scoped later

The following remain forbidden:

- solana program deploy
- solana program write-buffer
- solana program set-upgrade-authority
- solana program close
- solana program upgrade
- solana config set
- any signing command
- any keypair use
- any state initialization
- any SPL mint setup
- any SPL CPI minting
- any guardian package construction
- any transaction submit
- any mutation

## Required explicit GO phrase for A.5

A.5 may proceed only if Sergey provides the following explicit scoped GO phrase:

I approve Blocker A.5 read-only live ProgramData evidence only, scoped to X1 testnet RPC https://rpc.testnet.x1.xyz, program id D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my, ProgramData account 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T, and expected observed upgrade authority DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc, using read-only ProgramData inspection only, with no signing, no keypair use, no deploy, no write-buffer, no set-upgrade-authority, no close, no upgrade, no state initialization, no SPL setup, no guardian package construction, and no submit or mutation.

## Sign-off status

A.4 does not contain the sign-off.

A.4 only defines the sign-off phrase.

Current sign-off:

NOT PROVIDED IN A.4

## Relationship to Blocker A

A.4 does not close Blocker A.

A.4 prepares the safe boundary for A.5 read-only live ProgramData evidence.

Blocker A remains open until A.5 evidence and review confirm the selected A.3 authority model.

## Result

Current status:

BLOCKER_A_OPEN_READ_ONLY_PROGRAMDATA_EVIDENCE_GO_FORM_DEFINED_NO_RPC_NO_EXECUTION

Current decision:

A5_READ_ONLY_LIVE_PROGRAMDATA_EVIDENCE_REQUIRES_EXPLICIT_SCOPED_GO

NO-GO REMAINS_FOR_TESTNET_RPC_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_NETWORK_SUBMIT

## Next safe step

The next step may be A.5 only after Sergey provides the exact explicit scoped GO phrase.

Without that phrase, A.5 must not run.

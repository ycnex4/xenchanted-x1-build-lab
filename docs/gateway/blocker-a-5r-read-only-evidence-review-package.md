# Blocker A.5R — Read-only evidence review package

Status:

BLOCKER_A_REVIEW_READY_READ_ONLY_EVIDENCE_MATCHED_EXPECTED_AUTHORITY

Current decision:

BLOCKER_A_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker A.5R reviews the A.5 read-only live ProgramData evidence and prepares the Blocker A closure assessment.

A.5R does not call RPC.

A.5R does not use testnet.

A.5R does not use live RPC.

A.5R does not use keys.

A.5R does not sign.

A.5R does not inspect live ProgramData.

A.5R does not deploy, upgrade, initialize state, configure SPL, construct guardian packages, or submit to any network.

## Reviewed evidence

A.5R reviews:

- docs/gateway/blocker-a-5-read-only-live-programdata-evidence.md
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/metadata.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-show.stdout.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-show.exit-code.txt
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-account.stdout.json
- docs/gateway/evidence/blocker-a-5-read-only-live-programdata-evidence/program-account.exit-code.txt

## A.5 evidence result

A.5 recorded:

- evidence_status: READ_ONLY_PROGRAMDATA_EVIDENCE_MATCHED_EXPECTED_AUTHORITY
- rpc_url: https://rpc.testnet.x1.xyz
- program_id: D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
- observed_programdata_account: 9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
- observed_upgrade_authority: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
- programdata_matches_expected: true
- authority_matches_expected: true
- program_show_exit_code: 0
- program_account_exit_code: 0
- signing_used: false
- mutation_executed: false

## Authority model relationship

A.3 selected the authority model:

TEMPORARY_UPGRADEABLE_STAGED_FINALIZATION

Candidate Blocker A closure state:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

A.5 evidence supports this candidate closure state because the observed live upgrade authority matches the repo-grounded expected public authority.

This does not mean the program is immutable.

This does not mean upgrade authority is removed.

This means the selected temporary staged-finalization authority model is consistent with the observed live ProgramData evidence.

## Safety interpretation

The A.5 evidence supports the following narrow claims:

- the expected ProgramData account was observed
- the expected upgrade authority was observed
- read-only evidence command exited successfully
- no signing was used
- no mutation was executed

The A.5 evidence does not support these claims:

- runtime is deployable
- Program ID placeholder boundary is resolved in current source
- live route is enabled
- SPL CPI execution is enabled
- guardian descriptor is complete
- ProgramData hash blocker is closed
- rollback blocker is closed
- testnet upgrade is approved
- testnet state initialization is approved
- SPL mint setup is approved

## Remaining blocker separation

If Blocker A closes after review, only the authority model blocker closes.

The following remain separate blockers:

- B — expected post-upgrade ProgramData hash
- C — B1C7 handler production/testnet boundary
- D — state initialization design
- E — SPL mint authority architecture
- F — guardian descriptor
- G — rollback / recovery plan

Blocker A closure must not be interpreted as approval for any mutation or upgrade.

## Raw evidence preview

Solana CLI version:

```text
solana-cli 4.0.0 (src:8de42dc0; feat:dda54cf7, client:Agave)
```

A.5 metadata:

```text
phase=blocker-a-5-read-only-live-programdata-evidence
timestamp_utc=2026-07-06T16:36:33Z
rpc_url=https://rpc.testnet.x1.xyz
program_id=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
expected_programdata=9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T
expected_upgrade_authority=DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
signing_used=false
mutation_executed=false
commands_are_read_only=true
```

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

program-account stdout preview:

```json
{
  "pubkey": "D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my",
  "account": {
    "lamports": 1141440,
    "data": [
      "AgAAAIQpps1jS/5OJ2LdCzOlLJobo6Np9ZQpEt4gkoWHFAkO",
      "base64"
    ],
    "owner": "BPFLoaderUpgradeab1e11111111111111111111111",
    "executable": true,
    "rentEpoch": 18446744073709551615,
    "space": 36
  }
}
```

## Review question

Should Blocker A be closed as:

UPGRADE_AUTHORITY_PRESENT_BUT_ACCEPTED_FOR_TEST_PHASE

based on:

- A.1 planning boundary
- A.2 repo-grounded reconciliation
- A.3 temporary staged-finalization authority model
- A.4 explicit read-only evidence GO form
- A.5 matched live read-only ProgramData evidence

## Recommended closure statement if accepted

If accepted, the closure statement should be narrow:

Blocker A is closed only for the current X1 testnet ProgramData authority model, where upgrade authority is present and accepted for a bounded staged-finalization test phase.

This closure does not approve signing, upgrade, state initialization, SPL setup, guardian package construction, submit, or mutation.

## Result

Current status:

BLOCKER_A_REVIEW_READY_READ_ONLY_EVIDENCE_MATCHED_EXPECTED_AUTHORITY

Current decision:

BLOCKER_A_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

External review / closure decision for Blocker A.

Do not proceed to Blocker B/C/D/E/F/G mutation-related work as if Blocker A is closed until the closure decision is recorded.

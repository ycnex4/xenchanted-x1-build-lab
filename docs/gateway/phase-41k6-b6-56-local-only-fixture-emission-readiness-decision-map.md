# Phase 41K.6 B6.56 — Local-only fixture emission readiness decision map

Status:

LOCAL_ONLY_FIXTURE_EMISSION_READINESS_DECISION_MAP_READY_FOR_GO_FORM_NOT_EXECUTION

Current decision:

NO-GO

## Purpose

This checkpoint defines the readiness decision map for future local-only fixture file emission.

It prepares a safe path from in-memory local skeletons to a future local fixture bundle on disk.

This checkpoint is docs-only.

It does not implement fixture file emission.

It does not write fixture files.

It does not create an output directory.

It does not run a local validator.

It does not build, deploy, upgrade, sign, submit, spend SOL, initialize testnet accounts, configure SPL mint authority, perform SPL CPI minting, construct guardian packages, or rehearse live submit flow.

## Why B6.56 exists

B6.55 consolidated the guardian local safety lane and intentionally stopped at a decision boundary.

The selected next lane is local-only fixture emission readiness.

This is the correct next step because:

- the local fixture generator skeleton exists
- the local fixture file emitter skeleton exists
- the local guardian descriptor skeleton exists
- the local guardian fixture integration skeleton exists
- the local guardian failure matrix skeleton exists
- the fixture file inventory has already been defined
- the emitter is still non-writing
- local-validator execution is still not approved
- testnet action is still not approved

B6.56 does not add another automatic guardian skeleton.

It defines whether the existing skeleton layer is ready for a future explicit local-only fixture emission GO form.

## Current readiness answer

Readiness question:

Can the project prepare a future local-only fixture emission GO form without approving fixture emission now?

Answer:

YES.

But only as a readiness map.

B6.56 does not approve actual emission.

B6.56 does not close blocker H.

B6.56 does not approve local-validator execution.

Current decision remains:

NO-GO.

## Existing inputs

This readiness map relies on:

- B6.42 local fixture file emission plan
- B6.43 local fixture file emitter skeleton
- B6.44 local fixture file emitter safety checkpoint
- B6.48 local guardian descriptor skeleton
- B6.51 local guardian fixture integration skeleton
- B6.52 local guardian fixture integration safety checkpoint
- B6.53 local guardian failure matrix integration map
- B6.54 local guardian failure matrix skeleton
- B6.55 guardian local safety lane consolidation

## Future fixture output directory

Selected future local-only output directory:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

This directory is selected because it is:

- relative
- disposable
- local-only
- already aligned with the current emitter skeleton tests
- not a testnet deployment directory
- not a production directory
- not a key-material directory
- not a source-code directory

B6.56 does not create this directory.

## Future fixture files

A future local-only fixture emission step may create exactly these files:

1. manifest.json
2. accounts.json
3. instructions.json
4. scenarios.json
5. expected-snapshots.json
6. failure-matrix.json
7. mutation-invariance.json
8. logs.json
9. safety-report.json
10. README.local-only.txt

No other file is approved by B6.56.

No file is emitted by B6.56.

## File purpose map

### manifest.json

Purpose:

Record fixture set identity, local-only status, deterministic fixture ids, scenario ids, failure ids, mutation-invariance ids, and safety report id.

Required decision flags:

- local_only: true
- testnet_allowed: false
- live_rpc_allowed: false
- production_keys_allowed: false

### accounts.json

Purpose:

Record deterministic local account fixtures.

Required account groups:

- local program fixture
- local gateway_config fixture
- local guardian_set fixture
- local mint_state fixture
- local processed_event fixture
- local SPL mint fixture
- local recipient owner fixture
- local recipient token account fixture
- local token program fixture
- local system program fixture

No production account is allowed.

No testnet account is allowed.

### instructions.json

Purpose:

Record deterministic local instruction fixtures for the future local dry-run boundary.

Allowed instruction fixture ids:

- initialize_gateway_config
- initialize_guardian_set
- initialize_mint_state
- consume_gateway_mint

No instruction may imply live submit.

No instruction may imply testnet submit.

### scenarios.json

Purpose:

Record local-only success and failure scenario references.

Required scenario groups:

- one local success scenario
- local failure scenarios
- account-order references
- before snapshot references
- after snapshot references
- expected result labels
- expected error labels for failures
- mutation-invariance references for failures

### expected-snapshots.json

Purpose:

Record deterministic expected before and after snapshot references.

Required invariant:

Failure scenario after snapshots must match before snapshots for mutable accounts.

### failure-matrix.json

Purpose:

Record failure cases and their expected no-mutation behavior.

Required fields:

- failure_case_id
- category
- scenario_id
- expected_error_label
- expected_no_mutation
- mutable_account_ids

Required invariant:

Every failure case must require no mutation unless a later explicitly approved design changes this rule.

### mutation-invariance.json

Purpose:

Record how failure scenarios prove no mutation.

Required comparison mode:

byte_identical

### logs.json

Purpose:

Record expected local-only log labels.

Required groups:

- instruction decode labels
- account validation labels
- route validation labels
- guardian validation labels
- replay validation labels
- mint planning labels
- success labels
- failure labels
- forbidden text patterns

Logs must not include private material, endpoint material, or command material.

### safety-report.json

Purpose:

Record local-only safety checks.

Required pass conditions:

- local_only: true
- testnet_allowed: false
- live_rpc_detected: false
- production_keys_detected: false
- key_material_paths_detected: false
- private_material_detected: false
- submit_commands_detected: false
- deploy_commands_detected: false
- upgrade_commands_detected: false
- result: PASS

### README.local-only.txt

Purpose:

Warn that the fixture bundle is local-only, disposable, not for production, not for testnet, not signing material, and not an execution approval.

## Naming policy

Future emitted file names must be:

- lowercase
- stable
- deterministic
- exact-match against the approved 10-file inventory
- JSON for structured fixture files
- README.local-only.txt for human warning text

Future fixture set ids must be:

- local-only
- deterministic
- snake_case
- not empty
- not containing unsafe environment words
- not containing endpoint material
- not containing signing material
- not containing command material

Recommended future fixture set id:

phase_41k6_b6_local_only_fixture_set_001

Recommended future fixture set display name:

phase 41k6 b6 local only fixture set 001

Recommended future deterministic seed label:

phase 41k6 b6 local only deterministic seed 001

B6.56 does not instantiate or emit this fixture set.

## Determinism policy

A future emitter must be deterministic.

The same fixture identity and deterministic seed must produce byte-identical emitted fixture files.

A future emission command must preserve enough evidence to verify:

- commit used
- branch used
- output directory used
- exact allowed file list
- focused tests run
- safety checks passed
- generated file count
- generated file names
- source tree cleanliness after emission

## Safety checks required before future emission

A future emission step must abort if any of the following is detected:

- non-relative output directory
- output directory containing parent traversal
- output directory outside the approved disposable local area
- missing fixture_set_id
- missing deterministic seed label
- unexpected file name
- missing expected file
- forbidden endpoint material
- forbidden signing material
- forbidden command material
- production account material
- testnet account material
- live RPC material
- submit command material
- deploy command material
- upgrade command material
- non-PASS safety report
- dirty source tree before emission
- dirty source tree after cleanup, except explicitly generated disposable fixtures during the emission review window

## Cleanup policy

Generated local-only fixture files must be safe to delete.

Future cleanup may remove only:

tmp/local-validator-fixtures/phase-41k6-b6-local-only

Cleanup must not remove:

- docs
- source files
- program files
- checkpoint files
- git metadata
- command logs under /tmp unless explicitly requested
- safety summary evidence unless explicitly requested

A future cleanup command requires its own scoped command boundary if it removes emitted files.

## Explicit GO definition

Generic continuation is not sufficient.

A future local-only fixture emission GO must explicitly state:

- phase id
- repository path
- branch
- base commit
- output directory
- allowed file list
- fixture set id
- deterministic seed policy
- focused test command
- safety scan rules
- cleanup policy
- explicit statement that only local fixture files may be written
- explicit statement that local-validator execution is not approved
- explicit statement that testnet action is not approved
- explicit statement that signing is not approved
- explicit statement that guardian package construction is not approved
- explicit statement that submit is not approved

Recommended future phrase:

I approve B6.57 local-only fixture emission GO form preparation only. I do not approve actual fixture emission yet.

Actual fixture emission requires a later, separate explicit GO.

## Readiness matrix

| Area | B6.56 readiness state | Execution approved |
| --- | --- | --- |
| Fixture file inventory | Defined | No |
| Output directory | Selected | No |
| Naming policy | Defined | No |
| Deterministic seed policy | Defined for future GO | No |
| Safety checks | Defined | No |
| Cleanup policy | Defined | No |
| Explicit GO boundary | Defined | No |
| Fixture file emission implementation | Not approved | No |
| Output directory creation | Not approved | No |
| Local-validator execution | Not approved | No |
| Testnet action | Not approved | No |
| Signing | Not approved | No |
| Guardian package construction | Not approved | No |
| SPL mint authority setup | Not approved | No |
| SPL CPI minting | Not approved | No |
| Upgrade/state init/submit | Not approved | No |

## Blocker status

Blocker H:

local validator dry-run

Current state:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Reason:

B6.56 defines fixture emission readiness only. It does not emit fixtures and does not execute a local validator dry-run.

Blocker E:

SPL mint authority architecture

Current state:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Reason:

B6.56 does not configure SPL mint authority and does not approve SPL CPI minting.

Blocker F:

guardian set testnet descriptor

Current state:

OPEN_DESIGN_STARTED

Closure state:

NOT CLOSED

Reason:

B6.56 does not create a testnet guardian descriptor, does not construct guardian packages, and does not enable signing.

No GO blocker is closed by this checkpoint.

## Explicit non-approval

B6.56 does not approve fixture file emission.

B6.56 does not approve output directory creation.

B6.56 does not approve local-validator execution.

B6.56 does not approve testnet action.

B6.56 does not approve live RPC usage.

B6.56 does not approve signing.

B6.56 does not approve guardian package construction.

B6.56 does not approve descriptor file creation.

B6.56 does not approve SPL mint authority setup.

B6.56 does not approve SPL CPI minting.

B6.56 does not approve upgrade.

B6.56 does not approve state initialization.

B6.56 does not approve submit.

Current decision remains:

NO-GO.

## Next safe step

The next safe step is:

B6.57 local-only fixture emission GO form.

B6.57 should still be unapproved unless Sergey gives a separate explicit scoped GO.

No fixture files are emitted by B6.56.

No local-validator execution is approved by B6.56.

No testnet action is approved by B6.56.

Current decision remains:

NO-GO.

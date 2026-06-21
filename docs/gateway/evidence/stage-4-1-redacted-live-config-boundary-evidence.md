# Stage 4.1 Redacted Live Config Boundary Evidence

This document records Stage 4.1 redacted live config boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-1-redacted-live-config-boundary

Runtime commit:

    24e0246 Add Stage 4.1 redacted live config boundary

Base runtime commit:

    76478a7 Add Stage 3.10 final closure boundary

## Stage position

Stage 1 is closed as the deterministic model layer.

Stage 2 is closed as the runtime / evidence layer.

Stage 3 is closed as the offline tooling / production surface layer.

Theo confirmed that Stage 4 is not Stage 3.11.

Stage 4 is the live runtime / operations layer.

Stage 4.0 defined the live runtime charter.

Stage 4.1 adds the redacted live config boundary.

## Scope

Stage 4.1 defines live config schema, validation, and redaction before any live RPC access is introduced.

Stage 4.1 is config-only.

Stage 4.1 does not call RPC.

Stage 4.1 does not load a wallet.

Stage 4.1 does not submit transactions.

Stage 4.1 does not sign anything.

Stage 4.1 does not spend SOL.

Stage 4.1 does not start watcher loops.

Stage 4.1 does not start relayer loops.

Stage 4.1 does not deploy anything.

Stage 4.1 exists to prove that live configuration can be parsed, classified, validated, and redacted safely before Stage 4.2 introduces read-only RPC connectivity.

## Runtime changes

New helper:

    tests/helpers/stage4RedactedLiveConfigPrototype.ts

New test:

    tests/stage4_redacted_live_config_boundary.test.ts

## New config mode model

New type:

    Stage4LiveConfigMode

Modes:

    read_only
    dry_run
    live_send

Stage 4.1 accepts:

    read_only
    dry_run

Stage 4.1 rejects:

    live_send

Reason:

Stage 4.1 is config-only and must not open any live-send boundary.

## New config artifact type

New type:

    Stage4RedactedLiveConfig

Artifact type:

    stage4_redacted_live_config

Schema version:

    1

Stage:

    4.1

Execution mode:

    config_only_no_rpc

Fields:

    networkName
    rpcUrl
    programId
    payerPublicKey
    mode
    walletPath
    guardianPublicKeys

## New public redacted view type

New type:

    Stage4RedactedLiveConfigPublicView

Artifact type:

    stage4_redacted_live_config_public_view

Schema version:

    1

Stage:

    4.1

Execution mode:

    config_only_no_rpc

Redacted fields:

    rpcUrl: <redacted:rpc_url>
    walletPath: <redacted:wallet_path>

The public view preserves:

    networkName
    programId
    payerPublicKey
    mode
    guardianPublicKeyCount

The public view does not print:

    raw RPC URL
    wallet path
    private key
    mnemonic
    seed phrase
    wallet JSON
    RPC API key
    guardian private key
    deployer private key

## New env shape

New type:

    Stage4RedactedLiveConfigEnv

Supported env keys:

    STAGE4_NETWORK_NAME
    STAGE4_RPC_URL
    STAGE4_PROGRAM_ID
    STAGE4_PAYER_PUBLIC_KEY
    STAGE4_MODE
    STAGE4_WALLET_PATH
    STAGE4_GUARDIAN_PUBLIC_KEYS

## New error type

New class:

    Stage4RedactedLiveConfigError

New reason type:

    Stage4RedactedLiveConfigErrorReason

Reasons:

    missing_env_key
    invalid_network_name
    invalid_rpc_url
    invalid_program_id
    invalid_payer_public_key
    invalid_mode
    invalid_wallet_path
    invalid_guardian_public_keys
    forbidden_secret_value
    live_send_not_allowed

## New helpers

Config parser:

    parseStage4RedactedLiveConfigEnvPrototype

Public redaction helper:

    redactStage4LiveConfigPrototype

Config checker:

    checkStage4RedactedLiveConfigPrototype

## Validation rules

Stage 4.1 validates:

- required env keys are present
- networkName is non-empty and safe
- rpcUrl is HTTPS-shaped
- programId is public-key-like
- payerPublicKey is public-key-like
- mode is read_only or dry_run
- live_send is rejected
- walletPath is optional
- walletPath does not contain path traversal
- guardianPublicKeys is optional
- guardianPublicKeys must be public-key-like when present
- secret-bearing marker values are rejected case-insensitively

## Secret boundary

Stage 4.1 rejects values containing forbidden key/value markers such as:

- PRIVATE_KEY
- MNEMONIC
- SEED_PHRASE
- SECRET_KEY
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY
- secretKey
- privateKey
- mnemonic
- seed phrase
- seed_phrase
- wallet json
- wallet.json
- bearer
- api_key
- rpc_api_key
- guardianSigners

These strings are defensive markers only.

No real secret values are introduced.

## Fix during implementation

Initial Stage 4.1 test run exposed a case-sensitivity bug in forbidden marker matching.

Problem:

- value was lower-cased
- marker was not lower-cased
- contains-privateKey-marker did not match after the value became contains-privatekey-marker

Fix:

- marker comparison is now case-insensitive for both forbidden key markers and forbidden value markers

After the fix, Stage 4.1 tests passed.

## Successful config parse test

Confirmed behavior:

- parses a read-only live config shape
- executionMode is config_only_no_rpc
- no RPC call is made
- no wallet is loaded
- networkName is preserved
- rpcUrl is preserved internally
- programId is preserved
- payerPublicKey is preserved
- mode is read_only
- walletPath is preserved internally
- guardianPublicKeys are parsed
- checkStage4RedactedLiveConfigPrototype returns true

Known public X1 testnet values used in the test:

    RPC: https://rpc.testnet.x1.xyz
    Program id: 9tCJe4M1MJQtE1gDxNYNE75fNUGpSAKiX56rgUMR8984
    Payer public key: DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

These are public/non-secret values.

## Redacted public view test

Confirmed behavior:

- creates a public redacted config view
- rpcUrl is replaced by <redacted:rpc_url>
- walletPath is replaced by <redacted:wallet_path>
- guardian keys are represented only by guardianPublicKeyCount
- stable public JSON does not contain wallet path
- stable public JSON does not contain secret-bearing markers

Secret-bearing field checks:

- PRIVATE_KEY is absent
- MNEMONIC is absent
- SECRET_KEY is absent
- RPC_API_KEY is absent
- GUARDIAN_PRIVATE_KEY is absent
- DEPLOYER_PRIVATE_KEY is absent

## Rejection test

Confirmed behavior:

- missing STAGE4_RPC_URL is rejected as missing_env_key
- malformed networkName is rejected as invalid_network_name
- non-HTTPS RPC URL is rejected as invalid_rpc_url
- malformed programId is rejected as invalid_program_id
- malformed payerPublicKey is rejected as invalid_payer_public_key
- malformed mode is rejected as invalid_mode
- live_send mode is rejected as live_send_not_allowed
- wallet path traversal is rejected as invalid_wallet_path
- RPC URL containing RPC_API_KEY marker is rejected as forbidden_secret_value
- wallet path containing privateKey marker is rejected as forbidden_secret_value
- malformed guardian public key list is rejected as invalid_guardian_public_keys

## Stage 4.1 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts

Final result:

    Stage 4.1 redacted live config boundary
      ✔ parses a read-only live config shape without RPC calls or wallet loading
      ✔ creates a redacted public view that never prints live config secrets or wallet paths
      ✔ rejects missing, malformed, live-send, and forbidden secret-bearing config values

    3 passing

## Stage 3.10 plus Stage 4.1 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts

Final result:

    Stage 3.10 final closure boundary
      ✔ creates a final Stage 3 closure boundary from Stage 3.1 through Stage 3.9 evidence
      ✔ rejects missing, duplicate, unordered, unclosed, or non-offline evidence entries
      ✔ rejects malformed closure metadata, failed invariants, and forbidden values

    Stage 4.1 redacted live config boundary
      ✔ parses a read-only live config shape without RPC calls or wallet loading
      ✔ creates a redacted public view that never prints live config secrets or wallet paths
      ✔ rejects missing, malformed, live-send, and forbidden secret-bearing config values

    6 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No secret-like material was introduced.

## Stage 4.1 boundary classification

Stage 4.1 is:

    config-only
    no-RPC
    no-wallet-loading
    no-transaction
    no-signing
    no-SOL-spend

Stage 4.1 is not:

    read-only RPC connectivity
    watcher runtime
    relayer runtime
    dry-run transaction construction
    live-send
    deployment

## Current conclusion

Stage 4.1 establishes the redacted live config boundary.

It proves that live configuration can be parsed, validated, classified, and redacted before any RPC call, wallet loading, signing, transaction submission, or SOL-spending path is introduced.

The next valid stage is Stage 4.2 read-only RPC connectivity boundary.

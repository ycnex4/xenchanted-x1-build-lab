# Stage 4.2 Read-Only RPC Connectivity Boundary Evidence

This document records Stage 4.2 read-only RPC connectivity boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-2-read-only-rpc-connectivity-boundary

Runtime commit:

    eb6ef26 Add Stage 4.2 read-only RPC connectivity boundary

Base runtime commit:

    24e0246 Add Stage 4.1 redacted live config boundary

## Stage position

Stage 1 is closed as the deterministic model layer.

Stage 2 is closed as the runtime / evidence layer.

Stage 3 is closed as the offline tooling / production surface layer.

Stage 4.0 defined the live runtime charter.

Stage 4.1 established the redacted live config boundary.

Stage 4.2 adds the read-only RPC connectivity boundary.

## Scope

Stage 4.2 defines the first RPC-capable boundary.

It is read-only.

It does not load a wallet.

It does not sign anything.

It does not submit transactions.

It does not spend SOL.

It does not start watcher loops.

It does not start relayer loops.

It does not deploy anything.

The runtime tests use an injected transport recorder, not a live network call.

The boundary proves that only explicitly allowed read-only RPC methods can be used and that public output remains redacted.

## Runtime changes

New helper:

    tests/helpers/stage4ReadOnlyRpcConnectivityPrototype.ts

New test:

    tests/stage4_read_only_rpc_connectivity_boundary.test.ts

## Dependency on Stage 4.1

Stage 4.2 consumes the Stage 4.1 redacted live config shape.

Accepted config mode:

    read_only

Rejected config modes:

    dry_run
    live_send

Reason:

Stage 4.2 is read-only RPC connectivity only.

Dry-run transaction construction belongs to a later stage.

Live-send belongs to a later explicitly reviewed boundary.

## Allowed read-only methods

New type:

    Stage4ReadOnlyRpcMethod

Allowed methods:

    getHealth
    getVersion
    getAccountInfo
    getBalance

Non-read-only methods are rejected.

Example rejected method:

    sendTransaction

## New RPC check model

New type:

    Stage4ReadOnlyRpcCheckCode

Check codes:

    rpc_health
    rpc_version
    program_account_read
    payer_balance_read

New type:

    Stage4ReadOnlyRpcCheck

Fields:

    code
    method
    ok
    httpStatus
    hasResult
    errorMessage

## New transport model

New type:

    Stage4ReadOnlyRpcTransportRequest

Fields:

    rpcUrl
    method
    params
    id

New type:

    Stage4ReadOnlyRpcTransportResponse

Fields:

    httpStatus
    ok
    hasResult
    errorMessage

New type:

    Stage4ReadOnlyRpcTransport

The transport is injected.

This keeps the boundary testable without forcing live RPC calls.

## New result artifact

New type:

    Stage4ReadOnlyRpcConnectivityResult

Artifact type:

    stage4_read_only_rpc_connectivity_result

Schema version:

    1

Stage:

    4.2

Execution mode:

    read_only_rpc_no_wallet

Fields:

    checkedAtIso
    networkName
    rpcUrl
    programId
    payerPublicKey
    mode
    redactedConfig
    checks
    invariants
    ok

The result stores:

    rpcUrl: <redacted:rpc_url>

The result preserves public values:

    networkName
    programId
    payerPublicKey

The result does not print:

    wallet path
    private key
    mnemonic
    seed phrase
    wallet JSON
    RPC API key
    guardian private key
    deployer private key

## Invariants

Stage 4.2 result invariants:

    noWalletLoaded: true
    noSigning: true
    noTransactions: true
    noSolSpend: true
    readOnlyRpcOnly: true

All invariants must remain true.

## New error type

New class:

    Stage4ReadOnlyRpcConnectivityError

New reason type:

    Stage4ReadOnlyRpcConnectivityErrorReason

Reasons:

    invalid_checked_at_iso
    invalid_config
    live_send_not_allowed
    forbidden_config_value
    invalid_rpc_method
    rpc_transport_failed

## New helpers

Method assertion helper:

    assertStage4ReadOnlyRpcMethodPrototype

Connectivity runner:

    runStage4ReadOnlyRpcConnectivityPrototype

Fetch transport factory:

    createStage4FetchReadOnlyRpcTransportPrototype

Result checker:

    checkStage4ReadOnlyRpcConnectivityResultPrototype

## Secret boundary

Stage 4.2 rejects values containing forbidden markers such as:

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

## Successful read-only connectivity boundary test

Confirmed behavior:

- runs only read-only RPC checks
- uses injected transport
- does not load wallet
- does not sign
- does not submit transactions
- does not spend SOL
- calls only getHealth, getVersion, getAccountInfo, and getBalance
- preserves networkName
- preserves programId
- preserves payerPublicKey
- redacts rpcUrl in result
- redacts walletPath in public config view
- stores guardian keys only as guardianPublicKeyCount
- all checks are ok
- all invariants are true
- checkStage4ReadOnlyRpcConnectivityResultPrototype returns true

## Redacted result JSON test

Confirmed behavior:

- public result JSON does not contain wallet path
- public result JSON does not contain secret-bearing markers
- public result JSON does not contain sendTransaction

Secret-bearing field checks include:

- PRIVATE_KEY
- MNEMONIC
- SECRET_KEY
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY

## Rejection test

Confirmed behavior:

- bad checkedAtIso is rejected as invalid_checked_at_iso
- live_send config is rejected as live_send_not_allowed
- dry_run config is rejected as invalid_config
- RPC URL containing RPC API key marker is rejected as forbidden_config_value
- sendTransaction method is rejected as invalid_rpc_method
- transport throw is rejected as rpc_transport_failed

## Stage 4.2 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts

Result:

    Stage 4.2 read-only RPC connectivity boundary
      ✔ runs only read-only RPC checks without wallet loading, signing, transactions, or SOL spend
      ✔ keeps public result JSON redacted and free of wallet paths or secret-bearing markers
      ✔ rejects malformed metadata, live-send configs, forbidden values, non-read-only methods, and transport failures

    3 passing

## Stage 4.1 plus Stage 4.2 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts

Result:

    Stage 4.1 redacted live config boundary
      ✔ parses a read-only live config shape without RPC calls or wallet loading
      ✔ creates a redacted public view that never prints live config secrets or wallet paths
      ✔ rejects missing, malformed, live-send, and forbidden secret-bearing config values

    Stage 4.2 read-only RPC connectivity boundary
      ✔ runs only read-only RPC checks without wallet loading, signing, transactions, or SOL spend
      ✔ keeps public result JSON redacted and free of wallet paths or secret-bearing markers
      ✔ rejects malformed metadata, live-send configs, forbidden values, non-read-only methods, and transport failures

    6 passing

## Stage 3.10 plus Stage 4.1 plus Stage 4.2 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts

Result:

    Stage 3.10 final closure boundary
      ✔ creates a final Stage 3 closure boundary from Stage 3.1 through Stage 3.9 evidence
      ✔ rejects missing, duplicate, unordered, unclosed, or non-offline evidence entries
      ✔ rejects malformed closure metadata, failed invariants, and forbidden values

    Stage 4.1 redacted live config boundary
      ✔ parses a read-only live config shape without RPC calls or wallet loading
      ✔ creates a redacted public view that never prints live config secrets or wallet paths
      ✔ rejects missing, malformed, live-send, and forbidden secret-bearing config values

    Stage 4.2 read-only RPC connectivity boundary
      ✔ runs only read-only RPC checks without wallet loading, signing, transactions, or SOL spend
      ✔ keeps public result JSON redacted and free of wallet paths or secret-bearing markers
      ✔ rejects malformed metadata, live-send configs, forbidden values, non-read-only methods, and transport failures

    9 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No secret-like material was introduced.

## Boundary classification

Stage 4.2 is:

    read-only RPC boundary
    no-wallet-loading
    no-signing
    no-transaction
    no-SOL-spend

Stage 4.2 is not:

    watcher runtime loop
    relayer runtime loop
    transaction construction
    transaction simulation
    dry-run send
    live-send
    deployment

## Current conclusion

Stage 4.2 establishes the read-only RPC connectivity boundary.

It proves that read-only RPC connectivity can be modeled through an explicit allowlist of safe methods, an injected transport boundary, redacted output, and invariant checks that prevent wallet loading, signing, transaction submission, or SOL-spending paths.

The next valid stage is Stage 4.3 watcher runtime read-only observation boundary.

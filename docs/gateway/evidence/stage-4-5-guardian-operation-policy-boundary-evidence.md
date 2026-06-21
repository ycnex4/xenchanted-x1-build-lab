# Stage 4.5 Guardian Operation Policy Boundary Evidence

This document records Stage 4.5 guardian operation policy boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-5-guardian-operation-policy-boundary

Runtime commit:

    93665db Add Stage 4.5 guardian operation policy boundary

Base runtime commit:

    5b3be68 Add Stage 4.4 relayer dry-run no-send boundary

## Stage position

Stage 1 is closed as the deterministic model layer.

Stage 2 is closed as the runtime / evidence layer.

Stage 3 is closed as the offline tooling / production surface layer.

Stage 4.0 defined the live runtime charter.

Stage 4.1 established the redacted live config boundary.

Stage 4.2 established the read-only RPC connectivity boundary.

Stage 4.3 established the watcher read-only observation boundary.

Stage 4.4 established the relayer dry-run / no-send boundary.

Stage 4.5 adds the guardian operation policy boundary.

## Scope

Stage 4.5 introduces guardian operation policy.

It is policy-only.

It does not access guardian private keys.

It does not introduce guardian key material.

It does not load a wallet.

It does not sign anything.

It does not submit transactions.

It does not spend SOL.

It does not perform live-send.

It does not deploy anything.

The boundary proves that guardian operational rules can be modeled using public keys only, quorum policy, and no-secret-handling invariants before any signing, wallet, or live-send boundary is introduced.

## Runtime changes

New helper:

    tests/helpers/stage4GuardianOperationPolicyPrototype.ts

New test:

    tests/stage4_guardian_operation_policy_boundary.test.ts

## Dependency on Stage 4.4

Stage 4.5 consumes the Stage 4.4 relayer dry-run no-send result.

Required source artifact:

    stage4_relayer_dry_run_no_send_result

Required source stage:

    4.4

Required source execution mode:

    relayer_dry_run_no_send

Required source condition:

    dry-run result ok must be true

Stage 4.5 rejects failed dry-run evidence.

Stage 4.5 also requires the Stage 4.4 unsigned plan protections:

    signerCount: 0
    transactionSubmission: not_allowed
    walletRequired: false
    signatureRequired: false
    solSpendAllowed: false

## Allowed guardian policy actions

New type:

    Stage4GuardianPolicyAction

Allowed actions:

    reviewUnsignedPlan
    verifyQuorumPolicy
    recordNoSecretHandlingPolicy

Signing, private-key export, and send actions are rejected.

Example rejected actions:

    signTransaction
    exportPrivateKey

## New guardian policy step code model

New type:

    Stage4GuardianPolicyStepCode

Step codes:

    unsigned_plan_review_policy
    guardian_quorum_policy
    no_secret_handling_policy

## New guardian policy step model

New type:

    Stage4GuardianPolicyStep

Fields:

    code
    action
    ok
    policyOnly
    secretAccess
    signing
    transactionSubmission
    errorMessage

Required step values:

    policyOnly: true
    secretAccess: not_allowed
    signing: not_performed
    transactionSubmission: not_allowed

## New result artifact

New type:

    Stage4GuardianOperationPolicyResult

Artifact type:

    stage4_guardian_operation_policy_result

Schema version:

    1

Stage:

    4.5

Execution mode:

    guardian_policy_no_key_material

Fields:

    policyAtIso
    networkName
    programId
    payerPublicKey
    sourceDryRunStage
    sourceDryRunOk
    guardianPublicKeyCount
    quorumThreshold
    steps
    policy
    invariants
    ok

## Guardian public key and quorum policy

Stage 4.5 accepts guardian public keys only.

Guardian private keys are not accepted.

Guardian public keys must be unique and public-key-like.

Quorum threshold must be:

    >= 1
    <= guardianPublicKeyCount

Malformed guardian public keys are rejected.

Invalid quorum thresholds are rejected.

## Policy model

The result includes a policy object:

    keyMaterialHandling: public_keys_only
    privateKeyAccess: not_allowed
    walletLoading: not_allowed
    signingAuthorization: policy_only_not_signature
    transactionSubmission: not_allowed
    solSpendAllowed: false

This explicitly prevents guardian policy from becoming guardian signing.

## Invariants

Stage 4.5 result invariants:

    noGuardianPrivateKeys: true
    noSecretMaterial: true
    noWalletLoaded: true
    noSigning: true
    noTransactionsSubmitted: true
    noSolSpend: true
    policyOnly: true

All invariants must remain true.

## New error type

New class:

    Stage4GuardianOperationPolicyError

New reason type:

    Stage4GuardianOperationPolicyErrorReason

Reasons:

    invalid_policy_at_iso
    invalid_dry_run_result
    dry_run_not_ok
    invalid_guardian_public_keys
    invalid_quorum_threshold
    invalid_policy_action
    forbidden_policy_value

## New helpers

Action assertion helper:

    assertStage4GuardianPolicyActionPrototype

Policy runner:

    runStage4GuardianOperationPolicyPrototype

Result checker:

    checkStage4GuardianOperationPolicyResultPrototype

## Secret, signing, and transaction boundary

Stage 4.5 rejects values containing forbidden markers such as:

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
- sendTransaction
- signTransaction
- signedTransaction
- signMessage
- exportPrivateKey

These strings are defensive markers only.

No real secret values are introduced.

No guardian private key material is introduced.

No wallet-loading path is introduced.

No signing path is introduced.

No transaction submission path is introduced.

## Successful guardian policy test

Confirmed behavior:

- creates a guardian policy-only result
- consumes Stage 4.4 dry-run no-send evidence
- uses guardian public keys only
- validates quorum threshold
- does not access private keys
- does not load wallet
- does not sign
- does not submit transactions
- does not spend SOL
- does not perform live-send
- preserves networkName
- preserves programId
- preserves payerPublicKey
- sourceDryRunStage is 4.4
- sourceDryRunOk is true
- guardianPublicKeyCount is 2
- quorumThreshold is 2
- policy marks privateKeyAccess as not_allowed
- policy marks walletLoading as not_allowed
- policy marks signingAuthorization as policy_only_not_signature
- policy marks transactionSubmission as not_allowed
- policy marks solSpendAllowed as false
- all steps are policyOnly
- all steps have signing not_performed
- all invariants are true
- checkStage4GuardianOperationPolicyResultPrototype returns true

## Safe result JSON test

Confirmed behavior:

- guardian policy result JSON does not contain wallet path
- guardian policy result JSON does not contain secret-bearing markers
- guardian policy result JSON does not contain private key material
- guardian policy result JSON does not contain send/sign methods

Secret-bearing and forbidden action checks include:

- PRIVATE_KEY
- MNEMONIC
- SECRET_KEY
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY
- sendTransaction
- signTransaction
- signedTransaction
- signMessage
- exportPrivateKey

## Rejection test

Confirmed behavior:

- bad policyAtIso is rejected as invalid_policy_at_iso
- failed dry-run result is rejected as dry_run_not_ok
- malformed guardian public key is rejected as invalid_guardian_public_keys
- quorum threshold greater than guardian count is rejected as invalid_quorum_threshold
- dry-run value containing privateKey marker is rejected as forbidden_policy_value
- signTransaction guardian policy action is rejected as invalid_policy_action
- exportPrivateKey guardian policy action is rejected as invalid_policy_action

## Stage 4.5 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_guardian_operation_policy_boundary.test.ts

Result:

    Stage 4.5 guardian operation policy boundary
      ✔ creates a guardian policy-only result without private keys, wallet loading, signing, sending, or SOL spend
      ✔ keeps guardian policy result JSON free of wallet paths, secret markers, private key material, and send/sign methods
      ✔ rejects malformed metadata, failed dry-run, malformed guardians, bad quorum, forbidden values, and signing/send actions

    3 passing

## Stage 4.1 plus Stage 4.2 plus Stage 4.3 plus Stage 4.4 plus Stage 4.5 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts

Result:

    Stage 4.1 redacted live config boundary
      ✔ parses a read-only live config shape without RPC calls or wallet loading
      ✔ creates a redacted public view that never prints live config secrets or wallet paths
      ✔ rejects missing, malformed, live-send, and forbidden secret-bearing config values

    Stage 4.2 read-only RPC connectivity boundary
      ✔ runs only read-only RPC checks without wallet loading, signing, transactions, or SOL spend
      ✔ keeps public result JSON redacted and free of wallet paths or secret-bearing markers
      ✔ rejects malformed metadata, live-send configs, forbidden values, non-read-only methods, and transport failures

    Stage 4.3 watcher read-only observation boundary
      ✔ runs one watcher read-only observation cycle without wallet loading, signing, transactions, or SOL spend
      ✔ keeps watcher observation result JSON free of wallet paths, secret markers, and transaction methods
      ✔ rejects malformed metadata, failed connectivity, forbidden values, non-read-only watcher methods, and source failures

    Stage 4.4 relayer dry-run no-send boundary
      ✔ builds an unsigned relayer dry-run plan without wallet loading, signing, sending, or SOL spend
      ✔ keeps relayer dry-run result JSON free of wallet paths, secret markers, and send/sign transaction methods
      ✔ rejects malformed metadata, failed observation, bad mint intent, forbidden values, live operations, signed plans, and planner failures

    Stage 4.5 guardian operation policy boundary
      ✔ creates a guardian policy-only result without private keys, wallet loading, signing, sending, or SOL spend
      ✔ keeps guardian policy result JSON free of wallet paths, secret markers, private key material, and send/sign methods
      ✔ rejects malformed metadata, failed dry-run, malformed guardians, bad quorum, forbidden values, and signing/send actions

    15 passing

## Stage 3.10 plus Stage 4.1 plus Stage 4.2 plus Stage 4.3 plus Stage 4.4 plus Stage 4.5 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts

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

    Stage 4.3 watcher read-only observation boundary
      ✔ runs one watcher read-only observation cycle without wallet loading, signing, transactions, or SOL spend
      ✔ keeps watcher observation result JSON free of wallet paths, secret markers, and transaction methods
      ✔ rejects malformed metadata, failed connectivity, forbidden values, non-read-only watcher methods, and source failures

    Stage 4.4 relayer dry-run no-send boundary
      ✔ builds an unsigned relayer dry-run plan without wallet loading, signing, sending, or SOL spend
      ✔ keeps relayer dry-run result JSON free of wallet paths, secret markers, and send/sign transaction methods
      ✔ rejects malformed metadata, failed observation, bad mint intent, forbidden values, live operations, signed plans, and planner failures

    Stage 4.5 guardian operation policy boundary
      ✔ creates a guardian policy-only result without private keys, wallet loading, signing, sending, or SOL spend
      ✔ keeps guardian policy result JSON free of wallet paths, secret markers, private key material, and send/sign methods
      ✔ rejects malformed metadata, failed dry-run, malformed guardians, bad quorum, forbidden values, and signing/send actions

    18 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No secret-like material was introduced.

No guardian private key material was introduced.

No wallet-loading path was introduced.

No signing path was introduced.

No sendable transaction path was introduced.

## Boundary classification

Stage 4.5 is:

    guardian operation policy boundary
    public-keys-only policy boundary
    quorum policy boundary
    no-secret-handling boundary
    no-wallet-loading
    no-signing
    no-transaction-submission
    no-SOL-spend
    no-live-send
    policy-only

Stage 4.5 is not:

    guardian signing boundary
    guardian key loading boundary
    wallet access boundary
    transaction preflight boundary
    transaction simulation boundary
    live-send boundary
    deployment boundary

## Current conclusion

Stage 4.5 establishes the guardian operation policy boundary.

It proves that guardian operational rules can be modeled as public-keys-only policy with quorum validation and explicit no-secret/no-wallet/no-signing/no-send invariants, before any signing, wallet access, transaction preflight, or live-send boundary is introduced.

The next valid stage is Stage 4.6 transaction preflight / no-send boundary.

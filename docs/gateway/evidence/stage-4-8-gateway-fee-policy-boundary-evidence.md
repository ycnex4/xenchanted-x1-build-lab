# Stage 4.8 Gateway Fee Policy Boundary Evidence

This document records Stage 4.8 gateway fee policy boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-8-gateway-fee-policy-boundary

Runtime commit:

    a960c16 Add Stage 4.8 gateway fee policy boundary

Base runtime commit:

    f63397f Add Stage 4.7 fixed guardian set quorum boundary

## Stage position

Stage 4.7 fixed the guardian set model:

    guardian_count = 5
    quorum_threshold = 3
    guardian_set_version = 1

Stage 4.8 now adds the gateway fee policy boundary.

This stage is still fully offline.

It does not use live RPC.

It does not load a wallet.

It does not sign anything.

It does not submit transactions.

It does not spend SOL.

## Scope

Stage 4.8 defines the gateway fee policy model before guardian-approved fee-bound messages.

It establishes:

    feeMode
    feeAsset
    feeRecipient
    grossAmount
    feeAmount
    netAmount
    feeQuoteExpiresAtIso
    guardianSetVersion binding

The fee policy is a gateway service policy.

It is not part of xEnchanted Core protocol economics.

The gateway fee is an infrastructure/service fee for gateway operation, relayer coordination, watcher operation, guardian coordination, and operational support.

## Runtime changes

New helper:

    tests/helpers/stage4GatewayFeePolicyPrototype.ts

New test:

    tests/stage4_gateway_fee_policy_boundary.test.ts

## Dependency on Stage 4.7

Stage 4.8 consumes Stage 4.7 fixed guardian quorum evidence.

Required source artifact:

    stage4_fixed_guardian_set_quorum_result

Required source stage:

    4.7

Required source execution mode:

    fixed_guardian_set_quorum_policy

Required source conditions:

    guardianSetVersion must be 1
    guardianCount must be 5
    quorumThreshold must be 3
    quorumReached must be true
    result ok must be true

Stage 4.8 rejects failed guardian quorum evidence.

## Fee model

Stage 4.8 fixes the initial gateway fee model:

    feeMode: fixed_service_fee
    pricingSource: manual_config_only
    feeRecipientSource: configured_public_address
    netAmountRule: gross_amount_minus_fee_amount
    deadlineRequired: true
    boundToGuardianSetVersion: 1

Supported fee assets:

    X1_NATIVE
    CONFIGURED_TOKEN

Important: Stage 4.8 does not perform oracle lookup.

The fee amount is manually configured.

Dynamic pricing and oracle-based pricing remain out of scope.

## Fee amount rules

Stage 4.8 enforces:

    grossAmount > 0
    feeAmount > 0
    feeAmount < grossAmount
    netAmount = grossAmount - feeAmount
    netAmount > 0
    expectedNetAmount must match calculated netAmount when supplied

This prevents:

- negative fee
- zero fee
- fee equal to gross amount
- fee larger than gross amount
- incorrect net amount
- hidden net amount mismatch

## Fee quote deadline

Stage 4.8 requires:

    feeQuoteExpiresAtIso > quotedAtIso

Malformed quote times are rejected.

Expired or non-forward deadlines are rejected.

This prepares the model for later fee-bound guardian approval messages.

## New result artifact

New type:

    Stage4GatewayFeePolicyResult

Artifact type:

    stage4_gateway_fee_policy_result

Schema version:

    1

Stage:

    4.8

Execution mode:

    gateway_fee_policy_offline

Fields:

    quotedAtIso
    sourceGuardianQuorumStage
    sourceGuardianQuorumOk
    guardianSetVersion
    routeId
    feeQuoteId
    feeMode
    feeAsset
    feeRecipient
    grossAmount
    feeAmount
    netAmount
    feeQuoteExpiresAtIso
    steps
    policy
    invariants
    ok

## Policy object

The result includes a policy object:

    feeMode: fixed_service_fee
    pricingSource: manual_config_only
    feeRecipientSource: configured_public_address
    feeMustBeLessThanGrossAmount: true
    netAmountRule: gross_amount_minus_fee_amount
    deadlineRequired: true
    boundToGuardianSetVersion: 1
    oracleLookup: not_performed
    walletLoading: not_allowed
    signing: not_performed
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Invariants

Stage 4.8 invariants:

    offlineOnly: true
    fixedServiceFee: true
    manualConfigOnly: true
    feeLessThanGrossAmount: true
    netAmountMatchesGrossMinusFee: true
    deadlineAfterQuoteTime: true
    boundToGuardianSetVersion: true
    noOracleLookup: true
    noWalletLoaded: true
    noSigning: true
    noTransactionsSubmitted: true
    noSolSpend: true

All invariants must remain true.

## Allowed fee policy operations

New type:

    Stage4GatewayFeePolicyOperation

Allowed operations:

    validateGatewayFeePolicy
    calculateNetAmount
    recordFeeQuoteDeadline

Rejected example operation:

    sendTransaction

## Error model

New class:

    Stage4GatewayFeePolicyError

New reason type:

    Stage4GatewayFeePolicyErrorReason

Reasons:

    invalid_quoted_at_iso
    invalid_fee_quote_deadline_iso
    deadline_not_after_quote_time
    invalid_guardian_quorum_result
    guardian_quorum_not_ok
    invalid_route_id
    invalid_fee_quote_id
    invalid_fee_mode
    invalid_fee_asset
    invalid_fee_recipient
    invalid_gross_amount
    invalid_fee_amount
    fee_not_less_than_gross_amount
    invalid_expected_net_amount
    forbidden_value
    invalid_fee_policy_operation

## New helpers

Operation assertion helper:

    assertStage4GatewayFeePolicyOperationPrototype

Fee policy runner:

    runStage4GatewayFeePolicyPrototype

Result checker:

    checkStage4GatewayFeePolicyResultPrototype

## Secret, wallet, and transaction boundary

Stage 4.8 rejects values containing forbidden markers such as:

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
- serializedTransaction

These strings are defensive markers only.

No real secret values are introduced.

No wallet-loading path is introduced.

No guardian private key material is introduced.

No signing path is introduced.

No transaction submission path is introduced.

No serialized transaction material is introduced.

## Successful fee policy test

Confirmed behavior:

- creates an offline fixed service gateway fee quote from valid 3-of-5 guardian quorum
- uses feeMode fixed_service_fee
- supports feeAsset X1_NATIVE
- supports feeAsset CONFIGURED_TOKEN
- requires configured public fee recipient
- validates grossAmount
- validates feeAmount
- calculates netAmount as grossAmount minus feeAmount
- validates expectedNetAmount when supplied
- requires feeQuoteExpiresAtIso after quotedAtIso
- binds fee policy to guardianSetVersion 1
- does not perform oracle lookup
- does not load wallet
- does not sign
- does not submit transactions
- does not spend SOL
- checkStage4GatewayFeePolicyResultPrototype returns true

## Safe result JSON test

Confirmed behavior:

- gateway fee policy result JSON does not contain wallet path
- gateway fee policy result JSON does not contain private key markers
- gateway fee policy result JSON does not contain signing methods
- gateway fee policy result JSON does not contain transaction submission methods
- gateway fee policy result JSON does not contain serialized transaction marker

Forbidden method and marker checks include:

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
- serializedTransaction

## Rejection test

Confirmed behavior:

- failed 2-of-5 guardian quorum is rejected as guardian_quorum_not_ok
- malformed quotedAtIso is rejected as invalid_quoted_at_iso
- non-forward deadline is rejected as deadline_not_after_quote_time
- fee equal to gross amount is rejected as fee_not_less_than_gross_amount
- wrong expected net amount is rejected as invalid_expected_net_amount
- feeQuoteId containing privateKey marker is rejected as forbidden_value
- sendTransaction operation is rejected as invalid_fee_policy_operation

## Stage 4.8 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_gateway_fee_policy_boundary.test.ts

Result:

    Stage 4.8 gateway fee policy boundary
      ✔ creates an offline fixed service gateway fee quote from a valid 3-of-5 guardian quorum
      ✔ keeps gateway fee policy result JSON free of wallet paths, secrets, signing, and transaction submission methods
      ✔ rejects failed guardian quorum, bad fee math, malformed deadline, wrong expected net, forbidden values, and invalid operations

    3 passing

## Stage 4.1 through Stage 4.8 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts \
      tests/stage4_gateway_fee_policy_boundary.test.ts

Result:

    25 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.8 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts \
      tests/stage4_gateway_fee_policy_boundary.test.ts

Result:

    28 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No SOL is spent by this stage.

No live RPC is required by this stage.

No secret-like material was introduced.

## Boundary classification

Stage 4.8 is:

    gateway fee policy boundary
    fixed service fee boundary
    manual fee configuration boundary
    gross/net fee calculation boundary
    fee quote deadline boundary
    guardianSetVersion-bound fee policy boundary
    no-oracle boundary
    offline model boundary

Stage 4.8 is not:

    guardian approval message boundary
    fee-bound signature boundary
    oracle pricing boundary
    wallet access boundary
    signing boundary
    transaction preflight boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.8 establishes the gateway fee policy model as a fixed manually configured service fee with fee recipient, gross amount, fee amount, net amount, quote deadline, and guardian set version binding.

It proves that gateway fee policy can be modeled offline with no oracle lookup, no wallet loading, no signing, no transaction submission, and no SOL spend.

The next valid stage is Stage 4.9 guardian approval message with fee bound into message.

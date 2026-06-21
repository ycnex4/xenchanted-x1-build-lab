# Stage 4.11 XNTD -> XXXL Amount Conversion Boundary Evidence

This document records Stage 4.11 XNTD -> XXXL amount conversion boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-11-xntd-xxxl-amount-conversion-boundary

Runtime commit:

    0be7dd4 Add Stage 4.11 XNTD XXXL amount conversion boundary

Base runtime commit:

    fdbc3b8 Add Stage 4.10 guardian fee-bound approval verification boundary

## Why Stage 4.11 changed

Before entering production signature verification design, the amount conversion policy needed to be fixed explicitly.

The earlier runtime test placeholders used a raw 1:1-style amount example.

That was not the intended protocol economics.

The correct bridge conversion rule is:

    1 XXXL = 100,000,000 XNTD

With decimals:

    XNTD ERC-20 decimals = 18
    XXXL X1 decimals = 9

Therefore:

    100,000,000 XNTD raw = 100000000000000000000000000
    1 XXXL raw = 1000000000
    XNTD raw per 1 XXXL raw = 100000000000000000

Stage 4.11 fixes this as an explicit offline boundary before signature design.

Production signature verification is moved to Stage 4.12.

## Scope

Stage 4.11 defines the XNTD -> XXXL amount conversion policy.

It proves that the relayer mint intent must use exact raw conversion:

    burnedXntdRaw / xntdRawPerXxxlRaw = xxxlMintRaw

No rounding is allowed.

No flooring is allowed.

No ceiling is allowed.

Any non-exact conversion is rejected.

Any mismatch between provided xxxlMintAmount and expected derived amount is rejected.

This stage is still fully offline.

It does not use live RPC.

It does not load a wallet.

It does not access private keys.

It does not sign anything.

It does not perform cryptographic signature verification.

It does not submit transactions.

It does not spend SOL.

## Runtime changes

New helper:

    tests/helpers/stage4XntdXxxlAmountConversionPolicyPrototype.ts

New test:

    tests/stage4_xntd_xxxl_amount_conversion_boundary.test.ts

## Conversion constants

Stage 4.11 fixes these constants:

    xntdErc20Decimals = 18
    xxxlX1Decimals = 9
    xntdPerXxxl = 100000000
    xntdRawPerXxxlRaw = 100000000000000000

Human-unit meaning:

    1 XXXL = 100,000,000 XNTD

Raw-unit example:

    burnedAmount = 100000000000000000000000000
    xxxlMintAmount = 1000000000

This means:

    100,000,000 XNTD raw -> 1 XXXL raw with 9 decimals

## New result artifact

New result type:

    Stage4XntdXxxlAmountConversionPolicyResult

Artifact type:

    stage4_xntd_xxxl_amount_conversion_policy_result

Schema version:

    1

Stage:

    4.11

Execution mode:

    xntd_xxxl_amount_conversion_policy_offline

Fields:

    convertedAtIso
    routeId
    canonicalEventKey
    x1RecipientHash
    mintToken
    xntdErc20Decimals
    xxxlX1Decimals
    xntdPerXxxl
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    expectedXxxlMintRaw
    conversionRemainderRaw
    operations
    policy
    invariants
    ok

## Policy object

The result includes a policy object:

    exactRawConversionRequired: true
    noRoundingAllowed: true
    noFlooringAllowed: true
    noCeilingAllowed: true
    xntdDecimalsFixed: 18
    xxxlDecimalsFixed: 9
    xntdPerXxxlFixed: 100000000
    signing: not_performed
    walletLoading: not_allowed
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Invariants

Stage 4.11 invariants:

    offlineOnly: true
    amountConversionOnly: true
    xntdDecimalsAre18: true
    xxxlDecimalsAre9: true
    oneXxxlEqualsOneHundredMillionXntd: true
    exactRawConversion: true
    noRounding: true
    noWalletLoaded: true
    noSigning: true
    noTransactionsSubmitted: true
    noSolSpend: true

All invariants must remain true.

## Allowed amount conversion operations

New type:

    Stage4XntdXxxlAmountConversionOperation

Allowed operations:

    validateMintIntentAmounts
    deriveXxxlMintAmount
    recordAmountConversionPolicy

Rejected example operations:

    sendTransaction
    signMessage

## Error model

New class:

    Stage4XntdXxxlAmountConversionPolicyError

New reason type:

    Stage4XntdXxxlAmountConversionPolicyErrorReason

Reasons:

    invalid_converted_at_iso
    invalid_mint_intent
    invalid_amount_string
    zero_amount
    non_exact_conversion
    xxxl_mint_amount_mismatch
    forbidden_value
    invalid_amount_conversion_operation

## New helpers

Operation assertion helper:

    assertStage4XntdXxxlAmountConversionOperationPrototype

Conversion derivation helper:

    deriveStage4XxxlMintRawFromXntdBurnedRawPrototype

Conversion runner:

    runStage4XntdXxxlAmountConversionPolicyPrototype

Result checker:

    checkStage4XntdXxxlAmountConversionPolicyResultPrototype

## Successful conversion test

Confirmed behavior:

    burnedAmount = 100000000000000000000000000
    xxxlMintAmount = 1000000000

This is accepted because:

    100000000000000000000000000 / 100000000000000000 = 1000000000

The result confirms:

    xntdErc20Decimals = 18
    xxxlX1Decimals = 9
    xntdPerXxxl = 100000000
    xntdRawPerXxxlRaw = 100000000000000000
    expectedXxxlMintRaw = 1000000000
    conversionRemainderRaw = 0
    ok = true

## Old placeholder rejection

Stage 4.11 explicitly rejects the old placeholder-style mint intent:

    burnedAmount = 1000000000000000000
    xxxlMintAmount = 1000000000000000000

The derivation for burnedAmount 1000000000000000000 is:

    expectedXxxlMintRaw = 10
    conversionRemainderRaw = 0

So providing xxxlMintAmount 1000000000000000000 is rejected as:

    xxxl_mint_amount_mismatch

This prevents the old 1:1 raw placeholder from leaking into later stages.

## Non-exact conversion rejection

Stage 4.11 rejects non-exact conversions.

Example:

    burnedAmount = 1
    xxxlMintAmount = 1

This is rejected as:

    non_exact_conversion

No rounding, flooring, or ceiling is allowed.

## Secret, wallet, signing, and transaction boundary

Stage 4.11 rejects values containing forbidden markers such as:

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

No private key material is introduced.

No signing path is introduced.

No transaction submission path is introduced.

No serialized transaction material is introduced.

## Successful verification test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_xntd_xxxl_amount_conversion_boundary.test.ts

Result:

    Stage 4.11 XNTD -> XXXL amount conversion boundary
      ✔ converts 100,000,000 XNTD ERC-20 raw units into 1 XXXL with 9 decimals
      ✔ derives XXXL raw amount with 9 decimals and rejects old 1:1 placeholder amounts
      ✔ keeps amount conversion result JSON free of wallet paths, secrets, signing, and transaction submission methods
      ✔ rejects malformed metadata, invalid amounts, non-exact conversion, forbidden values, and invalid operations

    4 passing

## Stage 4.1 through Stage 4.11 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts \
      tests/stage4_gateway_fee_policy_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts \
      tests/stage4_xntd_xxxl_amount_conversion_boundary.test.ts

Result:

    37 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.11 smoke

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
      tests/stage4_gateway_fee_policy_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts \
      tests/stage4_xntd_xxxl_amount_conversion_boundary.test.ts

Result:

    40 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Exact safety marker verification:

    ok

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No SOL is spent by this stage.

No live RPC is required by this stage.

No secret-like material was introduced.

## Boundary classification

Stage 4.11 is:

    XNTD -> XXXL amount conversion policy boundary
    raw amount conversion boundary
    exact conversion boundary
    no rounding boundary
    18-decimal XNTD to 9-decimal XXXL boundary
    offline model boundary

Stage 4.11 is not:

    production cryptographic signature verification boundary
    guardian private key boundary
    wallet access boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.11 fixes the XNTD -> XXXL conversion policy:

    1 XXXL = 100,000,000 XNTD
    XNTD ERC-20 decimals = 18
    XXXL X1 decimals = 9
    xntdRawPerXxxlRaw = 100000000000000000

It proves that a relayer mint intent must contain the exact derived xxxlMintAmount.

It rejects the old raw 1:1 placeholder amount pattern.

The next valid stage is Stage 4.12 production signature verification design boundary.

# Stage 4.19 Evidence — Receipt-Bound External Signer Handoff Planning Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-19-receipt-bound-external-signer-handoff-planning-boundary

Runtime commit:

    0e877f9 Add Stage 4.19 receipt-bound external signer handoff planning boundary

## Purpose

Stage 4.19 adds the receipt-bound external signer handoff planning boundary.

Stage 4.18 planned the runtime message and account mapping. Stage 4.19 models the handoff contract for a future existing X1 wallet or external signer interface.

This stage does not build a standalone custody wallet product. It does not load a local signer. It does not access keypairs. It does not access private keys. It does not sign inside the runtime model. It does not submit transactions. It does not spend SOL.

The goal is to preserve the protocol boundary:

- The protocol prepares receipt-bound data.
- The runtime model remains non-custodial.
- A future external signer layer may request user approval.
- No private signing material enters the protocol runtime.

## Runtime files added

    tests/helpers/stage4ReceiptBoundExternalSignerHandoffPlanningPrototype.ts
    tests/stage4_receipt_bound_external_signer_handoff_planning_boundary.test.ts

## Artifact introduced

    stage4_receipt_bound_external_signer_handoff_planning_result

Artifact metadata:

    schemaVersion: 1
    stage: 4.19
    executionMode: receipt_bound_external_signer_handoff_planning_offline

## Source dependency

Stage 4.19 requires a valid Stage 4.18 source artifact:

    stage4_receipt_bound_transaction_message_planning_result

Required Stage 4.18 properties:

    sourceMessagePlanningStage: 4.18
    sourceMessagePlanningOk: true
    sourceMessagePlanningDigest: required
    sourceNoSignAssemblyDigest: required
    sourceAssemblyDesignDigest: required
    sourceReceiptDigest: required
    sourceResultDigest: required
    instructionDataDigest: required
    accountMappingDigest: required
    instructionMessagePlanDigest: required
    instructionName: mint_xxxl_from_receipt_bound_gateway_message
    blockhashPlanningStatus: not_requested_no_live_network
    signerPlanningStatus: not_performed
    compiledMessagePlanningStatus: not_created_message_plan_only
    transactionPlanningStatus: not_created_message_plan_only
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Handoff planning model

Stage 4.19 models two handoff planning layers:

    Stage4ReceiptBoundExternalSignerPayloadPlan
    Stage4ReceiptBoundExternalSignerInterfacePlan

Raw plan-kind markers:

    receipt_bound_external_signer_payload_plan
    receipt_bound_external_signer_interface_plan

Raw payload format marker:

    x1_external_wallet_receipt_bound_message_plan

Raw external signer layer marker:

    existing_x1_wallet_or_external_signer

Raw custody-scope marker:

    out_of_scope

The payload plan binds:

    sourceMessagePlanningDigest
    sourceNoSignAssemblyDigest
    sourceAssemblyDesignDigest
    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    accountMappingDigest
    instructionMessagePlanDigest
    instructionName
    programId
    payerPublicKey
    networkName
    externalSignerRole
    custodyModel
    localSignerLoadingStatus
    keypairAccessStatus
    privateKeyAccessStatus
    signingStatus
    transactionSubmissionStatus
    solSpendStatus
    payloadDigest

The interface plan binds:

    intendedSignerLayer
    runtimeCustody
    runtimePrivateKeyAccess
    runtimeKeypairAccess
    runtimeSigning
    runtimeSubmission
    runtimeSolSpendAllowed
    userApprovalRequiredLater
    externalSignerMustVerify
    externalSignerInterfacePlanDigest

External signer must verify:

    networkName
    programId
    payerPublicKey
    instructionName
    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    accountMappingDigest
    instructionMessagePlanDigest
    messagePlanningDigest

## Digest binding

Stage 4.19 derives an external signer handoff digest from:

    sourceMessagePlanningDigest
    sourceNoSignAssemblyDigest
    sourceAssemblyDesignDigest
    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    instructionName
    programId
    payerPublicKey
    networkName
    payloadDigest
    externalSignerInterfacePlanDigest

The handoff digest changes if the source message plan changes.

## Safety boundary

Stage 4.19 explicitly preserves:

    offlineOnly: true
    handoffPlanningOnly: true
    sourceMessagePlanningStage418Bound: true
    sourceMessagePlanningDigestBound: true
    sourceNoSignAssemblyDigestBound: true
    sourceAssemblyDesignDigestBound: true
    sourceReceiptDigestBound: true
    sourceResultDigestBound: true
    instructionDataDigestBound: true
    instructionNameBound: true
    accountMappingDigestBound: true
    instructionMessagePlanDigestBound: true
    externalSignerPayloadDigestBound: true
    externalSignerInterfacePlanDigestBound: true
    externalSignerHandoffDigestBound: true
    externalSignerOnly: true
    noCustodyWalletProduct: true
    noRuntimeCustody: true
    noLocalSignerLoaded: true
    noKeypairAccess: true
    noPrivateKeys: true
    noRuntimeSigning: true
    noRuntimeSubmission: true
    noRuntimeSolSpend: true
    noSerializedTransaction: true
    noSimulation: true
    noLiveRpc: true

## Policy boundary

Stage 4.19 policy states:

    handoffPlanningOnly: true
    custodyWalletProduct: out_of_scope
    externalSignerLayer: existing_x1_wallet_or_external_signer
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    runtimeSigning: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false

## Negative coverage

Stage 4.19 rejects:

- Malformed planning timestamps.
- Invalid Stage 4.18 message planning artifacts.
- Failed Stage 4.18 message planning artifacts.
- Public key shape failures.
- Handoff digest mismatches.
- Forbidden secret-bearing values.
- Invalid handoff operations.
- Runtime submit/sign operation attempts.

## Checks performed

Runtime checks passed:

    Strict Stage 4.19 marker check: passed
    Stage 4.19 test: 4 passing
    Stage 4.18 + Stage 4.19 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 4.19 smoke: 71 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 4.19 is complete when:

- The runtime commit is recorded.
- The evidence document is committed.
- The current design checkpoint references the Stage 4.19 boundary.
- Build-lab typecheck, tests, and build pass.
- No local signer loading, keypair access, private-key access, runtime signing, transaction submission, runtime SOL spend, serialized transaction output, simulation, or live RPC is introduced.

Stage 4.19 does not authorize live transaction submission.

## Next stage

The next valid stage is:

    Stage 4.20 — Stage 4 final no-send closure boundary

Stage 4.20 should close Stage 4 as a complete no-send/no-SOL readiness chain before any later live-send stage is explicitly opened.

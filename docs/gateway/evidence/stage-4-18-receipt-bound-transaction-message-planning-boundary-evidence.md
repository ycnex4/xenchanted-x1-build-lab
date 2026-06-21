# Stage 4.18 Evidence — Receipt-Bound Transaction Message Planning Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-18-receipt-bound-transaction-message-planning-boundary

Runtime commit:

    854ae6e Add Stage 4.18 receipt-bound transaction message planning boundary

## Purpose

Stage 4.18 adds the receipt-bound transaction message planning boundary.

Stage 4.17 modeled a receipt-bound no-sign assembly structure. Stage 4.18 moves one step further by planning the runtime message and account mapping that a future external signer integration can use.

This stage still does not create a real runtime transaction object. It does not create a compiled message. It does not fetch a recent blockhash. It does not load a local signer. It does not access keypairs or private keys. It does not sign. It does not serialize a transaction. It does not simulate. It does not submit. It does not spend SOL.

The project is not building a standalone custody wallet here. The Stage 4 wording remains a safety boundary: no local signer loading, no keypair access, no signing, and no transaction submission.

## Runtime files added

    tests/helpers/stage4ReceiptBoundTransactionMessagePlanningPrototype.ts
    tests/stage4_receipt_bound_transaction_message_planning_boundary.test.ts

## Artifact introduced

    stage4_receipt_bound_transaction_message_planning_result

Artifact metadata:

    schemaVersion: 1
    stage: 4.18
    executionMode: receipt_bound_transaction_message_planning_offline

## Source dependency

Stage 4.18 requires a valid Stage 4.17 source artifact:

    stage4_receipt_bound_transaction_assembly_no_sign_result

Required Stage 4.17 properties:

    sourceNoSignAssemblyStage: 4.17
    sourceNoSignAssemblyOk: true
    sourceNoSignAssemblyDigest: required
    sourceAssemblyDesignDigest: required
    sourceReceiptDigest: required
    sourceResultDigest: required
    instructionDataDigest: required
    instructionName: mint_xxxl_from_receipt_bound_gateway_message
    runtimeInstructionObjectStatus: not_created_no_sign_boundary
    recentBlockhashStatus: not_fetched_no_live_network
    compiledMessageStatus: not_created_no_wallet_no_blockhash
    signerResolutionStatus: not_performed
    requiredSignatureCount: 0
    runtimeTransactionObjectStatus: not_created_no_sign_boundary
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Message planning model

Stage 4.18 models two planning layers:

    Stage4ReceiptBoundAccountMappingPlan
    Stage4ReceiptBoundInstructionMessagePlan

The account mapping plan binds:

    requiredRoles
    accounts
    allAccountsNonSigners
    accountMappingDigest

Required runtime account roles:

    program
    payer
    mint
    recipient
    processed_burn_registry
    guardian_quorum
    cryptographic_verification_receipt

The instruction message plan binds:

    instructionName
    programId
    payerPublicKey
    sourceInstructionAssemblyDigest
    sourceUnsignedMessageAssemblyDigest
    sourceNoSignAssemblyDigest
    accountMappingDigest
    instructionDataDigest
    sourceReceiptDigest
    sourceResultDigest
    sourceAssemblyDesignDigest
    blockhashPlanningStatus
    feePayerPlanningStatus
    signerPlanningStatus
    compiledMessagePlanningStatus
    transactionPlanningStatus
    instructionMessagePlanDigest

Important message-planning statuses:

    blockhashPlanningStatus: not_requested_no_live_network
    feePayerPlanningStatus: payer_public_key_only
    signerPlanningStatus: not_performed
    compiledMessagePlanningStatus: not_created_message_plan_only
    transactionPlanningStatus: not_created_message_plan_only

## Digest binding

Stage 4.18 derives a message planning digest from:

    sourceNoSignAssemblyDigest
    sourceAssemblyDesignDigest
    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    instructionName
    programId
    payerPublicKey
    accountMappingDigest
    instructionMessagePlanDigest

The message planning digest changes if the source no-sign assembly changes.

## Safety boundary

Stage 4.18 explicitly preserves:

    offlineOnly: true
    messagePlanningOnly: true
    sourceNoSignAssemblyStage417Bound: true
    sourceNoSignAssemblyDigestBound: true
    sourceAssemblyDesignDigestBound: true
    sourceReceiptDigestBound: true
    sourceResultDigestBound: true
    instructionDataDigestBound: true
    instructionNameBound: true
    accountMappingDigestBound: true
    instructionMessagePlanDigestBound: true
    messagePlanningDigestBound: true
    accountRolesBound: true
    allAccountsNonSigners: true
    noWalletLoaded: true
    noPrivateKeys: true
    noSigning: true
    noSignerResolution: true
    noRecentBlockhashFetched: true
    noRuntimeInstructionObjectCreated: true
    noRuntimeTransactionObjectCreated: true
    noCompiledMessageCreated: true
    noSerializedTransaction: true
    noSimulation: true
    noLiveRpc: true
    noTransactionsSubmitted: true
    noSolSpend: true

## Negative coverage

Stage 4.18 rejects:

- Malformed planning timestamps.
- Invalid Stage 4.17 no-sign assembly artifacts.
- Failed Stage 4.17 no-sign assembly artifacts.
- Invalid account mapping.
- Message planning digest mismatches.
- Forbidden secret-bearing values.
- Invalid message planning operations.
- Send/sign operation attempts.

## Checks performed

Runtime checks passed:

    Corrected Stage 4.18 marker check: passed
    Stage 4.18 test: 4 passing
    Stage 4.17 + Stage 4.18 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 4.18 smoke: 67 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 4.18 is complete when:

- The runtime commit is recorded.
- The evidence document is committed.
- The current design checkpoint references the Stage 4.18 boundary.
- Build-lab typecheck, tests, and build pass.
- No local signer loading, keypair access, private-key access, signing, recent blockhash fetch, compiled message creation, runtime transaction object creation, serialization, simulation, transaction submission, or SOL spend is introduced.

Stage 4.18 does not authorize live transaction submission.

## Next stage

The next valid stage is:

    Stage 4.19 — receipt-bound external signer handoff planning boundary

Stage 4.19 may model a handoff contract for an existing external X1 wallet or signer interface, but must still preserve no local custody, no private-key access, no signing inside the runtime model, no transaction submission, and no SOL spend unless a later live-send stage is explicitly opened.

# Stage 4.17 Evidence — Receipt-Bound Transaction Assembly No-Sign Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-17-receipt-bound-transaction-assembly-no-sign-boundary

Runtime commit:

    7dd202b Add Stage 4.17 receipt-bound transaction assembly no-sign boundary

## Purpose

Stage 4.17 adds the receipt-bound transaction assembly no-sign boundary.

Stage 4.16 defined the design contract for future transaction assembly. Stage 4.17 moves one step further by modeling a no-sign assembly structure, while still preserving the no-wallet, no-private-key, no-sign, no-send, no-SOL boundary.

This stage does not create a real runtime transaction object. It does not compile a runtime message. It does not fetch a recent blockhash. It does not load a wallet. It does not sign. It does not serialize a transaction. It does not simulate. It does not submit. It does not spend SOL.

## Runtime files added

    tests/helpers/stage4ReceiptBoundTransactionAssemblyNoSignPrototype.ts
    tests/stage4_receipt_bound_transaction_assembly_no_sign_boundary.test.ts

## Artifact introduced

    stage4_receipt_bound_transaction_assembly_no_sign_result

Artifact metadata:

    schemaVersion: 1
    stage: 4.17
    executionMode: receipt_bound_transaction_assembly_no_sign_offline

## Source dependency

Stage 4.17 requires a valid Stage 4.16 source artifact:

    stage4_receipt_bound_transaction_assembly_design_result

Required Stage 4.16 properties:

    sourceAssemblyDesignStage: 4.16
    sourceAssemblyDesignOk: true
    sourceAssemblyDesignDigest: required
    sourceReceiptDigest: required
    sourceResultDigest: required
    instructionDataDigest: required
    instructionName: mint_xxxl_from_receipt_bound_gateway_message
    transactionObjectStatus: not_created_design_only
    compiledMessageStatus: not_created_design_only
    signerResolutionStatus: not_performed
    signatureStatus: not_performed
    walletLoading: not_allowed
    privateKeyAccess: not_allowed
    liveRpc: not_used
    simulation: not_performed
    transactionSubmission: not_allowed
    solSpendAllowed: false

## No-sign assembly model

Stage 4.17 models two assembly layers:

    Stage4ReceiptBoundNoSignInstructionAssembly
    Stage4ReceiptBoundNoSignMessageAssembly

The instruction assembly binds:

    instructionName
    programId
    accountMetas
    instructionLayout
    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    assemblyDesignDigest
    instructionAssemblyDigest

The unsigned message assembly binds:

    payerPublicKey
    programId
    instructionAssemblyDigest
    accountRolesDigest
    layoutFieldsDigest
    recentBlockhashStatus
    compiledMessageStatus
    signerResolutionStatus
    requiredSignatureCount
    runtimeTransactionObjectStatus

Important no-sign statuses:

    runtimeInstructionObjectStatus: not_created_no_sign_boundary
    recentBlockhashStatus: not_fetched_no_live_network
    compiledMessageStatus: not_created_no_wallet_no_blockhash
    signerResolutionStatus: not_performed
    requiredSignatureCount: 0
    runtimeTransactionObjectStatus: not_created_no_sign_boundary

## Digest binding

Stage 4.17 derives a no-sign assembly digest from:

    sourceAssemblyDesignDigest
    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    instructionName
    programId
    payerPublicKey
    instructionAssemblyDigest
    unsignedMessageAssemblyDigest

The no-sign assembly digest changes if the source assembly design changes.

## Safety boundary

Stage 4.17 explicitly preserves:

    offlineOnly: true
    assemblyNoSignOnly: true
    sourceAssemblyDesignStage416Bound: true
    sourceAssemblyDesignDigestBound: true
    sourceReceiptDigestBound: true
    sourceResultDigestBound: true
    instructionDataDigestBound: true
    noSignAssemblyDigestBound: true
    instructionNameBound: true
    accountRolesBound: true
    instructionLayoutBound: true
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

Stage 4.17 rejects:

- Malformed assembly timestamps.
- Invalid network names.
- Invalid public keys.
- Invalid Stage 4.16 assembly design artifacts.
- Failed Stage 4.16 assembly design artifacts.
- Wrong instruction names.
- No-sign assembly digest mismatches.
- Forbidden secret-bearing values.
- Invalid instruction assembly inputs.
- Invalid unsigned message assembly inputs.
- Invalid no-sign assembly operations.

The test contains exactly one intentional malformed instruction-name fixture:

    mint_xxxl_from_receipt_bound_gateway_messagee

This fixture is intentionally used to prove malformed receipt-bound assembly design input is rejected.

## Checks performed

Runtime checks passed:

    Stage 4.17 test: 4 passing
    Stage 4.16 + Stage 4.17 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 4.17 smoke: 63 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 4.17 is complete when:

- The runtime commit is recorded.
- The evidence document is committed.
- The current design checkpoint references the Stage 4.17 boundary.
- Build-lab typecheck, tests, and build pass.
- No wallet loading, private-key access, signing, recent blockhash fetch, compiled message creation, transaction object creation, serialization, simulation, transaction submission, or SOL spend is introduced.

Stage 4.17 does not authorize live transaction submission.

## Next stage

The next valid stage is:

    Stage 4.18 — receipt-bound transaction message planning boundary

Stage 4.18 may plan the message/account/runtime mapping more concretely, but must still preserve the no-wallet, no-private-key, no-sign, no-send, no-SOL boundary unless we explicitly decide to enter a later live-send stage.

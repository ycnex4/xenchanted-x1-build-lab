# Stage 4.16 Evidence — Receipt-Bound Transaction Assembly Design Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-16-receipt-bound-transaction-assembly-design-boundary

Runtime commit:

    1195de0 Add Stage 4.16 receipt-bound transaction assembly design boundary

## Purpose

Stage 4.16 adds the receipt-bound transaction assembly design boundary.

This stage does not create, serialize, sign, simulate, or submit a live transaction. It defines the offline design contract that a future transaction assembly step must preserve when turning the Stage 4.15 receipt-bound preflight result into a real runtime transaction path.

Stage 4.16 consumes the Stage 4.15 receipt-bound transaction preflight result and binds the future assembly design to:

- Stage 4.15 preflight artifact type.
- Stage 4.15 preflight stage.
- Source receipt digest.
- Source verification result digest.
- Receipt-bound instruction data digest.
- Exact instruction name.
- Account roles and writability/signing expectations.
- Instruction layout fields.
- No-wallet / no-sign / no-send safety contract.

## Runtime files added

    tests/helpers/stage4ReceiptBoundTransactionAssemblyDesignPrototype.ts
    tests/stage4_receipt_bound_transaction_assembly_design_boundary.test.ts

## Artifact introduced

    stage4_receipt_bound_transaction_assembly_design_result

Artifact metadata:

    schemaVersion: 1
    stage: 4.16
    executionMode: receipt_bound_transaction_assembly_design_offline

## Source dependency

Stage 4.16 requires a valid Stage 4.15 source artifact:

    stage4_receipt_bound_transaction_preflight_result

Required Stage 4.15 properties:

    sourcePreflightStage: 4.15
    sourcePreflightOk: true
    sourceReceiptDigest: required
    sourceResultDigest: required
    instructionDataDigest: required
    instructionName: mint_xxxl_from_receipt_bound_gateway_message
    transactionSerializationStatus: not_created_no_wallet_no_signing
    transactionSubmission: not_allowed
    simulation: not_performed
    signerCount: 0
    requiredSignatureCount: 0
    all account metas: non-signers

## Assembly design contract

Stage 4.16 defines the future transaction assembly contract as design-only:

    contractKind: receipt_bound_transaction_assembly_design
    sourcePreflightArtifactType: stage4_receipt_bound_transaction_preflight_result
    sourcePreflightStage: 4.15
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

This means Stage 4.16 is not a live transaction construction step. It is a boundary proving what the next construction step must preserve.

## Instruction layout

Stage 4.16 fixes the receipt-bound instruction layout fields:

    receiptDigest
    sourceResultDigest
    verifiedFeeBoundMessageDigest
    guardianSetVersion
    verifiedSignatureCount
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    messageBinding

Each field is required and sourced from Stage 4.15 receipt-bound instruction data.

## Digest binding

Stage 4.16 derives a deterministic assembly design digest from:

    preflightDigest
    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    instructionName
    programId
    payerPublicKey
    accountRolesDigest
    layoutFieldsDigest

The assembly design digest changes if the source preflight verification changes.

## Safety boundary

Stage 4.16 explicitly preserves:

    offlineOnly: true
    assemblyDesignOnly: true
    sourcePreflightStage415Bound: true
    sourceReceiptDigestBound: true
    sourceResultDigestBound: true
    instructionDataDigestBound: true
    assemblyDesignDigestBound: true
    instructionNameBound: true
    accountRolesBound: true
    instructionLayoutBound: true
    noWalletLoaded: true
    noPrivateKeys: true
    noSigning: true
    noSignerResolution: true
    noTransactionObjectCreated: true
    noCompiledMessageCreated: true
    noSimulation: true
    noLiveRpc: true
    noTransactionsSubmitted: true
    noSolSpend: true

## Negative coverage

Stage 4.16 rejects:

- Malformed design timestamps.
- Invalid network names.
- Invalid public keys.
- Invalid Stage 4.15 preflight artifacts.
- Failed Stage 4.15 preflight artifacts.
- Wrong instruction names.
- Assembly design digest mismatches.
- Forbidden values.
- Invalid assembly design operations.

The test contains exactly one intentional malformed instruction-name fixture:

    mint_xxxl_from_receipt_bound_gateway_messagee

This fixture is intentionally used to prove that malformed receipt-bound preflight input is rejected.

## Checks performed

Runtime checks passed:

    Stage 4.16 test: 4 passing
    Stage 4.15 + Stage 4.16 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 4.16 smoke: 59 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 4.16 is complete when:

- The runtime commit is recorded.
- The evidence document is committed.
- The current design checkpoint references the Stage 4.16 boundary.
- Build-lab typecheck, tests, and build pass.
- No live transaction path is introduced.

Stage 4.16 does not authorize live transaction submission.

## Next stage

The next valid stage is:

    Stage 4.17 — receipt-bound transaction assembly no-sign boundary

Stage 4.17 may model a no-sign assembly structure, but must still preserve:

    no wallet loading
    no private keys
    no signing
    no simulation
    no transaction submission
    no SOL spend

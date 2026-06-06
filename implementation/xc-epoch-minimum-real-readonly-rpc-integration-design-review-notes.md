# XC epoch minimum real read-only RPC integration design review notes

This review milestone checks the real read-only RPC integration design before any implementation.

Reviewed design:

    implementation/xc-epoch-minimum-real-readonly-rpc-integration-design.md

Reviewed branch:

    xc-epoch-minimum-real-readonly-rpc-integration-design

Reviewed commits:

    861f1d6 Add XC epoch minimum real read-only RPC integration design
    eb35c64 Update checkpoint after real read-only RPC integration design
    7736504 Merge branch 'xc-epoch-minimum-real-readonly-rpc-integration-design'

## Review conclusion

The real read-only RPC integration design boundary is clean.

This remains a design-only stage.

No real RPC implementation was added.

No runtime RPC execution was added.

No env reads were added in model or wrapper code.

No private keys, signers, wallet clients, or transaction-sending paths were added.

## Runtime / dependency review

A targeted grep confirmed that the design document contains RPC / env / secret / viem terms only as boundary rules, policy, conceptual examples, and non-goals.

The runtime source remains free from:

    real viem imports
    ethers imports
    process.env reads
    createPublicClient calls
    http transport construction
    RPC URL factories
    signer / wallet / write paths

The current branch remains review-only.

## Model and wrapper boundary review

The following layers must remain free from env reads and RPC URL construction:

    src/model/*
    src/ethereum/ethereum-viem-read-provider-wrapper.ts
    src/ethereum/ethereum-read-provider-wrapper.ts

Review decision:

    keep model layer provider-library agnostic
    keep wrappers free from RPC URL ownership
    keep public client construction outside model and wrapper
    pass constructed public client objects inward

## Config and env ownership review

Config ownership belongs to the outer integration layer.

Allowed future integration config:

    chainId
    lensAddress
    finalityPolicy
    lockEpochs
    epochMinimumFunctionName
    epochMinimumAbi
    public client object

Disallowed in model / wrapper input:

    RPC URL
    API key
    authorization header
    private key
    mnemonic
    signer
    wallet client
    account

If env is used later, it must be read only by an outer app / script / integration entrypoint.

Review decision:

    do not read process.env in model
    do not read process.env in wrapper
    do not read process.env in provider adapter
    do not read process.env in source builder

## RPC URL / API key policy review

RPC URLs and API keys are sensitive operational config.

They must not be:

    logged
    included in thrown error messages
    stored in snapshots
    stored in checkpoint records
    passed into model-layer constructors
    passed into createXcEpochMinimumSourceFromEthereumLensProvider()
    passed into createEthereumReadProviderFromViemPublicClient()

Review decision:

    first implementation should use a provided public client helper only
    do not add createEthereumReadProviderFromRpcUrl(rpcUrl)
    do not add createXcEpochMinimumSourceFromRpcUrl(rpcUrl)

## Public client construction review

The design correctly treats public client construction as an outer infrastructure concern.

Conceptual viem construction may happen later only outside model and wrapper code.

Review decision:

    do not add real public client construction in this design review
    do not add viem dependency in this review
    do not add real RPC tests in this review

## Read-only requirement review

The real integration path must remain read-only.

It must not require:

    private key
    mnemonic
    signer
    wallet client
    account
    sendTransaction
    writeContract
    approve
    transaction simulation for writes

Allowed read-only calls remain:

    getChainId()
    getBlock()
    readContract()

## Chain and address policy review

The integration should require explicit chainId and explicit Lens address.

The provider adapter already validates:

    configured chainId format
    provider chain ID match
    Lens address format
    finality policy
    lockEpochs
    selected block provenance
    read result shape

Review decision:

    outer integration must not bypass provider adapter checks
    future integration helper should pass inputs into existing provider adapter path

## Finality policy review

Supported finality policies remain:

    finalized
    safe
    confirmed

Unsupported as provenance policy:

    latest

Review decision:

    do not silently change finality policy
    do not silently downgrade finalized to latest
    do not silently downgrade safe to latest
    if finalized / safe is unsupported by a provider, surface a sanitized error
    any future fallback must be explicit in config and documented

## Confirmed policy review

Confirmed policy may read current head only to calculate an older confirmed block number.

Then all contract reads must use the selected confirmed block number.

Existing provider adapter behavior remains the source of truth.

## Provider error sanitization review

Allowed error context:

    operation name
    chain ID
    block tag
    block number
    contract address
    function name
    high-level provider failure category

Disallowed error context:

    RPC URL
    API key
    authorization header
    private key
    mnemonic
    signer object
    wallet account secret material
    full env dump
    transport internals that include URL / headers

Review decision:

    outer integration owns secret-bearing config
    future implementation must verify no RPC URL / API key appears in thrown messages
    wrapper may wrap low-level errors only if it never includes transport config

## Snapshot policy review

EthereumXcLensEpochMinimumSnapshot must not include:

    RPC URL
    API key
    env config
    provider object
    transport config
    private key
    signer
    wallet client

Snapshot may include:

    sourceChainId
    sourceBlockNumber
    sourceBlockHash
    observedAt
    finalizedPolicy
    epochMinimums

Review decision:

    keep snapshots portable and non-secret

## Logging policy review

No logging by default in reusable model / wrapper / provider layers.

Safe future script logs may include:

    chain ID
    finality policy
    selected block number
    selected block hash
    Lens address
    lockEpoch count
    function name

Unsafe fields include:

    RPC URL
    API key
    authorization header
    env dump
    private key
    mnemonic
    signer / wallet internals
    transport config

## ABI policy review

The real RPC integration should not hardcode a large ABI unless necessary.

Review decision:

    pass epochMinimumAbi explicitly from outer integration first
    pass epochMinimumFunctionName explicitly or use a safe default
    possible later minimal XC Lens ABI module must remain separate from model logic

## Testing strategy review

The next implementation milestone should still avoid real RPC.

Recommended mocked implementation tests:

    constructs source from a provided public client
    does not accept RPC URL in model-facing input
    does not read process.env in model / wrapper / integration helper
    passes public client into createEthereumReadProviderFromViemPublicClient()
    passes EthereumReadProvider into createXcEpochMinimumSourceFromEthereumLensProvider()
    preserves finalized finality policy
    preserves safe finality policy
    preserves confirmed finality policy
    does not downgrade finalized / safe
    propagates sanitized provider errors
    error messages do not contain RPC URL
    error messages do not contain API key
    snapshot does not contain RPC URL / API key / env config
    integration result works with authoritativeEpochMinimum(lockEpoch)

Real RPC tests should be a later separate milestone after mocked integration implementation and review.

## Verification

After review:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

    34 test files passed
    251 tests passed

## Final conclusion

The real read-only RPC integration design is ready for a mocked integration implementation milestone.

The next implementation should use a provided public client helper only, avoid real RPC, avoid env reads in model / wrapper / helper, avoid RPC URL factories, and verify sanitized error behavior.

Recommended next milestone:

    xc-epoch-minimum-mocked-readonly-rpc-integration

Suggested next scope:

    implement integration helper using provided public client
    no real RPC
    no env reads in model / wrapper / helper
    no secrets
    no RPC URL factory
    no private keys
    no signers
    no wallet client
    tests with mocked public client
    verify sanitized errors
    verify source integration

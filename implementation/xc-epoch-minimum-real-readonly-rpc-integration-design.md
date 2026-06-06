# XC epoch minimum real read-only RPC integration design

## Branch

xc-epoch-minimum-real-readonly-rpc-integration-design

## Purpose

This document designs the real read-only RPC integration boundary for the XC epoch minimum Ethereum provider path.

This is a design-only milestone.

It does not implement real RPC reads, does not add runtime RPC execution, does not read env in model or wrapper code, does not print secrets, does not accept private keys, does not accept signers, does not accept wallet clients, and does not send transactions.

## Current completed foundation

The provider path currently has these completed layers:

    EthereumReadProvider
    -> Ethereum Lens provider adapter
    -> EthereumXcLensEpochMinimumSnapshot
    -> Ethereum snapshot adapter
    -> XcEpochMinimumSource

The mocked real viem wrapper is already implemented and reviewed:

    src/ethereum/ethereum-viem-read-provider-wrapper.ts

It adapts a structurally typed viem-like PublicClient into EthereumReadProvider:

    viem-like PublicClient
    -> createEthereumReadProviderFromViemPublicClient(publicClient)
    -> EthereumReadProvider

The model layer remains provider-library agnostic.

## Design goal

The real read-only RPC integration should define where a real public client is constructed and how it is passed into the already-reviewed wrapper without moving RPC URLs, env, API keys, provider construction, or secret-bearing config into the model layer or wrapper.

Future intended flow:

    outer integration / script / app layer
    -> reads config from a safe source
    -> constructs real read-only public client
    -> passes public client into createEthereumReadProviderFromViemPublicClient(publicClient)
    -> passes EthereumReadProvider into createXcEpochMinimumSourceFromEthereumLensProvider()
    -> receives XcEpochMinimumSource

## Boundary rule

The following layers must remain free from env reads and RPC URL construction:

    src/model/*
    src/ethereum/ethereum-viem-read-provider-wrapper.ts
    src/ethereum/ethereum-read-provider-wrapper.ts

The real RPC integration boundary should live outside src/model and outside the generic wrapper.

Recommended future location candidates:

    src/ethereum/ethereum-readonly-rpc-integration.ts
    src/integration/ethereum-readonly-rpc-integration.ts
    scripts/read-xc-epoch-minimum-source.ts

Preferred direction:

    keep reusable integration code outside src/model
    keep CLI / script env handling outside reusable library code if possible
    pass constructed public client objects inward

## Config ownership

Config ownership belongs to the outer integration layer.

Config may include:

    chainId
    lensAddress
    finalityPolicy
    lockEpochs
    epochMinimumFunctionName
    epochMinimumAbi
    public client object

Config must not include in model/wrapper input:

    RPC URL
    API key
    authorization header
    private key
    mnemonic
    signer
    wallet client
    account

## Env ownership

The model layer must not read process.env.

The wrapper layer must not read process.env.

If env is used later, it must be read only by an outer app / script / integration entrypoint.

Allowed future pattern:

    script reads process.env.XC_ETHEREUM_RPC_URL
    script creates public client
    script passes public client to integration helper
    integration helper passes client to wrapper
    wrapper returns EthereumReadProvider
    provider adapter produces XcEpochMinimumSource

Disallowed pattern:

    model reads process.env
    wrapper reads process.env
    provider adapter reads process.env
    source builder reads process.env

## RPC URL / API key policy

RPC URLs and API keys are secret-bearing or sensitive operational config.

They must not be:

    logged
    included in thrown error messages
    stored in snapshots
    stored in checkpoint records
    passed into model-layer constructors
    passed into createXcEpochMinimumSourceFromEthereumLensProvider()
    passed into createEthereumReadProviderFromViemPublicClient()

A future public client construction helper, if added, should be treated as outer infrastructure.

Preferred first real integration approach:

    create the public client in a script / app entrypoint
    pass the public client object inward

Avoid first:

    createEthereumReadProviderFromRpcUrl(rpcUrl)
    createXcEpochMinimumSourceFromRpcUrl(rpcUrl)

## Public client construction

If viem is used later, the outer layer may construct a public client.

Conceptual example only:

    publicClient = createPublicClient({
      chain,
      transport: http(rpcUrl)
    })

This should not be added in this design milestone.

This should not be added to src/model.

If a reusable constructor is later added, it should live in an infrastructure module and must include strict error redaction policy.

## Read-only requirement

The real integration path must be read-only.

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

Allowed read-only calls:

    getChainId()
    getBlock()
    readContract()

## Chain and address policy

The integration should require explicit chainId and explicit Lens address.

The provider adapter already validates:

    configured chainId format
    provider chain ID match
    Lens address format
    finality policy
    lockEpochs
    selected block provenance
    read result shape

The outer integration should not bypass those checks.

Recommended future integration inputs:

    publicClient
    chainId
    lensAddress
    finalityPolicy
    lockEpochs
    epochMinimumFunctionName
    epochMinimumAbi

## Finality policy

Supported finality policies remain:

    finalized
    safe
    confirmed

Unsupported:

    latest as provenance policy

The real RPC integration should not silently change policy.

If finalized / safe is unsupported by a provider, the integration must surface a sanitized error.

Do not silently downgrade:

    finalized -> latest
    safe -> latest

If a fallback is ever added, it must be explicit in config and documented.

## Confirmed policy behavior

Confirmed policy may read the current head only to calculate an older confirmed block number.

Then all contract reads must use the selected confirmed block number.

Existing provider adapter behavior should remain the source of truth:

    getBlock({}) for head
    confirmedBlockNumber = head.number - confirmations
    getBlock({ blockNumber: confirmedBlockNumber })
    all readContract calls at selected block number

## Provider error sanitization

The real integration must define sanitized error handling.

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

Open design decision for implementation review:

    Should sanitization happen in the outer integration layer only?
    Or should the viem wrapper wrap provider errors with sanitized messages?

Preferred cautious decision:

    outer integration owns secret-bearing config
    wrapper can wrap low-level errors only if it never includes transport config
    tests should verify no RPC URL / API key appears in thrown messages

## Snapshot policy

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

This keeps snapshots portable and non-secret.

## Logging policy

No logging by default in reusable model / wrapper / provider layers.

If a future script logs, safe fields include:

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

## ABI policy

The real RPC integration should not hardcode a large ABI unless necessary.

Preferred initial approach:

    pass epochMinimumAbi explicitly from outer integration
    pass epochMinimumFunctionName explicitly or use a safe default

Possible later approach:

    add a minimal XC Lens ABI module
    keep it separate from model logic
    document source of ABI

## Testing strategy for future implementation

The next implementation milestone after design review should still avoid real RPC unless explicitly approved.

Recommended first implementation tests with mocked public client / mocked config:

1. constructs source from a provided public client
2. does not accept RPC URL in model-facing input
3. does not read process.env in model/wrapper/integration helper
4. passes public client into createEthereumReadProviderFromViemPublicClient()
5. passes EthereumReadProvider into createXcEpochMinimumSourceFromEthereumLensProvider()
6. preserves finalized finality policy
7. preserves safe finality policy
8. preserves confirmed finality policy
9. does not downgrade finalized / safe
10. propagates sanitized provider errors
11. error messages do not contain RPC URL
12. error messages do not contain API key
13. snapshot does not contain RPC URL / API key / env config
14. integration result works with authoritativeEpochMinimum(lockEpoch)

Real RPC tests should be a later separate milestone after design review and implementation review.

## Non-goals

This design does not add:

    real RPC execution
    viem dependency installation
    env reads
    RPC URL factory
    private key support
    signer support
    wallet client support
    transaction sending
    CLI command
    production address config
    snapshot persistence migration
    bridge signer verification
    X1-native verification

## Recommended next milestone

After this design is reviewed, the next milestone can be:

    xc-epoch-minimum-real-readonly-rpc-integration-design-review

Then, if accepted:

    xc-epoch-minimum-mocked-readonly-rpc-integration

Suggested implementation scope after review:

    implement integration helper using provided public client
    no real RPC
    no env reads in model/wrapper/helper
    no secrets
    no RPC URL factory
    no private keys
    no signers
    no wallet client
    tests with mocked public client
    verify sanitized errors
    verify source integration

## Conclusion

The real read-only RPC integration should be an outer infrastructure boundary.

It should construct or receive a real public client outside the model and wrapper layers, keep RPC URL / API key / env ownership outside protocol logic, and pass only a read-only public client inward to the existing viem-like wrapper and provider adapter path.

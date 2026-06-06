# XC epoch minimum script public client construction design

## Branch

xc-epoch-minimum-script-public-client-construction-design

## Purpose

This document designs the script-only public client construction boundary for the XC epoch minimum Ethereum provider path.

This is a design-only milestone.

It does not implement real RPC, does not install viem, does not add runtime viem imports, does not add env reads, does not add an RPC URL factory, does not accept private keys, does not accept signers, does not accept wallet clients, and does not send transactions.

## Current completed foundation

The current completed source path is:

    provided public client
    -> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
    -> createEthereumReadProviderFromReadonlyEthereumPublicClient()
    -> createEthereumReadProviderFromViemPublicClient()
    -> EthereumReadProvider
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> XcEpochMinimumSource

The real RPC read planning review concluded that the next milestone should design script-only public client construction.

## Design goal

Define how a future script will construct a real read-only public client while keeping secret-bearing configuration at the outermost edge.

The script should be the only layer allowed to read env.

The model layer, wrappers, and integration helper must remain free from env reads, RPC URL ownership, public client construction, private keys, signers, wallet clients, and transaction sending.

## Exact script path

Recommended future script path:

    scripts/read-xc-epoch-minimum-source.ts

Reason:

    script-only construction keeps RPC URL / API key ownership outside exported library APIs
    script-only construction is easier to gate with explicit confirmation
    script-only construction avoids creating a reusable RPC URL factory too early
    script-only construction keeps src/model and src/ethereum helper layers clean

This design does not create the script.

## Dependency decision

Recommended future dependency:

    viem

Reason:

    current wrapper is already viem-like
    viem PublicClient maps to getChainId / getBlock / readContract
    viem PublicClient does not require signer or wallet client
    viem supports read-only public client construction

Design-stage decision:

    do not install viem in this design milestone
    do not import viem in this design milestone
    decide exact viem version only in implementation or implementation-design milestone
    keep viem imports out of src/model permanently

## Future script responsibility

A future script may be responsible for:

    reading safe env config
    validating required fields
    constructing a read-only public client
    creating XcEpochMinimumSource through the existing helper
    printing safe summary output
    refusing unsafe or incomplete configuration

A future script must not:

    print RPC URL
    print API key
    print raw env object
    print full config object
    accept private key
    accept mnemonic
    construct signer
    construct wallet client
    send transaction
    call writeContract
    call sendTransaction
    run as part of npm test or CI if it uses real RPC

## Future env names

Required future env names:

    XC_ETHEREUM_RPC_URL
    XC_ETHEREUM_CHAIN_ID
    XC_ETHEREUM_LENS_ADDRESS
    XC_ETHEREUM_FINALITY
    XC_ETHEREUM_LOCK_EPOCHS

Required only for confirmed finality:

    XC_ETHEREUM_CONFIRMATIONS

Optional future env names:

    XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
    XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Manual real RPC confirmation:

    XC_ETHEREUM_REAL_RPC_CONFIRM

Required confirmation value:

    I_UNDERSTAND_THIS_USES_REAL_RPC

## Env parsing rules

XC_ETHEREUM_RPC_URL:

    required for real RPC script
    must be non-empty
    must not be printed
    must not be included in thrown errors
    must not be stored in snapshots

XC_ETHEREUM_CHAIN_ID:

    required
    expected format: eip155-N
    safe to print after validation

XC_ETHEREUM_LENS_ADDRESS:

    required
    expected format: 0x + 40 hex chars
    safe to print after validation

XC_ETHEREUM_FINALITY:

    required
    allowed values: finalized, safe, confirmed
    safe to print after validation

XC_ETHEREUM_CONFIRMATIONS:

    required only when XC_ETHEREUM_FINALITY=confirmed
    must be positive integer
    safe to print after validation

XC_ETHEREUM_LOCK_EPOCHS:

    required
    comma-separated positive integer list
    safe to print only as parsed count or explicit list if intentionally desired
    preferred first output: count only

XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION:

    optional
    default: epochMinimum
    safe to print after validation

XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH:

    optional
    if used, path should be local
    do not print file contents
    safe to print path only if it does not contain secrets
    preferred first implementation may avoid this and use minimal ABI design later

XC_ETHEREUM_REAL_RPC_CONFIRM:

    required for real RPC execution
    must equal I_UNDERSTAND_THIS_USES_REAL_RPC
    safe to print only as confirmation present / missing, not raw env dump

## Config object boundary

Future script may build an internal config object.

That config object must not be printed directly.

Safe derived config may include:

    chainId
    lensAddress
    finalityPolicy
    lockEpochCount
    functionName

Secret-bearing config includes:

    rpcUrl
    transport options
    authorization headers
    API-key-bearing URLs

Secret-bearing config must stay inside script-local construction scope.

It must not be passed into:

    src/model
    src/ethereum/ethereum-readonly-rpc-integration.ts
    createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
    createEthereumReadProviderFromReadonlyEthereumPublicClient()

Only the constructed public client object should be passed inward.

## Error handling design

Future script errors should be sanitized.

Allowed error context:

    operation name
    chain ID
    Lens address
    finality policy
    confirmations count
    lock epoch count
    block tag
    block number
    function name
    high-level failure category

Disallowed error context:

    RPC URL
    API key
    authorization header
    raw env object
    full config object
    transport config
    private key
    mnemonic
    signer object
    wallet client internals

Recommended future error helpers:

    sanitizeUnknownError(error)
    failWithSanitizedMessage(message)
    assertNoSecretLikeText(message)

This design does not implement these helpers.

## Provider unsupported finalized / safe policy

A future script must not silently downgrade:

    finalized -> latest
    safe -> latest

If provider does not support finalized or safe:

    fail with sanitized message
    recommend using explicit confirmed finality configuration if appropriate
    do not automatically change policy

## Future script output

Allowed output:

    real RPC confirmation accepted
    chain ID
    Lens address
    finality policy
    confirmations count if applicable
    lock epoch count
    function name
    selected block number
    selected block hash
    number of loaded epoch minimums

Disallowed output:

    RPC URL
    API key
    raw env values
    full config object
    transport config
    authorization headers
    private key
    mnemonic
    signer / wallet internals

## Package script design

Possible future package script:

    "smoke:xc-epoch-minimum:ethereum": "node ./dist/scripts/read-xc-epoch-minimum-source.js"

Design decision:

    do not add the package script in this design milestone
    add it only when a manual-only smoke implementation exists
    do not make it part of test, build, or CI

## Real RPC smoke policy

The future real RPC script should be manual-only.

It should require:

    npm run build
    XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

It should not run during:

    npm test
    npm run build
    CI

It should only perform:

    getChainId
    getBlock
    readContract

It must not perform:

    sendTransaction
    writeContract
    approve
    signer calls
    wallet calls

## Testing strategy for future implementation

Before any real RPC script is implemented, add mocked tests for env/config parsing and error sanitization.

Recommended mocked tests:

1. parses required env into safe config without printing RPC URL
2. rejects missing RPC URL with sanitized message
3. rejects invalid chain ID
4. rejects invalid Lens address
5. rejects invalid finality
6. requires confirmations for confirmed finality
7. rejects non-positive confirmations
8. parses lock epoch list
9. rejects empty lock epoch list
10. requires explicit real RPC confirmation
11. never includes RPC URL in error message
12. never includes API-key-looking text in error message
13. does not construct signer / wallet client
14. does not expose raw env object
15. passes only constructed public client into existing helper

Real RPC smoke test implementation should be a separate milestone after mocked config parsing and review.

## Non-goals

This design milestone does not add:

    real RPC
    viem dependency
    viem imports
    env reads
    public client construction
    RPC URL factory
    script implementation
    package script
    smoke test
    private key support
    signer support
    wallet client support
    transaction sending
    production address config

## Recommended next milestone

After this design is reviewed:

    xc-epoch-minimum-script-public-client-construction-design-review

Then, if accepted:

    xc-epoch-minimum-mocked-script-config-parsing

Suggested implementation scope after review:

    add mocked config parsing helpers
    no real RPC
    no viem dependency
    no public client construction
    no env reads outside test-provided objects
    no secrets printed
    no RPC URL leakage in errors
    tests only

## Conclusion

Script-only public client construction should keep real RPC at the outermost edge.

The future script may own env reading and public client construction, but model, wrappers, and integration helper must remain free from RPC URL ownership, env reads, provider construction, private keys, signers, wallet clients, and transaction sending.

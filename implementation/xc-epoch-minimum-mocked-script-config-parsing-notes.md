# XC epoch minimum mocked script config parsing notes

This milestone implements mocked Ethereum script config parsing for the future XC epoch minimum real RPC script path.

The implementation does not perform real RPC, does not install viem, does not import viem, does not construct a public client, does not read process.env directly, does not accept private keys, does not accept signers, does not accept wallet clients, and does not send transactions.

## Purpose

The previous script-only public client construction design review concluded that the next safe implementation should add mocked config parsing helpers.

This milestone adds a parser that accepts a test-provided env-like object:

    Record<string, string | undefined>

It does not read the real process.env object.

## Runtime additions

Added:

    src/ethereum/ethereum-script-config.ts

Exported through:

    src/index.ts

New exports:

    EthereumScriptConfigEnv
    EthereumScriptConfig
    EthereumScriptSafeConfigSummary
    parseEthereumScriptConfig()
    summarizeEthereumScriptConfig()

## Boundary

The parser lives outside src/model.

The model layer remains provider-library agnostic.

The parser does not import:

    viem
    ethers
    http
    createPublicClient
    wallet clients
    signer APIs
    transaction APIs

The parser does not call:

    process.env
    fetch
    sendTransaction
    writeContract

The parser does not construct:

    public client
    wallet client
    signer
    transaction sender

## Parsed config

The parser accepts:

    XC_ETHEREUM_RPC_URL
    XC_ETHEREUM_CHAIN_ID
    XC_ETHEREUM_LENS_ADDRESS
    XC_ETHEREUM_FINALITY
    XC_ETHEREUM_CONFIRMATIONS
    XC_ETHEREUM_LOCK_EPOCHS
    XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
    XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH
    XC_ETHEREUM_REAL_RPC_CONFIRM

The returned full config includes rpcUrl because a future outer script will need it to construct a read-only public client.

The safe summary intentionally excludes rpcUrl.

## Validation behavior

The parser validates:

    required RPC URL presence
    chain ID format eip155-N
    Lens address format 0x + 40 hex chars
    finality finalized / safe / confirmed
    confirmations required for confirmed finality
    confirmations positive integer
    lock epoch list non-empty
    lock epoch values numeric
    optional function name identifier shape
    explicit real RPC confirmation

The parser normalizes:

    Lens address to lowercase
    empty optional function name to epochMinimum
    empty optional ABI path to omitted optional property

The parser preserves exactOptionalPropertyTypes behavior by omitting optional fields when absent.

## Safe summary

summarizeEthereumScriptConfig() returns:

    chainId
    lensAddress
    finalityPolicy
    lockEpochCount
    epochMinimumFunctionName
    hasEpochMinimumAbiPath
    realRpcConfirmed

It does not return:

    rpcUrl
    API key
    raw env object
    full config object
    transport config

## Tests

Added:

    tests/ethereum-script-config.test.ts

Covered:

    parses required env into config
    normalizes Lens address to lowercase
    parses safe finality
    parses confirmed finality with confirmations
    parses optional function name and ABI path
    creates safe summary without RPC URL
    rejects missing RPC URL with sanitized error
    rejects invalid chain ID
    rejects invalid Lens address
    rejects invalid finality
    requires confirmations for confirmed finality
    rejects non-positive confirmations
    rejects empty lock epoch list
    rejects invalid lock epoch item
    requires explicit real RPC confirmation
    rejects invalid function name
    does not include RPC URL or API key in validation errors

## Verification

After implementation:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Test count after this milestone:

    36 test files passed
    278 tests passed

## Security / operational boundary

This milestone intentionally does not add:

    real Ethereum RPC
    viem dependency
    viem runtime imports
    ethers dependency
    process.env reads
    public client construction
    RPC URL factory
    private keys
    API keys as separate fields
    mnemonic
    signer support
    wallet client support
    account support
    transaction sending
    CLI commands
    package scripts
    production address config
    snapshot persistence
    bridge signer verification
    X1-native verification

## Conclusion

The mocked script config parsing layer is safe to keep.

It prepares a future manual-only real RPC script while keeping env ownership mocked/test-provided, keeping real RPC out of the runtime path, and ensuring safe summaries and validation errors do not leak RPC URL or API-key-like values.

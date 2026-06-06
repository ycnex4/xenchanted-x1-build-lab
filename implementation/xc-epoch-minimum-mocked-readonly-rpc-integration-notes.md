# XC epoch minimum mocked read-only RPC integration notes

This milestone implements the mocked read-only RPC integration helper using a provided public client.

The implementation does not perform real RPC, does not read env, does not accept secrets, does not add an RPC URL factory, does not accept private keys, does not accept signers, does not accept wallet clients, and does not send transactions.

## Purpose

The previous real read-only RPC integration design review concluded that the first implementation should use a provided public client helper only.

This milestone adds a thin orchestration layer:

    provided public client
    -> createEthereumReadProviderFromReadonlyEthereumPublicClient(publicClient)
    -> EthereumReadProvider
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> XcEpochMinimumSource

## Runtime additions

Added:

    src/ethereum/ethereum-readonly-rpc-integration.ts

Exported through:

    src/index.ts

New exports:

    EthereumReadonlyRpcIntegrationInput
    createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
    createEthereumReadProviderFromReadonlyEthereumPublicClient()

## Boundary

The helper lives outside src/model.

The helper does not construct a public client.

The helper receives an already-created public client object.

The helper does not accept:

    RPC URL
    API key
    authorization header
    private key
    mnemonic
    signer
    wallet client
    account
    env config

No viem dependency was installed.

No viem runtime import was added.

No ethers dependency or import was added.

No real RPC execution was added.

## Input shape

The helper accepts:

    publicClient
    chainId
    lensAddress
    finalityPolicy
    lockEpochs
    optional epochMinimumFunctionName
    optional epochMinimumAbi

The optional fields are only forwarded when present, preserving exactOptionalPropertyTypes behavior.

## Flow

createXcEpochMinimumSourceFromReadonlyEthereumPublicClient(input):

    create EthereumReadProvider from input.publicClient
    pass provider and source config into createXcEpochMinimumSourceFromEthereumLensProvider()
    return XcEpochMinimumSource

createEthereumReadProviderFromReadonlyEthereumPublicClient(publicClient):

    delegates to createEthereumReadProviderFromViemPublicClient(publicClient)

## Tests

Added:

    tests/ethereum-readonly-rpc-integration.test.ts

Covered:

    creates EthereumReadProvider from provided public client
    constructs source from provided public client without real RPC
    preserves finalized finality policy
    preserves safe finality policy
    preserves confirmed finality policy
    passes explicit function name and ABI through
    uses provider adapter defaults when optional metadata is omitted
    does not downgrade finalized to latest
    does not downgrade safe to latest
    propagates sanitized provider errors without adding RPC URL or API key
    does not expose RPC URL or API key in successful source state

## Verification

After implementation:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Test count after this milestone:

    35 test files passed
    261 tests passed

## Security / operational boundary

This milestone intentionally does not add:

    real Ethereum RPC
    viem dependency
    viem runtime imports
    ethers dependency
    env reads
    RPC URL factory
    private keys
    API keys
    mnemonic
    signer support
    wallet client support
    account support
    transaction sending
    CLI commands
    production address config
    snapshot persistence
    bridge signer verification
    X1-native verification

## Conclusion

The mocked read-only RPC integration helper now proves the intended outer integration boundary using a provided public client.

It keeps RPC URL / API key / env ownership outside model, wrapper, and helper code while integrating with the existing Ethereum provider adapter path.

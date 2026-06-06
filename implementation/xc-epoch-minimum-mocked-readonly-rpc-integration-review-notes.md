# XC epoch minimum mocked read-only RPC integration review notes

This review milestone checks the mocked read-only RPC integration helper boundary before any real RPC integration planning.

Reviewed runtime:

    src/ethereum/ethereum-readonly-rpc-integration.ts

Reviewed tests:

    tests/ethereum-readonly-rpc-integration.test.ts

Reviewed notes:

    implementation/xc-epoch-minimum-mocked-readonly-rpc-integration-notes.md

Reviewed commits:

    cd29e08 Add mocked read-only RPC integration helper
    f83b1ac Add mocked read-only RPC integration notes
    b3aed05 Update checkpoint after mocked read-only RPC integration
    eb2b661 Merge branch 'xc-epoch-minimum-mocked-readonly-rpc-integration'

## Review conclusion

The mocked read-only RPC integration helper boundary is clean.

The helper lives in:

    src/ethereum/ethereum-readonly-rpc-integration.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The model layer does not import the helper.

The helper imports model-facing provider adapter types and the existing viem-like wrapper, which is the intended dependency direction.

## Runtime / dependency boundary review

The helper does not import or call:

    viem
    ethers
    process.env
    fetch
    http / https
    createPublicClient
    wallet APIs
    signer APIs
    transaction APIs

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

The helper accepts only a provided public client and source configuration.

A targeted grep over the runtime helper, test file, and notes file found RPC / secret / provider-library terms only in notes and test descriptions where they are documented as boundaries or non-goals.

A targeted grep over src/model found no dependency on the integration helper.

## Helper behavior review

createXcEpochMinimumSourceFromReadonlyEthereumPublicClient(input):

    receives provided public client
    creates EthereumReadProvider from input.publicClient
    passes provider and source config into createXcEpochMinimumSourceFromEthereumLensProvider()
    returns XcEpochMinimumSource

createEthereumReadProviderFromReadonlyEthereumPublicClient(publicClient):

    delegates to createEthereumReadProviderFromViemPublicClient(publicClient)

This is the correct thin orchestration layer.

It does not construct a public client.

It does not own RPC URL / API key / env configuration.

## Input shape review

The helper accepts:

    publicClient
    chainId
    lensAddress
    finalityPolicy
    lockEpochs
    optional epochMinimumFunctionName
    optional epochMinimumAbi

The optional fields are forwarded only when present.

This correctly preserves exactOptionalPropertyTypes behavior.

## Finality behavior review

The helper preserves the existing provider adapter behavior.

Covered finality policies:

    finalized
    safe
    confirmed

Review decision:

    helper does not reinterpret finality policy
    helper does not downgrade finalized to latest
    helper does not downgrade safe to latest
    confirmed behavior remains controlled by the provider adapter

## Source integration review

The helper correctly integrates:

    provided public client
    -> viem-like read provider wrapper
    -> EthereumReadProvider
    -> Ethereum Lens provider adapter
    -> XcEpochMinimumSource

The helper does not bypass provider adapter validation.

## Test coverage review

The current tests cover:

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

This is sufficient for the current mocked read-only RPC integration milestone.

## Additional edge-case test decision

No additional tests are required before merging this review milestone.

Possible future tests for real RPC integration planning may include:

    explicit provider unsupported finalized / safe behavior
    provider error redaction with secret-bearing transport config
    real public client construction owned by script / outer app
    no RPC URL leakage from outer entrypoint errors
    no API key leakage from outer entrypoint errors

Those are not required in the current mocked helper layer.

## Security / operational review

This milestone does not add:

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

## Verification

After implementation and review:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

    35 test files passed
    261 tests passed

## Conclusion

The mocked read-only RPC integration helper is safe to keep as the provided-public-client integration boundary.

It proves the source integration path without moving RPC URL, API key, env, public-client construction, private keys, signers, wallet clients, or transaction sending into model, wrapper, or helper code.

Recommended next milestone:

    xc-epoch-minimum-real-rpc-read-planning

Suggested next scope:

    plan real read-only RPC usage
    decide if viem dependency is actually needed
    decide where the real public client construction will live
    define safe env names without printing values
    define exact redacted error handling for outer entrypoint
    define whether real RPC smoke test should exist as manual-only
    do not implement real RPC until planning / review is complete

# XC epoch minimum mocked Ethereum provider wrapper review notes

This review milestone checks the mocked Ethereum provider wrapper boundary before real viem wrapper design.

Reviewed runtime:

    src/ethereum/ethereum-read-provider-wrapper.ts

Reviewed tests:

    tests/ethereum-read-provider-wrapper.test.ts

Reviewed notes:

    implementation/xc-epoch-minimum-mocked-ethereum-provider-wrapper-notes.md

Reviewed commits:

    14d800a Add mocked Ethereum read provider wrapper
    8b67431 Add mocked Ethereum provider wrapper notes
    46be704 Update checkpoint after mocked Ethereum provider wrapper
    85de67b Merge branch 'xc-epoch-minimum-mocked-ethereum-provider-wrapper'

## Review conclusion

The mocked Ethereum provider wrapper boundary is clean.

The wrapper lives in:

    src/ethereum/ethereum-read-provider-wrapper.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The model layer does not import the wrapper.

The wrapper imports model-facing EthereumReadProvider types, which is the intended dependency direction.

## Runtime boundary review

The wrapper does not import or call:

    process.env
    fetch
    http / https
    viem
    ethers
    wallet APIs
    signer APIs

The wrapper does not accept:

    RPC URL
    private key
    mnemonic
    API key
    signer
    wallet account
    env config

A targeted grep over the runtime file, test file, and notes file found RPC / secret / provider-library terms only in the notes file where they are documented as non-goals.

A targeted grep over src/model found no references to:

    ethereum-read-provider-wrapper
    createEthereumReadProviderFromPublicClient

This confirms that src/model does not depend on the wrapper.

## Public client shape review

The wrapper uses a mocked viem-style public client shape:

    EthereumPublicClientLike {
      getChainId()
      getBlock()
      readContract()
    }

This remains dependency-free.

No real viem dependency was introduced.

No ethers dependency was introduced.

## Mapping review

getChainId:

    number | bigint -> bigint

getBlock:

    { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
    { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
    { blockNumber } -> publicClient.getBlock({ blockNumber })
    {} -> publicClient.getBlock({ blockTag: "latest" })

Review decision:

    empty input maps to latest only for confirmed-policy head calculation
    wrapper does not reinterpret empty input as finalized or safe

Block result mapping:

    missing block -> null
    missing block number -> null
    hash preserved as string | null
    timestamp number | bigint -> bigint

readContract:

    address passed unchanged
    abi passed unchanged
    functionName passed unchanged
    args passed unchanged
    blockNumber passed unchanged
    raw decoded result returned as unknown

## Test coverage review

The current tests cover:

    getChainId number result maps to bigint
    getChainId bigint result maps to bigint
    finalized block tag maps to public client getBlock
    safe block tag maps to public client getBlock
    number timestamp maps to bigint timestamp
    blockNumber read maps to public client getBlock
    empty getBlock input maps to latest head block read
    missing block maps to null
    missing block number maps to null
    missing block hash maps to hash null
    readContract input passes through unchanged
    integration with existing Ethereum Lens provider adapter without real RPC

This is sufficient for the current mocked wrapper milestone.

## Additional edge-case test decision

No additional tests are required before merging this review milestone.

Possible future tests for real viem wrapper design / implementation may include:

    viem-specific block timestamp shape
    viem-specific block number shape
    viem-specific null block response
    viem-specific readContract error mapping
    finalized / safe support differences across providers

Those are not required in the current dependency-free mocked wrapper layer.

## Security / operational review

This milestone does not add:

    real Ethereum RPC
    env reads
    RPC URL factory
    private keys
    API keys
    mnemonic
    signer support
    wallet support
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

    33 test files passed
    238 tests passed

## Conclusion

The mocked Ethereum provider wrapper is safe to keep as the dependency-free infrastructure wrapper boundary.

It adapts a viem-style public client shape into EthereumReadProvider without moving provider-library dependencies, RPC URLs, env, secrets, or signers into the model layer.

Recommended next milestone:

    xc-epoch-minimum-real-viem-wrapper-design

Suggested next scope:

    design real viem wrapper boundary only
    decide exact viem public client type shape
    define viem getBlock mapping
    define viem readContract mapping
    define error redaction policy
    keep RPC URL / env / API keys outside model code
    do not implement real RPC until design review is complete

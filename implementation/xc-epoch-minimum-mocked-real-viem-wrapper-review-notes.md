# XC epoch minimum mocked real viem wrapper review notes

This review milestone checks the mocked real viem wrapper boundary before real read-only RPC integration design.

Reviewed runtime:

    src/ethereum/ethereum-viem-read-provider-wrapper.ts

Reviewed tests:

    tests/ethereum-viem-read-provider-wrapper.test.ts

Reviewed notes:

    implementation/xc-epoch-minimum-mocked-real-viem-wrapper-notes.md

Reviewed commits:

    b6eb538 Add mocked real viem read provider wrapper
    e602c59 Add mocked real viem wrapper notes
    90ffb59 Update checkpoint after mocked real viem wrapper
    999f6ce Merge branch 'xc-epoch-minimum-mocked-real-viem-wrapper'

## Review conclusion

The mocked real viem wrapper boundary is clean.

The wrapper lives in:

    src/ethereum/ethereum-viem-read-provider-wrapper.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The model layer does not import the viem wrapper.

The wrapper imports only model-facing EthereumReadProvider types, which is the intended dependency direction.

## Runtime / dependency boundary review

The wrapper does not import or call:

    viem
    ethers
    process.env
    fetch
    http / https
    createPublicClient
    wallet APIs
    signer APIs
    transaction APIs

The wrapper does not accept:

    RPC URL
    private key
    mnemonic
    API key
    signer
    wallet client
    account
    env config

A targeted grep over the runtime file, test file, and notes file found RPC / secret / provider-library terms only in notes and test descriptions where they are documented as boundaries or non-goals.

A targeted grep over src/model found no references to:

    ethereum-viem-read-provider-wrapper
    createEthereumReadProviderFromViemPublicClient
    viem
    ethers

This confirms that src/model remains provider-library agnostic.

## Structurally typed client review

The wrapper uses a structurally typed read-only viem-like client:

    ViemLikePublicClient {
      getChainId()
      getBlock()
      readContract()
    }

This is the correct first implementation boundary.

No npm install viem was required.

No official viem types are required yet.

## Mapping review

getChainId:

    number -> bigint

getBlock:

    { blockTag: "finalized" } -> publicClient.getBlock({ blockTag: "finalized" })
    { blockTag: "safe" } -> publicClient.getBlock({ blockTag: "safe" })
    { blockNumber } -> publicClient.getBlock({ blockNumber })
    {} -> publicClient.getBlock({ blockTag: "latest" })

Review decision:

    empty input maps to latest only for confirmed-policy head calculation
    wrapper does not reinterpret empty input as finalized or safe
    wrapper does not silently downgrade finalized or safe to latest

Block result mapping:

    null block -> null
    null block number -> null
    null block hash -> hash: null
    timestamp number | bigint -> bigint

readContract:

    address passed unchanged
    abi passed unchanged
    functionName passed unchanged
    args passed unchanged
    blockNumber passed unchanged
    decoded result returned as unknown

The wrapper does not validate epoch minimum economics.

## Error propagation review

The current implementation intentionally does not wrap errors.

It propagates getBlock and readContract errors as received from the supplied mocked client.

This is acceptable for the current dependency-free mocked milestone because the wrapper does not know about RPC URLs, env, headers, private keys, signers, or transport config.

Future real read-only RPC integration design should define whether outer integration sanitizes provider errors or whether the wrapper should wrap them with sanitized messages.

## Test coverage review

The current tests cover:

    viem getChainId number maps to bigint
    finalized block tag maps to viem getBlock
    safe block tag maps to viem getBlock
    number timestamp maps to bigint timestamp
    blockNumber read maps to viem getBlock
    empty getBlock input maps to latest head block read
    null block maps to null
    null block number maps to null
    null block hash maps to hash null
    readContract input passes through unchanged
    readContract result returns as unknown
    getBlock errors propagate without adding secret-bearing config
    readContract errors propagate without adding secret-bearing config
    integration with existing Ethereum Lens provider adapter without real RPC

This is sufficient for the current mocked real viem wrapper milestone.

## Additional edge-case test decision

No additional tests are required before merging this review milestone.

Possible future tests for real read-only RPC integration design / implementation may include:

    provider error redaction policy
    finalized / safe unsupported-provider behavior
    real viem block timestamp type confirmation
    real viem readContract decoded result shape
    outer config construction without exposing RPC URL
    integration-level chain mismatch handling with real provider wrapper

Those are not required in the current structurally typed mocked wrapper layer.

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

    34 test files passed
    251 tests passed

## Conclusion

The mocked real viem wrapper is safe to keep as the structurally typed, dependency-free viem-like infrastructure boundary.

It adapts a viem-like PublicClient into EthereumReadProvider and integrates with the existing Ethereum Lens provider adapter without real network access.

Recommended next milestone:

    xc-epoch-minimum-real-readonly-rpc-integration-design

Suggested next scope:

    design real read-only RPC integration boundary
    decide where public client is constructed
    define config and env ownership outside model / wrapper
    define RPC URL / API key redaction policy
    define provider error sanitization policy
    define finalized / safe unsupported-provider behavior
    keep model layer provider-library agnostic
    do not implement real RPC until design review is complete

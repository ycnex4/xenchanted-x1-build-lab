# XC epoch minimum real viem wrapper design review notes

This review milestone checks the real viem wrapper design before any implementation.

Reviewed design:

    implementation/xc-epoch-minimum-real-viem-wrapper-design.md

Reviewed branch:

    xc-epoch-minimum-real-viem-wrapper-design

Reviewed commits:

    9289deb Add XC epoch minimum real viem wrapper design
    fb8f264 Update checkpoint after XC epoch minimum real viem wrapper design
    0717d05 Merge branch 'xc-epoch-minimum-real-viem-wrapper-design'

## Review conclusion

The real viem wrapper design boundary is clean.

This remains a design-only stage.

No runtime viem imports were added.

No viem dependency was installed.

No real RPC behavior was added.

The model layer remains viem-free.

## Runtime / dependency review

A targeted grep confirmed:

    no real viem imports in src
    no ethers imports in src
    no process.env reads in runtime wrapper path
    no direct RPC URL factory
    no signer / wallet / write path in runtime wrapper path

The design document contains viem / RPC / env / secret terms only as explicit boundary rules, non-goals, and future implementation policy.

## Model-layer boundary review

The following model files must remain free from viem imports:

    src/model/ethereum-xc-epoch-minimum-provider-source.ts
    src/model/ethereum-xc-epoch-minimum-source.ts
    src/model/xc-epoch-minimum-source.ts

Review decision:

    keep src/model provider-library agnostic
    keep EthereumReadProvider as the model-facing abstraction
    place real viem wrapper outside src/model

Recommended future location remains:

    src/ethereum/ethereum-viem-read-provider-wrapper.ts

## PublicClient construction boundary review

The design correctly prevents the wrapper from constructing a viem PublicClient from RPC URL.

Allowed future pattern:

    outer integration reads config
    outer integration creates viem PublicClient
    wrapper receives PublicClient
    wrapper returns EthereumReadProvider

Disallowed in wrapper:

    createPublicClient({ transport: http(process.env.RPC_URL) })
    process.env reads
    direct RPC URL input
    API key input
    private key input
    wallet client input
    signer input

Review decision:

    keep createEthereumReadProviderFromViemPublicClient(publicClient)-style construction
    do not add createEthereumReadProviderFromRpcUrl(rpcUrl) in the next implementation

## Real viem PublicClient shape review

The wrapper should depend only on read-only PublicClient capabilities:

    getChainId()
    getBlock()
    readContract()

The wrapper must not require:

    walletClient
    account
    signer
    sendTransaction
    writeContract
    private key
    mnemonic

Review decision:

    next implementation should stay read-only and structurally typed first
    do not add real RPC execution yet

## Mapping review

Chain ID:

    viem getChainId number -> bigint

Block reads:

    finalized -> publicClient.getBlock({ blockTag: "finalized" })
    safe -> publicClient.getBlock({ blockTag: "safe" })
    blockNumber -> publicClient.getBlock({ blockNumber })
    {} -> publicClient.getBlock({ blockTag: "latest" })

Review decision:

    empty input remains latest only for confirmed-policy head calculation
    do not reinterpret empty input as finalized or safe

Finality caveat:

    do not silently downgrade finalized / safe to latest
    if provider does not support finalized / safe, surface a sanitized error

Block mapping:

    missing block -> null
    missing block number -> null
    missing hash -> hash: null
    timestamp -> bigint

Contract read mapping:

    pass address unchanged
    pass abi unchanged
    pass functionName unchanged
    pass args unchanged
    pass blockNumber unchanged
    return decoded result as unknown

The wrapper must not validate epoch minimum economics.

## Address / ABI boundary review

The design keeps:

    model-facing address as string
    model-facing abi as unknown

Review decision:

    wrapper may cast after provider adapter validation
    wrapper should not loosen validation in model layer
    wrapper should not hardcode XC Lens ABI at this stage

## Error redaction review

Allowed error context:

    operation name
    block tag
    block number
    chain ID
    contract address
    function name

Disallowed error context:

    RPC URL
    API key
    authorization header
    env dump
    private key
    mnemonic
    signer object
    full transport config

Review decision:

    wrapper should not log by default
    future wrapped errors should use sanitized messages

## Dependency policy decision

The next implementation should stay structurally typed first.

Do not install viem yet unless the implementation clearly needs official types.

If viem is later added, keep imports isolated in:

    src/ethereum

and never in:

    src/model

## Testing strategy review

The next implementation should use mocked viem PublicClient objects.

No real RPC test in the implementation milestone.

Recommended tests:

    maps viem getChainId number to bigint
    maps finalized block tag to publicClient.getBlock({ blockTag: "finalized" })
    maps safe block tag to publicClient.getBlock({ blockTag: "safe" })
    maps blockNumber to publicClient.getBlock({ blockNumber })
    maps empty input to publicClient.getBlock({ blockTag: "latest" })
    maps null block to null
    maps null block number to null
    maps null block hash to hash null
    maps bigint timestamp to bigint
    maps number timestamp to bigint if test shape allows number
    passes readContract address / abi / functionName / args / blockNumber unchanged
    returns readContract result as unknown
    propagates sanitized getBlock errors
    propagates sanitized readContract errors
    does not read process.env
    does not accept RPC URL
    does not require private key
    does not require signer / wallet client
    integration with createXcEpochMinimumSourceFromEthereumLensProvider using mocked viem client

## Verification

After review:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

    33 test files passed
    238 tests passed

## Final conclusion

The real viem wrapper design is ready for a mocked implementation milestone.

The next implementation should stay structurally typed, use mocked viem PublicClient objects, and still avoid real RPC, env reads, RPC URL factories, private keys, signers, wallets, and transaction sending.

Recommended next milestone:

    xc-epoch-minimum-mocked-real-viem-wrapper

Suggested next scope:

    implement real viem wrapper boundary with mocked viem client
    no npm install viem unless clearly needed
    no real RPC
    no env reads
    no secrets
    no RPC URL factory
    no private keys
    no signers
    no wallet client
    no transaction sending
    tests only with mocked viem client
    integration test with existing provider adapter

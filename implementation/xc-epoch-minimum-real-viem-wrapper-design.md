# XC epoch minimum real viem wrapper design

## Branch

xc-epoch-minimum-real-viem-wrapper-design

## Purpose

This document designs the real viem wrapper boundary for the XC epoch minimum Ethereum provider path.

This is a design-only milestone.

It does not implement real RPC reads, does not install viem, does not add runtime viem imports, does not read env, does not accept secrets, does not accept private keys, does not accept signers, and does not add a direct RPC URL factory.

## Current completed foundation

The current dependency-free wrapper is already implemented and reviewed:

    src/ethereum/ethereum-read-provider-wrapper.ts

It adapts a mocked viem-style public client shape into:

    EthereumReadProvider

The provider path is now:

    public client-like object
    -> createEthereumReadProviderFromPublicClient(publicClient)
    -> EthereumReadProvider
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> EthereumXcLensEpochMinimumSnapshot
    -> createXcEpochMinimumSourceFromEthereumLensSnapshot()
    -> XcEpochMinimumSource

The model layer remains provider-library agnostic.

## Design goal

The real viem wrapper should adapt an actual viem PublicClient into the existing EthereumReadProvider interface without changing the model layer.

The future flow should be:

    outer integration / app / script layer
    -> creates viem PublicClient
    -> passes PublicClient to viem wrapper
    -> viem wrapper exposes EthereumReadProvider
    -> existing provider adapter consumes EthereumReadProvider

## Boundary rule

The model layer must not import viem.

These files must remain free from viem imports:

    src/model/ethereum-xc-epoch-minimum-provider-source.ts
    src/model/ethereum-xc-epoch-minimum-source.ts
    src/model/xc-epoch-minimum-source.ts

The real viem wrapper should live outside src/model.

Recommended future location:

    src/ethereum/ethereum-viem-read-provider-wrapper.ts

## Dependency direction

Allowed dependency direction:

    src/ethereum/ethereum-viem-read-provider-wrapper.ts
    -> imports viem types / functions if needed
    -> imports EthereumReadProvider model-facing types

Disallowed dependency direction:

    src/model/*
    -> imports viem
    src/model/*
    -> imports real viem wrapper

The model layer must only know about EthereumReadProvider.

## PublicClient construction boundary

The wrapper should not construct viem PublicClient from RPC URL.

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

Preferred future factory shape:

    createEthereumReadProviderFromViemPublicClient(publicClient)

Avoid in first implementation:

    createEthereumReadProviderFromRpcUrl(rpcUrl)

Reason:

    RPC URL / API key ownership belongs to infrastructure configuration
    model and wrapper layers should not log or store secret-bearing config
    tests can mock PublicClient-like objects
    signer/wallet capabilities remain outside this read-only path

## Real viem PublicClient shape

The real wrapper should depend only on read-only PublicClient capabilities.

Required methods:

    getChainId()
    getBlock()
    readContract()

Do not require:

    walletClient
    account
    signer
    sendTransaction
    writeContract
    simulateContract unless later explicitly needed
    private key
    mnemonic

## Chain ID mapping

Viem getChainId returns a number.

Wrapper mapping:

    number -> bigint

The Ethereum provider adapter later maps bigint into:

    eip155-<number>

and compares it with configured chainId.

The wrapper should not decide chain correctness.

It should only return the chain ID.

## Block read mapping

Existing EthereumBlockReadInput supports:

    { blockTag: "finalized" }
    { blockTag: "safe" }
    { blockNumber: bigint }
    {}

Real viem wrapper mapping should be:

    finalized -> publicClient.getBlock({ blockTag: "finalized" })
    safe -> publicClient.getBlock({ blockTag: "safe" })
    blockNumber -> publicClient.getBlock({ blockNumber })
    {} -> publicClient.getBlock({ blockTag: "latest" })

The empty input remains only for confirmed-policy head calculation.

The wrapper must not reinterpret empty input as finalized or safe.

## Finality support caveat

Some providers may not support finalized / safe tags consistently.

The wrapper should not silently downgrade:

    finalized -> latest
    safe -> latest

If viem / provider returns an error for unsupported finalized or safe block tags, the wrapper should surface a sanitized read error.

A later integration layer may choose a fallback policy explicitly, but fallback must not be hidden inside the wrapper.

## Block result mapping

Viem block result should be mapped into:

    EthereumBlockSnapshot {
      number: bigint;
      hash: string | null;
      timestamp: bigint;
    }

Mapping policy:

    missing block -> null
    missing block number -> null
    missing hash -> hash: null
    timestamp -> bigint

If viem returns timestamp as bigint, preserve it.

If viem returns timestamp as number in any mocked / test shape, convert explicitly.

If viem returns an unexpected timestamp shape, wrapper should throw sanitized error or map through a small explicit decoder.

## Contract read mapping

Existing EthereumContractReadInput:

    address
    abi
    functionName
    args
    blockNumber

Real viem wrapper mapping:

    publicClient.readContract({
      address,
      abi,
      functionName,
      args,
      blockNumber
    })

Required behavior:

    pass address unchanged
    pass abi unchanged
    pass functionName unchanged
    pass args unchanged
    pass blockNumber unchanged
    return decoded result as unknown

The wrapper should not validate epoch minimum economics.

The existing provider adapter decodes the expected result as bigint.

## Address typing

The model-facing type currently uses:

    address: string

Viem may expect a stricter 0x address type.

Design decision:

    keep model-facing address as string
    wrapper may cast after model/provider adapter validation
    wrapper should not loosen validation in model layer

The provider adapter already validates Lens address format before contract reads.

## ABI typing

The model-facing type currently uses:

    abi: unknown

Design decision:

    keep model-facing abi as unknown
    wrapper can pass it through to viem readContract
    avoid hardcoding XC Lens ABI in wrapper

A minimal XC Lens ABI module may be added later in an integration/config layer if needed.

## Error redaction policy

The real viem wrapper must not leak secret-bearing config.

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

The wrapper should not log by default.

If errors are wrapped, use sanitized messages.

## Logging policy

The wrapper should not log by default.

If future integration logs are added outside the wrapper, safe fields include:

    chain ID
    block tag
    block number
    block hash
    contract address
    function name

Unsafe fields include:

    RPC URL
    API keys
    authorization headers
    private keys
    mnemonic
    env dump
    viem transport internals

## Testing strategy for future implementation

The first real viem wrapper implementation should still use mocked viem PublicClient objects.

No real RPC test in the implementation milestone.

Recommended tests:

1. maps viem getChainId number to bigint
2. maps finalized block tag to publicClient.getBlock({ blockTag: "finalized" })
3. maps safe block tag to publicClient.getBlock({ blockTag: "safe" })
4. maps blockNumber to publicClient.getBlock({ blockNumber })
5. maps empty input to publicClient.getBlock({ blockTag: "latest" })
6. maps null block to null
7. maps null block number to null
8. maps null block hash to hash null
9. maps bigint timestamp to bigint
10. maps number timestamp to bigint if test shape allows number
11. passes readContract address / abi / functionName / args / blockNumber unchanged
12. returns readContract result as unknown
13. propagates sanitized getBlock errors
14. propagates sanitized readContract errors
15. does not read process.env
16. does not accept RPC URL
17. does not require private key
18. does not require signer / wallet client
19. integration with createXcEpochMinimumSourceFromEthereumLensProvider using mocked viem client

## Dependency policy

The design review should decide whether the next implementation adds a real viem dev/runtime dependency or continues with structural typing only.

Preferred cautious path:

    keep wrapper structurally typed first
    no npm install viem until an implementation truly needs official types
    if viem is added, keep import isolated in src/ethereum only

## Non-goals

This design does not add:

    real Ethereum RPC
    viem runtime code
    npm install viem
    env reads
    RPC URL factory
    private key support
    signer support
    wallet support
    transaction sending
    CLI commands
    production address config
    snapshot persistence
    bridge signer verification
    X1-native verification

## Recommended next milestone

After this design is reviewed, the next milestone can be:

    xc-epoch-minimum-real-viem-wrapper-design-review

Then, if accepted:

    xc-epoch-minimum-mocked-real-viem-wrapper

Suggested implementation scope after review:

    implement real viem wrapper boundary with mocked viem client
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

## Conclusion

The real viem wrapper should remain a read-only infrastructure adapter outside the model layer.

It should adapt a viem PublicClient into EthereumReadProvider while keeping RPC URLs, env, API keys, signers, wallets, and real network execution outside this layer.

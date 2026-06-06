# XC epoch minimum Ethereum provider wrapper design

## Branch

xc-epoch-minimum-ethereum-provider-wrapper-design

## Purpose

This document designs the concrete Ethereum provider wrapper boundary for the XC epoch minimum provider adapter.

This is a design-only milestone.

It does not implement real RPC reads, does not add viem / ethers runtime code, does not read env, does not accept secrets, and does not add CLI commands.

## Current completed foundation

The model-layer provider adapter is already implemented and reviewed:

    EthereumReadProvider
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> EthereumXcLensEpochMinimumSnapshot
    -> createXcEpochMinimumSourceFromEthereumLensSnapshot()
    -> XcEpochMinimumSource

The model-layer adapter uses only a custom read-only provider interface:

    getChainId()
    getBlock()
    readContract()

It does not import concrete provider libraries.

It does not accept:

    RPC URL
    private key
    mnemonic
    API key
    signer
    wallet account
    env config

## Design goal

The future concrete provider wrapper should adapt an external Ethereum client into the already-reviewed EthereumReadProvider interface.

Recommended high-level flow:

    outer integration / application layer
    -> constructs concrete Ethereum client
    -> wraps concrete client as EthereumReadProvider
    -> passes wrapper into createXcEpochMinimumSourceFromEthereumLensProvider()
    -> model layer remains provider-library agnostic

The wrapper boundary should keep infrastructure concerns outside model code.

## Wrapper responsibility

The concrete wrapper should be responsible for:

    mapping provider chain ID reads to bigint
    mapping block reads to EthereumBlockSnapshot
    mapping contract reads to unknown
    translating finalized / safe / blockNumber requests into concrete client calls
    normalizing provider-specific null / missing block behavior
    surfacing read errors without exposing secrets

The wrapper should not decide protocol economics.

The wrapper should not compute epoch minimums.

The wrapper should not perform Build-state validation.

The wrapper should only satisfy the EthereumReadProvider interface.

## Model-layer boundary

The following files should remain free from concrete provider dependency imports:

    src/model/ethereum-xc-epoch-minimum-provider-source.ts
    src/model/ethereum-xc-epoch-minimum-source.ts
    src/model/xc-epoch-minimum-source.ts

Do not import viem or ethers into these model files.

If a real wrapper is added later, it should live outside the model source layer or in a clearly separated adapter / infrastructure layer.

Possible future location:

    src/ethereum/...
    src/providers/...
    src/integrations/ethereum/...

The exact folder can be decided during implementation.

## Concrete provider library choice

Two realistic choices:

    viem
    ethers

Recommended first wrapper direction:

    viem-style read-only public client wrapper

Reason:

    viem is commonly used in modern TypeScript dapps
    read-only publicClient maps naturally to getBlock / readContract
    no signer is required
    block tags finalized and safe are first-class in many modern Ethereum client abstractions
    TypeScript typing is strong

Alternative:

    ethers provider wrapper

Reason to consider:

    broad ecosystem support
    familiar provider API
    may be simpler if the surrounding project already uses ethers

Decision for design:

    do not add either library yet
    keep implementation choice outside model layer
    prefer viem wrapper if the project later needs a concrete implementation and dependency fit is acceptable

## No-secret construction boundary

The concrete wrapper should not construct itself from env inside model code.

Allowed future pattern:

    app / script / integration reads config
    app / script / integration constructs concrete public client
    wrapper receives public client object
    wrapper implements EthereumReadProvider
    model adapter receives wrapper

Disallowed pattern:

    wrapper reads process.env.RPC_URL
    wrapper reads process.env.ALCHEMY_KEY
    wrapper reads process.env.INFURA_KEY
    wrapper accepts private key
    wrapper accepts mnemonic
    wrapper accepts signer
    wrapper logs RPC URL
    wrapper logs authorization headers

## RPC URL policy

RPC URLs may exist only in outer infrastructure configuration.

They must not be passed into:

    createXcEpochMinimumSourceFromEthereumLensProvider()

They also should not be passed into the model-layer wrapper interface.

If a concrete wrapper factory is later added, prefer:

    createEthereumReadProviderFromPublicClient(publicClient)

over:

    createEthereumReadProviderFromRpcUrl(rpcUrl)

Reason:

    public client construction is an infrastructure concern
    API keys and RPC URLs stay outside model code
    tests can inject mocked public client objects
    accidental secret logging risk is reduced

## Private key / signer policy

The concrete provider wrapper must be read-only.

It must not support:

    private keys
    mnemonic phrases
    signers
    wallet clients
    transaction sending
    account mutation
    approvals
    writes

If a library object can sign transactions, it should not be accepted by the wrapper unless the wrapper type restricts usage to read-only methods.

## Block read mapping

The wrapper must map:

    EthereumBlockReadInput

to concrete provider block reads.

Model input:

    { blockTag: "finalized" }
    { blockTag: "safe" }
    { blockNumber: bigint }
    {}

Mapping rules:

    finalized -> concrete finalized block tag
    safe -> concrete safe block tag
    blockNumber -> concrete block number read
    empty input -> current head read for confirmed-policy calculation only

The model adapter uses empty input only to compute confirmed block number.

The wrapper should not reinterpret empty input as a provenance-safe block.

## Block snapshot mapping

The wrapper must map concrete block result to:

    EthereumBlockSnapshot {
      number: bigint;
      hash: string | null;
      timestamp: bigint;
    }

Required behavior:

    missing block -> null
    missing block number -> null or wrapper error
    missing hash -> hash: null
    timestamp -> bigint seconds if provider returns seconds
    timestamp conversion must be explicit if provider returns number / hex / Date-like value

The model adapter will reject invalid block number, missing hash, or non-positive timestamp.

## Contract read mapping

The wrapper must map:

    EthereumContractReadInput

to concrete provider readContract calls.

Input fields:

    address
    abi
    functionName
    args
    blockNumber

Required behavior:

    use exactly input.blockNumber for the contract read
    pass address as provided by model adapter
    pass abi as provided by caller / integration
    pass functionName as provided
    pass args as provided
    return raw decoded result as unknown

The wrapper should not validate epoch minimum economics.

The model adapter will decode the result as bigint for the current epochMinimum path.

## ABI handling

The concrete wrapper should not hardcode XC Lens ABI unless the implementation milestone explicitly decides to include a minimal ABI module.

Preferred boundary:

    model adapter receives epochMinimumAbi as unknown
    wrapper passes abi through to concrete client
    outer integration chooses ABI

Possible future improvement:

    provide a small exported minimal XC Lens ABI constant in an infrastructure module

But do not add large ABIs or unrelated contract interfaces.

## Error handling policy

The wrapper should avoid leaking secrets in errors.

Allowed error content:

    chain ID mismatch
    missing block
    unsupported block tag
    readContract failed
    block number
    block tag
    contract address
    function name

Disallowed error content:

    RPC URL
    API key
    authorization header
    full env config
    private key
    mnemonic

The wrapper may preserve original error type internally during tests, but user-facing / logged messages must not expose secret-bearing config.

## Logging policy

The wrapper should not log by default.

If a future integration layer logs, safe fields are:

    chain ID
    block tag
    block number
    block hash
    contract address
    function name
    lockEpochs

Unsafe fields:

    RPC URL
    API keys
    authorization headers
    private keys
    mnemonic
    env dump

## Testing strategy

The concrete wrapper implementation should be tested with a mocked concrete client, not a real RPC endpoint.

Recommended tests:

1. maps getChainId result to bigint
2. maps finalized block tag to concrete client getBlock
3. maps safe block tag to concrete client getBlock
4. maps blockNumber read to concrete client getBlock
5. maps empty getBlock input to head block read
6. maps missing block to null
7. maps block hash / number / timestamp into EthereumBlockSnapshot
8. maps readContract input with exact blockNumber
9. passes abi / functionName / args through unchanged
10. does not accept RPC URL
11. does not read process.env
12. does not require private key
13. does not require signer
14. does not expose secret-bearing config in errors

## Integration with existing provider adapter

The wrapper implementation should be tested separately first.

Then add one integration-style unit test:

    mocked concrete client
    -> concrete wrapper
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> source.authoritativeEpochMinimum(lockEpoch)

Still no real network.

## Non-goals

This design does not add:

    real RPC execution
    env loading
    CLI command
    RPC URL factory
    private key support
    signer support
    transaction sending
    production address config
    snapshot persistence
    bridge signer verification
    X1-native verification

## Recommended next milestone

After this design is reviewed, next implementation can be:

    xc-epoch-minimum-mocked-ethereum-provider-wrapper

Suggested implementation scope:

    implement wrapper against mocked concrete public client shape
    no real RPC
    no env reads
    no secrets
    no private keys
    no signers
    no direct RPC URL factory
    test mapping into EthereumReadProvider
    test integration with existing mocked provider adapter

## Conclusion

The concrete Ethereum provider wrapper should remain an outer read-only infrastructure adapter.

It should adapt a concrete public client to EthereumReadProvider without moving RPC URLs, env, API keys, signers, or provider-library dependencies into the model-layer XC epoch minimum source logic.

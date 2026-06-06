# XC epoch minimum Ethereum provider wrapper design review notes

This review milestone checks the concrete Ethereum provider wrapper design before any implementation.

Reviewed design:

    implementation/xc-epoch-minimum-ethereum-provider-wrapper-design.md

Reviewed branch:

    xc-epoch-minimum-ethereum-provider-wrapper-design

Reviewed commits:

    bc084d9 Add XC epoch minimum Ethereum provider wrapper design
    ac5f6ec Update checkpoint after XC epoch minimum Ethereum provider wrapper design
    b0780d5 Merge branch 'xc-epoch-minimum-ethereum-provider-wrapper-design'

## Review conclusion

The concrete Ethereum provider wrapper design boundary is clean.

The design remains design-only.

No runtime implementation was added.

The design correctly keeps the model-layer provider adapter independent from concrete Ethereum client libraries.

The intended future flow remains:

    outer integration / application layer
    -> constructs concrete Ethereum client
    -> wraps concrete client as EthereumReadProvider
    -> passes wrapper into createXcEpochMinimumSourceFromEthereumLensProvider()
    -> model layer remains provider-library agnostic

## Runtime scope review

The design does not add:

    real RPC reads
    viem runtime code
    ethers runtime code
    env reads
    secrets
    CLI commands
    private key support
    signer support
    transaction sending

This is the correct boundary for the current stage.

## Model-layer boundary review

The following model files must remain free from concrete provider dependency imports:

    src/model/ethereum-xc-epoch-minimum-provider-source.ts
    src/model/ethereum-xc-epoch-minimum-source.ts
    src/model/xc-epoch-minimum-source.ts

Review decision:

    do not import viem or ethers into model files
    keep concrete wrapper outside the model source layer
    keep EthereumReadProvider as the stable model-facing interface

Possible future wrapper location can be decided during implementation, but it should be clearly separated as infrastructure / integration code.

## Concrete provider library decision

The design documents two realistic choices:

    viem
    ethers

Review decision:

    prefer viem-style public client wrapper for the first mocked wrapper implementation

Reason:

    read-only public client maps naturally to getChainId / getBlock / readContract
    no signer is required
    modern TypeScript typing is strong
    wrapper can remain isolated from model code

Important:

    do not add real viem dependency or real RPC in the next milestone
    implement against a mocked concrete public-client shape first

## No-secret boundary review

The design correctly rejects putting RPC URLs, env, API keys, private keys, mnemonics, or signers into model code.

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

Review decision:

    use createEthereumReadProviderFromPublicClient(publicClient)-style construction
    do not add createEthereumReadProviderFromRpcUrl(rpcUrl) in the first implementation

## RPC URL policy review

RPC URLs may exist only in outer infrastructure configuration.

They must not be passed into:

    createXcEpochMinimumSourceFromEthereumLensProvider()

They also should not be passed into the model-layer wrapper interface.

Review decision:

    no direct RPC URL factory in the first wrapper implementation

## Private key / signer policy review

The wrapper must remain read-only.

It must not support:

    private keys
    mnemonic phrases
    signers
    wallet clients
    transaction sending
    account mutation
    approvals
    writes

Review decision:

    first wrapper implementation should accept only a read-only public-client-like object
    do not accept signer-capable client types if avoidable

## Block read mapping review

The design correctly maps EthereumBlockReadInput to concrete provider block reads:

    finalized -> concrete finalized block tag
    safe -> concrete safe block tag
    blockNumber -> concrete block number read
    empty input -> current head read for confirmed-policy calculation only

Review decision:

    keep empty input mapped to head only
    do not reinterpret empty input as finalized / safe provenance block

## Block snapshot mapping review

The wrapper must map concrete block results to:

    EthereumBlockSnapshot {
      number: bigint;
      hash: string | null;
      timestamp: bigint;
    }

Review decision:

    missing block should map to null
    missing hash should map to hash: null
    timestamp conversion must be explicit
    block number conversion must be explicit

The model adapter will reject invalid block number, missing hash, and non-positive timestamp.

## Contract read mapping review

The wrapper must map EthereumContractReadInput to concrete readContract calls.

Review decision:

    use exactly input.blockNumber for each contract read
    pass address as provided
    pass abi as provided
    pass functionName as provided
    pass args as provided
    return raw decoded result as unknown

The wrapper should not validate epoch minimum economics.

## ABI boundary review

The design correctly avoids hardcoding XC Lens ABI at this stage.

Review decision:

    wrapper passes abi through to concrete client
    outer integration chooses ABI
    do not add large ABI modules or unrelated contract interfaces

A small minimal XC Lens ABI module can be considered later only if the implementation milestone needs it.

## Error and logging review

The wrapper should avoid leaking secrets in errors or logs.

Allowed error / log context:

    chain ID
    missing block
    unsupported block tag
    readContract failed
    block number
    block tag
    contract address
    function name

Disallowed:

    RPC URL
    API key
    authorization header
    full env config
    private key
    mnemonic

Review decision:

    wrapper should not log by default

## Testing strategy review

The design correctly requires mocked concrete-client tests, not real RPC tests.

Recommended first implementation tests:

    maps getChainId result to bigint
    maps finalized block tag to concrete client getBlock
    maps safe block tag to concrete client getBlock
    maps blockNumber read to concrete client getBlock
    maps empty getBlock input to head block read
    maps missing block to null
    maps block hash / number / timestamp into EthereumBlockSnapshot
    maps readContract input with exact blockNumber
    passes abi / functionName / args through unchanged
    does not accept RPC URL
    does not read process.env
    does not require private key
    does not require signer
    does not expose secret-bearing config in errors

Add one integration-style unit test:

    mocked concrete client
    -> concrete wrapper
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> source.authoritativeEpochMinimum(lockEpoch)

Still no real network.

## Next implementation recommendation

The next implementation milestone should be:

    xc-epoch-minimum-mocked-ethereum-provider-wrapper

Scope:

    implement wrapper against mocked viem-style public client shape
    no real RPC
    no env reads
    no secrets
    no private keys
    no signers
    no direct RPC URL factory
    test mapping into EthereumReadProvider
    test integration with existing mocked provider adapter

## Verification

After review:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

    32 test files passed
    227 tests passed

## Final conclusion

The concrete Ethereum provider wrapper design is ready to proceed to a mocked wrapper implementation milestone.

The implementation should use a mocked viem-style public-client shape first and must not perform real Ethereum RPC yet.

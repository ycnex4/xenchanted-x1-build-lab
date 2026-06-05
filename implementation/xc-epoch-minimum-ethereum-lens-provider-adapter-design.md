# XC epoch minimum Ethereum Lens provider adapter design

## Branch

xc-epoch-minimum-ethereum-lens-provider-adapter-design

## Purpose

This document designs a future real Ethereum / XC Lens provider adapter for authoritative XC epoch minimum records.

This is a design-only milestone.

It does not implement real RPC reads, provider configuration, ABI calls, CLI commands, or snapshot persistence.

No secrets are required.

## Current completed foundation

The current authoritative validation chain already supports an injected XC epoch minimum source:

    watcher candidate
    -> proof conversion
    -> appSubmitProof(..., xcEpochMinimumSource)
    -> appApplyRegistrarXntdLock() / appApplyRegistrarXntdRelock()
    -> applyRegistrarXntdLock() / applyRegistrarXntdRelock()
    -> assertAuthoritativeXcEpochMinimum()
    -> Build state

The current runtime assertion remains:

    observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

The generic source layer already supports:

    XcEpochMinimumRecord[]
    -> createXcEpochMinimumSourceFromRecords()
    -> XcEpochMinimumSource

The mocked Ethereum Lens snapshot adapter already supports:

    EthereumXcLensEpochMinimumSnapshot
    -> Ethereum-specific metadata validation
    -> XcEpochMinimumRecord[]
    -> createXcEpochMinimumSourceFromRecords()
    -> XcEpochMinimumSource

## Design goal

The future provider adapter should convert real finalized / safe / confirmed Ethereum XC Lens or XC Core reads into the same snapshot shape already accepted by the mocked snapshot adapter.

Recommended future high-level flow:

    adapter config
    -> provider read at finalized / safe / confirmed block
    -> XC Lens / Core calls
    -> EthereumXcLensEpochMinimumSnapshot
    -> createXcEpochMinimumSourceFromEthereumLensSnapshot()
    -> XcEpochMinimumSource

This keeps the already-reviewed mocked snapshot adapter as the deterministic validation boundary.

## Recommended adapter shape

A future provider adapter should be a thin read layer around the snapshot adapter.

Possible future exported function:

    createXcEpochMinimumSourceFromEthereumLensProvider(input)

Possible future input shape:

    EthereumXcLensProviderAdapterInput {
      provider: EthereumReadProvider;
      chainId: string;
      lensAddress: string;
      coreAddress?: string;
      finalityPolicy: EthereumFinalityPolicy;
      lockEpochs: readonly number[];
      observedAt: bigint;
    }

The adapter should not accept RPC URLs or private keys directly.

It should accept an already constructed read-only provider abstraction.

## Provider boundary

The provider adapter should be read-only.

It should not:

    sign transactions
    send transactions
    mutate Ethereum state
    require private keys
    require wallet accounts
    manage RPC URLs directly
    read process.env directly

The provider should be passed in from outer infrastructure.

Reason:

    secrets and infrastructure config must stay outside core model code
    tests can use mocked provider objects
    adapter code remains deterministic except for the explicit provider read step
    no accidental secret printing or env coupling

## No-secret config boundary

Do not put RPC URLs, API keys, private keys, mnemonics, tokens, or .env reads into model code.

Allowed future pattern:

    outer integration layer constructs provider
    provider is passed into adapter
    adapter performs read-only calls through interface

Disallowed future pattern:

    adapter reads process.env.RPC_URL
    adapter embeds provider URL
    adapter accepts private key
    adapter logs provider URL
    adapter logs request headers
    adapter prints raw config

## Provider interface

The first real provider design should define a narrow interface rather than binding directly to viem or ethers.

Possible interface:

    EthereumReadProvider {
      getChainId(): Promise<bigint>;
      getBlock(input: EthereumBlockReadInput): Promise<EthereumBlockSnapshot>;
      readContract(input: EthereumContractReadInput): Promise<unknown>;
    }

Possible block input:

    EthereumBlockReadInput {
      blockTag?: "finalized" | "safe";
      blockNumber?: bigint;
    }

Possible block snapshot:

    EthereumBlockSnapshot {
      number: bigint;
      hash: string;
      timestamp: bigint;
    }

Possible contract read input:

    EthereumContractReadInput {
      address: string;
      abi: unknown;
      functionName: string;
      args: readonly unknown[];
      blockNumber: bigint;
    }

This design keeps adapter logic testable without importing a concrete provider library.

## Chain ID policy

The provider adapter should validate that the actual provider chain ID matches configured chain ID.

Configured chain ID format should remain:

    eip155-1
    eip155-11155111

The adapter should:

    read provider chain ID
    convert it to eip155-<number>
    compare with configured chain ID
    reject mismatch

Mismatch should fail before any source records are produced.

## Address policy

The provider adapter should require explicit XC Lens / Core addresses.

The adapter should not hardcode addresses.

Recommended future config:

    lensAddress: EthereumAddress
    coreAddress?: EthereumAddress

Address validation should be Ethereum-specific:

    0x-prefixed
    20-byte hex
    normalized lowercase or checksum-preserving comparison policy

Do not add this validation to the generic source builder.

## ABI policy

This design milestone does not include final ABI definitions.

Future implementation should keep ABI scope minimal.

Possible first read strategies:

1. Lens direct epoch minimum read
2. Core protocol constants read + local computation
3. Lens protocol params read + local computation

The ABI used should include only the functions required for the chosen strategy.

The adapter should not import large unrelated ABIs if not needed.

## Epoch minimum derivation strategies

The provider adapter must ultimately answer:

    authoritativeEpochMinimum(lockEpoch)

Possible strategies:

### Strategy A: Direct Lens epoch minimum read

If XC Lens exposes a function similar to:

    epochMinimum(lockEpoch) -> minimumXntd

then the adapter can read each requested lockEpoch directly.

Pros:

    simplest adapter logic
    less local economic computation
    source of truth is explicit

Cons:

    depends on Lens exposing historical epoch values
    requires Lens ABI support

### Strategy B: Protocol constants + local computation

If XC Core / Lens exposes:

    initialNominal
    halvingIntervalSec
    genesisTs

and the epoch nominal rule is deterministic, adapter can compute:

    baseNominalForEpoch(lockEpoch)

Pros:

    fewer per-epoch contract reads
    works even if Lens does not expose direct historical values

Cons:

    adapter must mirror protocol math exactly
    requires strong tests against contract examples
    more risk of divergence if protocol formula is misunderstood

### Strategy C: Checkpointed Ethereum reads

An external integration can generate checkpointed epoch minimum records from finalized Ethereum reads and pass them as snapshots.

Pros:

    stable, auditable records
    reduced runtime RPC dependency

Cons:

    introduces checkpoint generation and storage lifecycle
    not a direct provider adapter by itself

## Recommended first provider design direction

Do not choose final ABI yet in runtime.

Design should support both:

    direct Lens epoch minimum reads
    protocol constants + local computation

The first provider adapter implementation should start with the simplest real read strategy available from the actual deployed XC Lens/Core interface.

Before implementation, confirm which on-chain view exists and is intended as the source of truth.

## Finality policy

The provider adapter must not read latest.

Allowed policies:

    finalized
    safe
    confirmed

### finalized

Use finalized block tag if provider supports it.

Expected behavior:

    read finalized block
    use finalized block number for all contract reads
    use finalized block hash in snapshot

### safe

Use safe block tag if finalized is unavailable or intentionally not used.

Expected behavior:

    read safe block
    use safe block number for all contract reads
    use safe block hash in snapshot

### confirmed

Use current head only to calculate an older confirmed block number, then read that older block.

Expected behavior:

    read head block
    confirmedBlockNumber = head.number - confirmations
    read confirmedBlockNumber block
    use confirmed block number for all contract reads
    use confirmed block hash in snapshot

Confirmed policy requires:

    confirmations > 0

Do not use latest block directly as the provenance block.

## Block consistency policy

All contract reads used to produce one snapshot should be performed at one selected provenance block number.

The snapshot must include:

    sourceBlockNumber
    sourceBlockHash

The source block hash must correspond to the same block number used for reads.

If a provider cannot provide block hash for the selected block, the adapter should reject.

## observedAt policy

The mocked snapshot adapter requires:

    observedAt > 0

For real provider adapter, observedAt should be set by the integration at observation time.

Possible choices:

    local wall-clock timestamp in seconds
    indexer observation timestamp
    selected Ethereum block timestamp

Recommendation:

    use selected Ethereum block timestamp when the goal is source provenance
    use separate integration observedAt only if we later distinguish observedAt from sourceBlockTimestamp

For the current existing shape, using selected Ethereum block timestamp is the cleanest first design.

## Requested lockEpochs policy

The provider adapter should accept an explicit list of requested lockEpochs.

It should not try to infer all possible epochs by default.

Reason:

    proofs only need validation for specific lockEpochs
    bounded reads are easier to audit
    tests remain deterministic
    avoids unbounded historical scan behavior

The adapter should reject empty lockEpochs list unless a future use case needs all-current snapshot generation.

## Error model

Malformed adapter config should use a source/adapter error path.

Current available error:

    InvalidXcEpochMinimumRecord

Future implementation may either reuse it or introduce a more specific adapter config error if needed.

For now, avoid new error codes unless runtime code demonstrates a real distinction.

Expected failures:

    invalid configured chain ID
    provider chain ID mismatch
    invalid Lens / Core address
    unsupported finality policy
    confirmed policy with confirmations <= 0
    selected block has no hash
    contract read result cannot be decoded into expected value
    computed / read minimum is invalid
    requested epoch missing

After source construction:

    missing authoritative epoch returns null
    runtime assertion throws MissingAuthoritativeXcEpochMinimum
    mismatch throws MismatchedAuthoritativeXcEpochMinimum

## Logging policy

The provider adapter should not log secrets or raw provider config.

If logging is later added, allowed logs should be limited to:

    chain ID
    selected block number
    selected block hash
    finality policy kind
    requested lockEpochs
    Lens / Core address

Do not log:

    RPC URLs
    API keys
    authorization headers
    private keys
    mnemonic
    full env config

## Testing strategy

Provider adapter implementation should start with mocked provider tests.

Recommended tests:

1. selects finalized block and builds source
2. selects safe block and builds source
3. selects confirmed block using positive confirmations
4. rejects latest policy
5. rejects provider chain ID mismatch
6. rejects invalid configured chain ID
7. rejects invalid Lens address
8. rejects missing block hash
9. performs all reads at selected block number
10. rejects empty requested lockEpochs
11. rejects invalid read result
12. propagates snapshot validation through existing mocked snapshot adapter
13. does not read process.env
14. does not require private keys
15. does not require RPC URL in adapter input

## Security / trust assumptions

A provider adapter is not fully trustless.

Its correctness depends on:

    provider honesty and availability
    finality policy correctness
    block hash / number consistency
    Lens / Core address correctness
    ABI correctness
    adapter read strategy correctness
    monitoring and replay/audit process

The provider adapter improves source provenance over arbitrary watcher payloads, but it does not eliminate all infrastructure trust.

## Non-goals for next implementation

The next implementation should not add:

    private key support
    signer support
    transaction sending
    CLI command
    env loading
    RPC URL config
    snapshot persistence migration
    bridge signer verification
    X1-native verification

## Recommended next implementation milestone

After this design is reviewed, the next implementation milestone can be:

    xc-epoch-minimum-mocked-ethereum-lens-provider-adapter

Suggested scope:

    implement provider adapter against a mocked read-only provider interface
    no real RPC
    no viem / ethers hard dependency unless isolated
    no env reads
    no secrets
    produce EthereumXcLensEpochMinimumSnapshot
    reuse createXcEpochMinimumSourceFromEthereumLensSnapshot()
    tests only with mocked provider

## Conclusion

The real Ethereum Lens provider adapter should be a thin, read-only provider layer that produces a reviewed Ethereum snapshot shape.

The already implemented mocked snapshot adapter remains the validation boundary.

Provider / ABI integration should be designed and reviewed before any real network implementation.

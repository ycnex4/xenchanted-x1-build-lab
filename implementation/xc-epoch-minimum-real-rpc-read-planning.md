# XC epoch minimum real RPC read planning

## Branch

xc-epoch-minimum-real-rpc-read-planning

## Purpose

This document plans the future real read-only RPC usage for the XC epoch minimum Ethereum provider path.

This is a planning-only milestone.

It does not implement real RPC, does not install viem, does not add runtime viem imports, does not read env, does not add RPC URL factories, does not accept secrets, does not accept private keys, does not accept signers, does not accept wallet clients, and does not send transactions.

## Current completed foundation

The current completed path is:

    provided public client
    -> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
    -> createEthereumReadProviderFromReadonlyEthereumPublicClient()
    -> createEthereumReadProviderFromViemPublicClient()
    -> EthereumReadProvider
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> XcEpochMinimumSource

The mocked read-only RPC integration helper is already implemented and reviewed.

The helper accepts only a provided public client and source configuration.

The helper does not own RPC URL / API key / env configuration.

The model layer remains provider-library agnostic.

## Planning goal

Plan how real read-only RPC should be introduced later without breaking the current boundaries.

The real RPC step should be treated as an outer infrastructure concern.

The core design should remain:

    model layer: protocol validation and source construction
    wrapper layer: adapt public client to EthereumReadProvider
    integration helper: accept provided public client and source config
    outer app / script: construct real public client from env / config

## Dependency decision

Recommended dependency direction:

    use viem for real public client construction later

Reason:

    the existing wrapper is already viem-like
    viem public client is read-only by default when using public client only
    getChainId / getBlock / readContract map cleanly to current wrapper shape
    the project already designed around a viem-style public client boundary

Planning decision:

    viem can be added later, but not in this planning milestone
    viem imports must stay outside src/model
    initial real viem construction should live in outer infrastructure code
    do not add ethers unless there is a strong reason

## Where public client construction should live

Public client construction must not live in:

    src/model/*
    src/ethereum/ethereum-viem-read-provider-wrapper.ts
    src/ethereum/ethereum-readonly-rpc-integration.ts
    src/ethereum/ethereum-read-provider-wrapper.ts

Recommended future location:

    scripts/read-xc-epoch-minimum-source.ts

Possible later reusable infrastructure module:

    src/integration/ethereum-public-client-factory.ts

Preferred first implementation:

    script-only construction first
    no reusable RPC URL factory exported from package
    keep secret-bearing config at the outermost edge

Reason:

    script-only construction reduces the chance of accidentally exposing RPC URL / API key through exported APIs
    it keeps library surfaces clean and safe
    it matches the existing helper design that accepts a provided public client

## Safe env names

Potential future env names:

    XC_ETHEREUM_RPC_URL
    XC_ETHEREUM_CHAIN_ID
    XC_ETHEREUM_LENS_ADDRESS
    XC_ETHEREUM_FINALITY
    XC_ETHEREUM_CONFIRMATIONS
    XC_ETHEREUM_LOCK_EPOCHS

Optional if needed later:

    XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
    XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Sensitive / never print values:

    XC_ETHEREUM_RPC_URL
    any API-key-bearing RPC URL
    any authorization header
    any token-like value

Safe to print after validation:

    XC_ETHEREUM_CHAIN_ID
    XC_ETHEREUM_LENS_ADDRESS
    XC_ETHEREUM_FINALITY
    XC_ETHEREUM_CONFIRMATIONS
    parsed lock epoch count
    selected block number
    selected block hash
    function name

## Env handling policy

Env reads may happen only in the outer script / app entrypoint.

Env reads must not happen in:

    src/model
    src/ethereum wrappers
    src/ethereum integration helper
    reusable source builders

A future script may read env, validate it, construct a public client, and pass the public client inward.

Do not print raw env values.

Do not print RPC URL.

Do not print full config object.

## Redacted error policy

Outer entrypoint errors must not include:

    RPC URL
    API key
    authorization header
    full env dump
    transport config
    private key
    mnemonic
    signer object
    wallet client internals

Allowed error context:

    operation name
    chain ID
    Lens address
    finality policy
    confirmations count
    block tag
    block number
    function name
    lock epoch count
    high-level provider failure category

Recommended pattern:

    catch provider/client construction errors at script boundary
    rethrow or print sanitized message
    never stringify full client / transport / env objects
    never include original error message if provider may include URL or headers
    optionally include original error name only

Example safe message:

    Ethereum read-only RPC failed during getBlock(finalized). Check RPC provider support and network configuration.

Example unsafe message:

    Ethereum RPC failed for https://provider.example/API_KEY/...

## Provider unsupported finalized / safe behavior

Do not silently downgrade:

    finalized -> latest
    safe -> latest

If a provider does not support finalized or safe:

    fail with sanitized error
    advise changing explicit finality config
    do not automatically change provenance policy

If fallback is ever allowed, it must be explicit:

    finalityPolicy: { kind: "confirmed", confirmations: N }

## Manual-only real RPC smoke test decision

Recommended first real RPC smoke test policy:

    manual-only
    not part of npm test
    not part of CI
    requires explicit env confirmation
    must not print RPC URL
    must not print secrets
    should print only safe summary fields

Possible future command:

    npm run smoke:xc-epoch-minimum:ethereum

Required confirmation env:

    XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

The smoke test should refuse to run without explicit confirmation.

The smoke test should refuse if required safe config is missing.

The smoke test should not send transactions.

The smoke test should only perform:

    getChainId
    getBlock
    readContract

## CLI / script output policy

Allowed output:

    chain ID
    Lens address
    finality policy
    selected block number
    selected block hash
    lock epochs count
    function name
    number of epoch minimums loaded

Disallowed output:

    RPC URL
    API key
    env dump
    authorization header
    full provider transport config
    private key
    mnemonic
    signer / wallet internals

## Production address config

This planning milestone does not define production addresses.

A future milestone may define:

    chain ID
    Lens address
    supported finality policy
    lock epochs to read
    ABI source

But it should still avoid storing or printing RPC URLs.

## Future implementation order

Recommended order:

1. Review this planning document.
2. Add script-only public client construction design.
3. Review script-only public client construction design.
4. Implement mocked script/config parsing without real RPC.
5. Review mocked script/config parsing.
6. Implement manual-only real RPC smoke script.
7. Review manual-only real RPC smoke script.
8. Only then consider production operational docs.

## Non-goals

This planning milestone does not add:

    real RPC execution
    viem dependency
    viem imports
    public client construction
    env reads
    RPC URL factory
    private key support
    signer support
    wallet client support
    transaction sending
    CLI command
    production address config
    real RPC smoke test

## Validation

Before merging this planning milestone:

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

Expected current count:

    35 test files passed
    261 tests passed

## Conclusion

Real read-only RPC should be introduced only at the outer infrastructure edge.

The safe next direction is script-only planning and design for public client construction, with RPC URL / API key ownership kept outside model, wrapper, and helper code.

No real RPC should be implemented until this planning step is reviewed.

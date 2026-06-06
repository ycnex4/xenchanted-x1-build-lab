# XC epoch minimum real RPC read planning review notes

This review milestone checks the real RPC read planning document before any real RPC implementation.

Reviewed planning document:

    implementation/xc-epoch-minimum-real-rpc-read-planning.md

Reviewed branch:

    xc-epoch-minimum-real-rpc-read-planning

Reviewed commits:

    f2916a5 Add XC epoch minimum real RPC read planning
    749fc9b Update checkpoint after real RPC read planning
    c3e43ef Merge branch 'xc-epoch-minimum-real-rpc-read-planning'

## Review conclusion

The real RPC read planning boundary is clean.

This remains a planning-only stage.

No real RPC implementation was added.

No viem dependency was installed.

No runtime viem imports were added.

No env reads were added.

No RPC URL factory was added.

No private keys, signers, wallet clients, or transaction-sending paths were added.

## Runtime / dependency review

A targeted grep confirmed that RPC / env / secret / viem terms appear only in the planning document as policy, planning, examples, and non-goals.

The runtime source remains free from:

    real viem imports
    ethers imports
    process.env reads
    createPublicClient calls
    http transport construction
    RPC URL factories
    signer / wallet / write paths

The current branch remains review-only.

## Current foundation review

The completed provider path remains:

    provided public client
    -> createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
    -> createEthereumReadProviderFromReadonlyEthereumPublicClient()
    -> createEthereumReadProviderFromViemPublicClient()
    -> EthereumReadProvider
    -> createXcEpochMinimumSourceFromEthereumLensProvider()
    -> XcEpochMinimumSource

The mocked read-only RPC integration helper remains the safe boundary.

The helper accepts only a provided public client and source configuration.

The helper does not own RPC URL / API key / env configuration.

The model layer remains provider-library agnostic.

## Dependency planning review

The planning document recommends viem for future real public client construction.

Review decision:

    viem is the preferred future dependency if real RPC is added
    viem should not be installed in this planning review
    viem imports must stay outside src/model
    initial real viem construction should live in outer infrastructure code
    ethers should not be added unless a strong reason appears

## Public client construction location review

Public client construction must not live in:

    src/model/*
    src/ethereum/ethereum-viem-read-provider-wrapper.ts
    src/ethereum/ethereum-readonly-rpc-integration.ts
    src/ethereum/ethereum-read-provider-wrapper.ts

Preferred first location remains:

    scripts/read-xc-epoch-minimum-source.ts

Possible later reusable infrastructure module:

    src/integration/ethereum-public-client-factory.ts

Review decision:

    script-only construction first
    no reusable RPC URL factory exported from package
    keep secret-bearing config at the outermost edge

## Env policy review

Potential future env names are acceptable:

    XC_ETHEREUM_RPC_URL
    XC_ETHEREUM_CHAIN_ID
    XC_ETHEREUM_LENS_ADDRESS
    XC_ETHEREUM_FINALITY
    XC_ETHEREUM_CONFIRMATIONS
    XC_ETHEREUM_LOCK_EPOCHS
    XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
    XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Sensitive values must never be printed:

    XC_ETHEREUM_RPC_URL
    API-key-bearing RPC URL
    authorization header
    token-like value

Safe values may be printed after validation:

    chain ID
    Lens address
    finality policy
    confirmations
    parsed lock epoch count
    selected block number
    selected block hash
    function name

Review decision:

    env reads may happen only in the outer script / app entrypoint
    env reads must not happen in model, wrappers, integration helper, or reusable source builders
    do not print raw env values
    do not print RPC URL
    do not print full config object

## Redacted error policy review

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

Review decision:

    catch provider / client construction errors at script boundary
    print or rethrow sanitized messages only
    never stringify full client / transport / env objects
    do not include original provider error messages if they may contain URL or headers
    optionally include original error name only

## Finality and provider support review

Do not silently downgrade:

    finalized -> latest
    safe -> latest

If a provider does not support finalized or safe:

    fail with sanitized error
    advise changing explicit finality config
    do not automatically change provenance policy

Fallback can only be explicit:

    finalityPolicy: { kind: "confirmed", confirmations: N }

## Manual-only smoke test review

The planned real RPC smoke test should be manual-only.

It should not be part of:

    npm test
    CI

It should require explicit confirmation:

    XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

It must not print:

    RPC URL
    secrets
    raw env values

It should only perform read operations:

    getChainId
    getBlock
    readContract

Review decision:

    manual-only smoke direction is acceptable
    do not implement smoke test in this planning review
    design script-only construction first before smoke implementation

## Output policy review

Allowed future script output:

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

## Future implementation order review

The planned order is acceptable:

1. Review this planning document.
2. Add script-only public client construction design.
3. Review script-only public client construction design.
4. Implement mocked script/config parsing without real RPC.
5. Review mocked script/config parsing.
6. Implement manual-only real RPC smoke script.
7. Review manual-only real RPC smoke script.
8. Only then consider production operational docs.

## Verification

After review:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities

Current test count:

    35 test files passed
    261 tests passed

## Final conclusion

The real RPC read planning is ready to proceed to script-only public client construction design.

The next milestone should still be design-only and should not implement real RPC.

Recommended next milestone:

    xc-epoch-minimum-script-public-client-construction-design

Suggested next scope:

    design script-only public client construction
    decide exact script path
    decide exact safe env parsing rules
    decide if viem dependency is introduced in the script-design stage or later
    define sanitized error boundaries for the script
    do not implement real RPC until script construction design review is complete

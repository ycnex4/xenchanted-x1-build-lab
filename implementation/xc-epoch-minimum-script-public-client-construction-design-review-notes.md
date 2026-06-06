# XC epoch minimum script public client construction design review notes

This review milestone checks the script-only public client construction design before any implementation.

Reviewed design:

    implementation/xc-epoch-minimum-script-public-client-construction-design.md

Reviewed branch:

    xc-epoch-minimum-script-public-client-construction-design

Reviewed commits:

    43fcd40 Add script public client construction design
    58de7e0 Update checkpoint after script public client construction design
    6a47780 Merge branch 'xc-epoch-minimum-script-public-client-construction-design'

## Review conclusion

The script-only public client construction design boundary is clean.

This remains a design-only stage.

No real RPC implementation was added.

No viem dependency was installed.

No runtime viem imports were added.

No env reads were added.

No RPC URL factory was added.

No private keys, signers, wallet clients, or transaction-sending paths were added.

## Runtime / dependency review

A targeted grep confirmed that RPC / env / secret / viem terms appear only in the design document as policy, design rules, examples, and non-goals.

The runtime source remains free from:

    real viem imports
    ethers imports
    process.env reads
    createPublicClient calls
    http transport construction
    RPC URL factories
    signer / wallet / write paths

The current branch remains review-only.

## Design boundary review

The future script path is acceptable:

    scripts/read-xc-epoch-minimum-source.ts

Review decision:

    script-only construction keeps RPC URL / API key ownership outside exported library APIs
    script-only construction is easier to gate with explicit confirmation
    script-only construction avoids creating a reusable RPC URL factory too early
    script-only construction keeps src/model and src/ethereum helper layers clean

## Dependency decision review

The design recommends viem as the future dependency for real public client construction.

Review decision:

    viem is the preferred future dependency if real RPC is added
    viem should not be installed in this design review
    viem imports must stay outside src/model
    exact viem version should be decided later
    ethers should not be added unless a strong reason appears

## Env parsing policy review

Future required env names are acceptable:

    XC_ETHEREUM_RPC_URL
    XC_ETHEREUM_CHAIN_ID
    XC_ETHEREUM_LENS_ADDRESS
    XC_ETHEREUM_FINALITY
    XC_ETHEREUM_LOCK_EPOCHS

Confirmed finality env:

    XC_ETHEREUM_CONFIRMATIONS

Optional env names:

    XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
    XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH

Manual real RPC confirmation:

    XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

Review decision:

    env reads may happen only in the future script entrypoint
    env reads must not happen in model, wrappers, or integration helper
    RPC URL must never be printed
    full config object must never be printed
    raw env object must never be printed
    secret-bearing config must stay inside script-local construction scope

## Config boundary review

Secret-bearing config includes:

    rpcUrl
    transport options
    authorization headers
    API-key-bearing URLs

Secret-bearing config must not be passed into:

    src/model
    src/ethereum/ethereum-readonly-rpc-integration.ts
    createXcEpochMinimumSourceFromReadonlyEthereumPublicClient()
    createEthereumReadProviderFromReadonlyEthereumPublicClient()

Only the constructed public client object should be passed inward.

## Error handling review

Allowed future error context:

    operation name
    chain ID
    Lens address
    finality policy
    confirmations count
    lock epoch count
    block tag
    block number
    function name
    high-level failure category

Disallowed future error context:

    RPC URL
    API key
    authorization header
    raw env object
    full config object
    transport config
    private key
    mnemonic
    signer object
    wallet client internals

Review decision:

    sanitized error boundary is acceptable
    future implementation should add mocked tests for RPC URL / API key leakage
    do not include original provider error messages if they may contain URL or headers

## Finality policy review

A future script must not silently downgrade:

    finalized -> latest
    safe -> latest

If provider does not support finalized or safe:

    fail with sanitized message
    recommend explicit confirmed finality configuration if appropriate
    do not automatically change policy

## Output policy review

Allowed future script output:

    real RPC confirmation accepted
    chain ID
    Lens address
    finality policy
    confirmations count if applicable
    lock epoch count
    function name
    selected block number
    selected block hash
    number of loaded epoch minimums

Disallowed future script output:

    RPC URL
    API key
    raw env values
    full config object
    transport config
    authorization headers
    private key
    mnemonic
    signer / wallet internals

## Manual-only smoke policy review

The future real RPC script should be manual-only.

It should require:

    npm run build
    XC_ETHEREUM_REAL_RPC_CONFIRM=I_UNDERSTAND_THIS_USES_REAL_RPC

It should not run during:

    npm test
    npm run build
    CI

It should only perform:

    getChainId
    getBlock
    readContract

It must not perform:

    sendTransaction
    writeContract
    approve
    signer calls
    wallet calls

## Testing strategy review

The recommended mocked tests are appropriate for the next implementation phase:

    parse required env into safe config without printing RPC URL
    reject missing RPC URL with sanitized message
    reject invalid chain ID
    reject invalid Lens address
    reject invalid finality
    require confirmations for confirmed finality
    reject non-positive confirmations
    parse lock epoch list
    reject empty lock epoch list
    require explicit real RPC confirmation
    never include RPC URL in error message
    never include API-key-looking text in error message
    do not construct signer / wallet client
    do not expose raw env object
    pass only constructed public client into existing helper

## Additional design change decision

No additional design changes are required before merging this review milestone.

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

The script-only public client construction design is ready to proceed to mocked script config parsing.

The next implementation should still avoid real RPC, viem dependency, public client construction, and env reads from real process.env.

Recommended next milestone:

    xc-epoch-minimum-mocked-script-config-parsing

Suggested next scope:

    add mocked config parsing helpers
    no real RPC
    no viem dependency
    no public client construction
    no env reads outside test-provided objects
    no secrets printed
    no RPC URL leakage in errors
    tests only

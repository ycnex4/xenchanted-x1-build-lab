# XC epoch minimum mocked script config parsing review notes

This review milestone checks the mocked Ethereum script config parsing boundary before any script implementation planning.

Reviewed runtime:

    src/ethereum/ethereum-script-config.ts

Reviewed tests:

    tests/ethereum-script-config.test.ts

Reviewed notes:

    implementation/xc-epoch-minimum-mocked-script-config-parsing-notes.md

Reviewed commits:

    3d4afa3 Add mocked Ethereum script config parsing
    b0d1aea Add mocked Ethereum script config parsing notes
    65eeac6 Update checkpoint after mocked script config parsing
    c83036a Merge branch 'xc-epoch-minimum-mocked-script-config-parsing'

## Review conclusion

The mocked script config parsing boundary is clean.

The parser accepts a test-provided env-like object.

The parser does not read the real process.env object.

No real RPC implementation was added.

No viem dependency was installed.

No runtime viem imports were added.

No ethers imports were added.

No public client construction was added.

No RPC URL factory was added.

No private keys, signers, wallet clients, or transaction-sending paths were added.

## Runtime / dependency boundary review

The parser lives in:

    src/ethereum/ethereum-script-config.ts

It does not live in src/model.

The model layer remains provider-library agnostic.

The parser does not import or call:

    viem
    ethers
    http
    createPublicClient
    process.env
    fetch
    sendTransaction
    writeContract
    wallet APIs
    signer APIs
    transaction APIs

The parser does not construct:

    public client
    wallet client
    signer
    transaction sender

A targeted grep confirmed that sensitive / provider-library / transaction terms appear only in the review notes and implementation notes as boundary documentation and non-goals, not as runtime implementation.

## Parser input review

The parser accepts:

    EthereumScriptConfigEnv

This is an env-like object:

    Record<string, string | undefined>

Review decision:

    this is acceptable for the mocked layer
    it allows tests to provide config without reading real process.env
    it keeps future env ownership at the outer script boundary

## Parsed config review

The parser accepts the intended future env names:

    XC_ETHEREUM_RPC_URL
    XC_ETHEREUM_CHAIN_ID
    XC_ETHEREUM_LENS_ADDRESS
    XC_ETHEREUM_FINALITY
    XC_ETHEREUM_CONFIRMATIONS
    XC_ETHEREUM_LOCK_EPOCHS
    XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION
    XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH
    XC_ETHEREUM_REAL_RPC_CONFIRM

The full config includes rpcUrl because a future script will need it to construct a read-only public client.

Review decision:

    keeping rpcUrl in the full parsed config is acceptable
    rpcUrl must stay out of safe summary
    rpcUrl must not appear in validation errors
    no public client construction is performed here

## Validation review

The parser validates:

    required RPC URL presence
    chain ID format eip155-N
    Lens address format 0x + 40 hex chars
    finality finalized / safe / confirmed
    confirmations required for confirmed finality
    confirmations positive integer
    lock epoch list non-empty
    lock epoch values numeric
    optional function name identifier shape
    explicit real RPC confirmation

The parser normalizes:

    Lens address to lowercase
    empty optional function name to epochMinimum
    empty optional ABI path to omitted optional property

The parser preserves exactOptionalPropertyTypes behavior by omitting optional fields when absent.

## Safe summary review

summarizeEthereumScriptConfig() returns:

    chainId
    lensAddress
    finalityPolicy
    lockEpochCount
    epochMinimumFunctionName
    hasEpochMinimumAbiPath
    realRpcConfirmed

It does not return:

    rpcUrl
    API key
    raw env object
    full config object
    transport config

Review decision:

    safe summary boundary is correct
    safe summary is suitable for future script output
    do not print full parsed config in future scripts

## Error leakage review

Tests verify that validation errors do not include:

    full RPC URL
    API-key-like value
    provider hostname
    https:// prefix

Review decision:

    current validation errors are sanitized enough for this mocked parser layer
    future script implementation should continue testing for RPC URL / API key leakage
    future provider errors still need a separate sanitized error boundary before real RPC

## Test coverage review

The current tests cover:

    parses required env into config
    normalizes Lens address to lowercase
    parses safe finality
    parses confirmed finality with confirmations
    parses optional function name and ABI path
    creates safe summary without RPC URL
    rejects missing RPC URL with sanitized error
    rejects invalid chain ID
    rejects invalid Lens address
    rejects invalid finality
    requires confirmations for confirmed finality
    rejects non-positive confirmations
    rejects empty lock epoch list
    rejects invalid lock epoch item
    requires explicit real RPC confirmation
    rejects invalid function name
    does not include RPC URL or API key in validation errors

This is sufficient for the mocked config parsing milestone.

## Additional hardening decision

No additional parser hardening tests are required before merging this review milestone.

Possible future hardening tests may include:

    duplicated lock epoch values policy
    very large lock epoch values policy
    whitespace normalization around all env values
    ABI path traversal policy if ABI path support becomes active
    explicit checks that safe summary is the only object printed by future scripts

Those are not required before this review milestone is merged.

## Security / operational review

This milestone does not add:

    real Ethereum RPC
    viem dependency
    viem runtime imports
    ethers dependency
    process.env reads
    public client construction
    RPC URL factory
    private keys
    API keys as separate fields
    mnemonic
    signer support
    wallet client support
    account support
    transaction sending
    CLI commands
    package scripts
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

    36 test files passed
    278 tests passed

## Conclusion

The mocked script config parsing layer is safe to keep.

It prepares future script implementation while keeping config input mocked/test-provided, avoiding real RPC, avoiding viem, avoiding public client construction, avoiding process.env reads, and preventing RPC URL / API-key-like leakage in safe summaries and validation errors.

Recommended next milestone:

    xc-epoch-minimum-mocked-script-config-parsing-review-checkpoint

Suggested next scope:

    update checkpoint after mocked script config parsing review
    merge review branch after clean checks
    then plan the next implementation step separately

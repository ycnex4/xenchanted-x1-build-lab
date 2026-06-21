# Stage 4.10 Pre-Signature Checkpoint

This checkpoint closes the pre-cryptographic-signature part of Stage 4 live runtime / operations design.

It records the boundary reached after Stage 4.10.

## Current position

Stage 4 is the live runtime / operations layer.

Stage 4 introduced topics that were intentionally excluded from Stage 3:

- live RPC
- wallet access
- signer and guardian key handling
- SOL balance requirements
- transaction submission
- deployment actions
- watcher and relayer operations
- production monitoring
- incident response

The working principle remains:

    first offline model / mocked tests
    then read-only online
    then no-send preflight
    only then controlled live testnet transaction

The system has not yet crossed into wallet loading, private key handling, signing, live transaction submission, or SOL spend.

## Closed Stage 4 boundaries

### Stage 4.1 redacted live config boundary

Closed by runtime commit:

    24e0246 Add Stage 4.1 redacted live config boundary

Evidence merged into build-lab main by:

    8d83c54 Merge branch 'document-stage-4-1-redacted-live-config-boundary-evidence'

Boundary:

- config parsing only
- redacted public output
- no RPC call
- no wallet load
- no transaction
- no SOL spend

### Stage 4.2 read-only RPC connectivity boundary

Closed by runtime commit:

    eb6ef26 Add Stage 4.2 read-only RPC connectivity boundary

Evidence merged into build-lab main by:

    b4eb1a6 Merge branch 'document-stage-4-2-read-only-rpc-connectivity-boundary-evidence'

Boundary:

- read-only RPC methods only
- no wallet
- no signing
- no transaction
- no SOL spend

### Stage 4.3 watcher read-only observation boundary

Closed by runtime commit:

    c5b77cf Add Stage 4.3 watcher read-only observation boundary

Evidence merged into build-lab main by:

    f7d27cd Merge branch 'document-stage-4-3-watcher-read-only-observation-boundary-evidence'

Boundary:

- one-shot watcher observation
- read-only state observation
- no continuous loop
- no wallet
- no signing
- no transaction
- no SOL spend

### Stage 4.4 relayer dry-run no-send boundary

Closed by runtime commit:

    5b3be68 Add Stage 4.4 relayer dry-run no-send boundary

Evidence merged into build-lab main by:

    12ccb9e Merge branch 'document-stage-4-4-relayer-dry-run-no-send-boundary-evidence'

Boundary:

- unsigned relayer plan
- no wallet
- no signing
- no transaction submission
- no SOL spend

### Stage 4.5 guardian operation policy boundary

Closed by runtime commit:

    93665db Add Stage 4.5 guardian operation policy boundary

Evidence merged into build-lab main by:

    bd557ba Merge branch 'document-stage-4-5-guardian-operation-policy-boundary-evidence'

Boundary:

- guardian policy only
- public keys only
- no guardian private keys
- no wallet
- no signing
- no transaction
- no SOL spend

### Stage 4.6 transaction preflight no-send boundary

Closed by runtime commit:

    5c967c0 Add Stage 4.6 transaction preflight no-send boundary

Evidence merged into build-lab main by:

    b68789e Merge branch 'document-stage-4-6-transaction-preflight-no-send-boundary-evidence'

Boundary:

- no-send transaction preflight envelope
- no serialized transaction
- no wallet
- no signing
- no simulation
- no transaction submission
- no SOL spend

### Stage 4.7 fixed guardian set quorum boundary

Closed by runtime commit:

    f63397f Add Stage 4.7 fixed guardian set quorum boundary

Evidence merged into build-lab main by:

    b39652f Merge branch 'document-stage-4-7-fixed-guardian-set-quorum-boundary-evidence'

Boundary:

- fixed guardian set
- 5 guardians
- 3-of-5 quorum
- public identity only
- duplicate approvals rejected
- unknown guardians rejected
- no private keys
- no signing
- no transaction
- no SOL spend

### Stage 4.8 gateway fee policy boundary

Closed by runtime commit:

    a960c16 Add Stage 4.8 gateway fee policy boundary

Evidence merged into build-lab main by:

    e373cce Merge branch 'document-stage-4-8-gateway-fee-policy-boundary-evidence'

Boundary:

- fixed service fee policy
- manual fee configuration
- fee recipient
- gross amount
- fee amount
- net amount
- fee quote deadline
- guardianSetVersion binding
- no oracle
- no wallet
- no signing
- no transaction
- no SOL spend

### Stage 4.9 guardian fee-bound approval message boundary

Closed by runtime commit:

    d4a7060 Add Stage 4.9 guardian fee-bound approval message boundary

Evidence merged into build-lab main by:

    10212d6 Merge branch 'document-stage-4-9-guardian-fee-bound-message-boundary-evidence'

Boundary:

- guardian approval message model
- fee fields bound into message digest
- canonical field order fixed
- digest changes if fee amount, net amount, fee asset, fee quote id, fee recipient, route id, guardian set version, or deadline changes
- no private keys
- no signing
- no signature verification
- no transaction
- no SOL spend

### Stage 4.10 guardian fee-bound approval verification boundary

Closed by runtime commit:

    fdbc3b8 Add Stage 4.10 guardian fee-bound approval verification boundary

Evidence merged into build-lab main by:

    ba81cc9 Merge branch 'document-stage-4-10-guardian-fee-bound-approval-verification-boundary-evidence'

Boundary:

- verifies approvals against exact Stage 4.9 fee-bound digest
- rejects 2-of-5
- accepts 3-of-5, 4-of-5, 5-of-5
- rejects duplicate guardian approval
- rejects unknown guardian approval
- rejects wrong digest
- preserves fee binding
- no private keys
- no signing
- no production cryptographic signature verification
- no transaction
- no SOL spend

## Current safety status

As of this checkpoint:

- no wallet-loading path has been introduced
- no private key path has been introduced
- no guardian private key material has been introduced
- no signing path has been introduced
- no production cryptographic signature verification has been introduced
- no live-send path has been introduced
- no transaction submission path has been introduced
- no SOL spend path has been introduced

All Stage 4.1 through Stage 4.10 runtime tests were model/offline tests.

## Current proof chain

The current pre-signature proof chain is:

    redacted live config
    -> read-only RPC boundary
    -> watcher read-only observation
    -> relayer dry-run no-send
    -> guardian operation policy
    -> transaction preflight no-send
    -> fixed 5 guardian / 3-of-5 quorum
    -> gateway fee policy
    -> guardian approval message with fee bound into digest
    -> guardian approval verification against exact fee-bound digest

## Why this checkpoint matters

This checkpoint separates two risk zones.

The completed zone is still safe/offline:

- config modeling
- read-only observation
- dry-run planning
- public guardian identity policy
- no-send preflight
- fixed quorum
- fee policy
- fee-bound digest
- approval verification against digest

The next zone is more sensitive:

- production signature format
- guardian public key encoding
- signature verification
- message digest compatibility
- possible external signing tools
- possible key custody assumptions

Before entering that zone, the model now clearly records what must be preserved:

- exact fee-bound digest matching
- 5 guardian set
- 3-of-5 quorum
- no duplicate guardian counting
- no unknown guardian approvals
- no fee substitution after approval
- no wallet/private-key exposure in logs or artifacts

## Next valid stage

The next valid stage is:

    Stage 4.11 production signature verification design boundary

That stage should still be offline first.

It should introduce verification of signatures over the Stage 4.9 fee-bound digest without introducing live wallet loading, private key handling, transaction submission, or SOL spend.

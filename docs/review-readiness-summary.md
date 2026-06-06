# xEnchanted X1 Build Lab — Review Readiness Summary

## Purpose

xenchanted-x1-build-lab is an implementation and design lab for the X1 Build model connected to the xEnchanted Crypto ecosystem.

The project models how verified activity from xEnchanted Crypto and X1 can be converted into Build state.

The current implementation focuses on:

- deterministic Build state accounting
- proof-based registrar input
- replay protection
- watcher candidate models
- proof conversion
- snapshot persistence and recovery safety
- read-only CLI inspection / verification / recovery commands
- commitmentStatus as current XNTD commitment signal
- appGetBuildView as read-only app/service view

This repository is not a production chain deployment yet.

It is a tested MVP implementation lab intended for design review, architectural review, and incremental hardening.

The MVP implementation lab is complete at the current scope.

It remains a lab, not a production deployment.

## Current review target

The current review target is the correctness of the completed MVP implementation lab and the safety of the MVP architecture.

The most important question is whether the current accounting model, proof flow, replay protection, snapshot behavior, and CLI safety boundaries are coherent and hard to misuse.

## High-level architecture

Implemented flow:

    Watcher candidate
      -> Proof object
      -> Application proof submission
      -> Registrar handler
      -> BuildState update
      -> Replay protection update
      -> Snapshot persistence / CLI inspection

The project separates these concerns:

- domain state types
- application service layer
- registrar handlers
- proof object builders
- watcher candidate models
- watcher-to-proof conversion
- snapshot serialization / verification / recovery
- CLI read-only commands

## Core state model

The central model is Build state.

Build state tracks contribution-related data such as:

- historical BLD
- available BLD
- origin BLD
- XEN burn power
- XNTD lock / relock status
- X1 fee contribution checkpoints
- replay protection state
- commitmentStatus as current XNTD commitment signal

The model distinguishes historical accounting from currently available / usable accounting.

This is important because some values should be non-decreasing historical records, while others can change through usage, transfer, burn, or lock / relock logic.

## BLD terminology

The current terminology is:

- history_bld — historical BLD, non-decreasing
- available_bld — usable / spendable / transferable BLD
- origin_bld — Genesis allocation, not history

This terminology replaced earlier earned_bld / bldxp wording to make the model easier to read and reason about.

## Genesis Origin model

Genesis Origin BLD is tiered by historical BLD.

Current intended tiers:

    history_bld >= 1     -> origin_bld = 11
    history_bld >= 11    -> origin_bld = 22
    history_bld >= 121   -> origin_bld = 55
    history_bld >= 1111  -> origin_bld = 121

121 is the maximum Genesis Origin cap, not a default allocation.

## XNTD lock model

XNTD lock is the commitment requirement for Build records that received historical BLD through Core redeem.

Current intended lock rule:

    required_xntd_lock = current epoch Core L1 nominal

Commitment status is exposed through:

    commitmentStatus

Implemented status values:

    COMMITTED
    UNCOMMITTED
    UNKNOWN

Meaning:

    commitmentStatus = current XNTD commitment signal

UNCOMMITTED does not mean invalid Build history.

`appGetBuildView()` may return `UNKNOWN` when context-dependent epoch information is required but unavailable. `UNKNOWN` is a status signal, not an error.

Relock is allowed only when:

    available_bld >= history_bld

This prevents a user from selling or spending available BLD and then freely relocking without restoring the full available balance.

## Source event protection

The MVP protects accounting from duplicated source events.

Protection concepts include:

- Core redeem event replay protection
- XEN burn event replay protection
- processed message protection
- canonical Build identity mapping
- Genesis Origin claim protection

The intended rule is:

    one source event -> one accounting action -> one Build

## Implemented layers

The current repository includes tested layers for:

- Build state
- Build registry
- create build flow
- Core redeem BLD accounting
- XEN burn power accounting
- XNTD lock and relock
- X1 fee contribution checkpoints
- registrar replay protection
- source event replay protection
- XNTD commitment event replay protection
- registrar handlers
- proof object types
- proof-to-registrar builders
- watcher candidate types
- watcher-to-proof conversion
- application proof submission
- end-to-end watcher-proof-registrar scenario
- storage serialization
- snapshot verification
- snapshot backup-enabled save
- snapshot recovery load
- CLI command layer
- CLI binary entry point
- CLI snapshot show
- CLI snapshot verify
- CLI snapshot recover
- authoritative XC epoch minimum source
- authoritative XC epoch minimum provider source
- XC protocol params source
- XC protocol params build validation
- XC Build validation context
- XC Build commitment status model
- app Build view exposing commitmentStatus

## Snapshot safety model

Snapshot persistence is intentionally conservative.

Implemented snapshot helpers include:

- saveSnapshotFile
- loadSnapshotFile
- verifySnapshotJson
- verifySnapshotFile
- saveSnapshotFileWithBackup
- loadSnapshotFileWithRecovery

Important properties:

- new snapshots are serialized deterministically
- snapshot verification reuses the same decode / deserialize path
- backup-enabled save verifies the temp snapshot before replacement
- existing canonical snapshot is verified before backup creation
- corrupted canonical snapshots are not silently replaced by backup-enabled save
- recovery load can fall back to backup
- recovery load does not repair, overwrite, delete, or migrate files

## CLI safety model

The CLI is intentionally minimal and read-only.

Current CLI commands:

    help
    version
    snapshot:show --file <path>
    snapshot:verify --file <path>
    snapshot:recover --file <path> [--backup <path>]

Important CLI boundaries:

- no protocol state mutation
- no snapshot mutation
- no backup creation from CLI
- no automatic restore
- no migration
- no deletion of corrupted files

The CLI only reads, validates, summarizes, and reports recovery source.

## Validation status

Current validation baseline:

    npm run typecheck: passed
    npm test: passed
    npm run build: passed
    npm audit --audit-level=moderate: found 0 vulnerabilities
    42 test files passed
    328 tests passed

Current test tooling:

- Vitest ^4.1.8
- vitest.config.ts excludes dist/**
- tests are focused on source tests under tests/

## MVP assumptions and known limitations

Explicit MVP assumptions and known limitations are documented in:

- docs/assumptions.md

The XNTD lock / relock event identity design path is documented in:

- docs/registrar/xntd-lock-event-identity.md

These documents should be reviewed before adding live indexer integration, production chain integration, bridge execution, or token issuance logic.

The final MVP readiness checkpoint is documented in:

- docs/final-mvp-readiness-checkpoint.md

## Important non-goals in the current MVP

The current MVP intentionally does not include:

- production on-chain deployment
- real indexer integration
- live chain RPC integration
- automatic snapshot restore
- snapshot migration execution
- CLI mutation commands
- bridge execution
- admin / governance logic
- token issuance logic on X1
- UI
- Build actor profile
- Forge participation requirement
- unlock flow
- BLD marketplace

These are future layers and should not be assumed to exist in the current MVP.

## Review questions for Theo

Recommended review focus:

1. Is the Build state model coherent?
2. Are history_bld, available_bld, and origin_bld separated correctly?
3. Is the replay protection model sufficient for the current MVP?
4. Are watcher candidates converted into proofs at the right abstraction level?
5. Is application proof submission correctly separated from low-level registrar handlers?
6. Is the XNTD lock / relock rule logically sound?
7. Is the snapshot verification / backup / recovery model conservative enough?
8. Is commitmentStatus framed correctly as current XNTD commitment signal rather than Build validity?
9. Is appGetBuildView the right minimal app/service view layer?
10. Are the final MVP non-goals clear enough to prevent scope expansion?
11. Are CLI commands safely read-only?
12. Are there any hidden places where one event could affect accounting more than once?
13. Are there any model assumptions that should be documented more explicitly before moving toward integration?

## Suggested review approach

A useful review path is:

1. Read this summary.
2. Read docs/checkpoints/current-design-checkpoint.md.
3. Review implementation/*notes.md files for milestone history.
4. Run the validation commands.
5. Inspect the end-to-end scenario test.
6. Inspect registrar replay protection tests.
7. Inspect snapshot recovery tests.
8. Give recommendations before any new integration layer is added.

## Validation commands

    npm run typecheck
    npm test
    npm run build
    npm audit --audit-level=moderate

## Current status

The repository is ready for design and implementation review.

The MVP layers are intentionally small, deterministic, and test-covered.

The next recommended step is external review before adding new integration complexity.

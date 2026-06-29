# XXXL Program Identity and Authority Procedure

Status: Completed
Branch: `stage-xxxl-program-identity-authority-procedure`
Base: `51cf4b3 Add XXXL real Program ID selection procedure`

## Purpose

This document defines the authority model for the future XXXL SVM program identity.

This is a procedure stage only.

This stage does not select a real Program ID.

This stage does not record a final Program ID.

This stage does not create or expose any keypair.

This stage does not regenerate production PDA fixtures.

This stage does not verify production PDA fixtures.

This stage does not remove `PLACEHOLDER_PROGRAM_ID`.

This stage does not remove any deployment blocker.

This stage does not make the runtime deployable.

## Authority Model

The intended authority model is:

- temporary deploy or upgrade authority during development
- no human owner in final production
- no team wallet as final owner
- no guardian as program owner
- no deployer as final owner
- mint authority controlled by PDA, not by a human key
- production target is immutable or upgrade authority removed/frozen before real release

## Important Terminology

On SVM/Solana, a deployed program is technically owned by the loader.

The practical control question is not ordinary token-style ownership.

The practical control question is:

- who can deploy the program
- who can upgrade the program
- who controls the mint authority
- who can authorize bridge-backed minting
- who can change production configuration

This procedure uses "authority" for these practical control boundaries.

## Roles

### Temporary Deploy Authority

During development and pre-production, a dedicated deploy authority may exist.

This authority may be controlled by Sergey for development purposes.

This authority is temporary.

This authority must not be treated as final protocol ownership.

This authority must not be used as mint authority.

This authority must not be confused with guardians.

This authority must not be committed to the repository.

### Production Program Authority

The production target is no human program owner.

Before real release, the upgrade authority should be removed, frozen, or otherwise made unable to change the released runtime.

If an upgradeable period is used before final release, it must be explicitly documented as temporary and pre-production.

Production release readiness requires a dedicated decision record for immutability or authority removal.

### Gateway Mint Authority PDA

The gateway mint authority must be a PDA controlled by the XXXL program.

Current PDA name:

- `gateway_mint_authority`

Current PDA seeds:

- `xxxl`
- `gateway-mint-authority`
- `v1`

The PDA must be derived from the final real Program ID.

The PDA must become the SPL Token mint authority for gateway-backed XXXL minting.

No human wallet should be final mint authority.

No guardian wallet should be final mint authority.

No deploy authority should be final mint authority.

### Guardians

Guardians are not program owners.

Guardians are not deploy authorities.

Guardians are not upgrade authorities.

Guardians are not mint authorities.

Guardians only approve or attest gateway proof material according to the gateway model.

Guardian approval must not bypass runtime validation.

Guardian approval must not bypass replay protection.

Guardian approval must not bypass PDA mint authority checks.

### Proof Log / Replay State

The proof log or processed-burn state is runtime state, not program ownership.

It must support replay protection.

It must not grant ownership of the program to any signer.

It must not grant mint authority to any signer.

## Secret Handling Rules

The repository may record public addresses only.

The repository must never record:

- deploy private key
- deploy keypair file
- seed phrase
- mnemonic phrase
- private key bytes
- local wallet secret
- hardware wallet recovery phrase
- guardian private key
- production signer private key
- `.json` keypair contents
- `.env` secrets
- RPC provider secrets

A future Program ID selection record may include the public Program ID string.

It must not include the keypair that created or controls the Program ID.

## Program ID Rules

The real Program ID must be public.

The real Program ID must be recorded as an address only.

The real Program ID must not be:

- `XXXLProgram111111111111111111111111111111111`
- `11111111111111111111111111111111`
- `BPFLoaderUpgradeab1e11111111111111111111111`
- SPL Token Program ID
- System Program ID
- local fixture Program ID
- placeholder Program ID
- unreviewed Program ID

A testnet Program ID must not be silently reused as a mainnet Program ID unless explicitly intended and documented.

A localnet Program ID must never be treated as production Program ID.

## Network Separation

Program identity must be network-aware.

The repository should clearly distinguish:

- localnet fixture Program ID
- devnet or testnet Program ID
- mainnet Program ID

A Program ID selected for one network must not automatically imply readiness for another network.

Each production target network must have its own explicit selection record if needed.

## Required Future Program ID Selection Record

A future Program ID selection record must include:

- public Program ID string
- target network
- branch name
- commit SHA
- selection date
- source of selection
- statement that no private key is recorded
- statement that no deployment secret is recorded
- statement that Program ID is not placeholder
- statement that Program ID is not local fixture
- statement that gateway mint authority PDA must be regenerated from it
- statement that Program ID selection alone does not enable deployment

## Required Future Authority Decision Record

Before production release, a future authority decision record must state:

- whether the program is upgradeable during pre-production
- who or what controls temporary upgrade authority
- when upgrade authority must be removed or frozen
- how removal or freeze is verified
- whether any emergency upgrade path exists
- why the chosen model is compatible with protocol immutability

The production target remains:

- no human owner
- no deployer owner
- no guardian owner
- no team-wallet owner
- no admin mint authority

## Required Future PDA Follow-Up

After real Program ID selection:

1. derive `gateway_mint_authority` PDA from final real Program ID
2. record PDA and bump as production fixture
3. verify production PDA fixture
4. reject wrong Program ID
5. reject wrong PDA
6. reject wrong bump
7. reject wrong name
8. reject wrong kind
9. reject wrong report count

## Required Blocker Preservation

This procedure does not remove any blocker.

The following blockers remain active:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Program identity work alone must not enable:

- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- production guardian set
- production proof log
- external review closure
- runtime release

## Recommended Future Stage Sequence

Recommended sequence after this procedure:

1. `stage-xxxl-real-program-id-selection-record`
2. `stage-xxxl-production-pda-fixture-regeneration`
3. `stage-xxxl-production-pda-fixture-verification`
4. `stage-xxxl-program-id-readiness-model-update`
5. `stage-xxxl-placeholder-program-id-blocker-transition`
6. `stage-xxxl-production-authority-removal-plan`

The authority removal plan may also be prepared earlier, but release readiness must not depend on an undocumented authority model.

## Result

This stage defines the intended program identity and authority model.

Temporary deploy or upgrade authority may exist during development.

Final production target is no human owner and no admin mint authority.

Gateway mint authority must be a PDA controlled by the program.

Guardians are not program owners.

No secret material is recorded.

No Program ID is selected.

No PDA fixture is regenerated.

No blocker is removed.

No blocker is transitioned.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

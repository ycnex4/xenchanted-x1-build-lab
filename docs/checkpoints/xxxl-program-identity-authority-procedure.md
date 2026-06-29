# XXXL Program Identity and Authority Procedure Checkpoint

Status: Completed
Branch: `stage-xxxl-program-identity-authority-procedure`
Base: `51cf4b3 Add XXXL real Program ID selection procedure`

## Summary

This checkpoint records the intended authority model for the future XXXL SVM program identity.

This is a procedure checkpoint only.

No Program ID is selected.

No PDA fixture is regenerated.

No deployment blocker is removed.

## Files Changed

Expected changed files:

- `docs/xxxl/xxxl-program-identity-authority-procedure.md`
- `docs/checkpoints/xxxl-program-identity-authority-procedure.md`
- `docs/checkpoints/current-design-checkpoint.md`

No Rust source changes are expected.

No Cargo changes are expected.

No test changes are expected.

## Authority Model

The intended model is:

- temporary deploy or upgrade authority during development
- no human owner in final production
- no team wallet as final owner
- no guardian as program owner
- no deployer as final owner
- gateway mint authority controlled by PDA
- production target is immutable or upgrade authority removed/frozen before real release

## Role Separation

Temporary deploy authority:

- may exist during development
- must not be final protocol ownership
- must not be mint authority
- must not be committed to the repository

Gateway mint authority PDA:

- must be controlled by the program
- must be derived from final real Program ID
- must become SPL Token mint authority for gateway-backed XXXL minting

Guardians:

- are not program owners
- are not deploy authorities
- are not upgrade authorities
- are not mint authorities
- only approve or attest gateway proof material

## Secret Handling

The repository may record public addresses only.

The repository must never record:

- deploy private key
- keypair file
- seed phrase
- mnemonic phrase
- private key bytes
- guardian private key
- production signer private key
- `.env` secrets
- RPC provider secrets

## Program ID Selection Requirements

A future Program ID selection record must include:

- public Program ID string
- target network
- branch and commit
- selection source
- confirmation that no private key is recorded
- confirmation that no deployment secret is recorded
- confirmation that Program ID is not placeholder
- confirmation that Program ID is not local fixture
- statement that gateway mint authority PDA must be regenerated from it

## Blocker Status

No blocker is removed.

No blocker is transitioned.

`PLACEHOLDER_PROGRAM_ID` remains active.

Remaining active blockers:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Non-Goals

This checkpoint does not enable:

- deployment
- runtime release
- live route execution
- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`
- PDA fixture finalization
- guardian production configuration
- proof-log production configuration
- external review closure

## Result

The program identity and authority procedure is recorded.

Temporary development authority is allowed only as temporary pre-production authority.

Final production target is no human owner and no admin mint authority.

Gateway mint authority must be PDA-controlled.

Guardians are not program owners.

No secret material is recorded.

The Program ID blocker remains active.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

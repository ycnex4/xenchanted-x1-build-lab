# Program ID Binding Source Change

Status: SOURCE_CHANGE_NO_ACTIVATION
Package: program-id-binding-source-change
Approval: APPROVE_PROGRAM_ID_BINDING_SOURCE_CHANGE_NO_ACTIVATION

This package binds the reviewed X1 testnet Program ID at source level.

It removes `PlaceholderProgramId` from active deployment blockers while preserving the blocker enum/code/resolution as a historical transitioned blocker.

It does not authorize activation, deploy, upgrade, RPC mutation, live route enablement, SPL CPI enablement, guardian set instantiation, proof log instantiation, local-only fixture emission, or unrelated cleanup.

## Bound Public Values

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- ProgramData: `9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T`
- Upgrade authority: `DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc`
- Gateway mint authority PDA: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- Gateway mint authority bump: `252`

Only public values are recorded.

No private key or keypair is committed.

## Preserved Active Blockers

After this package, the active blockers remain:

- LiveRouteDisabled;
- SplCpiExecutionDisabled;
- ProductionGuardianSetUnset;
- ProductionProofLogUnset.

## Removed Active Blockers

- ExternalReviewIncomplete remains REMOVED.
- PlaceholderProgramId becomes REMOVED from active source blockers.

## Review-Integrity Condition

The prior removal of `ExternalReviewIncomplete` does not pre-approve Program ID binding as deployment-ready.

The final deployment-readiness review must re-gate the complete post-Program-ID-binding state before any activation or deployment path.

## Expected State After This Package

- real_program_id_bound=true
- PlaceholderProgramId_source_blocker_removed=true
- PlaceholderProgramId_effective_status=REMOVED
- ExternalReviewIncomplete_effective_status=REMOVED
- LiveRouteDisabled remains ACTIVE
- SplCpiExecutionDisabled remains ACTIVE
- ProductionGuardianSetUnset remains ACTIVE
- ProductionProofLogUnset remains ACTIVE
- runtime_deployable=false
- predeploy_gate=blocked
- activation_authorized=false
- deploy_authorized=false
- route_enablement_authorized=false
- spl_cpi_enablement_authorized=false
- rpc_mutation_authorized=false

## Next Required Decision

Request approval for:

program-id-dependent-pda-evidence

That future package requires separate approval.

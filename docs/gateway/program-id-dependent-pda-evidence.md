# Program-ID-Dependent PDA Evidence

Status: COMPLETE
Package: program-id-dependent-pda-evidence
Approval: APPROVE_PROGRAM_ID_DEPENDENT_PDA_EVIDENCE_NO_ACTIVATION

This package records PDA evidence for the already-bound X1 testnet Program ID.

## Scope

Pure evidence/documentation only.

No code mutation.
No activation.
No deploy.
No upgrade.
No RPC mutation.
No route enablement.
No SPL CPI enablement.
No guardian set instantiation.
No proof log instantiation.
No local-only fixture emission.
No unrelated cleanup.
No private key or keypair material committed or shared.

## Public Inputs

- Program ID: `D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my`
- gateway_mint_authority PDA: `BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG`
- gateway_mint_authority bump: `252`

## Evidence

Primary dry-run evidence:

- `docs/gateway/evidence/program-id-dependent-pda-evidence/program-id-dependent-pda-dry-run.txt`

Summary:

- `docs/gateway/evidence/program-id-dependent-pda-evidence/program-id-dependent-pda-dry-run-summary.txt`

The PDA dry-run is off-chain only and records:

- OFFCHAIN_ONLY=true
- RPC_USED=false
- DEPLOYED=false
- SOL_SPENT=false

## Preserved State

- ExternalReviewIncomplete remains REMOVED.
- PlaceholderProgramId remains REMOVED.
- LiveRouteDisabled remains ACTIVE.
- SplCpiExecutionDisabled remains ACTIVE.
- ProductionGuardianSetUnset remains ACTIVE.
- ProductionProofLogUnset remains ACTIVE.
- runtime_deployable=false.
- predeploy_gate=blocked.

## Review-Integrity Condition

Program ID binding and PDA evidence do not pre-approve deployment readiness.

Final deployment-readiness review must re-gate the complete post-PDA-evidence state before any activation or deployment path.

## Next Required Decision

Request approval for:

live-route-spl-cpi-activation-source-plan

That future package requires separate approval.

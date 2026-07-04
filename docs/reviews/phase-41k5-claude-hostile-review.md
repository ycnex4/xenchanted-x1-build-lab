# Phase 41K.5 Claude hostile review note

Status: ACCEPT WITH NOTES.

Scope reviewed:
- Phase 41K.5D.1.5 test-only mark+mint harness.
- Phase 41K.5D.2 production-path gated mark+mint e2e.
- Feature safety, atomicity, harness isolation, production-path coverage, and account/confused-deputy risks.

## Merge decision

Safe to merge the gated proof scaffold into `main`.

Reason:
- Default/no-feature production path remains closed-gate.
- D2 SPL mint gate opens only under the explicit dangerous D2 feature pair.
- D1.5 harness is isolated behind its own feature pair and magic discriminator.
- The branch is still a scaffold/proof phase, not a deploy/live activation phase.

## Gate-open / deploy blocker

B1: Guardian quorum / signature authorization is not yet verified in the live `consume_gateway_mint` mark+mint path.

D2 proves that the real production `ConsumeGatewayMint` instruction can carry execution through:

decode -> prepare -> execution plan -> mint CPI planning -> mark_processed_event_atomic -> witness check -> guarded SPL mint_to CPI

However, D2 does not prove guardian quorum authorization.

The D2 test intentionally provides no guardian signatures and no guardian public keys. Therefore it proves transport and atomic execution, not authorization. Before opening the production gate or treating the scaffold as deployable, guardian quorum verification must be integrated into the same atomic path before mark+mint can succeed.

This is a gate-open/deploy blocker, not a merge blocker for the closed-gate proof checkpoint.

## Permanent invariant

Closed mint gate must imply full transaction rollback, including rollback of any processed-event mark attempted before the SPL mint gate returns `CpiBoundaryNotReady`.

Future changes must not introduce:
- standalone mark-without-mint success path;
- early `Ok` after marking and before mint;
- split mark and mint across separate production instructions;
- opened SPL mint gate without guardian quorum authorization in the same atomic path.

## Next phase impact

After merge checkpoint:
1. Add negative/failure-mode tests.
2. Track guardian quorum authorization as a mandatory gate-open/deploy blocker.
3. Only after positive + negative + authorization paths are proven, decide between:
   - runtime/on-chain activation gate;
   - permanent production gate opening.

# XC Build validation app service context design review

This document reviews the XC Build validation app service context design milestone.

Reviewed branch:

    xc-build-validation-app-service-context-design-review

Reviewed design milestone:

    xc-build-validation-app-service-context-design

Reviewed files:

- implementation/xc-build-validation-app-service-context-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC Build validation app service context design is accepted.

The design correctly proposes optional `XcBuildValidationContext` input at the app/proof submission boundary.

The design correctly preserves backward compatibility.

The design correctly avoids adding global Build validity enforcement in the first runtime implementation.

The design correctly avoids changing watcher, registrar, or proof payload behavior in the first runtime implementation.

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-build-validation-app-service-context-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## App boundary review

The app/proof submission boundary is the correct next integration point because it already orchestrates:

- proof input
- registrar payload building
- registrar app helpers
- Build state mutation
- replay protection

The design correctly avoids moving protocol context directly into low-level watcher or registrar logic in this milestone.

## Optional context review

The recommended optional input name is accepted:

    xcBuildValidationContext?: XcBuildValidationContext

This name is explicit and avoids confusion with generic validation context.

The design correctly requires existing behavior to remain unchanged when the context is absent.

## Enforcement boundary review

The design correctly says the first implementation should only accept and carry the context.

It should not yet enforce:

- historical proof rejection based on currentEpoch
- Forge participation globally
- Core redeem nominal globally
- current XEN burn amount globally
- lock/relock rule changes
- watcher candidate changes
- proof payload changes unless explicitly needed

This keeps context wiring separate from future rule milestones.

## Existing lock validation review

The design correctly notes that existing XNTD lock / relock validation already has an authoritative epoch minimum path.

`XcBuildValidationContext` should not replace or contradict that path immediately.

Future integration should compare observed values and authoritative values only in a focused enforcement milestone.

## Future milestones review

The recommended later milestones are appropriate:

- xc-build-validation-epoch-policy-design
- xc-build-validation-core-redeem-rule-design
- xc-build-validation-forge-participation-rule-design
- xc-build-validation-xntd-lock-rule-design

These should not be collapsed into the first app-service context branch.

## Boundary review

The future first implementation should not add:

- real RPC execution
- scripts
- dependencies
- direct XcProtocolParamsSource calls
- process.env reads
- viem imports
- ethers imports
- createPublicClient
- http transport
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction
- watcher candidate changes
- registrar behavior changes
- proof payload changes unless explicitly needed
- global Build validity enforcement

## Grep review

The review grep found architecture and boundary terms inside design/checkpoint text only.

That is expected.

No runtime files were added or changed by this design milestone.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 40 test files, 316 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC Build validation app service context design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-validation-app-service-context

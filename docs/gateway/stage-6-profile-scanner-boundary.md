# Stage 6 Gateway Profile Scanner Boundary

Stage 6 starts the external intake path for gateway Build.

Stage 5 already defines what happens after a full-profile activation bundle exists:

- preview can show eligibility and missing requirements
- JSON-safe DTO can be sent to UI/API
- activation can validate and write state atomically

Stage 6 defines the layer before that bundle exists.

## Boundary

Stage 6 input:

- Ethereum/XC identity
- target X1 owner
- target Build id

Stage 6 output:

- completed Core redeem scan flag
- completed XEN.burn scan flag
- completed XNTD lock scan flag
- Core redeem candidates
- XEN.burn candidates
- XNTD lock candidate, if present

The scanner does not create a Build.

The scanner does not mutate registry state.

The scanner does not touch replay sets.

The scanner does not reserve eligibility.

## Bundle builder

The bundle builder converts a scan result into a GatewayFullProfileBuildActivationBundle.

Only finalized watcher candidates can become validated proofs.

If a candidate is not finalized, bundle building must fail before preview or activation.

## Verified zero

A completed scan may return zero Core redeem candidates, zero XEN.burn candidates, or no XNTD lock candidate.

That is still a valid scan result.

It is not sufficient for gateway Build activation unless the resulting preview satisfies both minimum requirements:

- minimum Core redeem history
- minimum XNTD lock for the current XC epoch

## Real adapters are later

Stage 6.1 is a deterministic boundary layer.

Real Ethereum/XC adapters are a later Stage 6/7 step.

This keeps scanner shape, bundle building, preview, and activation separated.

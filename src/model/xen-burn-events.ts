import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { applyXenBurnPower } from "../instructions/xen-burn-power.js";
import { type BuildState } from "./build-state.js";

export type XenBurnEventKey = string;

export interface XenBurnEvent {
  xenBurnKey: XenBurnEventKey;
  build: BuildState;
  amountXbp: bigint;
  burnedAt: bigint;
}

export interface XenBurnEventState {
  usedXenBurnEvents: Set<XenBurnEventKey>;
}

export function createXenBurnEventState(): XenBurnEventState {
  return {
    usedXenBurnEvents: new Set<XenBurnEventKey>()
  };
}

export function acceptXenBurnEvent(
  state: XenBurnEventState,
  event: XenBurnEvent
): BuildState {
  if (state.usedXenBurnEvents.has(event.xenBurnKey)) {
    throw new BuildError(
      BuildErrorCode.DuplicateXenBurnEvent,
      `XEN burn event already used: ${event.xenBurnKey}`
    );
  }

  const build = applyXenBurnPower({
    build: event.build,
    amountXbp: event.amountXbp,
    burnedAt: event.burnedAt
  });

  state.usedXenBurnEvents.add(event.xenBurnKey);

  return build;
}

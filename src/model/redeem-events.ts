import { BuildError, BuildErrorCode } from "../errors/build-error.js";
import { applyCoreRedeemBld } from "../instructions/core-redeem.js";
import { type BuildState } from "./build-state.js";

export type RedeemEventKey = string;

export interface CoreRedeemEvent {
  redeemKey: RedeemEventKey;
  build: BuildState;
  amountBld: bigint;
  redeemedAt: bigint;
}

export interface RedeemEventState {
  usedRedeemEvents: Set<RedeemEventKey>;
}

export function createRedeemEventState(): RedeemEventState {
  return {
    usedRedeemEvents: new Set<RedeemEventKey>()
  };
}

export function acceptCoreRedeemEvent(
  state: RedeemEventState,
  event: CoreRedeemEvent
): BuildState {
  if (state.usedRedeemEvents.has(event.redeemKey)) {
    throw new BuildError(
      BuildErrorCode.DuplicateRedeemEvent,
      `Core redeem event already used: ${event.redeemKey}`
    );
  }

  const build = applyCoreRedeemBld({
    build: event.build,
    amountBld: event.amountBld,
    redeemedAt: event.redeemedAt
  });

  state.usedRedeemEvents.add(event.redeemKey);

  return build;
}

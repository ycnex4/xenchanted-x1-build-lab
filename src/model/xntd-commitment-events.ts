import { BuildError, BuildErrorCode } from "../errors/build-error.js";

export type XntdCommitmentEventKey = string;

export interface XntdCommitmentEventState {
  usedXntdCommitmentEvents: Set<XntdCommitmentEventKey>;
}

export function createXntdCommitmentEventState(): XntdCommitmentEventState {
  return {
    usedXntdCommitmentEvents: new Set<XntdCommitmentEventKey>()
  };
}

export function acceptXntdCommitmentEvent(
  state: XntdCommitmentEventState,
  eventKey: XntdCommitmentEventKey
): XntdCommitmentEventKey {
  if (state.usedXntdCommitmentEvents.has(eventKey)) {
    throw new BuildError(
      BuildErrorCode.DuplicateXntdCommitmentEvent,
      `XNTD commitment event already used: ${eventKey}`
    );
  }

  state.usedXntdCommitmentEvents.add(eventKey);

  return eventKey;
}

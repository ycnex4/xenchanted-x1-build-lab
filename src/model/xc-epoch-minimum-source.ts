import { BuildError, BuildErrorCode } from "../errors/build-error.js";

export interface XcEpochMinimumSource {
  authoritativeEpochMinimum(lockEpoch: number): bigint | null;
}

export function createStaticXcEpochMinimumSource(
  epochMinimums: ReadonlyMap<number, bigint>
): XcEpochMinimumSource {
  return {
    authoritativeEpochMinimum(lockEpoch: number): bigint | null {
      return epochMinimums.get(lockEpoch) ?? null;
    }
  };
}

export function assertAuthoritativeXcEpochMinimum(
  source: XcEpochMinimumSource,
  lockEpoch: number,
  observedRequiredXntdLock: bigint
): void {
  const authoritativeMinimum = source.authoritativeEpochMinimum(lockEpoch);

  if (authoritativeMinimum === null) {
    throw new BuildError(
      BuildErrorCode.MissingAuthoritativeXcEpochMinimum,
      `Missing authoritative XC epoch minimum for lockEpoch=${lockEpoch.toString()}`
    );
  }

  if (observedRequiredXntdLock !== authoritativeMinimum) {
    throw new BuildError(
      BuildErrorCode.MismatchedAuthoritativeXcEpochMinimum,
      `Observed required XNTD lock does not match authoritative XC epoch minimum: lockEpoch=${lockEpoch.toString()}, observed=${observedRequiredXntdLock.toString()}, authoritative=${authoritativeMinimum.toString()}`
    );
  }
}

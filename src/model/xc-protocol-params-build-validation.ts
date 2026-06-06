import type { XcProtocolParams } from "../ethereum/xc-protocol-params-source.js";

export interface XcBuildRequirementsFromProtocolParams {
  readonly currentEpoch: bigint;
  readonly requiredBaseNominal: bigint;
  readonly requiredXenBurnAmount: bigint;
  readonly requiredXntdLockMinimum: bigint;
  readonly requiredForgeMinimum: bigint;
  readonly nextHalvingTs: bigint;
  readonly genesisTs: bigint;
  readonly halvingInterval: bigint;
  readonly xenBurnHalvingInterval: bigint;
}

export interface XcProtocolParamsBuildValidationInput {
  readonly protocolParams: XcProtocolParams;
}

export interface XcBuildProtocolParamsValidationResult {
  readonly isValid: boolean;
  readonly requirements: XcBuildRequirementsFromProtocolParams;
}

const FORGE_MIN_MULTIPLIER = 5n;

export function deriveCurrentXcBuildRequirements(
  protocolParams: XcProtocolParams
): XcBuildRequirementsFromProtocolParams {
  assertPositiveBigInt(protocolParams.currentBaseNominal, "currentBaseNominal");
  assertPositiveBigInt(protocolParams.currentXenBurnAmount, "currentXenBurnAmount");
  assertPositiveBigInt(protocolParams.halvingInterval, "halvingInterval");
  assertPositiveBigInt(
    protocolParams.xenBurnHalvingInterval,
    "xenBurnHalvingInterval"
  );
  assertNonNegativeBigInt(protocolParams.currentEpoch, "currentEpoch");
  assertNonNegativeBigInt(protocolParams.nextHalvingTs, "nextHalvingTs");
  assertNonNegativeBigInt(protocolParams.genesisTs, "genesisTs");

  return {
    currentEpoch: protocolParams.currentEpoch,
    requiredBaseNominal: protocolParams.currentBaseNominal,
    requiredXenBurnAmount: protocolParams.currentXenBurnAmount,
    requiredXntdLockMinimum: protocolParams.currentBaseNominal,
    requiredForgeMinimum: protocolParams.currentBaseNominal * FORGE_MIN_MULTIPLIER,
    nextHalvingTs: protocolParams.nextHalvingTs,
    genesisTs: protocolParams.genesisTs,
    halvingInterval: protocolParams.halvingInterval,
    xenBurnHalvingInterval: protocolParams.xenBurnHalvingInterval
  };
}

export function validateXcBuildAgainstProtocolParams(
  input: XcProtocolParamsBuildValidationInput
): XcBuildProtocolParamsValidationResult {
  return {
    isValid: true,
    requirements: deriveCurrentXcBuildRequirements(input.protocolParams)
  };
}

function assertPositiveBigInt(value: bigint, fieldName: string): void {
  if (value <= 0n) {
    throw new Error(
      `Invalid XC build requirement: ${fieldName} must be positive`
    );
  }
}

function assertNonNegativeBigInt(value: bigint, fieldName: string): void {
  if (value < 0n) {
    throw new Error(
      `Invalid XC protocol params build validation: ${fieldName} must be non-negative`
    );
  }
}

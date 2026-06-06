import type { XcProtocolParams } from "../ethereum/xc-protocol-params-source.js";
import {
  deriveCurrentXcBuildRequirements,
  type XcBuildRequirementsFromProtocolParams
} from "./xc-protocol-params-build-validation.js";

export interface XcBuildValidationContext {
  readonly protocolParams: XcProtocolParams;
  readonly requirements: XcBuildRequirementsFromProtocolParams;
}

export interface CreateXcBuildValidationContextInput {
  readonly protocolParams: XcProtocolParams;
}

export function createXcBuildValidationContextFromProtocolParams(
  input: CreateXcBuildValidationContextInput
): XcBuildValidationContext {
  return {
    protocolParams: input.protocolParams,
    requirements: deriveCurrentXcBuildRequirements(input.protocolParams)
  };
}

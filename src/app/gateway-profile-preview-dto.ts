import type { GatewayFullProfileBuildActivationBundle } from "./gateway-profile-activation.js";
import type { BuildApplicationState } from "./build-service.js";
import {
  appGetGatewayProfilePreviewView,
  type AppGetGatewayProfilePreviewViewInput,
  type GatewayProfilePreviewAction,
  type GatewayProfilePreviewMetricValue,
  type GatewayProfilePreviewRequirementCode,
} from "./gateway-profile-preview-view.js";

export type GatewayProfilePreviewJsonMetricValue =
  | string
  | number
  | boolean
  | null;

export interface GatewayFullProfileBuildPreviewDto {
  readonly buildExists: boolean;
  readonly owner: string;
  readonly buildId: string;
  readonly ethereumIdentity: string;

  readonly coreRedeemScanCompleted: boolean;
  readonly xenBurnScanCompleted: boolean;
  readonly xntdLockScanCompleted: boolean;

  readonly coreRedeemProofCount: number;
  readonly xenBurnProofCount: number;
  readonly hasXntdLockProof: boolean;

  readonly existingHistoryBld: string;
  readonly incomingHistoryBld: string;
  readonly totalPreviewHistoryBld: string;

  readonly existingHistoryXbp: string;
  readonly incomingHistoryXbp: string;
  readonly totalPreviewHistoryXbp: string;

  readonly previewLockedXntd: string;
  readonly previewRequiredXntdLock: string;
  readonly previewLockEpoch: number | null;

  readonly hasMinimumCoreRedeemHistory: boolean;
  readonly hasMinimumXntdLock: boolean;
  readonly eligible: boolean;
  readonly missingRequirements: readonly string[];
}

export interface GatewayProfilePreviewRequirementDto {
  readonly code: GatewayProfilePreviewRequirementCode;
  readonly satisfied: boolean;
  readonly label: string;
  readonly detail: string;
}

export interface GatewayProfilePreviewMetricDto {
  readonly key: string;
  readonly label: string;
  readonly value: GatewayProfilePreviewJsonMetricValue;
  readonly unit: string | null;
}

export interface AppGatewayProfilePreviewDto {
  readonly preview: GatewayFullProfileBuildPreviewDto;
  readonly action: GatewayProfilePreviewAction;
  readonly canCreateOrActivateBuild: boolean;
  readonly title: string;
  readonly summary: string;
  readonly requirements: readonly GatewayProfilePreviewRequirementDto[];
  readonly metrics: readonly GatewayProfilePreviewMetricDto[];
}

export interface AppGetGatewayProfilePreviewDtoInput {
  readonly app: BuildApplicationState;
  readonly bundle: GatewayFullProfileBuildActivationBundle;
}

function jsonMetricValue(
  value: GatewayProfilePreviewMetricValue,
): GatewayProfilePreviewJsonMetricValue {
  if (typeof value === "bigint") {
    return value.toString();
  }

  return value;
}

export function appGetGatewayProfilePreviewDto(
  input: AppGetGatewayProfilePreviewViewInput,
): AppGatewayProfilePreviewDto {
  const view = appGetGatewayProfilePreviewView(input);
  const preview = view.preview;

  return {
    preview: {
      buildExists: preview.buildExists,
      owner: preview.owner,
      buildId: preview.buildId,
      ethereumIdentity: preview.ethereumIdentity,

      coreRedeemScanCompleted: preview.coreRedeemScanCompleted,
      xenBurnScanCompleted: preview.xenBurnScanCompleted,
      xntdLockScanCompleted: preview.xntdLockScanCompleted,

      coreRedeemProofCount: preview.coreRedeemProofCount,
      xenBurnProofCount: preview.xenBurnProofCount,
      hasXntdLockProof: preview.hasXntdLockProof,

      existingHistoryBld: preview.existingHistoryBld.toString(),
      incomingHistoryBld: preview.incomingHistoryBld.toString(),
      totalPreviewHistoryBld: preview.totalPreviewHistoryBld.toString(),

      existingHistoryXbp: preview.existingHistoryXbp.toString(),
      incomingHistoryXbp: preview.incomingHistoryXbp.toString(),
      totalPreviewHistoryXbp: preview.totalPreviewHistoryXbp.toString(),

      previewLockedXntd: preview.previewLockedXntd.toString(),
      previewRequiredXntdLock: preview.previewRequiredXntdLock.toString(),
      previewLockEpoch: preview.previewLockEpoch,

      hasMinimumCoreRedeemHistory: preview.hasMinimumCoreRedeemHistory,
      hasMinimumXntdLock: preview.hasMinimumXntdLock,
      eligible: preview.eligible,
      missingRequirements: preview.missingRequirements,
    },
    action: view.action,
    canCreateOrActivateBuild: view.canCreateOrActivateBuild,
    title: view.title,
    summary: view.summary,
    requirements: view.requirements.map((requirement) => ({
      code: requirement.code,
      satisfied: requirement.satisfied,
      label: requirement.label,
      detail: requirement.detail,
    })),
    metrics: view.metrics.map((metric) => ({
      key: metric.key,
      label: metric.label,
      value: jsonMetricValue(metric.value),
      unit: metric.unit,
    })),
  };
}

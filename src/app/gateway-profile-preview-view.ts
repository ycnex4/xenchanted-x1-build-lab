import type { BuildApplicationState } from "./build-service.js";
import {
  type GatewayFullProfileBuildActivationBundle,
  type GatewayFullProfileBuildPreview,
  previewGatewayFullProfileBuildActivation,
} from "./gateway-profile-activation.js";

export type GatewayProfilePreviewAction =
  | "CREATE_BUILD"
  | "ACTIVATE_BUILD"
  | "UNAVAILABLE";

export type GatewayProfilePreviewRequirementCode =
  | "CORE_REDEEM_SCAN"
  | "XEN_BURN_SCAN"
  | "XNTD_LOCK_SCAN"
  | "MINIMUM_CORE_REDEEM_HISTORY"
  | "MINIMUM_XNTD_LOCK";

export type GatewayProfilePreviewMetricValue =
  | string
  | number
  | bigint
  | boolean
  | null;

export interface GatewayProfilePreviewMetricView {
  readonly key: string;
  readonly label: string;
  readonly value: GatewayProfilePreviewMetricValue;
  readonly unit: string | null;
}

export interface GatewayProfilePreviewRequirementView {
  readonly code: GatewayProfilePreviewRequirementCode;
  readonly satisfied: boolean;
  readonly label: string;
  readonly detail: string;
}

export interface AppGatewayProfilePreviewView {
  readonly preview: GatewayFullProfileBuildPreview;
  readonly action: GatewayProfilePreviewAction;
  readonly canCreateOrActivateBuild: boolean;
  readonly title: string;
  readonly summary: string;
  readonly requirements: readonly GatewayProfilePreviewRequirementView[];
  readonly metrics: readonly GatewayProfilePreviewMetricView[];
}

export interface AppGetGatewayProfilePreviewViewInput {
  readonly app: BuildApplicationState;
  readonly bundle: GatewayFullProfileBuildActivationBundle;
}

function buildAction(
  preview: GatewayFullProfileBuildPreview,
): GatewayProfilePreviewAction {
  if (!preview.eligible) {
    return "UNAVAILABLE";
  }

  return preview.buildExists ? "ACTIVATE_BUILD" : "CREATE_BUILD";
}

function buildSummary(
  action: GatewayProfilePreviewAction,
  missingRequirements: readonly string[],
): string {
  if (action === "CREATE_BUILD") {
    return "Eligible to create Build.";
  }

  if (action === "ACTIVATE_BUILD") {
    return "Eligible to activate Build.";
  }

  return `Missing requirements: ${missingRequirements.join(", ")}`;
}

function requirement(
  missing: ReadonlySet<string>,
  code: GatewayProfilePreviewRequirementCode,
  label: string,
  detail: string,
): GatewayProfilePreviewRequirementView {
  return {
    code,
    satisfied: !missing.has(code),
    label,
    detail,
  };
}

export function appGetGatewayProfilePreviewView(
  input: AppGetGatewayProfilePreviewViewInput,
): AppGatewayProfilePreviewView {
  const preview = previewGatewayFullProfileBuildActivation(
    input.app,
    input.bundle,
  );

  const missing = new Set<string>(preview.missingRequirements);
  const action = buildAction(preview);

  const requirements: GatewayProfilePreviewRequirementView[] = [
    requirement(
      missing,
      "CORE_REDEEM_SCAN",
      "Core redeem scan completed",
      "Gateway must scan Core redeem history before Build creation or activation.",
    ),
    requirement(
      missing,
      "XEN_BURN_SCAN",
      "XEN.burn scan completed",
      "Gateway must scan global XEN.burn history before Build creation or activation.",
    ),
    requirement(
      missing,
      "XNTD_LOCK_SCAN",
      "XNTD lock scan completed",
      "Gateway must check the XNTD lock requirement for the current XC epoch.",
    ),
    requirement(
      missing,
      "MINIMUM_CORE_REDEEM_HISTORY",
      "Minimum Core redeem history",
      "Build requires at least one Core redeem history unit.",
    ),
    requirement(
      missing,
      "MINIMUM_XNTD_LOCK",
      "Minimum XNTD lock",
      "Build requires XNTD lock at or above the current XC epoch minimum.",
    ),
  ];

  const metrics: GatewayProfilePreviewMetricView[] = [
    {
      key: "build_exists",
      label: "Build already exists",
      value: preview.buildExists,
      unit: null,
    },
    {
      key: "core_redeem_proof_count",
      label: "Core redeem proofs",
      value: preview.coreRedeemProofCount,
      unit: null,
    },
    {
      key: "history_bld_total",
      label: "Preview history BLD",
      value: preview.totalPreviewHistoryBld,
      unit: "BLD",
    },
    {
      key: "xen_burn_proof_count",
      label: "XEN.burn proofs",
      value: preview.xenBurnProofCount,
      unit: null,
    },
    {
      key: "history_xbp_total",
      label: "Preview history XBP",
      value: preview.totalPreviewHistoryXbp,
      unit: "XBP",
    },
    {
      key: "locked_xntd",
      label: "Preview locked XNTD",
      value: preview.previewLockedXntd,
      unit: "XNTD",
    },
    {
      key: "required_xntd_lock",
      label: "Required XNTD lock",
      value: preview.previewRequiredXntdLock,
      unit: "XNTD",
    },
    {
      key: "lock_epoch",
      label: "Lock epoch",
      value: preview.previewLockEpoch,
      unit: null,
    },
  ];

  return {
    preview,
    action,
    canCreateOrActivateBuild: preview.eligible,
    title: preview.buildExists
      ? "Build activation preview"
      : "Build creation preview",
    summary: buildSummary(action, preview.missingRequirements),
    requirements,
    metrics,
  };
}

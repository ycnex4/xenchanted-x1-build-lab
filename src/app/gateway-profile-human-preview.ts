import type { AppGatewayProfilePreviewDto } from "./gateway-profile-preview-dto.js";

export type GatewayProfileHumanPreviewStatus =
  | "READY_TO_CREATE"
  | "READY_TO_UPDATE"
  | "NEEDS_REQUIREMENTS";

export type GatewayProfileHumanPreviewTone = "success" | "warning" | "neutral";

export interface GatewayProfileHumanPreviewCard {
  readonly key: string;
  readonly title: string;
  readonly value: string;
  readonly detail: string;
  readonly tone: GatewayProfileHumanPreviewTone;
}

export interface GatewayProfileHumanPreviewNextStep {
  readonly code: string;
  readonly label: string;
  readonly detail: string;
  readonly completed: boolean;
}

export interface AppGatewayProfileHumanPreview {
  readonly status: GatewayProfileHumanPreviewStatus;
  readonly tone: GatewayProfileHumanPreviewTone;
  readonly title: string;
  readonly summary: string;
  readonly primaryActionLabel: string | null;
  readonly canProceed: boolean;
  readonly cards: readonly GatewayProfileHumanPreviewCard[];
  readonly nextSteps: readonly GatewayProfileHumanPreviewNextStep[];
}

function statusFromDto(
  dto: AppGatewayProfilePreviewDto,
): GatewayProfileHumanPreviewStatus {
  if (dto.action === "CREATE_BUILD") {
    return "READY_TO_CREATE";
  }

  if (dto.action === "UPDATE_BUILD") {
    return "READY_TO_UPDATE";
  }

  return "NEEDS_REQUIREMENTS";
}

function primaryActionLabel(
  status: GatewayProfileHumanPreviewStatus,
): string | null {
  if (status === "READY_TO_CREATE") {
    return "Create Build";
  }

  if (status === "READY_TO_UPDATE") {
    return "Update Build";
  }

  return null;
}

function statusTitle(status: GatewayProfileHumanPreviewStatus): string {
  if (status === "READY_TO_CREATE") {
    return "Ready to create Build";
  }

  if (status === "READY_TO_UPDATE") {
    return "Ready to update Build";
  }

  return "Requirements needed";
}

function statusTone(
  status: GatewayProfileHumanPreviewStatus,
): GatewayProfileHumanPreviewTone {
  return status === "NEEDS_REQUIREMENTS" ? "warning" : "success";
}

function plural(count: number, word: string): string {
  return count === 1 ? `${count} ${word}` : `${count} ${word}s`;
}

function buildCards(
  dto: AppGatewayProfilePreviewDto,
): GatewayProfileHumanPreviewCard[] {
  const preview = dto.preview;
  const satisfiedRequirements = dto.requirements.filter(
    (requirement) => requirement.satisfied,
  ).length;
  const missingLabels = dto.requirements
    .filter((requirement) => !requirement.satisfied)
    .map((requirement) => requirement.label);

  return [
    {
      key: "build_status",
      title: "Build status",
      value: preview.buildExists ? "Existing Build" : "No Build yet",
      detail: preview.buildExists
        ? "The profile can update an existing Build."
        : "The profile can create a new Build if requirements are satisfied.",
      tone: "neutral",
    },
    {
      key: "core_redeem_history",
      title: "Core redeem history",
      value: preview.hasMinimumCoreRedeemHistory
        ? `${preview.totalPreviewHistoryBld} BLD`
        : "Missing",
      detail: `${plural(preview.coreRedeemProofCount, "Core redeem proof")} found.`,
      tone: preview.hasMinimumCoreRedeemHistory ? "success" : "warning",
    },
    {
      key: "xen_burn_power",
      title: "XEN burn power",
      value: `${preview.totalPreviewHistoryXbp} XBP`,
      detail: `${plural(preview.xenBurnProofCount, "XEN.burn proof")} found.`,
      tone: "neutral",
    },
    {
      key: "xntd_lock",
      title: "XNTD lock",
      value: preview.hasMinimumXntdLock
        ? `${preview.previewLockedXntd} XNTD`
        : "Missing",
      detail: `Required lock: ${preview.previewRequiredXntdLock} XNTD.`,
      tone: preview.hasMinimumXntdLock ? "success" : "warning",
    },
    {
      key: "requirements",
      title: "Requirements",
      value: `${satisfiedRequirements}/${dto.requirements.length} satisfied`,
      detail:
        missingLabels.length === 0
          ? "All requirements are satisfied."
          : `Missing: ${missingLabels.join(", ")}.`,
      tone: missingLabels.length === 0 ? "success" : "warning",
    },
  ];
}

function buildNextSteps(
  dto: AppGatewayProfilePreviewDto,
  status: GatewayProfileHumanPreviewStatus,
): GatewayProfileHumanPreviewNextStep[] {
  if (status === "READY_TO_CREATE") {
    return [
      {
        code: "CREATE_BUILD",
        label: "Create Build",
        detail:
          "The scanned profile satisfies the requirements for Build creation.",
        completed: false,
      },
    ];
  }

  if (status === "READY_TO_UPDATE") {
    return [
      {
        code: "UPDATE_BUILD",
        label: "Update Build",
        detail: "The scanned profile can update the existing Build state.",
        completed: false,
      },
    ];
  }

  return dto.requirements
    .filter((requirement) => !requirement.satisfied)
    .map((requirement) => ({
      code: requirement.code,
      label: requirement.label,
      detail: requirement.detail,
      completed: false,
    }));
}

export function appGetGatewayProfileHumanPreview(
  dto: AppGatewayProfilePreviewDto,
): AppGatewayProfileHumanPreview {
  const status = statusFromDto(dto);

  return {
    status,
    tone: statusTone(status),
    title: statusTitle(status),
    summary: dto.summary,
    primaryActionLabel: primaryActionLabel(status),
    canProceed: dto.canCreateOrUpdateBuild,
    cards: buildCards(dto),
    nextSteps: buildNextSteps(dto, status),
  };
}

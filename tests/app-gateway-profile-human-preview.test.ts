import { describe, expect, it } from "vitest";
import {
  appGetGatewayProfileHumanPreview,
  type AppGatewayProfilePreviewDto,
} from "../src/index.js";

type PreviewDtoOverrides = Omit<
  Partial<AppGatewayProfilePreviewDto>,
  "preview"
> & {
  readonly preview?: Partial<AppGatewayProfilePreviewDto["preview"]>;
};

function makeDto(overrides: PreviewDtoOverrides): AppGatewayProfilePreviewDto {
  const base: AppGatewayProfilePreviewDto = {
    action: "CREATE_BUILD",
    canCreateOrUpdateBuild: true,
    title: "Build creation preview",
    summary: "Build can be created.",
    preview: {
      buildExists: false,
      owner: "x1-owner",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemScanCompleted: true,
      xenBurnScanCompleted: true,
      xntdLockScanCompleted: true,
      coreRedeemProofCount: 1,
      xenBurnProofCount: 1,
      hasXntdLockProof: true,
      existingHistoryBld: "0",
      incomingHistoryBld: "121",
      totalPreviewHistoryBld: "121",
      existingHistoryXbp: "0",
      incomingHistoryXbp: "1000",
      totalPreviewHistoryXbp: "1000",
      previewLockedXntd: "100000000",
      previewRequiredXntdLock: "100000000",
      previewLockEpoch: 0,
      hasMinimumCoreRedeemHistory: true,
      hasMinimumXntdLock: true,
      eligible: true,
      missingRequirements: [],
    },
    requirements: [
      {
        code: "CORE_REDEEM_SCAN",
        satisfied: true,
        label: "Core redeem scan completed",
        detail:
          "Gateway must scan Core redeem history before Build creation or update.",
      },
      {
        code: "MINIMUM_CORE_REDEEM_HISTORY",
        satisfied: true,
        label: "Minimum Core redeem history",
        detail: "Build requires at least one Core redeem history unit.",
      },
      {
        code: "MINIMUM_XNTD_LOCK",
        satisfied: true,
        label: "Minimum XNTD lock",
        detail:
          "Build requires XNTD lock at or above the current XC epoch minimum.",
      },
    ],
    metrics: [],
  };

  return {
    ...base,
    ...overrides,
    preview: {
      ...base.preview,
      ...(overrides.preview ?? {}),
    },
    requirements: overrides.requirements ?? base.requirements,
    metrics: overrides.metrics ?? base.metrics,
  };
}

describe("gateway profile human preview", () => {
  it("builds a ready-to-create human preview", () => {
    const human = appGetGatewayProfileHumanPreview(makeDto({}));

    expect(human.status).toBe("READY_TO_CREATE");
    expect(human.tone).toBe("success");
    expect(human.title).toBe("Ready to create Build");
    expect(human.primaryActionLabel).toBe("Create Build");
    expect(human.canProceed).toBe(true);
    expect(human.cards.map((card) => card.key)).toEqual([
      "build_status",
      "core_redeem_history",
      "xen_burn_power",
      "xntd_lock",
      "requirements",
    ]);
    expect(human.nextSteps).toEqual([
      {
        code: "CREATE_BUILD",
        label: "Create Build",
        detail:
          "The scanned profile satisfies the requirements for Build creation.",
        completed: false,
      },
    ]);
  });

  it("builds a ready-to-update human preview", () => {
    const human = appGetGatewayProfileHumanPreview(
      makeDto({
        action: "UPDATE_BUILD",
        title: "Build update preview",
        summary: "Build can be updated with this profile.",
        preview: {
          buildExists: true,
        },
      }),
    );

    expect(human.status).toBe("READY_TO_UPDATE");
    expect(human.tone).toBe("success");
    expect(human.title).toBe("Ready to update Build");
    expect(human.primaryActionLabel).toBe("Update Build");
    expect(human.canProceed).toBe(true);
    expect(human.cards[0]).toEqual({
      key: "build_status",
      title: "Build status",
      value: "Existing Build",
      detail: "The profile can update an existing Build.",
      tone: "neutral",
    });
    expect(human.nextSteps).toEqual([
      {
        code: "UPDATE_BUILD",
        label: "Update Build",
        detail: "The scanned profile can update the existing Build state.",
        completed: false,
      },
    ]);
  });

  it("builds a requirements-needed human preview", () => {
    const human = appGetGatewayProfileHumanPreview(
      makeDto({
        action: "UNAVAILABLE",
        canCreateOrUpdateBuild: false,
        summary:
          "Missing requirements: MINIMUM_CORE_REDEEM_HISTORY, MINIMUM_XNTD_LOCK",
        preview: {
          coreRedeemProofCount: 0,
          xenBurnProofCount: 0,
          hasXntdLockProof: false,
          incomingHistoryBld: "0",
          totalPreviewHistoryBld: "0",
          incomingHistoryXbp: "0",
          totalPreviewHistoryXbp: "0",
          previewLockedXntd: "0",
          hasMinimumCoreRedeemHistory: false,
          hasMinimumXntdLock: false,
          eligible: false,
          missingRequirements: [
            "MINIMUM_CORE_REDEEM_HISTORY",
            "MINIMUM_XNTD_LOCK",
          ],
        },
        requirements: [
          {
            code: "CORE_REDEEM_SCAN",
            satisfied: true,
            label: "Core redeem scan completed",
            detail:
              "Gateway must scan Core redeem history before Build creation or update.",
          },
          {
            code: "MINIMUM_CORE_REDEEM_HISTORY",
            satisfied: false,
            label: "Minimum Core redeem history",
            detail: "Build requires at least one Core redeem history unit.",
          },
          {
            code: "MINIMUM_XNTD_LOCK",
            satisfied: false,
            label: "Minimum XNTD lock",
            detail:
              "Build requires XNTD lock at or above the current XC epoch minimum.",
          },
        ],
      }),
    );

    expect(human.status).toBe("NEEDS_REQUIREMENTS");
    expect(human.tone).toBe("warning");
    expect(human.title).toBe("Requirements needed");
    expect(human.primaryActionLabel).toBeNull();
    expect(human.canProceed).toBe(false);
    expect(human.cards.find((card) => card.key === "requirements")).toEqual({
      key: "requirements",
      title: "Requirements",
      value: "1/3 satisfied",
      detail: "Missing: Minimum Core redeem history, Minimum XNTD lock.",
      tone: "warning",
    });
    expect(human.nextSteps).toEqual([
      {
        code: "MINIMUM_CORE_REDEEM_HISTORY",
        label: "Minimum Core redeem history",
        detail: "Build requires at least one Core redeem history unit.",
        completed: false,
      },
      {
        code: "MINIMUM_XNTD_LOCK",
        label: "Minimum XNTD lock",
        detail:
          "Build requires XNTD lock at or above the current XC epoch minimum.",
        completed: false,
      },
    ]);
  });
});

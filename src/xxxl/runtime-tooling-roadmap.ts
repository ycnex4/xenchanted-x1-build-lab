export const XXXL_RUNTIME_TOOLING_ROADMAP_VERSION = 1;

export const XXXL_RUNTIME_TOOLING_ROADMAP_STATUS = {
  Planned: "RUNTIME_TOOLING_ROADMAP_PLANNED",
} as const;

export const XXXL_RUNTIME_TOOLING_ROADMAP_ERROR = {
  WrongStatus: "WRONG_STATUS",
  MissingImmediateBaseline: "MISSING_IMMEDIATE_BASELINE",
  MissingSecurityBaseline: "MISSING_SECURITY_BASELINE",
  MissingClippyCleanup: "MISSING_CLIPPY_CLEANUP",
  MissingManualAudit: "MISSING_MANUAL_AUDIT",
  MissingMolluskStage: "MISSING_MOLLUSK_STAGE",
  MissingTridentStage: "MISSING_TRIDENT_STAGE",
  MissingPredeployGate: "MISSING_PREDEPLOY_GATE",
  ClippyTooEarly: "CLIPPY_TOO_EARLY",
  HeavyFuzzingTooEarly: "HEAVY_FUZZING_TOO_EARLY",
} as const;

export type XXXLRuntimeToolingRoadmapErrorCode =
  (typeof XXXL_RUNTIME_TOOLING_ROADMAP_ERROR)[keyof typeof XXXL_RUNTIME_TOOLING_ROADMAP_ERROR];

export type XXXLRuntimeToolingRoadmapStage = {
  readonly id: string;
  readonly timing: string;
  readonly requiredTools: readonly string[];
  readonly reportOnlyTools: readonly string[];
  readonly hardGateTools: readonly string[];
  readonly purpose: string;
};

export type XXXLRuntimeToolingRoadmap = {
  readonly version: typeof XXXL_RUNTIME_TOOLING_ROADMAP_VERSION;
  readonly status: typeof XXXL_RUNTIME_TOOLING_ROADMAP_STATUS.Planned;
  readonly stages: readonly XXXLRuntimeToolingRoadmapStage[];
  readonly explicitDecisions: {
    readonly clippyWarningsAreNotHardGateUntilCleanup: true;
    readonly cargoGeigerIsReportOnlyUntilManualReview: true;
    readonly molluskAfterGuardedHandlerWiring: true;
    readonly tridentAfterMolluskAndInvariantCatalog: true;
    readonly finalPredeployGateCombinesAllTools: true;
  };
  readonly nonGoals: readonly string[];
};

export type XXXLRuntimeToolingRoadmapValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimeToolingRoadmapErrorCode[];
};

export function xxxlRuntimeToolingRoadmap(): XXXLRuntimeToolingRoadmap {
  return {
    version: XXXL_RUNTIME_TOOLING_ROADMAP_VERSION,
    status: XXXL_RUNTIME_TOOLING_ROADMAP_STATUS.Planned,
    stages: [
      {
        id: "current-runtime-layer-checks",
        timing: "now-and-every-runtime-fixture-stage",
        requiredTools: [
          "npm run typecheck",
          "npm test -- --reporter=dot",
          "npm run build",
          "cargo test targeted modules",
        ],
        reportOnlyTools: [],
        hardGateTools: [
          "npm run typecheck",
          "npm test -- --reporter=dot",
          "npm run build",
          "cargo test targeted modules",
        ],
        purpose:
          "Keep TypeScript fixtures, docs exports, and Rust runtime modules green while layering runtime logic.",
      },
      {
        id: "rust-quality-security-baseline",
        timing: "after-atomic-execution-plan-fixture",
        requiredTools: [
          "cargo fmt --check",
          "cargo test",
          "cargo audit",
          "cargo deny check",
          "manual account-constraint audit checklist draft",
        ],
        reportOnlyTools: ["cargo geiger"],
        hardGateTools: [
          "cargo fmt --check",
          "cargo test",
          "cargo audit",
          "cargo deny check",
        ],
        purpose:
          "Create the first Rust quality/security baseline without making unsafe statistics or known scaffold warnings block progress.",
      },
      {
        id: "rust-clippy-warning-cleanup",
        timing: "after-rust-quality-security-baseline",
        requiredTools: ["cargo clippy --all-targets --all-features -- -D warnings"],
        reportOnlyTools: [],
        hardGateTools: ["cargo clippy --all-targets --all-features -- -D warnings"],
        purpose:
          "Resolve or explicitly isolate the known solana_program entrypoint cfg warnings before making clippy a hard gate.",
      },
      {
        id: "manual-account-constraint-audit",
        timing: "before-guarded-live-handler-wiring",
        requiredTools: ["manual account-constraint audit checklist"],
        reportOnlyTools: [],
        hardGateTools: ["manual account-constraint audit checklist"],
        purpose:
          "Audit account indexes, owners, signer/writable flags, rent, PDA seeds, SPL mint authority, recipient token account constraints, replay state, CPI order, and rollback assumptions.",
      },
      {
        id: "mollusk-instruction-state-transition-tests",
        timing: "after-guarded-live-handler-wiring-model",
        requiredTools: ["Mollusk instruction/state-transition tests"],
        reportOnlyTools: [],
        hardGateTools: ["Mollusk instruction/state-transition tests"],
        purpose:
          "Run SVM-level instruction/account state transition tests for valid and invalid consume_gateway_mint paths.",
      },
      {
        id: "trident-fuzzing-invariants",
        timing: "after-mollusk-transition-suite-and-invariant-catalog",
        requiredTools: ["Trident fuzzing"],
        reportOnlyTools: [],
        hardGateTools: ["Trident fuzzing"],
        purpose:
          "Fuzz instruction sequences and invariants such as no double consume, no wrong recipient credit, no overflow, and no route mismatch acceptance.",
      },
      {
        id: "predeploy-security-readiness-gate",
        timing: "before-any-real-deploy-or-authority-freeze",
        requiredTools: [
          "npm run typecheck",
          "npm test -- --reporter=dot",
          "npm run build",
          "cargo fmt --check",
          "cargo clippy --all-targets --all-features -- -D warnings",
          "cargo test",
          "cargo audit",
          "cargo deny check",
          "cargo geiger",
          "Mollusk transition suite",
          "Trident fuzz suite",
          "manual account-constraint audit checklist",
          "manual authority/freeze checklist",
          "manual deployment config checklist",
        ],
        reportOnlyTools: [],
        hardGateTools: [
          "npm run typecheck",
          "npm test -- --reporter=dot",
          "npm run build",
          "cargo fmt --check",
          "cargo clippy --all-targets --all-features -- -D warnings",
          "cargo test",
          "cargo audit",
          "cargo deny check",
          "Mollusk transition suite",
          "Trident fuzz suite",
          "manual account-constraint audit checklist",
          "manual authority/freeze checklist",
          "manual deployment config checklist",
        ],
        purpose:
          "Combine all quality, security, transition, fuzzing, account, authority, and deployment gates before any real deploy/freeze action.",
      },
    ],
    explicitDecisions: {
      clippyWarningsAreNotHardGateUntilCleanup: true,
      cargoGeigerIsReportOnlyUntilManualReview: true,
      molluskAfterGuardedHandlerWiring: true,
      tridentAfterMolluskAndInvariantCatalog: true,
      finalPredeployGateCombinesAllTools: true,
    },
    nonGoals: [
      "NO_RUNTIME_LOGIC_CHANGE",
      "NO_DEPLOYMENT",
      "NO_ROUTE_ACTIVATION",
      "NO_TOOL_INSTALLATION_IN_THIS_STAGE",
      "NO_HEAVY_FUZZING_BEFORE_HANDLER_MODEL",
      "NO_CLIPPY_D_WARNINGS_HARD_GATE_BEFORE_WARNING_CLEANUP",
    ],
  };
}

export function validateXXXLRuntimeToolingRoadmap(
  roadmap: XXXLRuntimeToolingRoadmap = xxxlRuntimeToolingRoadmap(),
): XXXLRuntimeToolingRoadmapValidationResult {
  const errors: XXXLRuntimeToolingRoadmapErrorCode[] = [];
  const stageIds = new Set(roadmap.stages.map((stage) => stage.id));

  if (roadmap.status !== XXXL_RUNTIME_TOOLING_ROADMAP_STATUS.Planned) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.WrongStatus);
  }

  if (!stageIds.has("current-runtime-layer-checks")) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingImmediateBaseline);
  }

  if (!stageIds.has("rust-quality-security-baseline")) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingSecurityBaseline);
  }

  if (!stageIds.has("rust-clippy-warning-cleanup")) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingClippyCleanup);
  }

  if (!stageIds.has("manual-account-constraint-audit")) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingManualAudit);
  }

  if (!stageIds.has("mollusk-instruction-state-transition-tests")) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingMolluskStage);
  }

  if (!stageIds.has("trident-fuzzing-invariants")) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingTridentStage);
  }

  if (!stageIds.has("predeploy-security-readiness-gate")) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingPredeployGate);
  }

  const securityBaseline = roadmap.stages.find(
    (stage) => stage.id === "rust-quality-security-baseline",
  );

  if (
    securityBaseline?.hardGateTools.includes(
      "cargo clippy --all-targets --all-features -- -D warnings",
    )
  ) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.ClippyTooEarly);
  }

  const immediateBaseline = roadmap.stages.find(
    (stage) => stage.id === "current-runtime-layer-checks",
  );

  if (
    immediateBaseline?.hardGateTools.some(
      (tool) =>
        tool.includes("Mollusk") ||
        tool.includes("Trident") ||
        tool.includes("cargo geiger"),
    )
  ) {
    errors.push(XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.HeavyFuzzingTooEarly);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalRuntimeToolingRoadmapJson(
  roadmap: XXXLRuntimeToolingRoadmap = xxxlRuntimeToolingRoadmap(),
): string {
  return JSON.stringify([
    ["version", roadmap.version],
    ["status", roadmap.status],
    ["stages", roadmap.stages],
    ["explicitDecisions", roadmap.explicitDecisions],
    ["nonGoals", roadmap.nonGoals],
  ]);
}

export function xxxlRuntimeToolingRoadmapMarkdown(
  roadmap: XXXLRuntimeToolingRoadmap = xxxlRuntimeToolingRoadmap(),
): string {
  return [
    "# XXXL Runtime Tooling Roadmap",
    "",
    `Status: ${roadmap.status}`,
    "",
    "Stages:",
    ...roadmap.stages.flatMap((stage, index) => [
      "",
      `## ${index + 1}. ${stage.id}`,
      "",
      `Timing: ${stage.timing}`,
      "",
      `Purpose: ${stage.purpose}`,
      "",
      "Required tools:",
      ...stage.requiredTools.map((tool) => `- ${tool}`),
      "",
      "Report-only tools:",
      ...(stage.reportOnlyTools.length === 0
        ? ["- none"]
        : stage.reportOnlyTools.map((tool) => `- ${tool}`)),
      "",
      "Hard gates:",
      ...stage.hardGateTools.map((tool) => `- ${tool}`),
    ]),
    "",
    "Non-goals:",
    ...roadmap.nonGoals.map((item) => `- ${item}`),
    "",
  ].join("\n");
}

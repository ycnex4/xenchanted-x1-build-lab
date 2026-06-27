import {
  createXXXLRuntimeFixtureReportExport,
  validateXXXLRuntimeFixtureReportExport,
  type XXXLRuntimeFixtureReportExport,
  type XXXLRuntimeFixtureReportExportErrorCode,
} from "./runtime-fixture-report-export.js";
import {
  XXXL_RUNTIME_EXECUTION_VECTOR_ID,
  type XXXLRuntimeExecutionVectorId,
} from "./runtime-execution-vectors.js";

export const XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_VERSION = 1;
export const XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ID =
  "XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_V1";

export const XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS = {
  ReadyForTheoReview: "READY_FOR_THEO_REVIEW",
  ReadyForRuntimeImplementationPlanning:
    "READY_FOR_RUNTIME_IMPLEMENTATION_PLANNING",
} as const;

export type XXXLRuntimePredeployReviewReadiness =
  (typeof XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS)[keyof typeof XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS];

export const XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR = {
  ReportValidationFailed: "REPORT_VALIDATION_FAILED",
  PackageNotReady: "PACKAGE_NOT_READY",
  BaselineMismatch: "BASELINE_MISMATCH",
  MissingClosedRuntimePrepItem: "MISSING_CLOSED_RUNTIME_PREP_ITEM",
  MissingRemainingRuntimeItem: "MISSING_REMAINING_RUNTIME_ITEM",
  MissingRouteAwareCoverage: "MISSING_ROUTE_AWARE_COVERAGE",
  WrongCanonicalJson: "WRONG_CANONICAL_JSON",
  WrongMarkdown: "WRONG_MARKDOWN",
} as const;

export type XXXLRuntimePredeployReviewPackageErrorCode =
  (typeof XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR)[keyof typeof XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR];

export type XXXLRuntimePredeployReviewBaseline = {
  readonly testFiles: 82;
  readonly tests: 613;
  readonly typecheck: "passing";
  readonly build: "passing";
};

export type XXXLRuntimePredeployReviewReportSummary = {
  readonly reportAllOk: boolean;
  readonly reportValidationOk: boolean;
  readonly reportErrors: readonly XXXLRuntimeFixtureReportExportErrorCode[];
  readonly fixtureCount: number;
  readonly vectorCount: number;
  readonly cpiCommittedVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly cpiSkippedVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly supplyAuditOkVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly expectedRejectionVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly routeAwareSuccessVectorIds: readonly XXXLRuntimeExecutionVectorId[];
};

export type XXXLRuntimePredeployReviewPackageCore = {
  readonly version: typeof XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_VERSION;
  readonly packageId: typeof XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ID;
  readonly readiness: readonly XXXLRuntimePredeployReviewReadiness[];
  readonly baseline: XXXLRuntimePredeployReviewBaseline;
  readonly reportSummary: XXXLRuntimePredeployReviewReportSummary;
  readonly closedRuntimePrepItems: readonly string[];
  readonly remainingBeforeLiveRuntime: readonly string[];
  readonly nextRecommendedStages: readonly string[];
  readonly recommendation: string;
};

export type XXXLRuntimePredeployReviewPackage =
  XXXLRuntimePredeployReviewPackageCore & {
    readonly canonicalJson: string;
    readonly markdown: string;
  };

export type XXXLRuntimePredeployReviewPackageValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimePredeployReviewPackageErrorCode[];
};

const REQUIRED_CLOSED_RUNTIME_PREP_ITEMS = [
  "runtime serialization boundary",
  "account serialization vectors",
  "instruction serialization vectors",
  "multichain low-weight route policy",
  "runtime program skeleton",
  "runtime execution vectors",
  "runtime dry-run fixtures",
  "runtime fixture report export",
] as const;

const REQUIRED_REMAINING_RUNTIME_ITEMS = [
  "live X1/SVM program implementation",
  "real account byte serialization",
  "real instruction byte serialization",
  "real PDA derivation",
  "real SPL Token mint_to CPI integration",
  "deployment dry-run against target environment",
  "authority freeze execution procedure",
  "incident response runbook drill",
] as const;

export function createXXXLRuntimePredeployReviewPackage(
  report: XXXLRuntimeFixtureReportExport = createXXXLRuntimeFixtureReportExport(),
): XXXLRuntimePredeployReviewPackage {
  const reportValidation = validateXXXLRuntimeFixtureReportExport(report);

  const core: XXXLRuntimePredeployReviewPackageCore = {
    version: XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_VERSION,
    packageId: XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ID,
    readiness: [
      XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS.ReadyForTheoReview,
      XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS.ReadyForRuntimeImplementationPlanning,
    ],
    baseline: {
      testFiles: 82,
      tests: 613,
      typecheck: "passing",
      build: "passing",
    },
    reportSummary: {
      reportAllOk: report.allOk,
      reportValidationOk: reportValidation.ok,
      reportErrors: reportValidation.errors,
      fixtureCount: report.fixtureCount,
      vectorCount: report.vectorCount,
      cpiCommittedVectorIds: report.cpiCommittedVectorIds,
      cpiSkippedVectorIds: report.cpiSkippedVectorIds,
      supplyAuditOkVectorIds: report.supplyAuditOkVectorIds,
      expectedRejectionVectorIds: report.expectedRejectionVectorIds,
      routeAwareSuccessVectorIds: report.routeAwareSuccessVectorIds,
    },
    closedRuntimePrepItems: REQUIRED_CLOSED_RUNTIME_PREP_ITEMS,
    remainingBeforeLiveRuntime: REQUIRED_REMAINING_RUNTIME_ITEMS,
    nextRecommendedStages: [
      "Theo review of runtime-prep package",
      "production runtime account byte layout",
      "production runtime instruction byte layout",
      "X1/SVM implementation skeleton",
      "real deployment dry-run fixtures",
    ],
    recommendation:
      "Submit this package for Theo review before moving from TypeScript/model-layer runtime preparation into live X1/SVM implementation work.",
  };

  return {
    ...core,
    canonicalJson: xxxlCanonicalRuntimePredeployReviewPackageJson(core),
    markdown: xxxlRuntimePredeployReviewPackageMarkdown(core),
  };
}

export function validateXXXLRuntimePredeployReviewPackage(
  reviewPackage: XXXLRuntimePredeployReviewPackage,
): XXXLRuntimePredeployReviewPackageValidationResult {
  const errors: XXXLRuntimePredeployReviewPackageErrorCode[] = [];
  const {
    canonicalJson: _canonicalJson,
    markdown: _markdown,
    ...core
  } = reviewPackage;

  if (
    !reviewPackage.reportSummary.reportAllOk ||
    !reviewPackage.reportSummary.reportValidationOk
  ) {
    errors.push(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.ReportValidationFailed,
    );
  }

  if (
    !reviewPackage.readiness.includes(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS.ReadyForTheoReview,
    ) ||
    !reviewPackage.readiness.includes(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS.ReadyForRuntimeImplementationPlanning,
    )
  ) {
    errors.push(XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.PackageNotReady);
  }

  if (
    reviewPackage.baseline.testFiles !== 82 ||
    reviewPackage.baseline.tests !== 613 ||
    reviewPackage.baseline.typecheck !== "passing" ||
    reviewPackage.baseline.build !== "passing"
  ) {
    errors.push(XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.BaselineMismatch);
  }

  for (const item of REQUIRED_CLOSED_RUNTIME_PREP_ITEMS) {
    if (!reviewPackage.closedRuntimePrepItems.includes(item)) {
      errors.push(
        XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.MissingClosedRuntimePrepItem,
      );
    }
  }

  for (const item of REQUIRED_REMAINING_RUNTIME_ITEMS) {
    if (!reviewPackage.remainingBeforeLiveRuntime.includes(item)) {
      errors.push(
        XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.MissingRemainingRuntimeItem,
      );
    }
  }

  if (
    !reviewPackage.reportSummary.routeAwareSuccessVectorIds.includes(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
    ) ||
    !reviewPackage.reportSummary.routeAwareSuccessVectorIds.includes(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    )
  ) {
    errors.push(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.MissingRouteAwareCoverage,
    );
  }

  if (
    xxxlCanonicalRuntimePredeployReviewPackageJson(core) !==
    reviewPackage.canonicalJson
  ) {
    errors.push(XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.WrongCanonicalJson);
  }

  if (xxxlRuntimePredeployReviewPackageMarkdown(core) !== reviewPackage.markdown) {
    errors.push(XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.WrongMarkdown);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalRuntimePredeployReviewPackageJson(
  reviewPackage: XXXLRuntimePredeployReviewPackageCore,
): string {
  return JSON.stringify([
    ["version", reviewPackage.version],
    ["packageId", reviewPackage.packageId],
    ["readiness", reviewPackage.readiness],
    [
      "baseline",
      [
        ["testFiles", reviewPackage.baseline.testFiles],
        ["tests", reviewPackage.baseline.tests],
        ["typecheck", reviewPackage.baseline.typecheck],
        ["build", reviewPackage.baseline.build],
      ],
    ],
    [
      "reportSummary",
      [
        ["reportAllOk", reviewPackage.reportSummary.reportAllOk],
        ["reportValidationOk", reviewPackage.reportSummary.reportValidationOk],
        ["reportErrors", reviewPackage.reportSummary.reportErrors],
        ["fixtureCount", reviewPackage.reportSummary.fixtureCount],
        ["vectorCount", reviewPackage.reportSummary.vectorCount],
        [
          "cpiCommittedVectorIds",
          reviewPackage.reportSummary.cpiCommittedVectorIds,
        ],
        ["cpiSkippedVectorIds", reviewPackage.reportSummary.cpiSkippedVectorIds],
        [
          "supplyAuditOkVectorIds",
          reviewPackage.reportSummary.supplyAuditOkVectorIds,
        ],
        [
          "expectedRejectionVectorIds",
          reviewPackage.reportSummary.expectedRejectionVectorIds,
        ],
        [
          "routeAwareSuccessVectorIds",
          reviewPackage.reportSummary.routeAwareSuccessVectorIds,
        ],
      ],
    ],
    ["closedRuntimePrepItems", reviewPackage.closedRuntimePrepItems],
    ["remainingBeforeLiveRuntime", reviewPackage.remainingBeforeLiveRuntime],
    ["nextRecommendedStages", reviewPackage.nextRecommendedStages],
    ["recommendation", reviewPackage.recommendation],
  ]);
}

export function xxxlRuntimePredeployReviewPackageMarkdown(
  reviewPackage: XXXLRuntimePredeployReviewPackageCore,
): string {
  const lines: string[] = [];

  lines.push("# XXXL Runtime Predeploy Review Package");
  lines.push("");
  lines.push(`- Package: ${reviewPackage.packageId}`);
  lines.push(`- Version: ${reviewPackage.version}`);
  lines.push(`- Readiness: ${reviewPackage.readiness.join(", ")}`);
  lines.push(
    `- Baseline: ${reviewPackage.baseline.testFiles} files / ${reviewPackage.baseline.tests} tests passing`,
  );
  lines.push(`- Typecheck: ${reviewPackage.baseline.typecheck}`);
  lines.push(`- Build: ${reviewPackage.baseline.build}`);
  lines.push("");
  lines.push("## Runtime fixture report summary");
  lines.push("");
  lines.push(`- Report all OK: ${reviewPackage.reportSummary.reportAllOk}`);
  lines.push(
    `- Report validation OK: ${reviewPackage.reportSummary.reportValidationOk}`,
  );
  lines.push(`- Fixture count: ${reviewPackage.reportSummary.fixtureCount}`);
  lines.push(`- Vector count: ${reviewPackage.reportSummary.vectorCount}`);
  lines.push(
    `- CPI committed vectors: ${formatList(
      reviewPackage.reportSummary.cpiCommittedVectorIds,
    )}`,
  );
  lines.push(
    `- CPI skipped vectors: ${formatList(
      reviewPackage.reportSummary.cpiSkippedVectorIds,
    )}`,
  );
  lines.push(
    `- Supply audit OK vectors: ${formatList(
      reviewPackage.reportSummary.supplyAuditOkVectorIds,
    )}`,
  );
  lines.push(
    `- Route-aware success vectors: ${formatList(
      reviewPackage.reportSummary.routeAwareSuccessVectorIds,
    )}`,
  );
  lines.push("");
  lines.push("## Closed runtime-prep items");
  lines.push("");
  for (const item of reviewPackage.closedRuntimePrepItems) {
    lines.push(`- ${item}`);
  }
  lines.push("");
  lines.push("## Remaining before live runtime");
  lines.push("");
  for (const item of reviewPackage.remainingBeforeLiveRuntime) {
    lines.push(`- ${item}`);
  }
  lines.push("");
  lines.push("## Next recommended stages");
  lines.push("");
  for (const item of reviewPackage.nextRecommendedStages) {
    lines.push(`- ${item}`);
  }
  lines.push("");
  lines.push("## Recommendation");
  lines.push("");
  lines.push(reviewPackage.recommendation);

  return lines.join("\n");
}

function formatList(items: readonly string[]): string {
  return items.length === 0 ? "none" : items.join(", ");
}

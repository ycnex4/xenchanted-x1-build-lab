import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_EXECUTION_VECTOR_ID,
  XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR,
  XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ID,
  XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_VERSION,
  XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS,
  createXXXLRuntimeFixtureReportExport,
  createXXXLRuntimePredeployReviewPackage,
  validateXXXLRuntimePredeployReviewPackage,
  xxxlCanonicalRuntimePredeployReviewPackageJson,
  xxxlRuntimePredeployReviewPackageMarkdown,
  type XXXLRuntimePredeployReviewPackage,
} from "../../src/index.js";

describe("XXXL runtime predeploy review package", () => {
  it("exports deterministic predeploy package metadata", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.version).toBe(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_VERSION,
    );
    expect(reviewPackage.packageId).toBe(XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ID);
    expect(reviewPackage.readiness).toEqual([
      XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS.ReadyForTheoReview,
      XXXL_RUNTIME_PREDEPLOY_REVIEW_READINESS.ReadyForRuntimeImplementationPlanning,
    ]);
  });

  it("records the current validation baseline", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.baseline).toEqual({
      testFiles: 82,
      tests: 613,
      typecheck: "passing",
      build: "passing",
    });
  });

  it("summarizes a valid runtime fixture report", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.reportSummary.reportAllOk).toBe(true);
    expect(reviewPackage.reportSummary.reportValidationOk).toBe(true);
    expect(reviewPackage.reportSummary.reportErrors).toEqual([]);
    expect(reviewPackage.reportSummary.fixtureCount).toBe(4);
    expect(reviewPackage.reportSummary.vectorCount).toBe(8);
  });

  it("records closed runtime-prep items in canonical order", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.closedRuntimePrepItems).toEqual([
      "runtime serialization boundary",
      "account serialization vectors",
      "instruction serialization vectors",
      "multichain low-weight route policy",
      "runtime program skeleton",
      "runtime execution vectors",
      "runtime dry-run fixtures",
      "runtime fixture report export",
    ]);
  });

  it("records remaining work before live runtime", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.remainingBeforeLiveRuntime).toContain(
      "live X1/SVM program implementation",
    );
    expect(reviewPackage.remainingBeforeLiveRuntime).toContain(
      "real SPL Token mint_to CPI integration",
    );
    expect(reviewPackage.remainingBeforeLiveRuntime).toContain(
      "deployment dry-run against target environment",
    );
    expect(reviewPackage.remainingBeforeLiveRuntime).toContain(
      "authority freeze execution procedure",
    );
  });

  it("keeps route-aware success coverage visible", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.reportSummary.routeAwareSuccessVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    ]);
  });

  it("keeps CPI and supply-audit classification visible", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.reportSummary.cpiCommittedVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    ]);
    expect(reviewPackage.reportSummary.supplyAuditOkVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    ]);
    expect(reviewPackage.reportSummary.cpiSkippedVectorIds).toContain(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
    );
  });

  it("records expected rejection coverage", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.reportSummary.expectedRejectionVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InvalidRoutePolicyRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.MissingRouteRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.Stage1AuthorizationRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.EventKeyMismatchRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InstructionSerializationRejected,
    ]);
  });

  it("records next recommended stages for Theo review and runtime implementation", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    expect(reviewPackage.nextRecommendedStages).toEqual([
      "Theo review of runtime-prep package",
      "production runtime account byte layout",
      "production runtime instruction byte layout",
      "X1/SVM implementation skeleton",
      "real deployment dry-run fixtures",
    ]);
  });

  it("produces deterministic canonical JSON", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();
    const { canonicalJson: _canonicalJson, markdown: _markdown, ...core } =
      reviewPackage;

    expect(reviewPackage.canonicalJson).toBe(
      xxxlCanonicalRuntimePredeployReviewPackageJson(core),
    );
    expect(reviewPackage.canonicalJson).toContain(
      '["packageId","XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_V1"]',
    );
    expect(reviewPackage.canonicalJson).toContain('["tests",613]');
  });

  it("produces deterministic markdown summary", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();
    const { canonicalJson: _canonicalJson, markdown: _markdown, ...core } =
      reviewPackage;

    expect(reviewPackage.markdown).toBe(
      xxxlRuntimePredeployReviewPackageMarkdown(core),
    );
    expect(reviewPackage.markdown).toContain(
      "# XXXL Runtime Predeploy Review Package",
    );
    expect(reviewPackage.markdown).toContain("82 files / 613 tests passing");
    expect(reviewPackage.markdown).toContain("## Remaining before live runtime");
  });

  it("validates the default predeploy review package", () => {
    const validation = validateXXXLRuntimePredeployReviewPackage(
      createXXXLRuntimePredeployReviewPackage(),
    );

    expect(validation.ok).toBe(true);
    expect(validation.errors).toEqual([]);
  });

  it("detects invalid predeploy package mutations", () => {
    const reviewPackage = createXXXLRuntimePredeployReviewPackage();

    const badBaseline = validateXXXLRuntimePredeployReviewPackage({
      ...reviewPackage,
      baseline: {
        ...reviewPackage.baseline,
        tests: 0,
      },
    } as unknown as XXXLRuntimePredeployReviewPackage);

    expect(badBaseline.ok).toBe(false);
    expect(badBaseline.errors).toContain(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.BaselineMismatch,
    );

    const missingCoverage = validateXXXLRuntimePredeployReviewPackage({
      ...reviewPackage,
      reportSummary: {
        ...reviewPackage.reportSummary,
        routeAwareSuccessVectorIds: [
          XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
        ],
      },
    });

    expect(missingCoverage.ok).toBe(false);
    expect(missingCoverage.errors).toContain(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.MissingRouteAwareCoverage,
    );

    const badReport = createXXXLRuntimeFixtureReportExport();
    const notOkPackage = createXXXLRuntimePredeployReviewPackage({
      ...badReport,
      allOk: false,
    });

    const notOkValidation =
      validateXXXLRuntimePredeployReviewPackage(notOkPackage);

    expect(notOkValidation.ok).toBe(false);
    expect(notOkValidation.errors).toContain(
      XXXL_RUNTIME_PREDEPLOY_REVIEW_PACKAGE_ERROR.ReportValidationFailed,
    );
  });
});

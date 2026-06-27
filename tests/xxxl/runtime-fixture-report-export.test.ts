import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_DRY_RUN_FIXTURE_ID,
  XXXL_RUNTIME_EXECUTION_VECTOR_ID,
  XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR,
  XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ID,
  XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_VERSION,
  createXXXLRuntimeFixtureReportExport,
  validateXXXLRuntimeFixtureReportExport,
  xxxlCanonicalRuntimeFixtureReportExportJson,
  xxxlRuntimeDryRunFixtures,
  xxxlRuntimeFixtureReportExportMarkdown,
  type XXXLRuntimeDryRunFixture,
  type XXXLRuntimeFixtureReportExport,
  type XXXLRuntimeFixtureVectorSummary,
} from "../../src/index.js";

function fixtureAt(index: number): XXXLRuntimeDryRunFixture {
  const fixture = xxxlRuntimeDryRunFixtures()[index];

  expect(fixture).toBeDefined();

  return fixture as XXXLRuntimeDryRunFixture;
}

function vectorSummaryById(
  report: XXXLRuntimeFixtureReportExport,
  vectorId: string,
): XXXLRuntimeFixtureVectorSummary {
  const vector = report.fixtures
    .flatMap((fixture) => fixture.vectors)
    .find((item) => item.vectorId === vectorId);

  expect(vector).toBeDefined();

  return vector as XXXLRuntimeFixtureVectorSummary;
}

describe("XXXL runtime fixture report export", () => {
  it("exports deterministic default fixture report metadata", () => {
    const report = createXXXLRuntimeFixtureReportExport();

    expect(report.version).toBe(XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_VERSION);
    expect(report.reportId).toBe(XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ID);
    expect(report.allOk).toBe(true);
    expect(report.errors).toEqual([]);
  });

  it("summarizes default fixture and unique vector counts", () => {
    const report = createXXXLRuntimeFixtureReportExport();

    expect(report.fixtureCount).toBe(4);
    expect(report.vectorCount).toBe(8);
    expect(report.vectorIds).toEqual(Object.values(XXXL_RUNTIME_EXECUTION_VECTOR_ID));
  });

  it("preserves canonical fixture order", () => {
    const report = createXXXLRuntimeFixtureReportExport();

    expect(report.fixtureIds).toEqual([
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.AllExecutionVectors,
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.SuccessfulRoutes,
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.PreflightRejections,
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.TransitionRejections,
    ]);
  });

  it("classifies successful route vectors as CPI committed", () => {
    const report = createXXXLRuntimeFixtureReportExport();

    expect(report.cpiCommittedVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    ]);
    expect(report.routeAwareSuccessVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    ]);
  });

  it("classifies rejection vectors as CPI skipped", () => {
    const report = createXXXLRuntimeFixtureReportExport();

    expect(report.cpiSkippedVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InvalidRoutePolicyRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.MissingRouteRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.Stage1AuthorizationRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.EventKeyMismatchRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InstructionSerializationRejected,
    ]);
    expect(report.expectedRejectionVectorIds).toEqual(report.cpiSkippedVectorIds);
  });

  it("records supply audit OK only for successful mint vectors", () => {
    const report = createXXXLRuntimeFixtureReportExport();

    expect(report.supplyAuditOkVectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    ]);

    const replay = vectorSummaryById(
      report,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
    );
    expect(replay.supplyAuditOk).toBe(false);
    expect(replay.cpiSkipped).toBe(true);
  });

  it("keeps per-fixture report detail", () => {
    const report = createXXXLRuntimeFixtureReportExport();
    const allFixture = report.fixtures[0];

    expect(allFixture).toBeDefined();
    expect(allFixture?.fixtureId).toBe(
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.AllExecutionVectors,
    );
    expect(allFixture?.ok).toBe(true);
    expect(allFixture?.vectorCount).toBe(8);
    expect(allFixture?.vectors.every((vector) => vector.matchedExecutionVector)).toBe(
      true,
    );
  });

  it("validates the default report export", () => {
    const validation = validateXXXLRuntimeFixtureReportExport(
      createXXXLRuntimeFixtureReportExport(),
    );

    expect(validation.ok).toBe(true);
    expect(validation.errors).toEqual([]);
  });

  it("produces deterministic canonical JSON", () => {
    const report = createXXXLRuntimeFixtureReportExport();
    const { canonicalJson: _canonicalJson, markdown: _markdown, ...core } = report;

    expect(report.canonicalJson).toBe(
      xxxlCanonicalRuntimeFixtureReportExportJson(core),
    );
    expect(report.canonicalJson).toContain(
      '["reportId","XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_V1"]',
    );
    expect(report.canonicalJson).toContain('["vectorCount",8]');
  });

  it("produces deterministic markdown summary", () => {
    const report = createXXXLRuntimeFixtureReportExport();
    const { canonicalJson: _canonicalJson, markdown: _markdown, ...core } = report;

    expect(report.markdown).toBe(xxxlRuntimeFixtureReportExportMarkdown(core));
    expect(report.markdown).toContain("# XXXL Runtime Fixture Report Export");
    expect(report.markdown).toContain("- All OK: true");
    expect(report.markdown).toContain("## Fixtures");
  });

  it("can export a subset fixture report", () => {
    const report = createXXXLRuntimeFixtureReportExport([fixtureAt(1)]);

    expect(report.allOk).toBe(true);
    expect(report.fixtureCount).toBe(1);
    expect(report.vectorCount).toBe(2);
    expect(report.fixtureIds).toEqual([
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.SuccessfulRoutes,
    ]);
    expect(report.cpiCommittedVectorIds).toHaveLength(2);
    expect(report.cpiSkippedVectorIds).toEqual([]);
  });

  it("detects invalid report export mutations", () => {
    const report = createXXXLRuntimeFixtureReportExport();

    const notOkValidation = validateXXXLRuntimeFixtureReportExport({
      ...report,
      allOk: false,
    });
    expect(notOkValidation.ok).toBe(false);
    expect(notOkValidation.errors).toContain(
      XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR.ReportNotOk,
    );

    const wrongCanonicalValidation = validateXXXLRuntimeFixtureReportExport({
      ...report,
      canonicalJson: "wrong",
    });
    expect(wrongCanonicalValidation.ok).toBe(false);
    expect(wrongCanonicalValidation.errors).toContain(
      XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR.WrongCanonicalJson,
    );

    const wrongMarkdownValidation = validateXXXLRuntimeFixtureReportExport({
      ...report,
      markdown: "wrong",
    });
    expect(wrongMarkdownValidation.ok).toBe(false);
    expect(wrongMarkdownValidation.errors).toContain(
      XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR.WrongMarkdown,
    );
  });
});

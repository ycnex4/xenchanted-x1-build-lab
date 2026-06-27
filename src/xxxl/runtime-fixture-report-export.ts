import {
  XXXL_RUNTIME_DRY_RUN_FIXTURE_ID,
  executeXXXLRuntimeDryRunFixtureSet,
  xxxlRuntimeDryRunFixtures,
  type XXXLRuntimeDryRunFixture,
  type XXXLRuntimeDryRunFixtureErrorCode,
  type XXXLRuntimeDryRunFixtureId,
  type XXXLRuntimeDryRunReport,
  type XXXLRuntimeDryRunVectorReport,
} from "./runtime-dry-run-fixtures.js";
import {
  XXXL_RUNTIME_EXECUTION_VECTOR_ID,
  type XXXLRuntimeExecutionVectorId,
} from "./runtime-execution-vectors.js";

export const XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_VERSION = 1;
export const XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ID =
  "XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_V1";

export const XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR = {
  ReportNotOk: "REPORT_NOT_OK",
  WrongCanonicalJson: "WRONG_CANONICAL_JSON",
  WrongMarkdown: "WRONG_MARKDOWN",
} as const;

export type XXXLRuntimeFixtureReportExportErrorCode =
  (typeof XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR)[keyof typeof XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR];

export type XXXLRuntimeFixtureVectorSummary = {
  readonly vectorId: XXXLRuntimeExecutionVectorId;
  readonly ok: boolean;
  readonly actualOk: boolean;
  readonly matchedExecutionVector: boolean;
  readonly cpiSkipped: boolean;
  readonly supplyAuditOk: boolean;
};

export type XXXLRuntimeFixtureSummary = {
  readonly fixtureId: XXXLRuntimeDryRunFixtureId;
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimeDryRunFixtureErrorCode[];
  readonly vectorCount: number;
  readonly vectors: readonly XXXLRuntimeFixtureVectorSummary[];
};

export type XXXLRuntimeFixtureReportExportCore = {
  readonly version: typeof XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_VERSION;
  readonly reportId: typeof XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ID;
  readonly allOk: boolean;
  readonly fixtureCount: number;
  readonly vectorCount: number;
  readonly fixtureIds: readonly XXXLRuntimeDryRunFixtureId[];
  readonly vectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly cpiCommittedVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly cpiSkippedVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly supplyAuditOkVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly expectedRejectionVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly routeAwareSuccessVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly errors: readonly XXXLRuntimeDryRunFixtureErrorCode[];
  readonly fixtures: readonly XXXLRuntimeFixtureSummary[];
};

export type XXXLRuntimeFixtureReportExport =
  XXXLRuntimeFixtureReportExportCore & {
    readonly canonicalJson: string;
    readonly markdown: string;
  };

export type XXXLRuntimeFixtureReportExportValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimeFixtureReportExportErrorCode[];
};

export function createXXXLRuntimeFixtureReportExport(
  fixtures: readonly XXXLRuntimeDryRunFixture[] = xxxlRuntimeDryRunFixtures(),
): XXXLRuntimeFixtureReportExport {
  const dryRunReports = executeXXXLRuntimeDryRunFixtureSet(fixtures);
  const fixtureSummaries = dryRunReports.map(toFixtureSummary);
  const uniqueVectors = uniqueVectorReports(dryRunReports);
  const vectorIds = uniqueVectors.map((vector) => vector.vectorId);

  const core: XXXLRuntimeFixtureReportExportCore = {
    version: XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_VERSION,
    reportId: XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ID,
    allOk: dryRunReports.every((report) => report.ok),
    fixtureCount: dryRunReports.length,
    vectorCount: uniqueVectors.length,
    fixtureIds: dryRunReports.map((report) => report.fixtureId),
    vectorIds,
    cpiCommittedVectorIds: uniqueVectors
      .filter((vector) => !vector.cpiSkipped)
      .map((vector) => vector.vectorId),
    cpiSkippedVectorIds: uniqueVectors
      .filter((vector) => vector.cpiSkipped)
      .map((vector) => vector.vectorId),
    supplyAuditOkVectorIds: uniqueVectors
      .filter((vector) => vector.supplyAuditOk)
      .map((vector) => vector.vectorId),
    expectedRejectionVectorIds: uniqueVectors
      .filter((vector) => !vector.actualOk)
      .map((vector) => vector.vectorId),
    routeAwareSuccessVectorIds: uniqueVectors
      .filter(
        (vector) =>
          vector.vectorId ===
            XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint ||
          vector.vectorId ===
            XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
      )
      .map((vector) => vector.vectorId),
    errors: uniqueStrings(
      dryRunReports.flatMap((report) => report.errors),
    ) as XXXLRuntimeDryRunFixtureErrorCode[],
    fixtures: fixtureSummaries,
  };

  return {
    ...core,
    canonicalJson: xxxlCanonicalRuntimeFixtureReportExportJson(core),
    markdown: xxxlRuntimeFixtureReportExportMarkdown(core),
  };
}

export function validateXXXLRuntimeFixtureReportExport(
  report: XXXLRuntimeFixtureReportExport,
): XXXLRuntimeFixtureReportExportValidationResult {
  const errors: XXXLRuntimeFixtureReportExportErrorCode[] = [];
  const { canonicalJson: _canonicalJson, markdown: _markdown, ...core } = report;

  if (!report.allOk) {
    errors.push(XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR.ReportNotOk);
  }

  if (xxxlCanonicalRuntimeFixtureReportExportJson(core) !== report.canonicalJson) {
    errors.push(XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR.WrongCanonicalJson);
  }

  if (xxxlRuntimeFixtureReportExportMarkdown(core) !== report.markdown) {
    errors.push(XXXL_RUNTIME_FIXTURE_REPORT_EXPORT_ERROR.WrongMarkdown);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalRuntimeFixtureReportExportJson(
  report: XXXLRuntimeFixtureReportExportCore,
): string {
  return JSON.stringify([
    ["version", report.version],
    ["reportId", report.reportId],
    ["allOk", report.allOk],
    ["fixtureCount", report.fixtureCount],
    ["vectorCount", report.vectorCount],
    ["fixtureIds", report.fixtureIds],
    ["vectorIds", report.vectorIds],
    ["cpiCommittedVectorIds", report.cpiCommittedVectorIds],
    ["cpiSkippedVectorIds", report.cpiSkippedVectorIds],
    ["supplyAuditOkVectorIds", report.supplyAuditOkVectorIds],
    ["expectedRejectionVectorIds", report.expectedRejectionVectorIds],
    ["routeAwareSuccessVectorIds", report.routeAwareSuccessVectorIds],
    ["errors", report.errors],
    [
      "fixtures",
      report.fixtures.map((fixture) => [
        ["fixtureId", fixture.fixtureId],
        ["ok", fixture.ok],
        ["errors", fixture.errors],
        ["vectorCount", fixture.vectorCount],
        [
          "vectors",
          fixture.vectors.map((vector) => [
            ["vectorId", vector.vectorId],
            ["ok", vector.ok],
            ["actualOk", vector.actualOk],
            ["matchedExecutionVector", vector.matchedExecutionVector],
            ["cpiSkipped", vector.cpiSkipped],
            ["supplyAuditOk", vector.supplyAuditOk],
          ]),
        ],
      ]),
    ],
  ]);
}

export function xxxlRuntimeFixtureReportExportMarkdown(
  report: XXXLRuntimeFixtureReportExportCore,
): string {
  const lines: string[] = [];

  lines.push("# XXXL Runtime Fixture Report Export");
  lines.push("");
  lines.push(`- Report: ${report.reportId}`);
  lines.push(`- Version: ${report.version}`);
  lines.push(`- All OK: ${report.allOk}`);
  lines.push(`- Fixture count: ${report.fixtureCount}`);
  lines.push(`- Unique vector count: ${report.vectorCount}`);
  lines.push(`- CPI committed vectors: ${formatList(report.cpiCommittedVectorIds)}`);
  lines.push(`- CPI skipped vectors: ${formatList(report.cpiSkippedVectorIds)}`);
  lines.push(`- Supply audit OK vectors: ${formatList(report.supplyAuditOkVectorIds)}`);
  lines.push(
    `- Expected rejection vectors: ${formatList(report.expectedRejectionVectorIds)}`,
  );
  lines.push(
    `- Route-aware success vectors: ${formatList(report.routeAwareSuccessVectorIds)}`,
  );
  lines.push(`- Errors: ${formatList(report.errors)}`);
  lines.push("");
  lines.push("## Fixtures");

  for (const fixture of report.fixtures) {
    lines.push("");
    lines.push(`### ${fixture.fixtureId}`);
    lines.push("");
    lines.push(`- OK: ${fixture.ok}`);
    lines.push(`- Errors: ${formatList(fixture.errors)}`);
    lines.push(`- Vector count: ${fixture.vectorCount}`);
    lines.push("- Vectors:");

    for (const vector of fixture.vectors) {
      lines.push(
        `  - ${vector.vectorId} | ok=${vector.ok} | actualOk=${vector.actualOk} | matched=${vector.matchedExecutionVector} | cpiSkipped=${vector.cpiSkipped} | supplyAuditOk=${vector.supplyAuditOk}`,
      );
    }
  }

  return lines.join("\n");
}

function toFixtureSummary(
  report: XXXLRuntimeDryRunReport,
): XXXLRuntimeFixtureSummary {
  return {
    fixtureId: report.fixtureId,
    ok: report.ok,
    errors: report.errors,
    vectorCount: report.reports.length,
    vectors: report.reports.map(toVectorSummary),
  };
}

function toVectorSummary(
  report: XXXLRuntimeDryRunVectorReport,
): XXXLRuntimeFixtureVectorSummary {
  return {
    vectorId: report.vectorId,
    ok: report.ok,
    actualOk: report.actualOk,
    matchedExecutionVector: report.matchedExecutionVector,
    cpiSkipped: report.cpiSkipped,
    supplyAuditOk: report.supplyAuditOk,
  };
}

function uniqueVectorReports(
  reports: readonly XXXLRuntimeDryRunReport[],
): readonly XXXLRuntimeFixtureVectorSummary[] {
  const seen = new Set<XXXLRuntimeExecutionVectorId>();
  const vectors: XXXLRuntimeFixtureVectorSummary[] = [];

  for (const report of reports) {
    for (const vector of report.reports) {
      if (seen.has(vector.vectorId)) {
        continue;
      }

      seen.add(vector.vectorId);
      vectors.push(toVectorSummary(vector));
    }
  }

  return vectors;
}

function uniqueStrings(items: readonly string[]): readonly string[] {
  return [...new Set(items)];
}

function formatList(items: readonly string[]): string {
  return items.length === 0 ? "none" : items.join(", ");
}

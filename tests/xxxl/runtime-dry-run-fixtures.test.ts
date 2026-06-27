import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR,
  XXXL_RUNTIME_DRY_RUN_FIXTURE_ID,
  XXXL_RUNTIME_EXECUTION_VECTOR_ID,
  executeXXXLRuntimeDryRunFixture,
  executeXXXLRuntimeDryRunFixtureSet,
  validateXXXLRuntimeDryRunFixtures,
  xxxlRuntimeDryRunFixtures,
  xxxlRuntimeExecutionVectors,
  type XXXLRuntimeDryRunFixture,
  type XXXLRuntimeDryRunReport,
  type XXXLRuntimeDryRunVectorReport,
} from "../../src/index.js";

function fixtureAt(index: number): XXXLRuntimeDryRunFixture {
  const fixture = xxxlRuntimeDryRunFixtures()[index];

  expect(fixture).toBeDefined();

  return fixture as XXXLRuntimeDryRunFixture;
}

function reportAt(
  report: XXXLRuntimeDryRunReport,
  index: number,
): XXXLRuntimeDryRunVectorReport {
  const vectorReport = report.reports[index];

  expect(vectorReport).toBeDefined();

  return vectorReport as XXXLRuntimeDryRunVectorReport;
}

describe("XXXL runtime dry-run fixtures", () => {
  it("exports deterministic dry-run fixtures in canonical order", () => {
    expect(xxxlRuntimeDryRunFixtures().map((fixture) => fixture.fixtureId)).toEqual([
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.AllExecutionVectors,
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.SuccessfulRoutes,
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.PreflightRejections,
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.TransitionRejections,
    ]);
  });

  it("validates the default dry-run fixture set", () => {
    const errors = validateXXXLRuntimeDryRunFixtures(
      xxxlRuntimeDryRunFixtures(),
    );

    expect(errors).toEqual([]);
  });

  it("full fixture covers every runtime execution vector", () => {
    const fixture = fixtureAt(0);

    expect(fixture.vectorIds).toEqual(
      Object.values(XXXL_RUNTIME_EXECUTION_VECTOR_ID),
    );
  });

  it("successful-routes fixture covers Ethereum and Avalanche route-aware success cases", () => {
    const fixture = fixtureAt(1);

    expect(fixture.vectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    ]);
  });

  it("preflight fixture covers route and instruction serialization rejections", () => {
    const fixture = fixtureAt(2);

    expect(fixture.vectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InvalidRoutePolicyRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.MissingRouteRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InstructionSerializationRejected,
    ]);
  });

  it("transition rejection fixture covers Stage 1, replay, and event-key mismatch rejections", () => {
    const fixture = fixtureAt(3);

    expect(fixture.vectorIds).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.Stage1AuthorizationRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.EventKeyMismatchRejected,
    ]);
  });

  it("executes full dry-run fixture and matches all execution vectors", () => {
    const report = executeXXXLRuntimeDryRunFixture(fixtureAt(0));

    expect(report.ok).toBe(true);
    expect(report.errors).toEqual([]);
    expect(report.reports).toHaveLength(8);
    expect(report.reports.every((item) => item.matchedExecutionVector)).toBe(
      true,
    );
  });

  it("executes success dry-run fixture with CPI not skipped", () => {
    const report = executeXXXLRuntimeDryRunFixture(fixtureAt(1));

    expect(report.ok).toBe(true);
    expect(report.reports).toHaveLength(2);
    expect(report.reports.every((item) => item.actualOk)).toBe(true);
    expect(report.reports.every((item) => !item.cpiSkipped)).toBe(true);
    expect(report.reports.every((item) => item.supplyAuditOk)).toBe(true);
  });

  it("executes rejection dry-run fixtures as expected outcomes, not failures", () => {
    const preflightReport = executeXXXLRuntimeDryRunFixture(fixtureAt(2));
    const transitionReport = executeXXXLRuntimeDryRunFixture(fixtureAt(3));

    expect(preflightReport.ok).toBe(true);
    expect(transitionReport.ok).toBe(true);
    expect(preflightReport.reports.every((item) => !item.actualOk)).toBe(true);
    expect(transitionReport.reports.every((item) => !item.actualOk)).toBe(true);
    expect(preflightReport.reports.every((item) => item.cpiSkipped)).toBe(true);
    expect(transitionReport.reports.every((item) => item.cpiSkipped)).toBe(true);
  });

  it("reports individual vector execution details", () => {
    const report = executeXXXLRuntimeDryRunFixture(fixtureAt(0));
    const first = reportAt(report, 0);

    expect(first.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
    );
    expect(first.scenarioFound).toBe(true);
    expect(first.executionVectorFound).toBe(true);
    expect(first.matchedExecutionVector).toBe(true);
    expect(first.expectedOk).toBe(true);
    expect(first.actualOk).toBe(true);
  });

  it("executes the default fixture set", () => {
    const reports = executeXXXLRuntimeDryRunFixtureSet();

    expect(reports).toHaveLength(4);
    expect(reports.every((report) => report.ok)).toBe(true);
  });

  it("detects duplicate fixture ids and duplicate fixture vectors", () => {
    const fixture = fixtureAt(1);

    const errors = validateXXXLRuntimeDryRunFixtures([
      fixture,
      fixture,
      {
        ...fixture,
        fixtureId: XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.PreflightRejections,
        vectorIds: [
          XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
          XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
        ],
      },
    ]);

    expect(errors).toContain(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.DuplicateFixture);
    expect(errors).toContain(
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.DuplicateFixtureVector,
    );
  });

  it("reports missing execution vector when vector set is incomplete", () => {
    const fixture = fixtureAt(0);
    const vectors = xxxlRuntimeExecutionVectors().filter(
      (vector) =>
        vector.vectorId !==
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
    );

    const report = executeXXXLRuntimeDryRunFixture(fixture, {
      executionVectors: vectors,
    });

    expect(report.ok).toBe(false);
    expect(report.errors).toContain(
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.ExecutionVectorValidationFailed,
    );
    expect(report.errors).toContain(
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.MissingExecutionVector,
    );
    expect(reportAt(report, 0).executionVectorFound).toBe(false);
  });
});

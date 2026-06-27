import { describe, expect, it } from "vitest";

import {
  XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR,
  XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_STATUS,
  XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_VERSION,
  validateXXXLAtomicExecutionPlanFixture,
  xxxlAtomicExecutionPlanFixture,
  xxxlAtomicExecutionPlanFixtureMarkdown,
  xxxlCanonicalAtomicExecutionPlanFixtureJson,
} from "../../src/index.js";

describe("XXXL atomic execution plan fixture", () => {
  it("exports fixture metadata", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();

    expect(fixture.version).toBe(XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_VERSION);
    expect(fixture.status).toBe(
      XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_STATUS.PlanOnlyNotLiveRoute,
    );
    expect(fixture.rustModule).toBe("programs/xxxl-svm/src/execution_plan.rs");
  });

  it("fixes atomic execution step order", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();

    expect(fixture.stepOrder).toEqual([
      "VALIDATE_AND_PREPARE_CPI",
      "MARK_PROCESSED_EVENT_CONSUMED",
      "CREDIT_RECIPIENT_BALANCE",
      "KEEP_LIVE_ROUTE_DISABLED",
    ]);
  });

  it("requires atomic prechecks", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();

    expect(fixture.atomicPrechecks.rejectReplayBeforeCredit).toBe(true);
    expect(fixture.atomicPrechecks.rejectBalanceOverflowBeforeProcessedMark).toBe(true);
    expect(fixture.atomicPrechecks.rejectWrongRecipientBalanceOwnerBeforeMutation).toBe(true);
    expect(fixture.atomicPrechecks.rejectPreparedCpiAmountMismatch).toBe(true);
  });

  it("requires prepared CPI and state mutation boundaries", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();

    expect(fixture.preparedBoundaries.cpiBoundaryPrepared).toBe(true);
    expect(fixture.preparedBoundaries.processedEventMutationPrepared).toBe(true);
    expect(fixture.preparedBoundaries.recipientBalanceMutationPrepared).toBe(true);
  });

  it("keeps live route execution disabled", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();

    expect(fixture.liveExecution.processInstructionCallsMintToCpi).toBe(false);
    expect(fixture.liveExecution.processInstructionMarksProcessedEvent).toBe(false);
    expect(fixture.liveExecution.processInstructionCreditsRecipientBalance).toBe(false);
    expect(fixture.liveExecution.routeActivationEnabled).toBe(false);
  });

  it("validates default fixture", () => {
    const result = validateXXXLAtomicExecutionPlanFixture();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects wrong step order", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();
    const result = validateXXXLAtomicExecutionPlanFixture({
      ...fixture,
      stepOrder: [
        "VALIDATE_AND_PREPARE_CPI",
        "CREDIT_RECIPIENT_BALANCE",
        "MARK_PROCESSED_EVENT_CONSUMED",
        "KEEP_LIVE_ROUTE_DISABLED",
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.WrongStepOrder,
    );
  });

  it("rejects missing atomic precheck", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();
    const result = validateXXXLAtomicExecutionPlanFixture({
      ...fixture,
      atomicPrechecks: {
        ...fixture.atomicPrechecks,
        rejectBalanceOverflowBeforeProcessedMark: false as true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.MissingAtomicPrecheck,
    );
  });

  it("rejects live route activation", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();
    const result = validateXXXLAtomicExecutionPlanFixture({
      ...fixture,
      liveExecution: {
        ...fixture.liveExecution,
        processInstructionCallsMintToCpi: true as false,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.LiveRouteActivated,
    );
  });

  it("exports deterministic canonical fixture JSON", () => {
    const fixture = xxxlAtomicExecutionPlanFixture();
    const json = xxxlCanonicalAtomicExecutionPlanFixtureJson(fixture);

    expect(json).toBe(xxxlCanonicalAtomicExecutionPlanFixtureJson(fixture));
    expect(json).toContain(
      '["status","ATOMIC_EXECUTION_PLAN_FIXTURE_ONLY_NOT_LIVE_ROUTE"]',
    );
    expect(json).toContain('"VALIDATE_AND_PREPARE_CPI"');
    expect(json).toContain('"KEEP_LIVE_ROUTE_DISABLED"');
  });

  it("exports markdown report", () => {
    const markdown = xxxlAtomicExecutionPlanFixtureMarkdown();

    expect(markdown).toContain("# XXXL Atomic Execution Plan Fixture");
    expect(markdown).toContain("VALIDATE_AND_PREPARE_CPI");
    expect(markdown).toContain("NO_LIVE_ROUTE_ACTIVATION");
  });
});

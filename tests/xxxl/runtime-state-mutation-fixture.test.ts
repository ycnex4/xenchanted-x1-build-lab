import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR,
  XXXL_RUNTIME_STATE_MUTATION_FIXTURE_STATUS,
  XXXL_RUNTIME_STATE_MUTATION_FIXTURE_VERSION,
  validateXXXLRuntimeStateMutationFixture,
  xxxlCanonicalRuntimeStateMutationFixtureJson,
  xxxlRuntimeStateMutationFixture,
  xxxlRuntimeStateMutationFixtureMarkdown,
} from "../../src/index.js";

describe("XXXL runtime state mutation fixture", () => {
  it("exports fixture metadata", () => {
    const fixture = xxxlRuntimeStateMutationFixture();

    expect(fixture.version).toBe(XXXL_RUNTIME_STATE_MUTATION_FIXTURE_VERSION);
    expect(fixture.status).toBe(
      XXXL_RUNTIME_STATE_MUTATION_FIXTURE_STATUS.MutationFixtureOnly,
    );
    expect(fixture.rustModule).toBe("programs/xxxl-svm/src/state.rs");
  });

  it("describes processed event mutation guarantees", () => {
    const fixture = xxxlRuntimeStateMutationFixture();

    expect(fixture.processedEventMutation).toEqual({
      function: "mark_processed_event_consumed",
      writesConsumedFlag: true,
      writesConsumedAmount: true,
      writesConsumedSlot: true,
      rejectsReplay: true,
      requiresCanonicalEventKeyMatch: true,
      requiresRouteMatch: true,
      requiresRecipientMatch: true,
    });
  });

  it("describes recipient balance mutation guarantees", () => {
    const fixture = xxxlRuntimeStateMutationFixture();

    expect(fixture.recipientBalanceMutation).toEqual({
      function: "credit_recipient_balance",
      usesCheckedAdd: true,
      writesBalance: true,
      writesLastCanonicalEventKey: true,
      requiresOwnerMatch: true,
      requiresMintMatch: true,
    });
  });

  it("keeps live route execution disabled", () => {
    const fixture = xxxlRuntimeStateMutationFixture();

    expect(fixture.liveExecution.processInstructionMutatesProcessedEvent).toBe(false);
    expect(fixture.liveExecution.processInstructionMutatesRecipientBalance).toBe(false);
    expect(fixture.liveExecution.routeActivationEnabled).toBe(false);
    expect(fixture.liveExecution.mintToInvocationEnabled).toBe(false);
  });

  it("keeps non-goals explicit", () => {
    const fixture = xxxlRuntimeStateMutationFixture();

    expect(fixture.nonGoals).toContain("NO_LIVE_ROUTE_ACTIVATION");
    expect(fixture.nonGoals).toContain("NO_MINT_TO_INVOCATION_FROM_HANDLER");
    expect(fixture.nonGoals).toContain("NO_PROCESS_INSTRUCTION_STATE_MUTATION");
    expect(fixture.nonGoals).toContain("NO_DEPLOYMENT");
    expect(fixture.nonGoals).toContain("NO_AUTHORITY_FREEZE_EXECUTION");
  });

  it("validates default fixture", () => {
    const result = validateXXXLRuntimeStateMutationFixture();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects missing processed event mutation", () => {
    const fixture = xxxlRuntimeStateMutationFixture();
    const result = validateXXXLRuntimeStateMutationFixture({
      ...fixture,
      processedEventMutation: {
        ...fixture.processedEventMutation,
        writesConsumedSlot: false as true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingProcessedEventMutation,
    );
  });

  it("rejects missing replay protection", () => {
    const fixture = xxxlRuntimeStateMutationFixture();
    const result = validateXXXLRuntimeStateMutationFixture({
      ...fixture,
      processedEventMutation: {
        ...fixture.processedEventMutation,
        rejectsReplay: false as true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingReplayProtection,
    );
  });

  it("rejects missing overflow protection", () => {
    const fixture = xxxlRuntimeStateMutationFixture();
    const result = validateXXXLRuntimeStateMutationFixture({
      ...fixture,
      recipientBalanceMutation: {
        ...fixture.recipientBalanceMutation,
        usesCheckedAdd: false as true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingOverflowProtection,
    );
  });

  it("rejects live route activation", () => {
    const fixture = xxxlRuntimeStateMutationFixture();
    const result = validateXXXLRuntimeStateMutationFixture({
      ...fixture,
      liveExecution: {
        ...fixture.liveExecution,
        processInstructionMutatesProcessedEvent: true as false,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.LiveRouteActivated,
    );
  });

  it("exports deterministic canonical fixture JSON", () => {
    const fixture = xxxlRuntimeStateMutationFixture();
    const json = xxxlCanonicalRuntimeStateMutationFixtureJson(fixture);

    expect(json).toBe(xxxlCanonicalRuntimeStateMutationFixtureJson(fixture));
    expect(json).toContain(
      '["status","RUNTIME_STATE_MUTATION_FIXTURE_ONLY_NOT_LIVE_ROUTE"]',
    );
    expect(json).toContain('"mark_processed_event_consumed"');
    expect(json).toContain('"credit_recipient_balance"');
  });

  it("exports markdown report", () => {
    const markdown = xxxlRuntimeStateMutationFixtureMarkdown();

    expect(markdown).toContain("# XXXL Runtime State Mutation Fixture");
    expect(markdown).toContain("mark_processed_event_consumed");
    expect(markdown).toContain("credit_recipient_balance");
    expect(markdown).toContain("NO_LIVE_ROUTE_ACTIVATION");
  });
});

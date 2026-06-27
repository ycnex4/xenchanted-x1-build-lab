import { describe, expect, it } from "vitest";

import {
  XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR,
  XXXL_HANDLER_INTEGRATION_FIXTURE_STATUS,
  XXXL_HANDLER_INTEGRATION_FIXTURE_VERSION,
  validateXXXLHandlerIntegrationFixture,
  xxxlCanonicalHandlerIntegrationFixtureJson,
  xxxlHandlerIntegrationFixture,
  xxxlHandlerIntegrationFixtureMarkdown,
} from "../../src/index.js";

describe("XXXL handler integration fixture", () => {
  it("exports fixture metadata", () => {
    const fixture = xxxlHandlerIntegrationFixture();

    expect(fixture.version).toBe(XXXL_HANDLER_INTEGRATION_FIXTURE_VERSION);
    expect(fixture.status).toBe(
      XXXL_HANDLER_INTEGRATION_FIXTURE_STATUS.PreparedNotLive,
    );
    expect(fixture.rustModule).toBe("programs/xxxl-svm/src/processor.rs");
  });

  it("connects decode, account views, validation, and CPI preparation", () => {
    const fixture = xxxlHandlerIntegrationFixture();

    expect(fixture.flow).toEqual([
      "DECODE_CONSUME_GATEWAY_MINT_INSTRUCTION",
      "LOAD_CANONICAL_ACCOUNT_INDEXES",
      "PARSE_RUNTIME_ACCOUNT_VIEWS",
      "RUN_OWNER_AND_RENT_CHECKS",
      "RUN_MINT_AND_RECIPIENT_TOKEN_VALIDATION",
      "VERIFY_GATEWAY_MINT_AUTHORITY_PDA_AND_BUMP",
      "PREPARE_MINT_TO_CPI_BOUNDARY",
    ]);
  });

  it("describes prepared CPI boundary without invoking it from process_instruction", () => {
    const fixture = xxxlHandlerIntegrationFixture();

    expect(fixture.preparedBoundary).toEqual({
      tokenProgram: "SPL_TOKEN_PROGRAM",
      mint: "XXXL_SPL_MINT",
      recipientTokenAccount: "RECIPIENT_TOKEN_ACCOUNT",
      mintAuthorityPda: "GATEWAY_MINT_AUTHORITY_PDA",
      amountType: "u64",
    });

    expect(fixture.liveExecution.processInstructionCallsMintToCpi).toBe(false);
  });

  it("keeps live route execution disabled", () => {
    const fixture = xxxlHandlerIntegrationFixture();

    expect(fixture.liveExecution.routeActivationEnabled).toBe(false);
    expect(fixture.liveExecution.processedEventMutationEnabled).toBe(false);
    expect(fixture.liveExecution.recipientBalanceMutationEnabled).toBe(false);
  });

  it("keeps non-goals explicit", () => {
    const fixture = xxxlHandlerIntegrationFixture();

    expect(fixture.nonGoals).toContain("NO_LIVE_MINT_TO_INVOCATION_FROM_HANDLER");
    expect(fixture.nonGoals).toContain("NO_ROUTE_ACTIVATION");
    expect(fixture.nonGoals).toContain("NO_PROCESSED_EVENT_MUTATION");
    expect(fixture.nonGoals).toContain("NO_RECIPIENT_BALANCE_MUTATION");
    expect(fixture.nonGoals).toContain("NO_DEPLOYMENT");
    expect(fixture.nonGoals).toContain("NO_AUTHORITY_FREEZE_EXECUTION");
  });

  it("validates default fixture", () => {
    const result = validateXXXLHandlerIntegrationFixture();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects missing decode boundary", () => {
    const fixture = xxxlHandlerIntegrationFixture();
    const result = validateXXXLHandlerIntegrationFixture({
      ...fixture,
      flow: fixture.flow.filter(
        (item) => item !== "DECODE_CONSUME_GATEWAY_MINT_INSTRUCTION",
      ) as typeof fixture.flow,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.MissingDecodeBoundary,
    );
  });

  it("rejects missing validation boundary", () => {
    const fixture = xxxlHandlerIntegrationFixture();
    const result = validateXXXLHandlerIntegrationFixture({
      ...fixture,
      flow: fixture.flow.filter(
        (item) => item !== "RUN_OWNER_AND_RENT_CHECKS",
      ) as typeof fixture.flow,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.MissingValidationBoundary,
    );
  });

  it("rejects live route activation", () => {
    const fixture = xxxlHandlerIntegrationFixture();
    const result = validateXXXLHandlerIntegrationFixture({
      ...fixture,
      liveExecution: {
        ...fixture.liveExecution,
        routeActivationEnabled: true as false,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.LiveRouteActivated,
    );
  });

  it("exports deterministic canonical fixture JSON", () => {
    const fixture = xxxlHandlerIntegrationFixture();
    const json = xxxlCanonicalHandlerIntegrationFixtureJson(fixture);

    expect(json).toBe(xxxlCanonicalHandlerIntegrationFixtureJson(fixture));
    expect(json).toContain(
      '["status","HANDLER_INTEGRATION_FIXTURE_PREPARED_NOT_LIVE_ROUTE"]',
    );
    expect(json).toContain('"PREPARE_MINT_TO_CPI_BOUNDARY"');
  });

  it("exports markdown report", () => {
    const markdown = xxxlHandlerIntegrationFixtureMarkdown();

    expect(markdown).toContain("# XXXL Handler Integration Fixture");
    expect(markdown).toContain("PREPARE_MINT_TO_CPI_BOUNDARY");
    expect(markdown).toContain("NO_ROUTE_ACTIVATION");
  });
});

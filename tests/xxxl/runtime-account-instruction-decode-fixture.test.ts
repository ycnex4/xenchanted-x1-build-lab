import { describe, expect, it } from "vitest";

import {
  XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE,
  XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE,
  XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR,
  XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_STATUS,
  XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_VERSION,
  validateXXXLRuntimeAccountInstructionDecodeFixture,
  xxxlCanonicalRuntimeAccountInstructionDecodeFixtureJson,
  xxxlRuntimeAccountInstructionDecodeFixture,
  xxxlRuntimeAccountInstructionDecodeFixtureMarkdown,
} from "../../src/index.js";

describe("XXXL runtime account/instruction decode fixture", () => {
  it("exports fixture metadata", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.version).toBe(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_VERSION,
    );
    expect(fixture.status).toBe(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_STATUS.RustDecodeFixtureOnly,
    );
  });

  it("points to Rust instruction and state decode modules", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.rustModules.instruction).toBe(
      "programs/xxxl-svm/src/instruction.rs",
    );
    expect(fixture.rustModules.state).toBe("programs/xxxl-svm/src/state.rs");
  });

  it("keeps consume_gateway_mint instruction length and discriminator fixed", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.instruction.handler).toBe("consume_gateway_mint");
    expect(fixture.instruction.byteLength).toBe(
      XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE.ConsumeGatewayMint,
    );
    expect(fixture.instruction.byteLength).toBe(208);
    expect(fixture.instruction.discriminatorHex).toBe("f2f4a868bb89fe52");
    expect(fixture.instruction.version).toBe(1);
    expect(fixture.instruction.accountMetaCount).toBe(9);
  });

  it("documents parsed consume_gateway_mint fields", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.instruction.parsedFields).toContain("routeId");
    expect(fixture.instruction.parsedFields).toContain("guardianSetId");
    expect(fixture.instruction.parsedFields).toContain("mintId");
    expect(fixture.instruction.parsedFields).toContain("canonicalEventKey");
    expect(fixture.instruction.parsedFields).toContain("recipient");
    expect(fixture.instruction.parsedFields).toContain("amount");
    expect(fixture.instruction.parsedFields).toContain("sourceChainWeightBps");
  });

  it("keeps account decode lengths aligned with production layout", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(
      fixture.accounts.find((account) => account.kind === "MINT_STATE_ACCOUNT")
        ?.byteLength,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.MintState);

    expect(
      fixture.accounts.find(
        (account) => account.kind === "GATEWAY_CONFIG_ACCOUNT",
      )?.byteLength,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GatewayConfig);

    expect(
      fixture.accounts.find((account) => account.kind === "GUARDIAN_SET_ACCOUNT")
        ?.byteLength,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GuardianSet);

    expect(
      fixture.accounts.find(
        (account) => account.kind === "PROCESSED_EVENT_ACCOUNT",
      )?.byteLength,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.ProcessedEvent);

    expect(
      fixture.accounts.find(
        (account) => account.kind === "RECIPIENT_BALANCE_ACCOUNT",
      )?.byteLength,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.RecipientBalance);
  });

  it("keeps account discriminators fixed", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.accounts.map((account) => account.discriminatorHex)).toEqual([
      "18f0f49966906660",
      "a6120c7ed76902ae",
      "a6f6ef1aaec613ae",
      "8f545b8140a2d5b5",
      "b56386245014f5f4",
    ]);
  });

  it("lists required negative Rust tests", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.requiredNegativeRustTests).toContain(
      "WRONG_INSTRUCTION_LENGTH_REJECTED",
    );
    expect(fixture.requiredNegativeRustTests).toContain(
      "WRONG_INSTRUCTION_DISCRIMINATOR_REJECTED",
    );
    expect(fixture.requiredNegativeRustTests).toContain(
      "WRONG_INSTRUCTION_VERSION_REJECTED",
    );
    expect(fixture.requiredNegativeRustTests).toContain(
      "WRONG_ACCOUNT_DISCRIMINATOR_REJECTED",
    );
    expect(fixture.requiredNegativeRustTests).toContain(
      "WRONG_ACCOUNT_VERSION_REJECTED",
    );
    expect(fixture.requiredNegativeRustTests).toContain(
      "TRUNCATED_ACCOUNT_DATA_REJECTED",
    );
  });

  it("keeps non-goals explicit", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.nonGoals).toContain("NO_SPL_TOKEN_CPI_YET");
    expect(fixture.nonGoals).toContain("NO_DEPLOYMENT");
    expect(fixture.nonGoals).toContain("NO_ROUTE_ACTIVATION");
    expect(fixture.nonGoals).toContain("NO_AUTHORITY_FREEZE_EXECUTION");
  });

  it("validates default fixture", () => {
    const result = validateXXXLRuntimeAccountInstructionDecodeFixture();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects wrong instruction length", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();
    const result = validateXXXLRuntimeAccountInstructionDecodeFixture({
      ...fixture,
      instruction: {
        ...fixture.instruction,
        byteLength: 207,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.WrongInstructionLength,
    );
  });

  it("rejects wrong account length", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();
    const result = validateXXXLRuntimeAccountInstructionDecodeFixture({
      ...fixture,
      accounts: [
        {
          ...fixture.accounts[0]!,
          byteLength: 175,
        },
        ...fixture.accounts.slice(1),
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.WrongAccountLength,
    );
  });

  it("rejects missing negative test coverage", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();
    const result = validateXXXLRuntimeAccountInstructionDecodeFixture({
      ...fixture,
      requiredNegativeRustTests: ["WRONG_INSTRUCTION_LENGTH_REJECTED"],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.MissingNegativeTest,
    );
  });

  it("exports deterministic canonical fixture JSON", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();
    const json = xxxlCanonicalRuntimeAccountInstructionDecodeFixtureJson(fixture);

    expect(json).toBe(
      xxxlCanonicalRuntimeAccountInstructionDecodeFixtureJson(fixture),
    );
    expect(json).toContain('["status","RUST_DECODE_FIXTURE_ONLY_NOT_DEPLOYABLE"]');
    expect(json).toContain('"discriminatorHex":"f2f4a868bb89fe52"');
  });

  it("exports markdown report", () => {
    const markdown = xxxlRuntimeAccountInstructionDecodeFixtureMarkdown();

    expect(markdown).toContain(
      "# XXXL Runtime Account/Instruction Decode Fixture",
    );
    expect(markdown).toContain("NO_SPL_TOKEN_CPI_YET");
    expect(markdown).toContain("consume_gateway_mint");
  });

  it("keeps decode stage before CPI stage", () => {
    const fixture = xxxlRuntimeAccountInstructionDecodeFixture();

    expect(fixture.guarantees).toContain(
      "RUST_CONSUME_GATEWAY_MINT_BYTES_ARE_PARSED_BEFORE_CPI",
    );
    expect(fixture.nonGoals).toContain("NO_SPL_TOKEN_CPI_YET");
  });
});

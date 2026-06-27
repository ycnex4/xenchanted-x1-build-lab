import { describe, expect, it } from "vitest";

import {
  XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR,
  XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_STATUS,
  XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_VERSION,
  validateXXXLSplTokenMintToCpiFixture,
  xxxlCanonicalSplTokenMintToCpiFixtureJson,
  xxxlSplTokenMintToCpiFixture,
  xxxlSplTokenMintToCpiFixtureMarkdown,
} from "../../src/index.js";

describe("XXXL SPL Token mint_to CPI fixture", () => {
  it("exports fixture metadata", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();

    expect(fixture.version).toBe(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_VERSION);
    expect(fixture.status).toBe(
      XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_STATUS.FixtureOnlyNotLiveHandler,
    );
  });

  it("points to Rust CPI, validation, PDA, and processor modules", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();

    expect(fixture.rustModules.cpi).toBe("programs/xxxl-svm/src/cpi.rs");
    expect(fixture.rustModules.validation).toBe(
      "programs/xxxl-svm/src/validation.rs",
    );
    expect(fixture.rustModules.pda).toBe("programs/xxxl-svm/src/pda.rs");
    expect(fixture.rustModules.processor).toBe(
      "programs/xxxl-svm/src/processor.rs",
    );
  });

  it("fixes SPL Token mint_to and invoke_signed boundary", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();

    expect(fixture.splTokenProgramId).toBe(
      "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    );
    expect(fixture.cpiInstruction.function).toBe(
      "spl_token::instruction::mint_to",
    );
    expect(fixture.cpiInstruction.invoke).toBe(
      "solana_program::program::invoke_signed",
    );
    expect(fixture.cpiInstruction.amountType).toBe("u64");
    expect(fixture.cpiInstruction.zeroAmountRejected).toBe(true);
  });

  it("keeps gateway mint authority PDA signer exact", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();

    expect(fixture.signer.pdaSeeds).toEqual([
      "xxxl",
      "gateway-mint-authority",
      "v1",
    ]);
    expect(fixture.signer.bumpRequired).toBe(true);
    expect(fixture.signer.findProgramAddressRequired).toBe(true);
  });

  it("requires mint and recipient token account validation", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();

    expect(fixture.accountChecks.mintOwnedBySplToken).toBe(true);
    expect(fixture.accountChecks.mintInitialized).toBe(true);
    expect(fixture.accountChecks.mintAuthorityMustMatchGatewayPda).toBe(true);
    expect(fixture.accountChecks.recipientTokenAccountOwnedBySplToken).toBe(true);
    expect(fixture.accountChecks.recipientTokenAccountInitialized).toBe(true);
    expect(fixture.accountChecks.recipientTokenAccountMintMustMatch).toBe(true);
    expect(fixture.accountChecks.recipientTokenAccountOwnerMustMatch).toBe(true);
    expect(fixture.accountChecks.rentExemptionHelperAvailable).toBe(true);
  });

  it("keeps live route execution non-goals explicit", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();

    expect(fixture.nonGoals).toContain("NO_LIVE_ROUTE_ACTIVATION");
    expect(fixture.nonGoals).toContain("NO_DEPLOYMENT");
    expect(fixture.nonGoals).toContain("NO_AUTHORITY_FREEZE_EXECUTION");
    expect(fixture.nonGoals).toContain("NO_PROCESSED_EVENT_MUTATION_YET");
    expect(fixture.nonGoals).toContain("NO_RECIPIENT_BALANCE_MUTATION_YET");
  });

  it("validates default fixture", () => {
    const result = validateXXXLSplTokenMintToCpiFixture();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects missing mint_to boundary", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();
    const result = validateXXXLSplTokenMintToCpiFixture({
      ...fixture,
      cpiInstruction: {
        ...fixture.cpiInstruction,
        function: "wrong" as "spl_token::instruction::mint_to",
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingMintTo,
    );
  });

  it("rejects missing invoke_signed boundary", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();
    const result = validateXXXLSplTokenMintToCpiFixture({
      ...fixture,
      cpiInstruction: {
        ...fixture.cpiInstruction,
        invoke: "wrong" as "solana_program::program::invoke_signed",
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingInvokeSigned,
    );
  });

  it("rejects missing PDA signer dependency", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();
    const result = validateXXXLSplTokenMintToCpiFixture({
      ...fixture,
      signer: {
        ...fixture.signer,
        bumpRequired: false as true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingPdaSigner,
    );
  });

  it("rejects missing ATA validation dependency", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();
    const result = validateXXXLSplTokenMintToCpiFixture({
      ...fixture,
      accountChecks: {
        ...fixture.accountChecks,
        recipientTokenAccountMintMustMatch: false as true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingAtaValidation,
    );
  });

  it("exports deterministic canonical fixture JSON", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();
    const json = xxxlCanonicalSplTokenMintToCpiFixtureJson(fixture);

    expect(json).toBe(xxxlCanonicalSplTokenMintToCpiFixtureJson(fixture));
    expect(json).toContain(
      '["status","SPL_TOKEN_MINT_TO_CPI_FIXTURE_ONLY_NOT_LIVE_HANDLER"]',
    );
    expect(json).toContain('"function":"spl_token::instruction::mint_to"');
  });

  it("exports markdown report", () => {
    const markdown = xxxlSplTokenMintToCpiFixtureMarkdown();

    expect(markdown).toContain("# XXXL SPL Token mint_to CPI Fixture");
    expect(markdown).toContain("spl_token::instruction::mint_to");
    expect(markdown).toContain("NO_LIVE_ROUTE_ACTIVATION");
  });

  it("keeps CPI fixture separate from live handler activation", () => {
    const fixture = xxxlSplTokenMintToCpiFixture();

    expect(fixture.guarantees).toContain("INVOKE_SIGNED_BOUNDARY_EXISTS");
    expect(fixture.guarantees).toContain(
      "PROCESSOR_NOT_WIRED_FOR_LIVE_ROUTE_EXECUTION_YET",
    );
    expect(fixture.nonGoals).toContain("NO_LIVE_ROUTE_ACTIVATION");
  });
});

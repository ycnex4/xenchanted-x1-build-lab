import { describe, expect, it } from "vitest";

import {
  XXXL_REAL_PDA_DERIVATION_CARGO_TEST,
  XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR,
  XXXL_REAL_PDA_DERIVATION_FIXTURE_PROGRAM_ID,
  XXXL_REAL_PDA_DERIVATION_FIXTURE_STATUS,
  XXXL_REAL_PDA_DERIVATION_FIXTURE_VERSION,
  XXXL_REAL_PDA_DERIVATION_RUST_FUNCTION,
  XXXL_REAL_PDA_DERIVATION_RUST_MODULE,
  XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
  validateXXXLRealPdaDerivationFixture,
  xxxlCanonicalRealPdaDerivationFixtureJson,
  xxxlRealPdaDerivationFixture,
  xxxlRealPdaDerivationFixtureMarkdown,
} from "../../src/index.js";

describe("XXXL real PDA derivation fixture", () => {
  it("exports fixture metadata", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.version).toBe(XXXL_REAL_PDA_DERIVATION_FIXTURE_VERSION);
    expect(fixture.status).toBe(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_STATUS.FixtureBoundaryOnly,
    );
  });

  it("uses a valid real Pubkey fixture program id instead of placeholder", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.fixtureProgramId).toBe(XXXL_REAL_PDA_DERIVATION_FIXTURE_PROGRAM_ID);
    expect(fixture.fixtureProgramId).toBe("11111111111111111111111111111111");
    expect(fixture.fixtureProgramId).not.toBe(XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER);
  });

  it("keeps placeholder program id as deploy-time boundary only", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.placeholderProgramId).toBe(XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER);
    expect(fixture.placeholderAcceptedAsLive).toBe(false);
    expect(fixture.deployTimeProgramIdRequired).toBe(true);
  });

  it("rejects model-only PDA for live use", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.modelOnlyPdaAcceptedAsLive).toBe(false);
  });

  it("uses real SVM find_program_address function name", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.realFindProgramAddressRequired).toBe(true);
    expect(fixture.rustFunction).toBe(XXXL_REAL_PDA_DERIVATION_RUST_FUNCTION);
    expect(fixture.rustFunction).toBe("Pubkey::find_program_address");
  });

  it("points to Rust PDA module and cargo test command", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.rustModule).toBe(XXXL_REAL_PDA_DERIVATION_RUST_MODULE);
    expect(fixture.rustModule).toBe("programs/xxxl-svm/src/pda.rs");
    expect(fixture.cargoTest).toBe(XXXL_REAL_PDA_DERIVATION_CARGO_TEST);
  });

  it("keeps PDA seeds exact", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.seeds).toEqual([
      ...XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
    ]);
    expect(fixture.seeds).toEqual(["xxxl", "gateway-mint-authority", "v1"]);
  });

  it("exports seed bytes as canonical hex", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.seedBytesHex).toEqual([
      "7878786c",
      "676174657761792d6d696e742d617574686f72697479",
      "7631",
    ]);
  });

  it("links PDA to invoke_signed and mint_to dependency", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.cpiSignerDependency.requiredForInvokeSigned).toBe(true);
    expect(fixture.cpiSignerDependency.usedBySplTokenMintTo).toBe(true);
    expect(fixture.cpiSignerDependency.bumpRequired).toBe(true);
  });

  it("lists core guarantees", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.guarantees).toContain(
      "REAL_SVM_PUBKEY_FIND_PROGRAM_ADDRESS_IS_USED_IN_RUST_FIXTURE",
    );
    expect(fixture.guarantees).toContain("PDA_SEEDS_MATCH_MODEL_LAYER");
    expect(fixture.guarantees).toContain("DEPLOY_TIME_PROGRAM_ID_REQUIRED_FOR_LIVE_PDA");
  });

  it("keeps stage non-goals explicit", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.nonGoals).toContain("NO_DEPLOYMENT");
    expect(fixture.nonGoals).toContain("NO_SPL_TOKEN_CPI_YET");
    expect(fixture.nonGoals).toContain("NO_ROUTE_ACTIVATION");
  });

  it("validates default fixture", () => {
    const result = validateXXXLRealPdaDerivationFixture();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects wrong seeds", () => {
    const fixture = xxxlRealPdaDerivationFixture();
    const result = validateXXXLRealPdaDerivationFixture({
      ...fixture,
      seeds: ["xxxl", "wrong", "v1"],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.WrongSeeds,
    );
  });

  it("rejects placeholder accepted as live", () => {
    const fixture = xxxlRealPdaDerivationFixture();
    const result = validateXXXLRealPdaDerivationFixture({
      ...fixture,
      placeholderAcceptedAsLive: true as false,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.PlaceholderAcceptedAsLive,
    );
  });

  it("rejects model-only PDA accepted as live", () => {
    const fixture = xxxlRealPdaDerivationFixture();
    const result = validateXXXLRealPdaDerivationFixture({
      ...fixture,
      modelOnlyPdaAcceptedAsLive: true as false,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.ModelOnlyPdaAcceptedAsLive,
    );
  });

  it("rejects missing deploy-time dependency", () => {
    const fixture = xxxlRealPdaDerivationFixture();
    const result = validateXXXLRealPdaDerivationFixture({
      ...fixture,
      deployTimeProgramIdRequired: false as true,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.MissingDeployTimeDependency,
    );
  });

  it("rejects missing CPI dependency", () => {
    const fixture = xxxlRealPdaDerivationFixture();
    const result = validateXXXLRealPdaDerivationFixture({
      ...fixture,
      cpiSignerDependency: {
        ...fixture.cpiSignerDependency,
        bumpRequired: false as true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.MissingCpiDependency,
    );
  });

  it("exports deterministic canonical fixture JSON", () => {
    const fixture = xxxlRealPdaDerivationFixture();
    const json = xxxlCanonicalRealPdaDerivationFixtureJson(fixture);

    expect(json).toBe(xxxlCanonicalRealPdaDerivationFixtureJson(fixture));
    expect(json).toContain('["rustFunction","Pubkey::find_program_address"]');
    expect(json).toContain('["placeholderAcceptedAsLive",false]');
  });

  it("exports markdown fixture report", () => {
    const markdown = xxxlRealPdaDerivationFixtureMarkdown();

    expect(markdown).toContain("# XXXL Real PDA Derivation Fixture");
    expect(markdown).toContain("Pubkey::find_program_address");
    expect(markdown).toContain("gateway-mint-authority");
  });

  it("keeps fixture boundary connected to live deployment dependency", () => {
    const fixture = xxxlRealPdaDerivationFixture();

    expect(fixture.deployTimeProgramIdRequired).toBe(true);
    expect(fixture.guarantees).toContain(
      "MODEL_ONLY_PDA_REJECTED_FOR_LIVE_USE",
    );
  });
});

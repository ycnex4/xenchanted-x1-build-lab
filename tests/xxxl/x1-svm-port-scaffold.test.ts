import { describe, expect, it } from "vitest";

import {
  XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_X1_SVM_PORT_SCAFFOLD_ERROR,
  XXXL_X1_SVM_PORT_SCAFFOLD_FRAMEWORK,
  XXXL_X1_SVM_PORT_SCAFFOLD_ROOT,
  XXXL_X1_SVM_PORT_SCAFFOLD_STATUS,
  XXXL_X1_SVM_PORT_SCAFFOLD_VERSION,
  XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  validateXXXLX1SvmPortScaffold,
  xxxlCanonicalX1SvmPortScaffoldJson,
  xxxlX1SvmPortScaffold,
  xxxlX1SvmPortScaffoldFiles,
  xxxlX1SvmPortScaffoldMarkdown,
} from "../../src/index.js";

describe("XXXL X1/SVM port scaffold", () => {
  it("exports scaffold metadata", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.version).toBe(XXXL_X1_SVM_PORT_SCAFFOLD_VERSION);
    expect(scaffold.status).toBe(XXXL_X1_SVM_PORT_SCAFFOLD_STATUS.ScaffoldOnly);
    expect(scaffold.framework).toBe(XXXL_X1_SVM_PORT_SCAFFOLD_FRAMEWORK);
  });

  it("sets native SVM scaffold root", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.root).toBe(XXXL_X1_SVM_PORT_SCAFFOLD_ROOT);
    expect(scaffold.root).toBe("programs/xxxl-svm");
  });

  it("does not claim deployability", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.liveDeployable).toBe(false);
    expect(scaffold.nonGoals).toContain("NO_DEPLOYMENT");
    expect(scaffold.nonGoals).toContain("NO_LIVE_TRANSACTION_SUBMISSION");
  });

  it("keeps route activation blocked", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.routeActivationAllowed).toBe(false);
    expect(scaffold.nonGoals).toContain("NO_ROUTE_ACTIVATION");
    expect(scaffold.nonGoals).toContain("NO_AVALANCHE_ACTIVATION");
  });

  it("keeps guardian signatures outside runtime", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.guardianSignatureVerificationInRuntime).toBe(false);
    expect(scaffold.nonGoals).toContain(
      "NO_GUARDIAN_SIGNATURE_VERIFICATION_INSIDE_XXXL_RUNTIME",
    );
  });

  it("defines scaffold files in canonical order", () => {
    expect(xxxlX1SvmPortScaffoldFiles().map((file) => file.path)).toEqual([
      "programs/xxxl-svm/Cargo.toml",
      "programs/xxxl-svm/src/lib.rs",
      "programs/xxxl-svm/src/entrypoint.rs",
      "programs/xxxl-svm/src/processor.rs",
      "programs/xxxl-svm/src/instruction.rs",
      "programs/xxxl-svm/src/state.rs",
      "programs/xxxl-svm/src/pda.rs",
      "programs/xxxl-svm/src/cpi.rs",
      "programs/xxxl-svm/src/validation.rs",
      "programs/xxxl-svm/src/error.rs",
    ]);
  });

  it("defines expected scaffold roles", () => {
    expect(xxxlX1SvmPortScaffoldFiles().map((file) => file.role)).toEqual([
      "CARGO_MANIFEST",
      "LIBRARY_ROOT",
      "ENTRYPOINT",
      "PROCESSOR",
      "INSTRUCTION_DECODE",
      "STATE_LAYOUTS",
      "PDA_DERIVATION",
      "SPL_TOKEN_CPI_BOUNDARY",
      "RUNTIME_VALIDATION",
      "ERROR_MAPPING",
    ]);
  });

  it("keeps Program ID as deploy-time boundary", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.programIdBoundary.placeholderProgramId).toBe(
      XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
    );
    expect(scaffold.programIdBoundary.realProgramIdRequired).toBe(true);
    expect(scaffold.programIdBoundary.deployTimeDecision).toBe(true);
  });

  it("keeps Token Program ID fixed", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.tokenProgramBoundary.tokenProgramId).toBe(
      XXXL_X1_SVM_TOKEN_PROGRAM_ID,
    );
    expect(scaffold.tokenProgramBoundary.realSplTokenProgramRequired).toBe(true);
  });

  it("keeps PDA fixture plan exact", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.pdaFixturePlan.seeds).toEqual([
      ...XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
    ]);
    expect(scaffold.pdaFixturePlan.realFindProgramAddressRequired).toBe(true);
    expect(scaffold.pdaFixturePlan.modelOnlyPdaRejectedForLive).toBe(true);
  });

  it("defines account and instruction decode fixture plan", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.decodeFixturePlan.accountLayoutCount).toBe(5);
    expect(scaffold.decodeFixturePlan.instructionLayoutCount).toBe(1);
    expect(scaffold.decodeFixturePlan.realAccountDiscriminatorsRequired).toBe(true);
    expect(scaffold.decodeFixturePlan.realInstructionDiscriminatorRequired).toBe(true);
    expect(scaffold.decodeFixturePlan.realByteParsingRequired).toBe(true);
  });

  it("defines SPL Token CPI fixture plan", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.cpiFixturePlan.realInitializedMintRequired).toBe(true);
    expect(scaffold.cpiFixturePlan.realInitializedRecipientAtaRequired).toBe(true);
    expect(scaffold.cpiFixturePlan.invokeSignedRequired).toBe(true);
    expect(scaffold.cpiFixturePlan.mintAuthorityPdaSignerRequired).toBe(true);
  });

  it("defines runtime check requirements", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.runtimeChecks.accountOwnerChecksRequired).toBe(true);
    expect(scaffold.runtimeChecks.rentExemptionChecksRequired).toBe(true);
    expect(scaffold.runtimeChecks.recipientAtaValidationRequired).toBe(true);
    expect(scaffold.runtimeChecks.clockSourceRequired).toBe(true);
  });

  it("carries implementation requirements from readiness package", () => {
    const scaffold = xxxlX1SvmPortScaffold();

    expect(scaffold.implementationRequirements).toContain("REAL_PROGRAM_ID");
    expect(scaffold.implementationRequirements).toContain(
      "REAL_FIND_PROGRAM_ADDRESS",
    );
    expect(scaffold.implementationRequirements).toContain(
      "REAL_SPL_TOKEN_MINT_TO_CPI",
    );
  });

  it("validates default scaffold", () => {
    const result = validateXXXLX1SvmPortScaffold();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects deployability claim", () => {
    const scaffold = xxxlX1SvmPortScaffold();
    const result = validateXXXLX1SvmPortScaffold({
      ...scaffold,
      liveDeployable: true as false,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.LiveDeployableClaimed,
    );
  });

  it("rejects route activation", () => {
    const scaffold = xxxlX1SvmPortScaffold();
    const result = validateXXXLX1SvmPortScaffold({
      ...scaffold,
      routeActivationAllowed: true as false,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.RouteActivationAllowed,
    );
  });

  it("rejects missing scaffold file", () => {
    const scaffold = xxxlX1SvmPortScaffold();
    const result = validateXXXLX1SvmPortScaffold({
      ...scaffold,
      files: scaffold.files.slice(1),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.MissingScaffoldFile,
    );
  });

  it("exports deterministic scaffold JSON", () => {
    const scaffold = xxxlX1SvmPortScaffold();
    const json = xxxlCanonicalX1SvmPortScaffoldJson(scaffold);

    expect(json).toBe(xxxlCanonicalX1SvmPortScaffoldJson(scaffold));
    expect(json).toContain('["framework","NATIVE_SVM_RUST_SCAFFOLD"]');
    expect(json).toContain('["liveDeployable",false]');
  });

  it("exports scaffold markdown", () => {
    const markdown = xxxlX1SvmPortScaffoldMarkdown();

    expect(markdown).toContain("# XXXL X1/SVM Port Scaffold");
    expect(markdown).toContain("programs/xxxl-svm");
    expect(markdown).toContain("NO_DEPLOYMENT");
    expect(markdown).toContain("Token Program ID");
  });
});

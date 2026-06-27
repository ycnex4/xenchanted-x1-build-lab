import { describe, expect, it } from "vitest";

import {
  XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT,
  XXXL_SVM_RUNTIME_PORT_NEXT_STAGE,
  XXXL_SVM_RUNTIME_PORT_READINESS_ERROR,
  XXXL_SVM_RUNTIME_PORT_READINESS_PACKAGE_VERSION,
  XXXL_SVM_RUNTIME_PORT_READINESS_STATUS,
  XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
  XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  validateXXXLSvmRuntimePortReadinessPackage,
  xxxlCanonicalSvmRuntimePortReadinessPackageJson,
  xxxlSvmRuntimePortImplementationRequirements,
  xxxlSvmRuntimePortNextStages,
  xxxlSvmRuntimePortReadinessPackage,
  xxxlSvmRuntimePortReadinessPackageMarkdown,
  xxxlSvmRuntimePortReadinessProofChain,
} from "../../src/index.js";

describe("XXXL SVM runtime port readiness package", () => {
  it("exports readiness package metadata", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.version).toBe(XXXL_SVM_RUNTIME_PORT_READINESS_PACKAGE_VERSION);
    expect(pkg.status).toBe(XXXL_SVM_RUNTIME_PORT_READINESS_STATUS.ReadyForPort);
  });

  it("marks model layer complete and ready for X1/SVM port", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.modelLayerComplete).toBe(true);
    expect(pkg.readyForX1SvmPort).toBe(true);
  });

  it("does not claim live deployability", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.liveDeployable).toBe(false);
    expect(pkg.routeActivationAllowed).toBe(false);
  });

  it("summarizes production byte layouts", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.byteLayouts.ok).toBe(true);
    expect(pkg.byteLayouts.count).toBe(6);
    expect(pkg.byteLayouts.kinds).toContain("MINT_STATE_ACCOUNT");
    expect(pkg.byteLayouts.kinds).toContain("CONSUME_GATEWAY_MINT_INSTRUCTION");
  });

  it("summarizes serialized vectors and CPI boundary", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.serializedVectors.ok).toBe(true);
    expect(pkg.serializedVectors.vectorCount).toBe(6);
    expect(pkg.serializedVectors.cpiPrepared).toBe(true);
    expect(pkg.serializedVectors.cpiAtomicWithParentTransaction).toBe(true);
  });

  it("summarizes decoder handler result", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.decoderHandler.ok).toBe(true);
    expect(pkg.decoderHandler.steps).toContain("DECODE_ACCOUNT_BYTES");
    expect(pkg.decoderHandler.steps).toContain("PREPARE_CPI_BOUNDARY");
    expect(pkg.decoderHandler.cpiPrepared).toBe(true);
  });

  it("summarizes X1/SVM skeleton boundary", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.skeleton.ok).toBe(true);
    expect(pkg.skeleton.tokenProgramId).toBe(XXXL_X1_SVM_TOKEN_PROGRAM_ID);
    expect(pkg.skeleton.gatewayMintAuthorityPda.length).toBeGreaterThan(0);
    expect(pkg.skeleton.accountMetaRoles).toHaveLength(9);
  });

  it("keeps authority boundary explicit", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.authorityBoundary.programId).toBe(
      XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
    );
    expect(pkg.authorityBoundary.programIdIsPlaceholder).toBe(true);
    expect(pkg.authorityBoundary.tokenProgramId).toBe(XXXL_X1_SVM_TOKEN_PROGRAM_ID);
    expect(pkg.authorityBoundary.gatewayMintAuthorityPdaSeeds).toEqual([
      ...XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
    ]);
  });

  it("keeps guardian signature verification outside runtime", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();

    expect(pkg.guardianSignatureVerificationInRuntime).toBe(false);
    expect(pkg.authorityBoundary.guardianSignatureBoundary).toBe(
      XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
    );
  });

  it("lists all required runtime implementation items", () => {
    expect(xxxlSvmRuntimePortImplementationRequirements()).toEqual([
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealProgramId,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealFindProgramAddress,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealAccountDiscriminators,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealInstructionDiscriminator,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealAccountDecode,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealInstructionDecode,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealAccountOwnerChecks,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealRentExemptionChecks,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealAtaValidation,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealSplTokenMintToCpi,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealClockSlot,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealDeploymentDryRun,
      XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT.RealAuthorityFreezeExecution,
    ]);
  });

  it("lists next stages for the port", () => {
    expect(xxxlSvmRuntimePortNextStages()).toEqual([
      XXXL_SVM_RUNTIME_PORT_NEXT_STAGE.X1SvmPortScaffold,
      XXXL_SVM_RUNTIME_PORT_NEXT_STAGE.RealPdaDerivationFixture,
      XXXL_SVM_RUNTIME_PORT_NEXT_STAGE.RealSplTokenCpiFixture,
      XXXL_SVM_RUNTIME_PORT_NEXT_STAGE.RuntimeAccountDecodeFixture,
      XXXL_SVM_RUNTIME_PORT_NEXT_STAGE.RuntimeInstructionDecodeFixture,
      XXXL_SVM_RUNTIME_PORT_NEXT_STAGE.PredeployDryRunFixture,
    ]);
  });

  it("exports proof chain in canonical order", () => {
    expect(xxxlSvmRuntimePortReadinessProofChain().map((proof) => proof.id)).toEqual([
      "PRODUCTION_BYTE_LAYOUTS",
      "X1_SVM_PROGRAM_SKELETON",
      "SVM_SERIALIZED_RUNTIME_VECTORS",
      "SVM_RUNTIME_DECODER_HANDLER_MODEL",
      "GUARDIAN_SIGNATURE_BOUNDARY",
      "ROUTE_ACTIVATION_BOUNDARY",
    ]);
  });

  it("validates default readiness package", () => {
    const result = validateXXXLSvmRuntimePortReadinessPackage();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects package that claims live deployability", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();
    const result = validateXXXLSvmRuntimePortReadinessPackage({
      ...pkg,
      liveDeployable: true,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.LiveDeployableIncorrectlyClaimed,
    );
  });

  it("rejects package that allows route activation", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();
    const result = validateXXXLSvmRuntimePortReadinessPackage({
      ...pkg,
      routeActivationAllowed: true,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.RouteActivationAllowed,
    );
  });

  it("rejects guardian boundary violation", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();
    const result = validateXXXLSvmRuntimePortReadinessPackage({
      ...pkg,
      guardianSignatureVerificationInRuntime: true,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.GuardianBoundaryViolated,
    );
  });

  it("rejects missing implementation requirement", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();
    const result = validateXXXLSvmRuntimePortReadinessPackage({
      ...pkg,
      implementationRequirements: pkg.implementationRequirements.slice(1),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.MissingImplementationRequirement,
    );
  });

  it("exports deterministic canonical package JSON", () => {
    const pkg = xxxlSvmRuntimePortReadinessPackage();
    const json = xxxlCanonicalSvmRuntimePortReadinessPackageJson(pkg);

    expect(json).toBe(xxxlCanonicalSvmRuntimePortReadinessPackageJson(pkg));
    expect(json).toContain(
      '["status","READY_FOR_X1_SVM_PORT_MODEL_LAYER_COMPLETE"]',
    );
    expect(json).toContain('["liveDeployable",false]');
  });

  it("exports markdown review package", () => {
    const markdown = xxxlSvmRuntimePortReadinessPackageMarkdown();

    expect(markdown).toContain("# XXXL SVM Runtime Port Readiness Package");
    expect(markdown).toContain("Model layer complete: yes");
    expect(markdown).toContain("Live deployable: no");
    expect(markdown).toContain("REAL_PROGRAM_ID");
    expect(markdown).toContain("X1_SVM_PORT_SCAFFOLD");
  });
});

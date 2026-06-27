import { describe, expect, it } from "vitest";

import {
  XXXL_X1_SVM_ACCOUNT_ROLE,
  XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
  XXXL_X1_SVM_HANDLER,
  XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
  XXXL_X1_SVM_PROGRAM_SKELETON_ERROR,
  XXXL_X1_SVM_PROGRAM_SKELETON_STATUS,
  XXXL_X1_SVM_PROGRAM_SKELETON_STEP,
  XXXL_X1_SVM_PROGRAM_SKELETON_VERSION,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  deriveXXXLX1SvmGatewayMintAuthorityPdaModel,
  executeXXXLX1SvmConsumeGatewayMintSkeleton,
  validateXXXLX1SvmProgramSkeletonConfig,
  xxxlCanonicalX1SvmProgramSkeletonConfigJson,
  xxxlX1SvmConsumeGatewayMintAccountMetas,
  xxxlX1SvmDefaultConsumeGatewayMintInput,
  xxxlX1SvmProgramSkeletonConfig,
  type XXXLX1SvmAccountMeta,
} from "../../src/index.js";

function metaByRole(role: string): XXXLX1SvmAccountMeta {
  const meta = xxxlX1SvmConsumeGatewayMintAccountMetas().find(
    (item) => item.role === role,
  );

  expect(meta).toBeDefined();

  return meta as XXXLX1SvmAccountMeta;
}

describe("XXXL X1/SVM program skeleton", () => {
  it("exports model-only skeleton metadata", () => {
    const config = xxxlX1SvmProgramSkeletonConfig();

    expect(config.version).toBe(XXXL_X1_SVM_PROGRAM_SKELETON_VERSION);
    expect(config.status).toBe(XXXL_X1_SVM_PROGRAM_SKELETON_STATUS.ModelOnly);
    expect(config.handler).toBe(XXXL_X1_SVM_HANDLER.ConsumeGatewayMint);
  });

  it("keeps Program ID as explicit placeholder boundary", () => {
    const config = xxxlX1SvmProgramSkeletonConfig();

    expect(config.programId).toBe(XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER);
    expect(config.programId).toContain("XXXLProgram");
  });

  it("uses canonical SPL Token Program ID constant", () => {
    expect(XXXL_X1_SVM_TOKEN_PROGRAM_ID).toBe(
      "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    );
    expect(xxxlX1SvmProgramSkeletonConfig().tokenProgramId).toBe(
      XXXL_X1_SVM_TOKEN_PROGRAM_ID,
    );
  });

  it("keeps gateway mint authority PDA seeds exact", () => {
    expect(XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS).toEqual([
      "xxxl",
      "gateway-mint-authority",
      "v1",
    ]);
  });

  it("derives deterministic model PDA for gateway mint authority", () => {
    const left = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
      XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
    );
    const right = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
      XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
    );

    expect(left).toEqual(right);
    expect(left.modelOnly).toBe(true);
    expect(left.seeds).toEqual(XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS);
    expect(left.address).toContain("pda_");
  });

  it("changes model PDA when program id or seeds change", () => {
    const baseline = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
      XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
    );
    const differentProgram = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
      "OtherProgram1111111111111111111111111111111",
    );
    const differentSeeds = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
      XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
      ["xxxl", "other", "v1"],
    );

    expect(differentProgram.address).not.toBe(baseline.address);
    expect(differentSeeds.address).not.toBe(baseline.address);
  });

  it("exports consume_gateway_mint account metas in canonical order", () => {
    expect(xxxlX1SvmConsumeGatewayMintAccountMetas().map((meta) => meta.role)).toEqual([
      XXXL_X1_SVM_ACCOUNT_ROLE.MintState,
      XXXL_X1_SVM_ACCOUNT_ROLE.GatewayConfig,
      XXXL_X1_SVM_ACCOUNT_ROLE.GuardianSet,
      XXXL_X1_SVM_ACCOUNT_ROLE.ProcessedEvent,
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientBalance,
      XXXL_X1_SVM_ACCOUNT_ROLE.SplTokenMint,
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientTokenAccount,
      XXXL_X1_SVM_ACCOUNT_ROLE.MintAuthorityPda,
      XXXL_X1_SVM_ACCOUNT_ROLE.TokenProgram,
    ]);
  });

  it("keeps writable account metas explicit", () => {
    expect(metaByRole(XXXL_X1_SVM_ACCOUNT_ROLE.MintState).writable).toBe(true);
    expect(metaByRole(XXXL_X1_SVM_ACCOUNT_ROLE.ProcessedEvent).writable).toBe(true);
    expect(metaByRole(XXXL_X1_SVM_ACCOUNT_ROLE.RecipientBalance).writable).toBe(true);
    expect(metaByRole(XXXL_X1_SVM_ACCOUNT_ROLE.SplTokenMint).writable).toBe(true);
    expect(metaByRole(XXXL_X1_SVM_ACCOUNT_ROLE.RecipientTokenAccount).writable).toBe(true);
    expect(metaByRole(XXXL_X1_SVM_ACCOUNT_ROLE.GatewayConfig).writable).toBe(false);
  });

  it("keeps mint authority PDA as CPI signer only", () => {
    const pda = metaByRole(XXXL_X1_SVM_ACCOUNT_ROLE.MintAuthorityPda);

    expect(pda.signer).toBe(false);
    expect(pda.cpiSigner).toBe(true);
    expect(pda.writable).toBe(false);
  });

  it("keeps guardian signature verification outside runtime", () => {
    const config = xxxlX1SvmProgramSkeletonConfig();

    expect(config.guardianSignatureBoundary).toBe(
      XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
    );
  });

  it("validates default skeleton config", () => {
    const validation = validateXXXLX1SvmProgramSkeletonConfig(
      xxxlX1SvmProgramSkeletonConfig(),
    );

    expect(validation.ok).toBe(true);
    expect(validation.errors).toEqual([]);
  });

  it("exports deterministic canonical config JSON", () => {
    const config = xxxlX1SvmProgramSkeletonConfig();
    const json = xxxlCanonicalX1SvmProgramSkeletonConfigJson(config);

    expect(json).toBe(xxxlCanonicalX1SvmProgramSkeletonConfigJson(config));
    expect(json).toContain('["handler","consume_gateway_mint"]');
    expect(json).toContain('["tokenProgramId","TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"]');
  });

  it("executes valid consume_gateway_mint skeleton", () => {
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.steps).toEqual([
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateModelOnlyBoundary,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateProgramIdBoundary,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateTokenProgram,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.DeriveGatewayMintAuthorityPda,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateAccountMetas,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateProductionByteLayouts,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ConsumeStage1AuthorizationBoundary,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.PrepareSplTokenMintToCpi,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.MarkProcessedEventBoundary,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ReturnSkeletonResult,
    ]);
  });

  it("prepares SPL Token mint_to CPI boundary on valid skeleton execution", () => {
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton();

    expect(result.cpiBoundary).toMatchObject({
      modelOnly: true,
      instruction: "spl_token_mint_to",
      tokenProgramId: XXXL_X1_SVM_TOKEN_PROGRAM_ID,
      atomicWithParentTransaction: true,
      prepared: true,
    });
    expect(result.cpiBoundary.cpiSignerPda).toBe(
      xxxlX1SvmProgramSkeletonConfig().gatewayMintAuthorityPda.address,
    );
  });

  it("rejects Token Program mismatch", () => {
    const input = xxxlX1SvmDefaultConsumeGatewayMintInput();
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton({
      ...input,
      config: {
        ...input.config,
        tokenProgramId: "WrongTokenProgram",
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.TokenProgramMismatch,
    );
    expect(result.cpiBoundary.prepared).toBe(false);
  });

  it("rejects gateway mint authority PDA mismatch", () => {
    const input = xxxlX1SvmDefaultConsumeGatewayMintInput();
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton({
      ...input,
      config: {
        ...input.config,
        gatewayMintAuthorityPda: {
          ...input.config.gatewayMintAuthorityPda,
          address: "wrong_pda",
        },
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.GatewayMintAuthorityPdaMismatch,
    );
  });

  it("rejects account meta mismatch", () => {
    const input = xxxlX1SvmDefaultConsumeGatewayMintInput();
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton({
      ...input,
      config: {
        ...input.config,
        accountMetas: input.config.accountMetas.slice(1),
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.AccountMetasMismatch,
    );
  });

  it("rejects guardian signature verification inside runtime", () => {
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton({
      ...xxxlX1SvmDefaultConsumeGatewayMintInput(),
      guardianSignatureVerificationInsideRuntime: true,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.GuardianSignatureVerificationRequested,
    );
  });

  it("rejects route activation request", () => {
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton({
      ...xxxlX1SvmDefaultConsumeGatewayMintInput(),
      routeActivationRequested: true,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.RouteActivationRequested,
    );
  });

  it("rejects live transaction submission request", () => {
    const result = executeXXXLX1SvmConsumeGatewayMintSkeleton({
      ...xxxlX1SvmDefaultConsumeGatewayMintInput(),
      liveTransactionSubmissionRequested: true,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.LiveTransactionSubmissionRequested,
    );
  });
});

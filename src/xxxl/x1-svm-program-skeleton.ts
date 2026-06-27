import {
  validateXXXLProductionRuntimeByteLayouts,
  xxxlProductionRuntimeByteLayouts,
  type XXXLProductionRuntimeByteLayoutKind,
} from "./runtime-production-byte-layout.js";

export const XXXL_X1_SVM_PROGRAM_SKELETON_VERSION = 1;

export const XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER =
  "XXXLProgram111111111111111111111111111111111";

export const XXXL_X1_SVM_TOKEN_PROGRAM_ID =
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

export const XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS = [
  "xxxl",
  "gateway-mint-authority",
  "v1",
] as const;

export const XXXL_X1_SVM_PROGRAM_SKELETON_STATUS = {
  ModelOnly: "MODEL_ONLY_NOT_DEPLOYABLE",
} as const;

export type XXXLX1SvmProgramSkeletonStatus =
  (typeof XXXL_X1_SVM_PROGRAM_SKELETON_STATUS)[keyof typeof XXXL_X1_SVM_PROGRAM_SKELETON_STATUS];

export const XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY =
  "STAGE_1_AUTHORIZATION_RESULT_ONLY";

export const XXXL_X1_SVM_HANDLER = {
  ConsumeGatewayMint: "consume_gateway_mint",
} as const;

export type XXXLX1SvmHandler =
  (typeof XXXL_X1_SVM_HANDLER)[keyof typeof XXXL_X1_SVM_HANDLER];

export const XXXL_X1_SVM_ACCOUNT_ROLE = {
  MintState: "MINT_STATE",
  GatewayConfig: "GATEWAY_CONFIG",
  GuardianSet: "GUARDIAN_SET",
  ProcessedEvent: "PROCESSED_EVENT",
  RecipientBalance: "RECIPIENT_BALANCE",
  SplTokenMint: "SPL_TOKEN_MINT",
  RecipientTokenAccount: "RECIPIENT_TOKEN_ACCOUNT",
  MintAuthorityPda: "MINT_AUTHORITY_PDA",
  TokenProgram: "TOKEN_PROGRAM",
} as const;

export type XXXLX1SvmAccountRole =
  (typeof XXXL_X1_SVM_ACCOUNT_ROLE)[keyof typeof XXXL_X1_SVM_ACCOUNT_ROLE];

export const XXXL_X1_SVM_PROGRAM_SKELETON_STEP = {
  ValidateModelOnlyBoundary: "VALIDATE_MODEL_ONLY_BOUNDARY",
  ValidateProgramIdBoundary: "VALIDATE_PROGRAM_ID_BOUNDARY",
  ValidateTokenProgram: "VALIDATE_TOKEN_PROGRAM",
  DeriveGatewayMintAuthorityPda: "DERIVE_GATEWAY_MINT_AUTHORITY_PDA",
  ValidateAccountMetas: "VALIDATE_ACCOUNT_METAS",
  ValidateProductionByteLayouts: "VALIDATE_PRODUCTION_BYTE_LAYOUTS",
  ConsumeStage1AuthorizationBoundary: "CONSUME_STAGE_1_AUTHORIZATION_BOUNDARY",
  PrepareSplTokenMintToCpi: "PREPARE_SPL_TOKEN_MINT_TO_CPI",
  MarkProcessedEventBoundary: "MARK_PROCESSED_EVENT_BOUNDARY",
  ReturnSkeletonResult: "RETURN_SKELETON_RESULT",
} as const;

export type XXXLX1SvmProgramSkeletonStep =
  (typeof XXXL_X1_SVM_PROGRAM_SKELETON_STEP)[keyof typeof XXXL_X1_SVM_PROGRAM_SKELETON_STEP];

export const XXXL_X1_SVM_PROGRAM_SKELETON_ERROR = {
  ProgramIdNotPlaceholder: "PROGRAM_ID_NOT_PLACEHOLDER",
  TokenProgramMismatch: "TOKEN_PROGRAM_MISMATCH",
  GatewayMintAuthorityPdaMismatch: "GATEWAY_MINT_AUTHORITY_PDA_MISMATCH",
  AccountMetasMismatch: "ACCOUNT_METAS_MISMATCH",
  ByteLayoutsInvalid: "BYTE_LAYOUTS_INVALID",
  GuardianSignatureVerificationRequested:
    "GUARDIAN_SIGNATURE_VERIFICATION_REQUESTED",
  RouteActivationRequested: "ROUTE_ACTIVATION_REQUESTED",
  LiveTransactionSubmissionRequested: "LIVE_TRANSACTION_SUBMISSION_REQUESTED",
} as const;

export type XXXLX1SvmProgramSkeletonErrorCode =
  (typeof XXXL_X1_SVM_PROGRAM_SKELETON_ERROR)[keyof typeof XXXL_X1_SVM_PROGRAM_SKELETON_ERROR];

export type XXXLX1SvmPdaModel = {
  readonly modelOnly: true;
  readonly programId: string;
  readonly seeds: readonly string[];
  readonly bump: number;
  readonly address: string;
};

export type XXXLX1SvmAccountMeta = {
  readonly role: XXXLX1SvmAccountRole;
  readonly index: number;
  readonly writable: boolean;
  readonly signer: boolean;
  readonly cpiSigner: boolean;
  readonly description: string;
};

export type XXXLX1SvmProgramSkeletonConfig = {
  readonly version: typeof XXXL_X1_SVM_PROGRAM_SKELETON_VERSION;
  readonly status: XXXLX1SvmProgramSkeletonStatus;
  readonly programId: string;
  readonly tokenProgramId: string;
  readonly handler: XXXLX1SvmHandler;
  readonly gatewayMintAuthorityPda: XXXLX1SvmPdaModel;
  readonly accountMetas: readonly XXXLX1SvmAccountMeta[];
  readonly guardianSignatureBoundary: typeof XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY;
  readonly routeActivationAllowed: false;
  readonly liveTransactionSubmissionAllowed: false;
  readonly byteLayoutKinds: readonly XXXLProductionRuntimeByteLayoutKind[];
};

export type XXXLX1SvmProgramSkeletonValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLX1SvmProgramSkeletonErrorCode[];
};

export type XXXLX1SvmConsumeGatewayMintSkeletonInput = {
  readonly config: XXXLX1SvmProgramSkeletonConfig;
  readonly guardianSignatureVerificationInsideRuntime: boolean;
  readonly routeActivationRequested: boolean;
  readonly liveTransactionSubmissionRequested: boolean;
};

export type XXXLX1SvmCpiBoundary = {
  readonly modelOnly: true;
  readonly instruction: "spl_token_mint_to";
  readonly tokenProgramId: string;
  readonly cpiSignerPda: string;
  readonly atomicWithParentTransaction: true;
  readonly prepared: boolean;
};

export type XXXLX1SvmConsumeGatewayMintSkeletonResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLX1SvmProgramSkeletonErrorCode[];
  readonly steps: readonly XXXLX1SvmProgramSkeletonStep[];
  readonly cpiBoundary: XXXLX1SvmCpiBoundary;
};

export function xxxlX1SvmProgramSkeletonConfig(): XXXLX1SvmProgramSkeletonConfig {
  const pda = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
    XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
  );

  return {
    version: XXXL_X1_SVM_PROGRAM_SKELETON_VERSION,
    status: XXXL_X1_SVM_PROGRAM_SKELETON_STATUS.ModelOnly,
    programId: XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
    tokenProgramId: XXXL_X1_SVM_TOKEN_PROGRAM_ID,
    handler: XXXL_X1_SVM_HANDLER.ConsumeGatewayMint,
    gatewayMintAuthorityPda: pda,
    accountMetas: xxxlX1SvmConsumeGatewayMintAccountMetas(),
    guardianSignatureBoundary: XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
    routeActivationAllowed: false,
    liveTransactionSubmissionAllowed: false,
    byteLayoutKinds: xxxlProductionRuntimeByteLayouts().map(
      (layout) => layout.kind,
    ),
  };
}

export function xxxlX1SvmDefaultConsumeGatewayMintInput(): XXXLX1SvmConsumeGatewayMintSkeletonInput {
  return {
    config: xxxlX1SvmProgramSkeletonConfig(),
    guardianSignatureVerificationInsideRuntime: false,
    routeActivationRequested: false,
    liveTransactionSubmissionRequested: false,
  };
}

export function xxxlX1SvmConsumeGatewayMintAccountMetas(): readonly XXXLX1SvmAccountMeta[] {
  return [
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.MintState,
      0,
      true,
      false,
      false,
      "Mint State account, writable mirror of total supply and authorities.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.GatewayConfig,
      1,
      false,
      false,
      false,
      "Gateway route config account, read-only.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.GuardianSet,
      2,
      false,
      false,
      false,
      "Guardian set account, read-only runtime reference.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.ProcessedEvent,
      3,
      true,
      false,
      false,
      "Processed Event account, writable replay-protection marker.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientBalance,
      4,
      true,
      false,
      false,
      "Recipient balance mirror account, writable.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.SplTokenMint,
      5,
      true,
      false,
      false,
      "XXXL SPL Token mint account, writable for mint_to CPI.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientTokenAccount,
      6,
      true,
      false,
      false,
      "Recipient token account / ATA, writable for mint_to CPI.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.MintAuthorityPda,
      7,
      false,
      false,
      true,
      "Gateway mint authority PDA, CPI signer only.",
    ),
    meta(
      XXXL_X1_SVM_ACCOUNT_ROLE.TokenProgram,
      8,
      false,
      false,
      false,
      "SPL Token Program account.",
    ),
  ];
}

export function deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
  programId: string,
  seeds: readonly string[] = XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
): XXXLX1SvmPdaModel {
  const material = [...seeds, programId].join("|");
  const digest = stableHexDigest(material);
  const bump = 255 - (parseInt(digest.slice(0, 2), 16) % 256);

  return {
    modelOnly: true,
    programId,
    seeds,
    bump,
    address: `pda_${digest}_${bump.toString(16).padStart(2, "0")}`,
  };
}

export function validateXXXLX1SvmProgramSkeletonConfig(
  config: XXXLX1SvmProgramSkeletonConfig,
): XXXLX1SvmProgramSkeletonValidationResult {
  const errors: XXXLX1SvmProgramSkeletonErrorCode[] = [];

  if (config.programId !== XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER) {
    errors.push(XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.ProgramIdNotPlaceholder);
  }

  if (config.tokenProgramId !== XXXL_X1_SVM_TOKEN_PROGRAM_ID) {
    errors.push(XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.TokenProgramMismatch);
  }

  const expectedPda = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(
    config.programId,
  );

  if (
    config.gatewayMintAuthorityPda.address !== expectedPda.address ||
    config.gatewayMintAuthorityPda.bump !== expectedPda.bump ||
    !sameStrings(
      config.gatewayMintAuthorityPda.seeds,
      XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
    )
  ) {
    errors.push(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.GatewayMintAuthorityPdaMismatch,
    );
  }

  if (!sameAccountMetas(config.accountMetas, xxxlX1SvmConsumeGatewayMintAccountMetas())) {
    errors.push(XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.AccountMetasMismatch);
  }

  const byteLayoutValidation = validateXXXLProductionRuntimeByteLayouts();

  if (!byteLayoutValidation.ok) {
    errors.push(XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.ByteLayoutsInvalid);
  }

  if (config.routeActivationAllowed) {
    errors.push(XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.RouteActivationRequested);
  }

  if (config.liveTransactionSubmissionAllowed) {
    errors.push(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.LiveTransactionSubmissionRequested,
    );
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function executeXXXLX1SvmConsumeGatewayMintSkeleton(
  input: XXXLX1SvmConsumeGatewayMintSkeletonInput = xxxlX1SvmDefaultConsumeGatewayMintInput(),
): XXXLX1SvmConsumeGatewayMintSkeletonResult {
  const steps: XXXLX1SvmProgramSkeletonStep[] = [
    XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateModelOnlyBoundary,
    XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateProgramIdBoundary,
    XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateTokenProgram,
    XXXL_X1_SVM_PROGRAM_SKELETON_STEP.DeriveGatewayMintAuthorityPda,
    XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateAccountMetas,
    XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ValidateProductionByteLayouts,
  ];

  const errors = [
    ...validateXXXLX1SvmProgramSkeletonConfig(input.config).errors,
  ];

  if (input.guardianSignatureVerificationInsideRuntime) {
    errors.push(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.GuardianSignatureVerificationRequested,
    );
  }

  if (input.routeActivationRequested) {
    errors.push(XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.RouteActivationRequested);
  }

  if (input.liveTransactionSubmissionRequested) {
    errors.push(
      XXXL_X1_SVM_PROGRAM_SKELETON_ERROR.LiveTransactionSubmissionRequested,
    );
  }

  const ok = errors.length === 0;

  if (ok) {
    steps.push(
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ConsumeStage1AuthorizationBoundary,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.PrepareSplTokenMintToCpi,
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.MarkProcessedEventBoundary,
    );
  }

  steps.push(XXXL_X1_SVM_PROGRAM_SKELETON_STEP.ReturnSkeletonResult);

  return {
    ok,
    errors,
    steps,
    cpiBoundary: {
      modelOnly: true,
      instruction: "spl_token_mint_to",
      tokenProgramId: input.config.tokenProgramId,
      cpiSignerPda: input.config.gatewayMintAuthorityPda.address,
      atomicWithParentTransaction: true,
      prepared: ok,
    },
  };
}

export function xxxlCanonicalX1SvmProgramSkeletonConfigJson(
  config: XXXLX1SvmProgramSkeletonConfig,
): string {
  return JSON.stringify([
    ["version", config.version],
    ["status", config.status],
    ["programId", config.programId],
    ["tokenProgramId", config.tokenProgramId],
    ["handler", config.handler],
    [
      "gatewayMintAuthorityPda",
      [
        ["modelOnly", config.gatewayMintAuthorityPda.modelOnly],
        ["programId", config.gatewayMintAuthorityPda.programId],
        ["seeds", config.gatewayMintAuthorityPda.seeds],
        ["bump", config.gatewayMintAuthorityPda.bump],
        ["address", config.gatewayMintAuthorityPda.address],
      ],
    ],
    [
      "accountMetas",
      config.accountMetas.map((accountMeta) => [
        ["role", accountMeta.role],
        ["index", accountMeta.index],
        ["writable", accountMeta.writable],
        ["signer", accountMeta.signer],
        ["cpiSigner", accountMeta.cpiSigner],
        ["description", accountMeta.description],
      ]),
    ],
    ["guardianSignatureBoundary", config.guardianSignatureBoundary],
    ["routeActivationAllowed", config.routeActivationAllowed],
    ["liveTransactionSubmissionAllowed", config.liveTransactionSubmissionAllowed],
    ["byteLayoutKinds", config.byteLayoutKinds],
  ]);
}

function meta(
  role: XXXLX1SvmAccountRole,
  index: number,
  writable: boolean,
  signer: boolean,
  cpiSigner: boolean,
  description: string,
): XXXLX1SvmAccountMeta {
  return {
    role,
    index,
    writable,
    signer,
    cpiSigner,
    description,
  };
}

function sameAccountMetas(
  left: readonly XXXLX1SvmAccountMeta[],
  right: readonly XXXLX1SvmAccountMeta[],
): boolean {
  return (
    left.length === right.length &&
    left.every((item, index) => {
      const expected = right[index];

      return (
        expected !== undefined &&
        item.role === expected.role &&
        item.index === expected.index &&
        item.writable === expected.writable &&
        item.signer === expected.signer &&
        item.cpiSigner === expected.cpiSigner
      );
    })
  );
}

function sameStrings(left: readonly string[], right: readonly string[]): boolean {
  return (
    left.length === right.length &&
    left.every((item, index) => item === right[index])
  );
}

function stableHexDigest(input: string): string {
  let a = 0x811c9dc5;
  let b = 0x9e3779b9;
  let c = 0x85ebca6b;
  let d = 0xc2b2ae35;

  for (let index = 0; index < input.length; index += 1) {
    const code = input.charCodeAt(index);

    a = Math.imul(a ^ code, 0x01000193) >>> 0;
    b = Math.imul(b + code + index, 0x27d4eb2d) >>> 0;
    c = Math.imul(c ^ (code << (index % 8)), 0x165667b1) >>> 0;
    d = Math.imul(d + (code ^ index), 0xd3a2646c) >>> 0;
  }

  return [a, b, c, d]
    .map((item) => item.toString(16).padStart(8, "0"))
    .join("");
}

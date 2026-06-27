import {
  validateXXXLProductionRuntimeByteLayouts,
  xxxlProductionRuntimeByteLayouts,
} from "./runtime-production-byte-layout.js";
import {
  validateXXXLSvmSerializedRuntimeVectors,
  xxxlSvmSerializedRuntimeBundle,
} from "./svm-serialized-runtime-vectors.js";
import {
  executeXXXLSvmRuntimeDecoderHandlerModel,
} from "./svm-runtime-decoder-handler-model.js";
import {
  XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
  XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  executeXXXLX1SvmConsumeGatewayMintSkeleton,
} from "./x1-svm-program-skeleton.js";

export const XXXL_SVM_RUNTIME_PORT_READINESS_PACKAGE_VERSION = 1;

export const XXXL_SVM_RUNTIME_PORT_READINESS_STATUS = {
  ReadyForPort: "READY_FOR_X1_SVM_PORT_MODEL_LAYER_COMPLETE",
  NotDeployable: "NOT_LIVE_DEPLOYABLE",
} as const;

export type XXXLSvmRuntimePortReadinessStatus =
  (typeof XXXL_SVM_RUNTIME_PORT_READINESS_STATUS)[keyof typeof XXXL_SVM_RUNTIME_PORT_READINESS_STATUS];

export const XXXL_SVM_RUNTIME_PORT_READINESS_ERROR = {
  ModelLayerIncomplete: "MODEL_LAYER_INCOMPLETE",
  ByteLayoutsInvalid: "BYTE_LAYOUTS_INVALID",
  SerializedVectorsInvalid: "SERIALIZED_VECTORS_INVALID",
  DecoderHandlerInvalid: "DECODER_HANDLER_INVALID",
  SkeletonBoundaryInvalid: "SKELETON_BOUNDARY_INVALID",
  CpiBoundaryMissing: "CPI_BOUNDARY_MISSING",
  LiveDeployableIncorrectlyClaimed: "LIVE_DEPLOYABLE_INCORRECTLY_CLAIMED",
  RouteActivationAllowed: "ROUTE_ACTIVATION_ALLOWED",
  GuardianBoundaryViolated: "GUARDIAN_BOUNDARY_VIOLATED",
  MissingImplementationRequirement: "MISSING_IMPLEMENTATION_REQUIREMENT",
  MissingNextStage: "MISSING_NEXT_STAGE",
} as const;

export type XXXLSvmRuntimePortReadinessErrorCode =
  (typeof XXXL_SVM_RUNTIME_PORT_READINESS_ERROR)[keyof typeof XXXL_SVM_RUNTIME_PORT_READINESS_ERROR];

export const XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT = {
  RealProgramId: "REAL_PROGRAM_ID",
  RealFindProgramAddress: "REAL_FIND_PROGRAM_ADDRESS",
  RealAccountDiscriminators: "REAL_ACCOUNT_DISCRIMINATORS",
  RealInstructionDiscriminator: "REAL_INSTRUCTION_DISCRIMINATOR",
  RealAccountDecode: "REAL_ACCOUNT_DECODE",
  RealInstructionDecode: "REAL_INSTRUCTION_DECODE",
  RealAccountOwnerChecks: "REAL_ACCOUNT_OWNER_CHECKS",
  RealRentExemptionChecks: "REAL_RENT_EXEMPTION_CHECKS",
  RealAtaValidation: "REAL_RECIPIENT_ATA_VALIDATION",
  RealSplTokenMintToCpi: "REAL_SPL_TOKEN_MINT_TO_CPI",
  RealClockSlot: "REAL_CLOCK_SLOT",
  RealDeploymentDryRun: "REAL_DEPLOYMENT_DRY_RUN",
  RealAuthorityFreezeExecution: "REAL_AUTHORITY_FREEZE_EXECUTION",
} as const;

export type XXXLSvmRuntimePortImplementationRequirement =
  (typeof XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT)[keyof typeof XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT];

export const XXXL_SVM_RUNTIME_PORT_NEXT_STAGE = {
  X1SvmPortScaffold: "X1_SVM_PORT_SCAFFOLD",
  RealPdaDerivationFixture: "REAL_PDA_DERIVATION_FIXTURE",
  RealSplTokenCpiFixture: "REAL_SPL_TOKEN_CPI_FIXTURE",
  RuntimeAccountDecodeFixture: "RUNTIME_ACCOUNT_DECODE_FIXTURE",
  RuntimeInstructionDecodeFixture: "RUNTIME_INSTRUCTION_DECODE_FIXTURE",
  PredeployDryRunFixture: "PREDEPLOY_DRY_RUN_FIXTURE",
} as const;

export type XXXLSvmRuntimePortNextStage =
  (typeof XXXL_SVM_RUNTIME_PORT_NEXT_STAGE)[keyof typeof XXXL_SVM_RUNTIME_PORT_NEXT_STAGE];

export type XXXLSvmRuntimePortReadinessProof = {
  readonly id: string;
  readonly result: string;
  readonly boundary: string;
};

export type XXXLSvmRuntimePortReadinessPackage = {
  readonly version: typeof XXXL_SVM_RUNTIME_PORT_READINESS_PACKAGE_VERSION;
  readonly status: XXXLSvmRuntimePortReadinessStatus;
  readonly modelLayerComplete: boolean;
  readonly readyForX1SvmPort: boolean;
  readonly liveDeployable: boolean;
  readonly routeActivationAllowed: boolean;
  readonly guardianSignatureVerificationInRuntime: boolean;
  readonly byteLayouts: {
    readonly ok: boolean;
    readonly count: number;
    readonly kinds: readonly string[];
  };
  readonly serializedVectors: {
    readonly ok: boolean;
    readonly vectorCount: number;
    readonly cpiPrepared: boolean;
    readonly cpiAtomicWithParentTransaction: boolean;
  };
  readonly decoderHandler: {
    readonly ok: boolean;
    readonly steps: readonly string[];
    readonly cpiPrepared: boolean;
    readonly cpiAtomicWithParentTransaction: boolean;
  };
  readonly skeleton: {
    readonly ok: boolean;
    readonly tokenProgramId: string;
    readonly gatewayMintAuthorityPda: string;
    readonly accountMetaRoles: readonly string[];
    readonly cpiPrepared: boolean;
    readonly cpiAtomicWithParentTransaction: boolean;
  };
  readonly authorityBoundary: {
    readonly programId: string;
    readonly programIdIsPlaceholder: boolean;
    readonly tokenProgramId: string;
    readonly gatewayMintAuthorityPdaSeeds: readonly string[];
    readonly guardianSignatureBoundary: string;
  };
  readonly implementationRequirements: readonly XXXLSvmRuntimePortImplementationRequirement[];
  readonly nextStages: readonly XXXLSvmRuntimePortNextStage[];
  readonly proofChain: readonly XXXLSvmRuntimePortReadinessProof[];
};

export type XXXLSvmRuntimePortReadinessValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLSvmRuntimePortReadinessErrorCode[];
};

export function xxxlSvmRuntimePortReadinessPackage(): XXXLSvmRuntimePortReadinessPackage {
  const byteLayoutValidation = validateXXXLProductionRuntimeByteLayouts();
  const byteLayouts = xxxlProductionRuntimeByteLayouts();
  const serializedVectorValidation = validateXXXLSvmSerializedRuntimeVectors();
  const serializedBundle = xxxlSvmSerializedRuntimeBundle();
  const decoderHandlerResult = executeXXXLSvmRuntimeDecoderHandlerModel();
  const skeletonResult = executeXXXLX1SvmConsumeGatewayMintSkeleton();

  const modelLayerComplete =
    byteLayoutValidation.ok &&
    serializedVectorValidation.ok &&
    decoderHandlerResult.ok &&
    skeletonResult.ok &&
    serializedBundle.cpiPrepared &&
    serializedBundle.cpiAtomicWithParentTransaction &&
    decoderHandlerResult.cpiPrepared &&
    decoderHandlerResult.cpiAtomicWithParentTransaction &&
    skeletonResult.cpiBoundary.prepared &&
    skeletonResult.cpiBoundary.atomicWithParentTransaction;

  return {
    version: XXXL_SVM_RUNTIME_PORT_READINESS_PACKAGE_VERSION,
    status: XXXL_SVM_RUNTIME_PORT_READINESS_STATUS.ReadyForPort,
    modelLayerComplete,
    readyForX1SvmPort: modelLayerComplete,
    liveDeployable: false,
    routeActivationAllowed: false,
    guardianSignatureVerificationInRuntime: false,
    byteLayouts: {
      ok: byteLayoutValidation.ok,
      count: byteLayouts.length,
      kinds: byteLayouts.map((layout) => layout.kind),
    },
    serializedVectors: {
      ok: serializedVectorValidation.ok,
      vectorCount: serializedBundle.vectors.length,
      cpiPrepared: serializedBundle.cpiPrepared,
      cpiAtomicWithParentTransaction:
        serializedBundle.cpiAtomicWithParentTransaction,
    },
    decoderHandler: {
      ok: decoderHandlerResult.ok,
      steps: decoderHandlerResult.steps,
      cpiPrepared: decoderHandlerResult.cpiPrepared,
      cpiAtomicWithParentTransaction:
        decoderHandlerResult.cpiAtomicWithParentTransaction,
    },
    skeleton: {
      ok: skeletonResult.ok,
      tokenProgramId: XXXL_X1_SVM_TOKEN_PROGRAM_ID,
      gatewayMintAuthorityPda: skeletonResult.cpiBoundary.cpiSignerPda,
      accountMetaRoles: serializedBundle.accountMetaRoles,
      cpiPrepared: skeletonResult.cpiBoundary.prepared,
      cpiAtomicWithParentTransaction:
        skeletonResult.cpiBoundary.atomicWithParentTransaction,
    },
    authorityBoundary: {
      programId: XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
      programIdIsPlaceholder: true,
      tokenProgramId: XXXL_X1_SVM_TOKEN_PROGRAM_ID,
      gatewayMintAuthorityPdaSeeds: [
        ...XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
      ],
      guardianSignatureBoundary: XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
    },
    implementationRequirements: xxxlSvmRuntimePortImplementationRequirements(),
    nextStages: xxxlSvmRuntimePortNextStages(),
    proofChain: xxxlSvmRuntimePortReadinessProofChain(),
  };
}

export function validateXXXLSvmRuntimePortReadinessPackage(
  pkg: XXXLSvmRuntimePortReadinessPackage = xxxlSvmRuntimePortReadinessPackage(),
): XXXLSvmRuntimePortReadinessValidationResult {
  const errors: XXXLSvmRuntimePortReadinessErrorCode[] = [];

  if (!pkg.modelLayerComplete || !pkg.readyForX1SvmPort) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.ModelLayerIncomplete);
  }

  if (!pkg.byteLayouts.ok || pkg.byteLayouts.count < 6) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.ByteLayoutsInvalid);
  }

  if (!pkg.serializedVectors.ok || pkg.serializedVectors.vectorCount < 6) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.SerializedVectorsInvalid);
  }

  if (!pkg.decoderHandler.ok) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.DecoderHandlerInvalid);
  }

  if (!pkg.skeleton.ok) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.SkeletonBoundaryInvalid);
  }

  if (
    !pkg.serializedVectors.cpiPrepared ||
    !pkg.serializedVectors.cpiAtomicWithParentTransaction ||
    !pkg.decoderHandler.cpiPrepared ||
    !pkg.decoderHandler.cpiAtomicWithParentTransaction ||
    !pkg.skeleton.cpiPrepared ||
    !pkg.skeleton.cpiAtomicWithParentTransaction
  ) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.CpiBoundaryMissing);
  }

  if (pkg.liveDeployable) {
    errors.push(
      XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.LiveDeployableIncorrectlyClaimed,
    );
  }

  if (pkg.routeActivationAllowed) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.RouteActivationAllowed);
  }

  if (
    pkg.guardianSignatureVerificationInRuntime ||
    pkg.authorityBoundary.guardianSignatureBoundary !==
      XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY
  ) {
    errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.GuardianBoundaryViolated);
  }

  for (const requirement of xxxlSvmRuntimePortImplementationRequirements()) {
    if (!pkg.implementationRequirements.includes(requirement)) {
      errors.push(
        XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.MissingImplementationRequirement,
      );
      break;
    }
  }

  for (const nextStage of xxxlSvmRuntimePortNextStages()) {
    if (!pkg.nextStages.includes(nextStage)) {
      errors.push(XXXL_SVM_RUNTIME_PORT_READINESS_ERROR.MissingNextStage);
      break;
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlSvmRuntimePortImplementationRequirements(): readonly XXXLSvmRuntimePortImplementationRequirement[] {
  return Object.values(XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT);
}

export function xxxlSvmRuntimePortNextStages(): readonly XXXLSvmRuntimePortNextStage[] {
  return Object.values(XXXL_SVM_RUNTIME_PORT_NEXT_STAGE);
}

export function xxxlSvmRuntimePortReadinessProofChain(): readonly XXXLSvmRuntimePortReadinessProof[] {
  return [
    {
      id: "PRODUCTION_BYTE_LAYOUTS",
      result: "Exact offsets, sizes, alignment, discriminators, version fields, and padding are fixed.",
      boundary: "Model byte layouts only; no live account decode yet.",
    },
    {
      id: "X1_SVM_PROGRAM_SKELETON",
      result: "Program ID placeholder, Token Program ID, PDA seeds, account metas, and CPI boundary are fixed.",
      boundary: "Model-only skeleton; not deployable.",
    },
    {
      id: "SVM_SERIALIZED_RUNTIME_VECTORS",
      result: "Canonical account and instruction bytes are generated and validated.",
      boundary: "Deterministic vectors only; no live chain state.",
    },
    {
      id: "SVM_RUNTIME_DECODER_HANDLER_MODEL",
      result: "Bytes decode into handler input and prepare CPI only after valid decoded input.",
      boundary: "Decoder/handler model only; no real SPL Token CPI.",
    },
    {
      id: "GUARDIAN_SIGNATURE_BOUNDARY",
      result: "Runtime consumes Stage 1 authorization result only.",
      boundary: "Guardian signature verification remains outside XXXL runtime.",
    },
    {
      id: "ROUTE_ACTIVATION_BOUNDARY",
      result: "Runtime remains route-aware but this package activates no non-Ethereum route.",
      boundary: "Avalanche remains candidate-only, not activated.",
    },
  ];
}

export function xxxlCanonicalSvmRuntimePortReadinessPackageJson(
  pkg: XXXLSvmRuntimePortReadinessPackage = xxxlSvmRuntimePortReadinessPackage(),
): string {
  return JSON.stringify([
    ["version", pkg.version],
    ["status", pkg.status],
    ["modelLayerComplete", pkg.modelLayerComplete],
    ["readyForX1SvmPort", pkg.readyForX1SvmPort],
    ["liveDeployable", pkg.liveDeployable],
    ["routeActivationAllowed", pkg.routeActivationAllowed],
    [
      "guardianSignatureVerificationInRuntime",
      pkg.guardianSignatureVerificationInRuntime,
    ],
    [
      "byteLayouts",
      [
        ["ok", pkg.byteLayouts.ok],
        ["count", pkg.byteLayouts.count],
        ["kinds", pkg.byteLayouts.kinds],
      ],
    ],
    [
      "serializedVectors",
      [
        ["ok", pkg.serializedVectors.ok],
        ["vectorCount", pkg.serializedVectors.vectorCount],
        ["cpiPrepared", pkg.serializedVectors.cpiPrepared],
        [
          "cpiAtomicWithParentTransaction",
          pkg.serializedVectors.cpiAtomicWithParentTransaction,
        ],
      ],
    ],
    [
      "decoderHandler",
      [
        ["ok", pkg.decoderHandler.ok],
        ["steps", pkg.decoderHandler.steps],
        ["cpiPrepared", pkg.decoderHandler.cpiPrepared],
        [
          "cpiAtomicWithParentTransaction",
          pkg.decoderHandler.cpiAtomicWithParentTransaction,
        ],
      ],
    ],
    [
      "skeleton",
      [
        ["ok", pkg.skeleton.ok],
        ["tokenProgramId", pkg.skeleton.tokenProgramId],
        ["gatewayMintAuthorityPda", pkg.skeleton.gatewayMintAuthorityPda],
        ["accountMetaRoles", pkg.skeleton.accountMetaRoles],
        ["cpiPrepared", pkg.skeleton.cpiPrepared],
        [
          "cpiAtomicWithParentTransaction",
          pkg.skeleton.cpiAtomicWithParentTransaction,
        ],
      ],
    ],
    [
      "authorityBoundary",
      [
        ["programId", pkg.authorityBoundary.programId],
        ["programIdIsPlaceholder", pkg.authorityBoundary.programIdIsPlaceholder],
        ["tokenProgramId", pkg.authorityBoundary.tokenProgramId],
        [
          "gatewayMintAuthorityPdaSeeds",
          pkg.authorityBoundary.gatewayMintAuthorityPdaSeeds,
        ],
        [
          "guardianSignatureBoundary",
          pkg.authorityBoundary.guardianSignatureBoundary,
        ],
      ],
    ],
    ["implementationRequirements", pkg.implementationRequirements],
    ["nextStages", pkg.nextStages],
    [
      "proofChain",
      pkg.proofChain.map((proof) => [
        ["id", proof.id],
        ["result", proof.result],
        ["boundary", proof.boundary],
      ]),
    ],
  ]);
}

export function xxxlSvmRuntimePortReadinessPackageMarkdown(
  pkg: XXXLSvmRuntimePortReadinessPackage = xxxlSvmRuntimePortReadinessPackage(),
): string {
  return [
    "# XXXL SVM Runtime Port Readiness Package",
    "",
    `Status: ${pkg.status}`,
    `Model layer complete: ${pkg.modelLayerComplete ? "yes" : "no"}`,
    `Ready for X1/SVM port: ${pkg.readyForX1SvmPort ? "yes" : "no"}`,
    `Live deployable: ${pkg.liveDeployable ? "yes" : "no"}`,
    "",
    "## Proven model-layer chain",
    ...pkg.proofChain.map(
      (proof) => `- ${proof.id}: ${proof.result} Boundary: ${proof.boundary}`,
    ),
    "",
    "## Remaining implementation requirements",
    ...pkg.implementationRequirements.map((requirement) => `- ${requirement}`),
    "",
    "## Next stages",
    ...pkg.nextStages.map((stage) => `- ${stage}`),
    "",
  ].join("\n");
}

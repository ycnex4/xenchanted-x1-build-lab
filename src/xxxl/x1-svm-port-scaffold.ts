import {
  XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT,
  XXXL_SVM_RUNTIME_PORT_READINESS_STATUS,
  validateXXXLSvmRuntimePortReadinessPackage,
  xxxlSvmRuntimePortImplementationRequirements,
} from "./svm-runtime-port-readiness-package.js";
import {
  XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY,
  XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
} from "./x1-svm-program-skeleton.js";

export const XXXL_X1_SVM_PORT_SCAFFOLD_VERSION = 1;

export const XXXL_X1_SVM_PORT_SCAFFOLD_STATUS = {
  ScaffoldOnly: "X1_SVM_PORT_SCAFFOLD_ONLY_NOT_DEPLOYABLE",
} as const;

export type XXXLX1SvmPortScaffoldStatus =
  (typeof XXXL_X1_SVM_PORT_SCAFFOLD_STATUS)[keyof typeof XXXL_X1_SVM_PORT_SCAFFOLD_STATUS];

export const XXXL_X1_SVM_PORT_SCAFFOLD_FRAMEWORK =
  "NATIVE_SVM_RUST_SCAFFOLD";

export const XXXL_X1_SVM_PORT_SCAFFOLD_ROOT = "programs/xxxl-svm";

export const XXXL_X1_SVM_PORT_SCAFFOLD_ERROR = {
  ReadinessPackageInvalid: "READINESS_PACKAGE_INVALID",
  LiveDeployableClaimed: "LIVE_DEPLOYABLE_CLAIMED",
  RouteActivationAllowed: "ROUTE_ACTIVATION_ALLOWED",
  GuardianBoundaryViolated: "GUARDIAN_BOUNDARY_VIOLATED",
  MissingScaffoldFile: "MISSING_SCAFFOLD_FILE",
  MissingImplementationRequirement: "MISSING_IMPLEMENTATION_REQUIREMENT",
  ProgramIdBoundaryInvalid: "PROGRAM_ID_BOUNDARY_INVALID",
  TokenProgramBoundaryInvalid: "TOKEN_PROGRAM_BOUNDARY_INVALID",
  PdaFixturePlanInvalid: "PDA_FIXTURE_PLAN_INVALID",
  CpiFixturePlanInvalid: "CPI_FIXTURE_PLAN_INVALID",
} as const;

export type XXXLX1SvmPortScaffoldErrorCode =
  (typeof XXXL_X1_SVM_PORT_SCAFFOLD_ERROR)[keyof typeof XXXL_X1_SVM_PORT_SCAFFOLD_ERROR];

export const XXXL_X1_SVM_PORT_SCAFFOLD_FILE_ROLE = {
  CargoManifest: "CARGO_MANIFEST",
  LibraryRoot: "LIBRARY_ROOT",
  Entrypoint: "ENTRYPOINT",
  Processor: "PROCESSOR",
  InstructionDecode: "INSTRUCTION_DECODE",
  StateLayouts: "STATE_LAYOUTS",
  PdaDerivation: "PDA_DERIVATION",
  SplTokenCpiBoundary: "SPL_TOKEN_CPI_BOUNDARY",
  RuntimeValidation: "RUNTIME_VALIDATION",
  ErrorMapping: "ERROR_MAPPING",
} as const;

export type XXXLX1SvmPortScaffoldFileRole =
  (typeof XXXL_X1_SVM_PORT_SCAFFOLD_FILE_ROLE)[keyof typeof XXXL_X1_SVM_PORT_SCAFFOLD_FILE_ROLE];

export type XXXLX1SvmPortScaffoldFile = {
  readonly path: string;
  readonly role: XXXLX1SvmPortScaffoldFileRole;
  readonly description: string;
};

export type XXXLX1SvmPortScaffold = {
  readonly version: typeof XXXL_X1_SVM_PORT_SCAFFOLD_VERSION;
  readonly status: XXXLX1SvmPortScaffoldStatus;
  readonly framework: typeof XXXL_X1_SVM_PORT_SCAFFOLD_FRAMEWORK;
  readonly root: typeof XXXL_X1_SVM_PORT_SCAFFOLD_ROOT;
  readonly readinessPackageStatus: typeof XXXL_SVM_RUNTIME_PORT_READINESS_STATUS.ReadyForPort;
  readonly liveDeployable: false;
  readonly routeActivationAllowed: false;
  readonly guardianSignatureVerificationInRuntime: false;
  readonly files: readonly XXXLX1SvmPortScaffoldFile[];
  readonly programIdBoundary: {
    readonly placeholderProgramId: typeof XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER;
    readonly realProgramIdRequired: true;
    readonly deployTimeDecision: true;
  };
  readonly tokenProgramBoundary: {
    readonly tokenProgramId: typeof XXXL_X1_SVM_TOKEN_PROGRAM_ID;
    readonly realSplTokenProgramRequired: true;
  };
  readonly pdaFixturePlan: {
    readonly seeds: readonly string[];
    readonly realFindProgramAddressRequired: true;
    readonly modelOnlyPdaRejectedForLive: true;
  };
  readonly decodeFixturePlan: {
    readonly accountLayoutCount: 5;
    readonly instructionLayoutCount: 1;
    readonly realAccountDiscriminatorsRequired: true;
    readonly realInstructionDiscriminatorRequired: true;
    readonly realByteParsingRequired: true;
  };
  readonly cpiFixturePlan: {
    readonly realInitializedMintRequired: true;
    readonly realInitializedRecipientAtaRequired: true;
    readonly invokeSignedRequired: true;
    readonly mintAuthorityPdaSignerRequired: true;
  };
  readonly runtimeChecks: {
    readonly accountOwnerChecksRequired: true;
    readonly rentExemptionChecksRequired: true;
    readonly recipientAtaValidationRequired: true;
    readonly clockSourceRequired: true;
  };
  readonly implementationRequirements: readonly string[];
  readonly nonGoals: readonly string[];
};

export type XXXLX1SvmPortScaffoldValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLX1SvmPortScaffoldErrorCode[];
};

export function xxxlX1SvmPortScaffold(): XXXLX1SvmPortScaffold {
  return {
    version: XXXL_X1_SVM_PORT_SCAFFOLD_VERSION,
    status: XXXL_X1_SVM_PORT_SCAFFOLD_STATUS.ScaffoldOnly,
    framework: XXXL_X1_SVM_PORT_SCAFFOLD_FRAMEWORK,
    root: XXXL_X1_SVM_PORT_SCAFFOLD_ROOT,
    readinessPackageStatus:
      XXXL_SVM_RUNTIME_PORT_READINESS_STATUS.ReadyForPort,
    liveDeployable: false,
    routeActivationAllowed: false,
    guardianSignatureVerificationInRuntime: false,
    files: xxxlX1SvmPortScaffoldFiles(),
    programIdBoundary: {
      placeholderProgramId: XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
      realProgramIdRequired: true,
      deployTimeDecision: true,
    },
    tokenProgramBoundary: {
      tokenProgramId: XXXL_X1_SVM_TOKEN_PROGRAM_ID,
      realSplTokenProgramRequired: true,
    },
    pdaFixturePlan: {
      seeds: [...XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS],
      realFindProgramAddressRequired: true,
      modelOnlyPdaRejectedForLive: true,
    },
    decodeFixturePlan: {
      accountLayoutCount: 5,
      instructionLayoutCount: 1,
      realAccountDiscriminatorsRequired: true,
      realInstructionDiscriminatorRequired: true,
      realByteParsingRequired: true,
    },
    cpiFixturePlan: {
      realInitializedMintRequired: true,
      realInitializedRecipientAtaRequired: true,
      invokeSignedRequired: true,
      mintAuthorityPdaSignerRequired: true,
    },
    runtimeChecks: {
      accountOwnerChecksRequired: true,
      rentExemptionChecksRequired: true,
      recipientAtaValidationRequired: true,
      clockSourceRequired: true,
    },
    implementationRequirements: [
      ...xxxlSvmRuntimePortImplementationRequirements(),
    ],
    nonGoals: [
      "NO_DEPLOYMENT",
      "NO_LIVE_TRANSACTION_SUBMISSION",
      "NO_ROUTE_ACTIVATION",
      "NO_AVALANCHE_ACTIVATION",
      "NO_GUARDIAN_SIGNATURE_VERIFICATION_INSIDE_XXXL_RUNTIME",
      "NO_AUTHORITY_FREEZE_EXECUTION",
    ],
  };
}

export function xxxlX1SvmPortScaffoldFiles(): readonly XXXLX1SvmPortScaffoldFile[] {
  return [
    file("programs/xxxl-svm/Cargo.toml", "CARGO_MANIFEST", "Rust/SVM package manifest."),
    file("programs/xxxl-svm/src/lib.rs", "LIBRARY_ROOT", "Program library root and public constants."),
    file("programs/xxxl-svm/src/entrypoint.rs", "ENTRYPOINT", "SVM entrypoint bridge."),
    file("programs/xxxl-svm/src/processor.rs", "PROCESSOR", "Instruction processor scaffold."),
    file("programs/xxxl-svm/src/instruction.rs", "INSTRUCTION_DECODE", "Instruction decode scaffold."),
    file("programs/xxxl-svm/src/state.rs", "STATE_LAYOUTS", "Runtime account layout constants and views."),
    file("programs/xxxl-svm/src/pda.rs", "PDA_DERIVATION", "Gateway mint authority PDA seed boundary."),
    file("programs/xxxl-svm/src/cpi.rs", "SPL_TOKEN_CPI_BOUNDARY", "SPL Token mint_to CPI boundary scaffold."),
    file("programs/xxxl-svm/src/validation.rs", "RUNTIME_VALIDATION", "Owner, rent, and ATA validation scaffold."),
    file("programs/xxxl-svm/src/error.rs", "ERROR_MAPPING", "Runtime error mapping scaffold."),
  ];
}

export function validateXXXLX1SvmPortScaffold(
  scaffold: XXXLX1SvmPortScaffold = xxxlX1SvmPortScaffold(),
): XXXLX1SvmPortScaffoldValidationResult {
  const errors: XXXLX1SvmPortScaffoldErrorCode[] = [];
  const readiness = validateXXXLSvmRuntimePortReadinessPackage();

  if (!readiness.ok) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.ReadinessPackageInvalid);
  }

  if (scaffold.liveDeployable) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.LiveDeployableClaimed);
  }

  if (scaffold.routeActivationAllowed) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.RouteActivationAllowed);
  }

  if (
    scaffold.guardianSignatureVerificationInRuntime ||
    !scaffold.nonGoals.includes(
      "NO_GUARDIAN_SIGNATURE_VERIFICATION_INSIDE_XXXL_RUNTIME",
    )
  ) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.GuardianBoundaryViolated);
  }

  for (const expectedFile of xxxlX1SvmPortScaffoldFiles()) {
    if (!scaffold.files.some((file) => file.path === expectedFile.path)) {
      errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.MissingScaffoldFile);
      break;
    }
  }

  for (const requirement of Object.values(XXXL_SVM_RUNTIME_PORT_IMPLEMENTATION_REQUIREMENT)) {
    if (!scaffold.implementationRequirements.includes(requirement)) {
      errors.push(
        XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.MissingImplementationRequirement,
      );
      break;
    }
  }

  if (
    scaffold.programIdBoundary.placeholderProgramId !==
      XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER ||
    !scaffold.programIdBoundary.realProgramIdRequired ||
    !scaffold.programIdBoundary.deployTimeDecision
  ) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.ProgramIdBoundaryInvalid);
  }

  if (
    scaffold.tokenProgramBoundary.tokenProgramId !== XXXL_X1_SVM_TOKEN_PROGRAM_ID ||
    !scaffold.tokenProgramBoundary.realSplTokenProgramRequired
  ) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.TokenProgramBoundaryInvalid);
  }

  if (
    !sameStrings(
      scaffold.pdaFixturePlan.seeds,
      XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
    ) ||
    !scaffold.pdaFixturePlan.realFindProgramAddressRequired ||
    !scaffold.pdaFixturePlan.modelOnlyPdaRejectedForLive
  ) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.PdaFixturePlanInvalid);
  }

  if (
    !scaffold.cpiFixturePlan.realInitializedMintRequired ||
    !scaffold.cpiFixturePlan.realInitializedRecipientAtaRequired ||
    !scaffold.cpiFixturePlan.invokeSignedRequired ||
    !scaffold.cpiFixturePlan.mintAuthorityPdaSignerRequired
  ) {
    errors.push(XXXL_X1_SVM_PORT_SCAFFOLD_ERROR.CpiFixturePlanInvalid);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalX1SvmPortScaffoldJson(
  scaffold: XXXLX1SvmPortScaffold = xxxlX1SvmPortScaffold(),
): string {
  return JSON.stringify([
    ["version", scaffold.version],
    ["status", scaffold.status],
    ["framework", scaffold.framework],
    ["root", scaffold.root],
    ["readinessPackageStatus", scaffold.readinessPackageStatus],
    ["liveDeployable", scaffold.liveDeployable],
    ["routeActivationAllowed", scaffold.routeActivationAllowed],
    [
      "guardianSignatureVerificationInRuntime",
      scaffold.guardianSignatureVerificationInRuntime,
    ],
    [
      "files",
      scaffold.files.map((file) => [
        ["path", file.path],
        ["role", file.role],
        ["description", file.description],
      ]),
    ],
    [
      "programIdBoundary",
      [
        ["placeholderProgramId", scaffold.programIdBoundary.placeholderProgramId],
        ["realProgramIdRequired", scaffold.programIdBoundary.realProgramIdRequired],
        ["deployTimeDecision", scaffold.programIdBoundary.deployTimeDecision],
      ],
    ],
    [
      "tokenProgramBoundary",
      [
        ["tokenProgramId", scaffold.tokenProgramBoundary.tokenProgramId],
        [
          "realSplTokenProgramRequired",
          scaffold.tokenProgramBoundary.realSplTokenProgramRequired,
        ],
      ],
    ],
    [
      "pdaFixturePlan",
      [
        ["seeds", scaffold.pdaFixturePlan.seeds],
        [
          "realFindProgramAddressRequired",
          scaffold.pdaFixturePlan.realFindProgramAddressRequired,
        ],
        [
          "modelOnlyPdaRejectedForLive",
          scaffold.pdaFixturePlan.modelOnlyPdaRejectedForLive,
        ],
      ],
    ],
    [
      "decodeFixturePlan",
      [
        ["accountLayoutCount", scaffold.decodeFixturePlan.accountLayoutCount],
        [
          "instructionLayoutCount",
          scaffold.decodeFixturePlan.instructionLayoutCount,
        ],
        [
          "realAccountDiscriminatorsRequired",
          scaffold.decodeFixturePlan.realAccountDiscriminatorsRequired,
        ],
        [
          "realInstructionDiscriminatorRequired",
          scaffold.decodeFixturePlan.realInstructionDiscriminatorRequired,
        ],
        ["realByteParsingRequired", scaffold.decodeFixturePlan.realByteParsingRequired],
      ],
    ],
    [
      "cpiFixturePlan",
      [
        [
          "realInitializedMintRequired",
          scaffold.cpiFixturePlan.realInitializedMintRequired,
        ],
        [
          "realInitializedRecipientAtaRequired",
          scaffold.cpiFixturePlan.realInitializedRecipientAtaRequired,
        ],
        ["invokeSignedRequired", scaffold.cpiFixturePlan.invokeSignedRequired],
        [
          "mintAuthorityPdaSignerRequired",
          scaffold.cpiFixturePlan.mintAuthorityPdaSignerRequired,
        ],
      ],
    ],
    [
      "runtimeChecks",
      [
        [
          "accountOwnerChecksRequired",
          scaffold.runtimeChecks.accountOwnerChecksRequired,
        ],
        [
          "rentExemptionChecksRequired",
          scaffold.runtimeChecks.rentExemptionChecksRequired,
        ],
        [
          "recipientAtaValidationRequired",
          scaffold.runtimeChecks.recipientAtaValidationRequired,
        ],
        ["clockSourceRequired", scaffold.runtimeChecks.clockSourceRequired],
      ],
    ],
    ["implementationRequirements", scaffold.implementationRequirements],
    ["nonGoals", scaffold.nonGoals],
  ]);
}

export function xxxlX1SvmPortScaffoldMarkdown(
  scaffold: XXXLX1SvmPortScaffold = xxxlX1SvmPortScaffold(),
): string {
  return [
    "# XXXL X1/SVM Port Scaffold",
    "",
    `Status: ${scaffold.status}`,
    `Framework: ${scaffold.framework}`,
    `Root: ${scaffold.root}`,
    `Live deployable: ${scaffold.liveDeployable ? "yes" : "no"}`,
    "",
    "## Files",
    ...scaffold.files.map((file) => `- ${file.path}: ${file.role}`),
    "",
    "## Boundaries",
    `- Program ID placeholder: ${scaffold.programIdBoundary.placeholderProgramId}`,
    `- Token Program ID: ${scaffold.tokenProgramBoundary.tokenProgramId}`,
    `- PDA seeds: ${scaffold.pdaFixturePlan.seeds.join(", ")}`,
    `- Guardian boundary: ${XXXL_X1_SVM_GUARDIAN_SIGNATURE_BOUNDARY}`,
    "",
    "## Non-goals",
    ...scaffold.nonGoals.map((goal) => `- ${goal}`),
    "",
  ].join("\n");
}

function file(
  path: string,
  role: XXXLX1SvmPortScaffoldFileRole,
  description: string,
): XXXLX1SvmPortScaffoldFile {
  return {
    path,
    role,
    description,
  };
}

function sameStrings(left: readonly string[], right: readonly string[]): boolean {
  return (
    left.length === right.length &&
    left.every((item, index) => item === right[index])
  );
}

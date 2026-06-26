import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_INSTRUCTION,
  type XXXLRuntimeAccountKind,
  type XXXLRuntimeInstruction,
} from "./runtime-candidate.js";

export const XXXL_RUNTIME_SERIALIZATION_BOUNDARY_VERSION = 1;

export const XXXL_RUNTIME_SERIALIZATION_ENCODING = {
  CanonicalBinaryV1: "CANONICAL_BINARY_V1",
} as const;

export type XXXLRuntimeSerializationEncoding =
  (typeof XXXL_RUNTIME_SERIALIZATION_ENCODING)[keyof typeof XXXL_RUNTIME_SERIALIZATION_ENCODING];

export const XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY = {
  GatewayMintAuthorityPda: "GATEWAY_MINT_AUTHORITY_PDA",
} as const;

export type XXXLRuntimeMintAuthorityPdaStrategy =
  (typeof XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY)[keyof typeof XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY];

export const XXXL_RUNTIME_AUTHORITY_SURFACE = {
  ProgramUpgradeAuthority: "PROGRAM_UPGRADE_AUTHORITY",
  SplTokenMintAuthority: "SPL_TOKEN_MINT_AUTHORITY",
} as const;

export type XXXLRuntimeAuthoritySurface =
  (typeof XXXL_RUNTIME_AUTHORITY_SURFACE)[keyof typeof XXXL_RUNTIME_AUTHORITY_SURFACE];

export const XXXL_RUNTIME_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY = {
  Stage1AuthorizationResultOnly: "STAGE_1_AUTHORIZATION_RESULT_ONLY",
  RuntimeEd25519Verification: "RUNTIME_ED25519_VERIFICATION",
} as const;

export type XXXLRuntimeGuardianSignatureVerificationBoundary =
  (typeof XXXL_RUNTIME_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY)[keyof typeof XXXL_RUNTIME_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY];

export const XXXL_RUNTIME_MANDATORY_SERIALIZED_ACCOUNT_KINDS: readonly XXXLRuntimeAccountKind[] =
  [
    XXXL_RUNTIME_ACCOUNT_KIND.MintState,
    XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
    XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
    XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
    XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
  ];

export const XXXL_RUNTIME_MANDATORY_SERIALIZED_INSTRUCTIONS: readonly XXXLRuntimeInstruction[] =
  [XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint];

export const XXXL_RUNTIME_MANDATORY_AUTHORITY_SURFACES: readonly XXXLRuntimeAuthoritySurface[] =
  [
    XXXL_RUNTIME_AUTHORITY_SURFACE.ProgramUpgradeAuthority,
    XXXL_RUNTIME_AUTHORITY_SURFACE.SplTokenMintAuthority,
  ];

export const XXXL_RUNTIME_GATEWAY_MINT_AUTHORITY_PDA_SEEDS = [
  "xxxl",
  "gateway-mint-authority",
  "v1",
] as const;

export const XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR = {
  UnsupportedVersion: "UNSUPPORTED_VERSION",
  WrongAccountEncoding: "WRONG_ACCOUNT_ENCODING",
  WrongInstructionEncoding: "WRONG_INSTRUCTION_ENCODING",
  MissingSerializedAccountKind: "MISSING_SERIALIZED_ACCOUNT_KIND",
  DuplicateSerializedAccountKind: "DUPLICATE_SERIALIZED_ACCOUNT_KIND",
  MissingSerializedInstruction: "MISSING_SERIALIZED_INSTRUCTION",
  DuplicateSerializedInstruction: "DUPLICATE_SERIALIZED_INSTRUCTION",
  MissingMintAuthorityPda: "MISSING_MINT_AUTHORITY_PDA",
  WrongMintAuthorityPdaStrategy: "WRONG_MINT_AUTHORITY_PDA_STRATEGY",
  MissingMintAuthorityPdaSeed: "MISSING_MINT_AUTHORITY_PDA_SEED",
  MintAuthorityPdaDoesNotSignMintToCpi:
    "MINT_AUTHORITY_PDA_DOES_NOT_SIGN_MINT_TO_CPI",
  MissingAuthoritySurface: "MISSING_AUTHORITY_SURFACE",
  DuplicateAuthoritySurface: "DUPLICATE_AUTHORITY_SURFACE",
  MissingCpiAtomicityNote: "MISSING_CPI_ATOMICITY_NOTE",
  UpgradeAuthorityNotSeparatedFromMintAuthority:
    "UPGRADE_AUTHORITY_NOT_SEPARATED_FROM_MINT_AUTHORITY",
  FreezeDoesNotCoverProgramUpgradeAuthority:
    "FREEZE_DOES_NOT_COVER_PROGRAM_UPGRADE_AUTHORITY",
  FreezeDoesNotCoverSplTokenMintAuthority:
    "FREEZE_DOES_NOT_COVER_SPL_TOKEN_MINT_AUTHORITY",
  WrongGuardianSignatureVerificationBoundary:
    "WRONG_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY",
  MissingSupplyAuditFunctionShape: "MISSING_SUPPLY_AUDIT_FUNCTION_SHAPE",
  MissingDeterministicVectorPlan: "MISSING_DETERMINISTIC_VECTOR_PLAN",
} as const;

export type XXXLRuntimeSerializationBoundaryErrorCode =
  (typeof XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR)[keyof typeof XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR];

export type XXXLRuntimeMintAuthorityPdaPlan = {
  readonly strategy: XXXLRuntimeMintAuthorityPdaStrategy;
  readonly seeds: readonly string[];
  readonly signsSplTokenMintToCpi: boolean;
};

export type XXXLRuntimeSerializationBoundaryCandidate = {
  readonly version: number;
  readonly accountEncoding: XXXLRuntimeSerializationEncoding;
  readonly instructionEncoding: XXXLRuntimeSerializationEncoding;
  readonly accountKinds: readonly XXXLRuntimeAccountKind[];
  readonly instructionKinds: readonly XXXLRuntimeInstruction[];
  readonly mintAuthorityPda: XXXLRuntimeMintAuthorityPdaPlan;
  readonly authoritySurfaces: readonly XXXLRuntimeAuthoritySurface[];
  readonly cpiAtomicityNote: boolean;
  readonly programUpgradeAuthoritySeparatedFromMintAuthority: boolean;
  readonly authorityFreezeCoversProgramUpgradeAuthority: boolean;
  readonly authorityFreezeCoversSplTokenMintAuthority: boolean;
  readonly guardianSignatureVerificationBoundary: XXXLRuntimeGuardianSignatureVerificationBoundary;
  readonly supplyAuditFunctionPlanned: boolean;
  readonly deterministicVectorsPlanned: boolean;
};

export type XXXLRuntimeSerializationBoundaryValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimeSerializationBoundaryErrorCode[];
};

export type XXXLRuntimeSupplyAuditFunctionShape = {
  readonly name: "auditGenesisSupplyInvariant";
  readonly readonlyOnly: true;
  readonly inputs: readonly string[];
  readonly invariant: string;
};

export type XXXLRuntimeSerializationVectorPlanItem = {
  readonly vectorId: string;
  readonly target: string;
  readonly purpose: string;
};

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

export function validateXXXLRuntimeSerializationBoundary(
  candidate: XXXLRuntimeSerializationBoundaryCandidate,
): XXXLRuntimeSerializationBoundaryValidationResult {
  const errors: XXXLRuntimeSerializationBoundaryErrorCode[] = [];

  if (candidate.version !== XXXL_RUNTIME_SERIALIZATION_BOUNDARY_VERSION) {
    errors.push(XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.UnsupportedVersion);
  }

  if (
    candidate.accountEncoding !==
    XXXL_RUNTIME_SERIALIZATION_ENCODING.CanonicalBinaryV1
  ) {
    errors.push(XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.WrongAccountEncoding);
  }

  if (
    candidate.instructionEncoding !==
    XXXL_RUNTIME_SERIALIZATION_ENCODING.CanonicalBinaryV1
  ) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.WrongInstructionEncoding,
    );
  }

  const accountKinds = new Set(candidate.accountKinds);
  for (const kind of XXXL_RUNTIME_MANDATORY_SERIALIZED_ACCOUNT_KINDS) {
    if (!accountKinds.has(kind)) {
      errors.push(
        XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
          .MissingSerializedAccountKind,
      );
    }
  }

  if (hasDuplicates(candidate.accountKinds)) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .DuplicateSerializedAccountKind,
    );
  }

  const instructionKinds = new Set(candidate.instructionKinds);
  for (const instruction of XXXL_RUNTIME_MANDATORY_SERIALIZED_INSTRUCTIONS) {
    if (!instructionKinds.has(instruction)) {
      errors.push(
        XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingSerializedInstruction,
      );
    }
  }

  if (hasDuplicates(candidate.instructionKinds)) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .DuplicateSerializedInstruction,
    );
  }

  if (!candidate.mintAuthorityPda) {
    errors.push(XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingMintAuthorityPda);
  } else {
    if (
      candidate.mintAuthorityPda.strategy !==
      XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY.GatewayMintAuthorityPda
    ) {
      errors.push(
        XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.WrongMintAuthorityPdaStrategy,
      );
    }

    const pdaSeeds = new Set(candidate.mintAuthorityPda.seeds);
    for (const seed of XXXL_RUNTIME_GATEWAY_MINT_AUTHORITY_PDA_SEEDS) {
      if (!pdaSeeds.has(seed)) {
        errors.push(
          XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingMintAuthorityPdaSeed,
        );
      }
    }

    if (!candidate.mintAuthorityPda.signsSplTokenMintToCpi) {
      errors.push(
        XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
          .MintAuthorityPdaDoesNotSignMintToCpi,
      );
    }
  }

  const authoritySurfaces = new Set(candidate.authoritySurfaces);
  for (const surface of XXXL_RUNTIME_MANDATORY_AUTHORITY_SURFACES) {
    if (!authoritySurfaces.has(surface)) {
      errors.push(XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingAuthoritySurface);
    }
  }

  if (hasDuplicates(candidate.authoritySurfaces)) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.DuplicateAuthoritySurface,
    );
  }

  if (!candidate.cpiAtomicityNote) {
    errors.push(XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingCpiAtomicityNote);
  }

  if (!candidate.programUpgradeAuthoritySeparatedFromMintAuthority) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .UpgradeAuthorityNotSeparatedFromMintAuthority,
    );
  }

  if (!candidate.authorityFreezeCoversProgramUpgradeAuthority) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .FreezeDoesNotCoverProgramUpgradeAuthority,
    );
  }

  if (!candidate.authorityFreezeCoversSplTokenMintAuthority) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .FreezeDoesNotCoverSplTokenMintAuthority,
    );
  }

  if (
    candidate.guardianSignatureVerificationBoundary !==
    XXXL_RUNTIME_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY
      .Stage1AuthorizationResultOnly
  ) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .WrongGuardianSignatureVerificationBoundary,
    );
  }

  if (!candidate.supplyAuditFunctionPlanned) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .MissingSupplyAuditFunctionShape,
    );
  }

  if (!candidate.deterministicVectorsPlanned) {
    errors.push(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .MissingDeterministicVectorPlan,
    );
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlRuntimeSupplyAuditFunctionShape(): XXXLRuntimeSupplyAuditFunctionShape {
  return {
    name: "auditGenesisSupplyInvariant",
    readonlyOnly: true,
    inputs: [
      "mintState.totalSupply",
      "splTokenMint.supply",
      "processedEvents[].consumed",
      "processedEvents[].consumedAmount",
    ],
    invariant:
      "mintState.totalSupply == splTokenMint.supply == sum(processedEvents[].consumedAmount where consumed == true)",
  };
}

export function xxxlRuntimeDeterministicSerializationVectorPlan(): readonly XXXLRuntimeSerializationVectorPlanItem[] {
  return [
    {
      vectorId: "XXXL_RUNTIME_MINT_STATE_ACCOUNT_V1",
      target: XXXL_RUNTIME_ACCOUNT_KIND.MintState,
      purpose: "Canonical account discriminator, version, mint id, supply, and authority fields.",
    },
    {
      vectorId: "XXXL_RUNTIME_GATEWAY_CONFIG_ACCOUNT_V1",
      target: XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
      purpose: "Canonical route, source chain, target mint, guardian set, quorum, and finality fields.",
    },
    {
      vectorId: "XXXL_RUNTIME_GUARDIAN_SET_ACCOUNT_V1",
      target: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
      purpose: "Canonical guardian set id, public keys, quorum, and status fields.",
    },
    {
      vectorId: "XXXL_RUNTIME_PROCESSED_EVENT_ACCOUNT_V1",
      target: XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
      purpose: "Canonical replay-protection event key, route id, recipient, amount, and consumed flag.",
    },
    {
      vectorId: "XXXL_RUNTIME_RECIPIENT_BALANCE_ACCOUNT_V1",
      target: XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
      purpose: "Canonical recipient owner, mint id, and balance fields.",
    },
    {
      vectorId: "XXXL_RUNTIME_CONSUME_GATEWAY_MINT_INSTRUCTION_V1",
      target: XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
      purpose: "Canonical instruction discriminator, account metas, route id, guardian set id, event key, recipient, and amount.",
    },
  ];
}

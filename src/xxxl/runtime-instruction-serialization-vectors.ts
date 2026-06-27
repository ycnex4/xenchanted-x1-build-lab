import {
  XXXL_RUNTIME_INSTRUCTION,
  type XXXLRuntimeInstruction,
} from "./runtime-candidate.js";

export const XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_VERSION = 1;

export const XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ENCODING = {
  CanonicalBinaryV1: "CANONICAL_BINARY_V1",
} as const;

export type XXXLRuntimeInstructionSerializationEncoding =
  (typeof XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ENCODING)[keyof typeof XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ENCODING];

export const XXXL_RUNTIME_INSTRUCTION_DISCRIMINATOR = {
  ConsumeGatewayMint: "XXXL_CONSUME_GATEWAY_MINT_V1",
} as const;

export const XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE = {
  InstructionKind: "INSTRUCTION_KIND",
  VersionU16: "VERSION_U16",
  Utf8String: "UTF8_STRING",
  U128DecimalString: "U128_DECIMAL_STRING",
} as const;

export type XXXLRuntimeInstructionFieldType =
  (typeof XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE)[keyof typeof XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE];

export const XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE = {
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

export type XXXLRuntimeInstructionAccountMetaRole =
  (typeof XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE)[keyof typeof XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE];

export type XXXLRuntimeInstructionAccountMeta = {
  readonly role: XXXLRuntimeInstructionAccountMetaRole;
  readonly position: number;
  readonly writable: boolean;
  readonly parentInstructionSigner: boolean;
  readonly cpiSigner: boolean;
  readonly description: string;
};

export type XXXLRuntimeInstructionSerializationField = {
  readonly name: string;
  readonly position: number;
  readonly fieldType: XXXLRuntimeInstructionFieldType;
};

export type XXXLRuntimeInstructionSerializationLayout = {
  readonly instruction: XXXLRuntimeInstruction;
  readonly version: number;
  readonly encoding: XXXLRuntimeInstructionSerializationEncoding;
  readonly discriminator: string;
  readonly accountMetas: readonly XXXLRuntimeInstructionAccountMeta[];
  readonly fields: readonly XXXLRuntimeInstructionSerializationField[];
};

export type XXXLRuntimeConsumeGatewayMintSerializableInstruction = {
  readonly instruction: typeof XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint;
  readonly version: number;
  readonly routeId: string;
  readonly guardianSetId: string;
  readonly mintId: string;
  readonly canonicalEventKey: string;
  readonly recipient: string;
  readonly amount: bigint;
};

export type XXXLRuntimeInstructionSerializationVector = {
  readonly vectorId: string;
  readonly instruction: XXXLRuntimeInstruction;
  readonly layoutVersion: number;
  readonly encoding: XXXLRuntimeInstructionSerializationEncoding;
  readonly discriminator: string;
  readonly accountMetaOrder: readonly XXXLRuntimeInstructionAccountMetaRole[];
  readonly fieldOrder: readonly string[];
  readonly data: XXXLRuntimeConsumeGatewayMintSerializableInstruction;
  readonly canonicalJson: string;
};

export const XXXL_RUNTIME_MANDATORY_SERIALIZED_INSTRUCTION_KINDS: readonly XXXLRuntimeInstruction[] =
  [XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint];

export const XXXL_RUNTIME_CONSUME_GATEWAY_MINT_ACCOUNT_META_ORDER: readonly XXXLRuntimeInstructionAccountMetaRole[] =
  [
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintState,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.GatewayConfig,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.GuardianSet,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.ProcessedEvent,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientBalance,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.SplTokenMint,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientTokenAccount,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda,
    XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram,
  ];

export const XXXL_RUNTIME_CONSUME_GATEWAY_MINT_FIELD_ORDER = [
  "instruction",
  "version",
  "routeId",
  "guardianSetId",
  "mintId",
  "canonicalEventKey",
  "recipient",
  "amount",
] as const;

export const XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR = {
  MissingLayout: "MISSING_LAYOUT",
  DuplicateLayout: "DUPLICATE_LAYOUT",
  UnsupportedLayoutVersion: "UNSUPPORTED_LAYOUT_VERSION",
  WrongEncoding: "WRONG_ENCODING",
  WrongDiscriminator: "WRONG_DISCRIMINATOR",
  MissingInstructionField: "MISSING_INSTRUCTION_FIELD",
  MissingVersionField: "MISSING_VERSION_FIELD",
  MissingField: "MISSING_FIELD",
  DuplicateField: "DUPLICATE_FIELD",
  WrongFieldOrder: "WRONG_FIELD_ORDER",
  MissingAccountMeta: "MISSING_ACCOUNT_META",
  DuplicateAccountMeta: "DUPLICATE_ACCOUNT_META",
  WrongAccountMetaOrder: "WRONG_ACCOUNT_META_ORDER",
  WrongWritableFlag: "WRONG_WRITABLE_FLAG",
  WrongParentSignerFlag: "WRONG_PARENT_SIGNER_FLAG",
  MintAuthorityPdaDoesNotSignCpi: "MINT_AUTHORITY_PDA_DOES_NOT_SIGN_CPI",
  TokenProgramMustBeReadonly: "TOKEN_PROGRAM_MUST_BE_READONLY",
  MissingVector: "MISSING_VECTOR",
  DuplicateVector: "DUPLICATE_VECTOR",
  VectorLayoutMismatch: "VECTOR_LAYOUT_MISMATCH",
  WrongCanonicalJson: "WRONG_CANONICAL_JSON",
} as const;

export type XXXLRuntimeInstructionSerializationErrorCode =
  (typeof XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR)[keyof typeof XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR];

export type XXXLRuntimeInstructionSerializationValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimeInstructionSerializationErrorCode[];
};

function field(
  name: string,
  position: number,
  fieldType: XXXLRuntimeInstructionFieldType,
): XXXLRuntimeInstructionSerializationField {
  return {
    name,
    position,
    fieldType,
  };
}

function accountMeta(
  role: XXXLRuntimeInstructionAccountMetaRole,
  position: number,
  writable: boolean,
  parentInstructionSigner: boolean,
  cpiSigner: boolean,
  description: string,
): XXXLRuntimeInstructionAccountMeta {
  return {
    role,
    position,
    writable,
    parentInstructionSigner,
    cpiSigner,
    description,
  };
}

function expectedDiscriminator(instruction: XXXLRuntimeInstruction): string {
  switch (instruction) {
    case XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint:
      return XXXL_RUNTIME_INSTRUCTION_DISCRIMINATOR.ConsumeGatewayMint;
  }
}

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

export function xxxlRuntimeConsumeGatewayMintExpectedAccountMetas(): readonly XXXLRuntimeInstructionAccountMeta[] {
  return [
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintState,
      0,
      true,
      false,
      false,
      "Writable XXXL mint state mirror used for total supply and authority mode checks.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.GatewayConfig,
      1,
      false,
      false,
      false,
      "Read-only gateway route configuration.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.GuardianSet,
      2,
      false,
      false,
      false,
      "Read-only guardian set reference for Stage 1 authorization context.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.ProcessedEvent,
      3,
      true,
      false,
      false,
      "Writable replay-protection account for the consumed canonical event key.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientBalance,
      4,
      true,
      false,
      false,
      "Writable runtime balance mirror for the X1 recipient.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.SplTokenMint,
      5,
      true,
      false,
      false,
      "Writable SPL Token mint account whose supply changes through mint_to CPI.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientTokenAccount,
      6,
      true,
      false,
      false,
      "Writable SPL Token account receiving XXXL.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda,
      7,
      false,
      false,
      true,
      "Program-derived mint authority used as CPI signer for SPL Token mint_to.",
    ),
    accountMeta(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram,
      8,
      false,
      false,
      false,
      "Read-only SPL Token program account.",
    ),
  ];
}

export function xxxlRuntimeInstructionSerializationLayouts(): readonly XXXLRuntimeInstructionSerializationLayout[] {
  return [
    {
      instruction: XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
      version: XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_VERSION,
      encoding:
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ENCODING.CanonicalBinaryV1,
      discriminator:
        XXXL_RUNTIME_INSTRUCTION_DISCRIMINATOR.ConsumeGatewayMint,
      accountMetas: xxxlRuntimeConsumeGatewayMintExpectedAccountMetas(),
      fields: [
        field(
          "instruction",
          0,
          XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.InstructionKind,
        ),
        field("version", 1, XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.VersionU16),
        field("routeId", 2, XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.Utf8String),
        field(
          "guardianSetId",
          3,
          XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.Utf8String,
        ),
        field("mintId", 4, XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.Utf8String),
        field(
          "canonicalEventKey",
          5,
          XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.Utf8String,
        ),
        field("recipient", 6, XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.Utf8String),
        field(
          "amount",
          7,
          XXXL_RUNTIME_INSTRUCTION_FIELD_TYPE.U128DecimalString,
        ),
      ],
    },
  ];
}

export function validateXXXLRuntimeInstructionSerializationLayouts(
  layouts: readonly XXXLRuntimeInstructionSerializationLayout[],
): XXXLRuntimeInstructionSerializationValidationResult {
  const errors: XXXLRuntimeInstructionSerializationErrorCode[] = [];
  const layoutKinds = new Set<XXXLRuntimeInstruction>();

  for (const layout of layouts) {
    if (layoutKinds.has(layout.instruction)) {
      errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.DuplicateLayout);
    }
    layoutKinds.add(layout.instruction);
  }

  for (const instruction of XXXL_RUNTIME_MANDATORY_SERIALIZED_INSTRUCTION_KINDS) {
    if (!layoutKinds.has(instruction)) {
      errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.MissingLayout);
    }
  }

  const expectedMetas = xxxlRuntimeConsumeGatewayMintExpectedAccountMetas();

  for (const layout of layouts) {
    if (layout.version !== XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_VERSION) {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.UnsupportedLayoutVersion,
      );
    }

    if (
      layout.encoding !==
      XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ENCODING.CanonicalBinaryV1
    ) {
      errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongEncoding);
    }

    if (layout.discriminator !== expectedDiscriminator(layout.instruction)) {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongDiscriminator,
      );
    }

    const fieldNames = layout.fields.map((item) => item.name);

    if (fieldNames[0] !== "instruction") {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.MissingInstructionField,
      );
    }

    if (fieldNames[1] !== "version") {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.MissingVersionField,
      );
    }

    if (hasDuplicates(fieldNames)) {
      errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.DuplicateField);
    }

    for (const expected of XXXL_RUNTIME_CONSUME_GATEWAY_MINT_FIELD_ORDER) {
      if (!fieldNames.includes(expected)) {
        errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.MissingField);
      }
    }

    const positionsAreCanonical = layout.fields.every(
      (item, index) => item.position === index,
    );

    if (
      !positionsAreCanonical ||
      fieldNames.join("|") !==
        XXXL_RUNTIME_CONSUME_GATEWAY_MINT_FIELD_ORDER.join("|")
    ) {
      errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongFieldOrder);
    }

    const roles = layout.accountMetas.map((item) => item.role);

    for (const expected of XXXL_RUNTIME_CONSUME_GATEWAY_MINT_ACCOUNT_META_ORDER) {
      if (!roles.includes(expected)) {
        errors.push(
          XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.MissingAccountMeta,
        );
      }
    }

    if (hasDuplicates(roles)) {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.DuplicateAccountMeta,
      );
    }

    const metaPositionsAreCanonical = layout.accountMetas.every(
      (item, index) => item.position === index,
    );

    if (
      !metaPositionsAreCanonical ||
      roles.join("|") !==
        XXXL_RUNTIME_CONSUME_GATEWAY_MINT_ACCOUNT_META_ORDER.join("|")
    ) {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongAccountMetaOrder,
      );
    }

    for (const expected of expectedMetas) {
      const actual = layout.accountMetas.find(
        (item) => item.role === expected.role,
      );

      if (!actual) {
        continue;
      }

      if (actual.writable !== expected.writable) {
        errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongWritableFlag);
      }

      if (actual.parentInstructionSigner !== expected.parentInstructionSigner) {
        errors.push(
          XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongParentSignerFlag,
        );
      }

      if (
        actual.role === XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda &&
        !actual.cpiSigner
      ) {
        errors.push(
          XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR
            .MintAuthorityPdaDoesNotSignCpi,
        );
      }

      if (
        actual.role === XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram &&
        (actual.writable || actual.parentInstructionSigner || actual.cpiSigner)
      ) {
        errors.push(
          XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.TokenProgramMustBeReadonly,
        );
      }
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

function normalizeRuntimeInstructionValue(value: unknown): unknown {
  if (typeof value === "bigint") {
    return value.toString();
  }

  return value;
}

export function xxxlCanonicalRuntimeInstructionJson(
  layout: XXXLRuntimeInstructionSerializationLayout,
  data: XXXLRuntimeConsumeGatewayMintSerializableInstruction,
): string {
  const record = data as unknown as Record<string, unknown>;

  return JSON.stringify(
    layout.fields.map((item) => [
      item.name,
      normalizeRuntimeInstructionValue(record[item.name]),
    ]),
  );
}

function sampleConsumeGatewayMintInstruction(): XXXLRuntimeConsumeGatewayMintSerializableInstruction {
  return {
    instruction: XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
    version: XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_VERSION,
    routeId: "ETHEREUM_XNTD_TO_X1_XXXL",
    guardianSetId: "guardian-set-1",
    mintId: "xxxl-mint-core",
    canonicalEventKey:
      "ethereum:1:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:0",
    recipient: "x1-recipient-1",
    amount: 1_000_000_000_000n,
  };
}

export function xxxlRuntimeInstructionSerializationVectors(): readonly XXXLRuntimeInstructionSerializationVector[] {
  return xxxlRuntimeInstructionSerializationLayouts().map((layout) => {
    const data = sampleConsumeGatewayMintInstruction();

    return {
      vectorId: "XXXL_RUNTIME_CONSUME_GATEWAY_MINT_INSTRUCTION_V1",
      instruction: layout.instruction,
      layoutVersion: layout.version,
      encoding: layout.encoding,
      discriminator: layout.discriminator,
      accountMetaOrder: layout.accountMetas.map((item) => item.role),
      fieldOrder: layout.fields.map((item) => item.name),
      data,
      canonicalJson: xxxlCanonicalRuntimeInstructionJson(layout, data),
    };
  });
}

export function validateXXXLRuntimeInstructionSerializationVectors(
  layouts: readonly XXXLRuntimeInstructionSerializationLayout[],
  vectors: readonly XXXLRuntimeInstructionSerializationVector[],
): XXXLRuntimeInstructionSerializationValidationResult {
  const errors: XXXLRuntimeInstructionSerializationErrorCode[] = [
    ...validateXXXLRuntimeInstructionSerializationLayouts(layouts).errors,
  ];

  const vectorsByInstruction = new Map<
    XXXLRuntimeInstruction,
    XXXLRuntimeInstructionSerializationVector
  >();

  for (const vector of vectors) {
    if (vectorsByInstruction.has(vector.instruction)) {
      errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.DuplicateVector);
    }
    vectorsByInstruction.set(vector.instruction, vector);
  }

  for (const instruction of XXXL_RUNTIME_MANDATORY_SERIALIZED_INSTRUCTION_KINDS) {
    if (!vectorsByInstruction.has(instruction)) {
      errors.push(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.MissingVector);
    }
  }

  for (const vector of vectors) {
    const layout = layouts.find((item) => item.instruction === vector.instruction);

    if (!layout || vector.data.instruction !== vector.instruction) {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.VectorLayoutMismatch,
      );
      continue;
    }

    const expectedMetaOrder = layout.accountMetas.map((item) => item.role);
    const expectedFieldOrder = layout.fields.map((item) => item.name);

    if (
      vector.layoutVersion !== layout.version ||
      vector.encoding !== layout.encoding ||
      vector.discriminator !== layout.discriminator ||
      vector.accountMetaOrder.join("|") !== expectedMetaOrder.join("|") ||
      vector.fieldOrder.join("|") !== expectedFieldOrder.join("|")
    ) {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.VectorLayoutMismatch,
      );
    }

    const canonicalJson = xxxlCanonicalRuntimeInstructionJson(
      layout,
      vector.data,
    );

    if (canonicalJson !== vector.canonicalJson) {
      errors.push(
        XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongCanonicalJson,
      );
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
} from "./program-v1.js";
import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_AUTHORITY_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_ROUTE_STATUS,
  type XXXLGatewayConfigAccount,
  type XXXLGuardianSetAccount,
  type XXXLMintStateAccount,
  type XXXLProcessedEventAccount,
  type XXXLRecipientBalanceAccount,
  type XXXLRuntimeAccountKind,
} from "./runtime-candidate.js";

export const XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION = 1;

export const XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING = {
  CanonicalBinaryV1: "CANONICAL_BINARY_V1",
} as const;

export type XXXLRuntimeAccountSerializationEncoding =
  (typeof XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING)[keyof typeof XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING];

export const XXXL_RUNTIME_ACCOUNT_FIELD_TYPE = {
  AccountKind: "ACCOUNT_KIND",
  VersionU16: "VERSION_U16",
  U8: "U8",
  U16: "U16",
  U64DecimalString: "U64_DECIMAL_STRING",
  U128DecimalString: "U128_DECIMAL_STRING",
  Boolean: "BOOLEAN",
  Utf8String: "UTF8_STRING",
  Utf8StringArray: "UTF8_STRING_ARRAY",
  EnumString: "ENUM_STRING",
} as const;

export type XXXLRuntimeAccountFieldType =
  (typeof XXXL_RUNTIME_ACCOUNT_FIELD_TYPE)[keyof typeof XXXL_RUNTIME_ACCOUNT_FIELD_TYPE];

export const XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR = {
  MintState: "XXXL_MINT_STATE_V1",
  GatewayConfig: "XXXL_GATEWAY_CONFIG_V1",
  GuardianSet: "XXXL_GUARDIAN_SET_V1",
  ProcessedEvent: "XXXL_PROCESSED_EVENT_V1",
  RecipientBalance: "XXXL_RECIPIENT_BALANCE_V1",
} as const;

export type XXXLRuntimeSerializableAccount =
  | XXXLMintStateAccount
  | XXXLGatewayConfigAccount
  | XXXLGuardianSetAccount
  | XXXLProcessedEventAccount
  | XXXLRecipientBalanceAccount;

export type XXXLRuntimeAccountSerializationField = {
  readonly name: string;
  readonly position: number;
  readonly fieldType: XXXLRuntimeAccountFieldType;
};

export type XXXLRuntimeAccountSerializationLayout = {
  readonly accountKind: XXXLRuntimeAccountKind;
  readonly version: number;
  readonly encoding: XXXLRuntimeAccountSerializationEncoding;
  readonly discriminator: string;
  readonly fields: readonly XXXLRuntimeAccountSerializationField[];
};

export type XXXLRuntimeAccountSerializationVector = {
  readonly vectorId: string;
  readonly accountKind: XXXLRuntimeAccountKind;
  readonly layoutVersion: number;
  readonly encoding: XXXLRuntimeAccountSerializationEncoding;
  readonly discriminator: string;
  readonly fieldOrder: readonly string[];
  readonly account: XXXLRuntimeSerializableAccount;
  readonly canonicalJson: string;
};

export const XXXL_RUNTIME_ACCOUNT_SERIALIZATION_FIELD_ORDER: Record<
  XXXLRuntimeAccountKind,
  readonly string[]
> = {
  [XXXL_RUNTIME_ACCOUNT_KIND.MintState]: [
    "kind",
    "version",
    "mintId",
    "decimals",
    "totalSupply",
    "authorityMode",
    "upgradeAuthorityStatus",
  ],
  [XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig]: [
    "kind",
    "version",
    "routeId",
    "sourceChainId",
    "sourceToken",
    "targetMintToken",
    "targetX1NetworkId",
    "targetMintCoreId",
    "guardianSetId",
    "quorumThreshold",
    "finalityRuleId",
    "status",
  ],
  [XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet]: [
    "kind",
    "version",
    "guardianSetId",
    "guardianPublicKeys",
    "quorumThreshold",
    "status",
  ],
  [XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent]: [
    "kind",
    "version",
    "canonicalEventKey",
    "routeId",
    "consumed",
    "consumedAmount",
    "recipient",
  ],
  [XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance]: [
    "kind",
    "version",
    "mintId",
    "owner",
    "balance",
  ],
};

export const XXXL_RUNTIME_MANDATORY_ACCOUNT_SERIALIZATION_KINDS: readonly XXXLRuntimeAccountKind[] =
  [
    XXXL_RUNTIME_ACCOUNT_KIND.MintState,
    XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
    XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
    XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
    XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
  ];

export const XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR = {
  MissingLayout: "MISSING_LAYOUT",
  DuplicateLayout: "DUPLICATE_LAYOUT",
  UnsupportedLayoutVersion: "UNSUPPORTED_LAYOUT_VERSION",
  WrongEncoding: "WRONG_ENCODING",
  WrongDiscriminator: "WRONG_DISCRIMINATOR",
  MissingKindField: "MISSING_KIND_FIELD",
  MissingVersionField: "MISSING_VERSION_FIELD",
  MissingField: "MISSING_FIELD",
  DuplicateField: "DUPLICATE_FIELD",
  WrongFieldOrder: "WRONG_FIELD_ORDER",
  MissingVector: "MISSING_VECTOR",
  DuplicateVector: "DUPLICATE_VECTOR",
  VectorLayoutMismatch: "VECTOR_LAYOUT_MISMATCH",
  WrongCanonicalJson: "WRONG_CANONICAL_JSON",
} as const;

export type XXXLRuntimeAccountSerializationErrorCode =
  (typeof XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR)[keyof typeof XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR];

export type XXXLRuntimeAccountSerializationValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLRuntimeAccountSerializationErrorCode[];
};

function expectedDiscriminator(kind: XXXLRuntimeAccountKind): string {
  switch (kind) {
    case XXXL_RUNTIME_ACCOUNT_KIND.MintState:
      return XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.MintState;
    case XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig:
      return XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.GatewayConfig;
    case XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet:
      return XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.GuardianSet;
    case XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent:
      return XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.ProcessedEvent;
    case XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance:
      return XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.RecipientBalance;
  }
}

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

function field(
  name: string,
  position: number,
  fieldType: XXXLRuntimeAccountFieldType,
): XXXLRuntimeAccountSerializationField {
  return {
    name,
    position,
    fieldType,
  };
}

export function xxxlRuntimeAccountSerializationLayouts(): readonly XXXLRuntimeAccountSerializationLayout[] {
  return [
    {
      accountKind: XXXL_RUNTIME_ACCOUNT_KIND.MintState,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      encoding: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING.CanonicalBinaryV1,
      discriminator: XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.MintState,
      fields: [
        field("kind", 0, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.AccountKind),
        field("version", 1, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.VersionU16),
        field("mintId", 2, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("decimals", 3, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.U8),
        field("totalSupply", 4, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.U128DecimalString),
        field("authorityMode", 5, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.EnumString),
        field("upgradeAuthorityStatus", 6, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.EnumString),
      ],
    },
    {
      accountKind: XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      encoding: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING.CanonicalBinaryV1,
      discriminator: XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.GatewayConfig,
      fields: [
        field("kind", 0, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.AccountKind),
        field("version", 1, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.VersionU16),
        field("routeId", 2, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("sourceChainId", 3, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.U64DecimalString),
        field("sourceToken", 4, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("targetMintToken", 5, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("targetX1NetworkId", 6, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("targetMintCoreId", 7, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("guardianSetId", 8, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("quorumThreshold", 9, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.U16),
        field("finalityRuleId", 10, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("status", 11, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.EnumString),
      ],
    },
    {
      accountKind: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      encoding: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING.CanonicalBinaryV1,
      discriminator: XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.GuardianSet,
      fields: [
        field("kind", 0, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.AccountKind),
        field("version", 1, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.VersionU16),
        field("guardianSetId", 2, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("guardianPublicKeys", 3, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8StringArray),
        field("quorumThreshold", 4, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.U16),
        field("status", 5, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.EnumString),
      ],
    },
    {
      accountKind: XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      encoding: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING.CanonicalBinaryV1,
      discriminator: XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.ProcessedEvent,
      fields: [
        field("kind", 0, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.AccountKind),
        field("version", 1, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.VersionU16),
        field("canonicalEventKey", 2, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("routeId", 3, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("consumed", 4, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Boolean),
        field("consumedAmount", 5, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.U128DecimalString),
        field("recipient", 6, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
      ],
    },
    {
      accountKind: XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      encoding: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING.CanonicalBinaryV1,
      discriminator: XXXL_RUNTIME_ACCOUNT_DISCRIMINATOR.RecipientBalance,
      fields: [
        field("kind", 0, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.AccountKind),
        field("version", 1, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.VersionU16),
        field("mintId", 2, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("owner", 3, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.Utf8String),
        field("balance", 4, XXXL_RUNTIME_ACCOUNT_FIELD_TYPE.U128DecimalString),
      ],
    },
  ];
}

export function validateXXXLRuntimeAccountSerializationLayouts(
  layouts: readonly XXXLRuntimeAccountSerializationLayout[],
): XXXLRuntimeAccountSerializationValidationResult {
  const errors: XXXLRuntimeAccountSerializationErrorCode[] = [];
  const layoutKinds = new Set<XXXLRuntimeAccountKind>();

  for (const layout of layouts) {
    if (layoutKinds.has(layout.accountKind)) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.DuplicateLayout);
    }
    layoutKinds.add(layout.accountKind);
  }

  for (const kind of XXXL_RUNTIME_MANDATORY_ACCOUNT_SERIALIZATION_KINDS) {
    if (!layoutKinds.has(kind)) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.MissingLayout);
    }
  }

  for (const layout of layouts) {
    if (layout.version !== XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION) {
      errors.push(
        XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.UnsupportedLayoutVersion,
      );
    }

    if (
      layout.encoding !==
      XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING.CanonicalBinaryV1
    ) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.WrongEncoding);
    }

    if (layout.discriminator !== expectedDiscriminator(layout.accountKind)) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.WrongDiscriminator);
    }

    const fieldNames = layout.fields.map((item) => item.name);
    const expectedFieldNames =
      XXXL_RUNTIME_ACCOUNT_SERIALIZATION_FIELD_ORDER[layout.accountKind];

    if (fieldNames[0] !== "kind") {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.MissingKindField);
    }

    if (fieldNames[1] !== "version") {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.MissingVersionField);
    }

    if (hasDuplicates(fieldNames)) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.DuplicateField);
    }

    for (const expected of expectedFieldNames) {
      if (!fieldNames.includes(expected)) {
        errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.MissingField);
      }
    }

    const positionsAreCanonical = layout.fields.every(
      (item, index) => item.position === index,
    );

    if (
      !positionsAreCanonical ||
      fieldNames.join("|") !== expectedFieldNames.join("|")
    ) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.WrongFieldOrder);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

function normalizeRuntimeAccountValue(value: unknown): unknown {
  if (typeof value === "bigint") {
    return value.toString();
  }

  if (Array.isArray(value)) {
    return value.map((item) => normalizeRuntimeAccountValue(item));
  }

  return value;
}

export function xxxlCanonicalRuntimeAccountJson(
  layout: XXXLRuntimeAccountSerializationLayout,
  account: XXXLRuntimeSerializableAccount,
): string {
  const record = account as unknown as Record<string, unknown>;

  return JSON.stringify(
    layout.fields.map((item) => [
      item.name,
      normalizeRuntimeAccountValue(record[item.name]),
    ]),
  );
}

function sampleAccounts(): readonly XXXLRuntimeSerializableAccount[] {
  return [
    {
      kind: XXXL_RUNTIME_ACCOUNT_KIND.MintState,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      mintId: "xxxl-mint-core",
      decimals: 9,
      totalSupply: 1_000_000_000_000n,
      authorityMode: XXXL_RUNTIME_AUTHORITY_MODE.GatewayOnly,
      upgradeAuthorityStatus: "STAGED_FINALIZATION",
    },
    {
      kind: XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      routeId: XXXL_GATEWAY_ROUTE_ID,
      sourceChainId: BigInt(ETHEREUM_MAINNET_CHAIN_ID),
      sourceToken: "0x1111111111111111111111111111111111111111",
      targetMintToken: XXXL_MINT_TOKEN,
      targetX1NetworkId: "x1-mainnet",
      targetMintCoreId: "xxxl-mint-core",
      guardianSetId: "guardian-set-1",
      quorumThreshold: 2,
      finalityRuleId: "ethereum-finalized",
      status: XXXL_RUNTIME_ROUTE_STATUS.Active,
    },
    {
      kind: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      guardianSetId: "guardian-set-1",
      guardianPublicKeys: ["guardian-1", "guardian-2", "guardian-3"],
      quorumThreshold: 2,
      status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active,
    },
    {
      kind: XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      canonicalEventKey:
        "ethereum:1:0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:0",
      routeId: XXXL_GATEWAY_ROUTE_ID,
      consumed: true,
      consumedAmount: 1_000_000_000_000n,
      recipient: "x1-recipient-1",
    },
    {
      kind: XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
      version: XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
      mintId: "xxxl-mint-core",
      owner: "x1-recipient-1",
      balance: 1_000_000_000_000n,
    },
  ];
}

function vectorIdForKind(kind: XXXLRuntimeAccountKind): string {
  switch (kind) {
    case XXXL_RUNTIME_ACCOUNT_KIND.MintState:
      return "XXXL_RUNTIME_MINT_STATE_ACCOUNT_V1";
    case XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig:
      return "XXXL_RUNTIME_GATEWAY_CONFIG_ACCOUNT_V1";
    case XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet:
      return "XXXL_RUNTIME_GUARDIAN_SET_ACCOUNT_V1";
    case XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent:
      return "XXXL_RUNTIME_PROCESSED_EVENT_ACCOUNT_V1";
    case XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance:
      return "XXXL_RUNTIME_RECIPIENT_BALANCE_ACCOUNT_V1";
  }
}

export function xxxlRuntimeAccountSerializationVectors(): readonly XXXLRuntimeAccountSerializationVector[] {
  const layouts = xxxlRuntimeAccountSerializationLayouts();
  const accounts = sampleAccounts();

  return layouts.map((layout) => {
    const account = accounts.find((item) => item.kind === layout.accountKind);

    if (!account) {
      throw new Error(`Missing sample account for ${layout.accountKind}`);
    }

    return {
      vectorId: vectorIdForKind(layout.accountKind),
      accountKind: layout.accountKind,
      layoutVersion: layout.version,
      encoding: layout.encoding,
      discriminator: layout.discriminator,
      fieldOrder: layout.fields.map((item) => item.name),
      account,
      canonicalJson: xxxlCanonicalRuntimeAccountJson(layout, account),
    };
  });
}

export function validateXXXLRuntimeAccountSerializationVectors(
  layouts: readonly XXXLRuntimeAccountSerializationLayout[],
  vectors: readonly XXXLRuntimeAccountSerializationVector[],
): XXXLRuntimeAccountSerializationValidationResult {
  const errors: XXXLRuntimeAccountSerializationErrorCode[] = [
    ...validateXXXLRuntimeAccountSerializationLayouts(layouts).errors,
  ];

  const vectorsByKind = new Map<
    XXXLRuntimeAccountKind,
    XXXLRuntimeAccountSerializationVector
  >();

  for (const vector of vectors) {
    if (vectorsByKind.has(vector.accountKind)) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.DuplicateVector);
    }
    vectorsByKind.set(vector.accountKind, vector);
  }

  for (const kind of XXXL_RUNTIME_MANDATORY_ACCOUNT_SERIALIZATION_KINDS) {
    if (!vectorsByKind.has(kind)) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.MissingVector);
    }
  }

  for (const vector of vectors) {
    const layout = layouts.find((item) => item.accountKind === vector.accountKind);

    if (!layout || vector.account.kind !== vector.accountKind) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.VectorLayoutMismatch);
      continue;
    }

    const expectedFieldOrder = layout.fields.map((item) => item.name);

    if (
      vector.layoutVersion !== layout.version ||
      vector.encoding !== layout.encoding ||
      vector.discriminator !== layout.discriminator ||
      vector.fieldOrder.join("|") !== expectedFieldOrder.join("|")
    ) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.VectorLayoutMismatch);
    }

    const canonicalJson = xxxlCanonicalRuntimeAccountJson(layout, vector.account);
    if (canonicalJson !== vector.canonicalJson) {
      errors.push(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.WrongCanonicalJson);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

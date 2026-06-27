export const XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_VERSION = 1;
export const XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ENCODING =
  "FIXED_BINARY_LE_V1";

export const XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND = {
  MintStateAccount: "MINT_STATE_ACCOUNT",
  GatewayConfigAccount: "GATEWAY_CONFIG_ACCOUNT",
  GuardianSetAccount: "GUARDIAN_SET_ACCOUNT",
  ProcessedEventAccount: "PROCESSED_EVENT_ACCOUNT",
  RecipientBalanceAccount: "RECIPIENT_BALANCE_ACCOUNT",
  ConsumeGatewayMintInstruction: "CONSUME_GATEWAY_MINT_INSTRUCTION",
} as const;

export type XXXLProductionRuntimeByteLayoutKind =
  (typeof XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND)[keyof typeof XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND];

export const XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE = {
  Discriminator8: "DISCRIMINATOR_8",
  VersionU16Le: "VERSION_U16_LE",
  U8: "U8",
  BoolU8: "BOOL_U8",
  U16Le: "U16_LE",
  U32Le: "U32_LE",
  U64Le: "U64_LE",
  U128Le: "U128_LE",
  Pubkey32: "PUBKEY_32",
  Hash32: "HASH_32",
  FixedBytes32: "FIXED_BYTES_32",
  Padding: "PADDING",
} as const;

export type XXXLProductionRuntimeByteFieldType =
  (typeof XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE)[keyof typeof XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE];

export const XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR = {
  EmptyLayout: "EMPTY_LAYOUT",
  DuplicateFieldName: "DUPLICATE_FIELD_NAME",
  MissingDiscriminator: "MISSING_DISCRIMINATOR",
  MissingVersion: "MISSING_VERSION",
  NonContiguousFields: "NON_CONTIGUOUS_FIELDS",
  WrongFieldSize: "WRONG_FIELD_SIZE",
  WrongTotalSize: "WRONG_TOTAL_SIZE",
  InvalidAlignment: "INVALID_ALIGNMENT",
} as const;

export type XXXLProductionRuntimeByteLayoutErrorCode =
  (typeof XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR)[keyof typeof XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR];

export type XXXLProductionRuntimeByteField = {
  readonly name: string;
  readonly type: XXXLProductionRuntimeByteFieldType;
  readonly offset: number;
  readonly size: number;
  readonly description: string;
};

export type XXXLProductionRuntimeByteLayout = {
  readonly version: typeof XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_VERSION;
  readonly encoding: typeof XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ENCODING;
  readonly kind: XXXLProductionRuntimeByteLayoutKind;
  readonly totalSize: number;
  readonly alignment: 8;
  readonly fields: readonly XXXLProductionRuntimeByteField[];
};

export type XXXLProductionRuntimeByteLayoutValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLProductionRuntimeByteLayoutErrorCode[];
};

export const XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE = {
  MintState: 176,
  GatewayConfig: 256,
  GuardianSet: 320,
  ProcessedEvent: 144,
  RecipientBalance: 144,
} as const;

export const XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE = {
  ConsumeGatewayMint: 208,
} as const;

export function xxxlProductionRuntimeAccountByteLayouts(): readonly XXXLProductionRuntimeByteLayout[] {
  return [
    mintStateAccountLayout(),
    gatewayConfigAccountLayout(),
    guardianSetAccountLayout(),
    processedEventAccountLayout(),
    recipientBalanceAccountLayout(),
  ];
}

export function xxxlProductionRuntimeInstructionByteLayouts(): readonly XXXLProductionRuntimeByteLayout[] {
  return [consumeGatewayMintInstructionLayout()];
}

export function xxxlProductionRuntimeByteLayouts(): readonly XXXLProductionRuntimeByteLayout[] {
  return [
    ...xxxlProductionRuntimeAccountByteLayouts(),
    ...xxxlProductionRuntimeInstructionByteLayouts(),
  ];
}

export function validateXXXLProductionRuntimeByteLayout(
  layout: XXXLProductionRuntimeByteLayout,
): XXXLProductionRuntimeByteLayoutValidationResult {
  const errors: XXXLProductionRuntimeByteLayoutErrorCode[] = [];

  if (layout.fields.length === 0) {
    errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.EmptyLayout);
  }

  if (
    layout.fields[0]?.name !== "discriminator" ||
    layout.fields[0]?.type !==
      XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Discriminator8 ||
    layout.fields[0]?.offset !== 0 ||
    layout.fields[0]?.size !== 8
  ) {
    errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.MissingDiscriminator);
  }

  if (
    !layout.fields.some(
      (field) =>
        field.name === "version" &&
        field.type === XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.VersionU16Le,
    )
  ) {
    errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.MissingVersion);
  }

  const seenNames = new Set<string>();
  let expectedOffset = 0;

  for (const field of layout.fields) {
    if (seenNames.has(field.name)) {
      errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.DuplicateFieldName);
    }
    seenNames.add(field.name);

    if (field.offset !== expectedOffset) {
      errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.NonContiguousFields);
    }

    if (field.size !== xxxlProductionRuntimeByteFieldTypeSize(field.type, field.size)) {
      errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.WrongFieldSize);
    }

    const alignment = xxxlProductionRuntimeByteFieldAlignment(field.type);

    if (field.offset % alignment !== 0) {
      errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.InvalidAlignment);
    }

    expectedOffset = field.offset + field.size;
  }

  if (expectedOffset !== layout.totalSize) {
    errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.WrongTotalSize);
  }

  if (layout.totalSize % layout.alignment !== 0) {
    errors.push(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.InvalidAlignment);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function validateXXXLProductionRuntimeByteLayouts(
  layouts: readonly XXXLProductionRuntimeByteLayout[] = xxxlProductionRuntimeByteLayouts(),
): XXXLProductionRuntimeByteLayoutValidationResult {
  const errors = layouts.flatMap(
    (layout) => validateXXXLProductionRuntimeByteLayout(layout).errors,
  );

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalProductionRuntimeByteLayoutJson(
  layout: XXXLProductionRuntimeByteLayout,
): string {
  return JSON.stringify([
    ["version", layout.version],
    ["encoding", layout.encoding],
    ["kind", layout.kind],
    ["totalSize", layout.totalSize],
    ["alignment", layout.alignment],
    [
      "fields",
      layout.fields.map((field) => [
        ["name", field.name],
        ["type", field.type],
        ["offset", field.offset],
        ["size", field.size],
        ["description", field.description],
      ]),
    ],
  ]);
}

export function xxxlProductionRuntimeByteFieldTypeSize(
  type: XXXLProductionRuntimeByteFieldType,
  declaredSize: number,
): number {
  switch (type) {
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Discriminator8:
      return 8;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.VersionU16Le:
      return 2;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U8:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.BoolU8:
      return 1;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U16Le:
      return 2;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U32Le:
      return 4;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U64Le:
      return 8;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le:
      return 16;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Pubkey32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Hash32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.FixedBytes32:
      return 32;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Padding:
      return declaredSize;
  }
}

export function xxxlProductionRuntimeByteFieldAlignment(
  type: XXXLProductionRuntimeByteFieldType,
): number {
  switch (type) {
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Discriminator8:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U64Le:
      return 8;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le:
      return 16;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U32Le:
      return 4;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.VersionU16Le:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U16Le:
      return 2;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Pubkey32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Hash32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.FixedBytes32:
      return 8;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U8:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.BoolU8:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Padding:
      return 1;
  }
}

function mintStateAccountLayout(): XXXLProductionRuntimeByteLayout {
  return layout(
    XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.MintState,
    [
      field("discriminator", "DISCRIMINATOR_8", 0, 8, "Account discriminator."),
      field("version", "VERSION_U16_LE", 8, 2, "Layout version."),
      field("decimals", "U8", 10, 1, "SPL mint decimals."),
      field("authorityMode", "U8", 11, 1, "Gateway-only authority mode."),
      field("upgradeAuthorityStatus", "U8", 12, 1, "Temporary/frozen upgrade status."),
      field("gatewayMintAuthorityBump", "U8", 13, 1, "PDA bump for gateway mint authority."),
      field("reserved0", "PADDING", 14, 2, "Reserved alignment padding."),
      field("mintPubkey", "PUBKEY_32", 16, 32, "SPL Token mint account."),
      field("totalSupply", "U128_LE", 48, 16, "Runtime mirror of total XXXL supply."),
      field("gatewayMintAuthorityPda", "PUBKEY_32", 64, 32, "PDA that signs SPL Token mint_to CPI."),
      field("programUpgradeAuthority", "PUBKEY_32", 96, 32, "Program upgrade authority surface."),
      field("splTokenMintAuthority", "PUBKEY_32", 128, 32, "SPL Token mint authority surface."),
      field("reserved1", "PADDING", 160, 16, "Reserved future-proof padding."),
    ],
  );
}

function gatewayConfigAccountLayout(): XXXLProductionRuntimeByteLayout {
  return layout(
    XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
    XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GatewayConfig,
    [
      field("discriminator", "DISCRIMINATOR_8", 0, 8, "Account discriminator."),
      field("version", "VERSION_U16_LE", 8, 2, "Layout version."),
      field("status", "U8", 10, 1, "Route status."),
      field("reserved0", "PADDING", 11, 1, "Reserved alignment padding."),
      field("sourceChainWeightBps", "U16_LE", 12, 2, "Route mint weight in basis points."),
      field("reserved1", "PADDING", 14, 2, "Reserved alignment padding."),
      field("routeId", "FIXED_BYTES_32", 16, 32, "Fixed route identifier."),
      field("sourceChainId", "U64_LE", 48, 8, "Source chain id."),
      field("sourceToken", "PUBKEY_32", 56, 32, "Source token identifier encoded as 32 bytes."),
      field("targetMint", "PUBKEY_32", 88, 32, "Target XXXL mint account."),
      field("guardianSetId", "FIXED_BYTES_32", 120, 32, "Guardian set identifier."),
      field("finalityRuleId", "FIXED_BYTES_32", 152, 32, "Finality rule identifier."),
      field("reserved2", "PADDING", 184, 8, "Padding before u128 caps."),
      field("perEventCap", "U128_LE", 192, 16, "Per-event mint cap."),
      field("dailyCap", "U128_LE", 208, 16, "Daily route mint cap."),
      field("epochCap", "U128_LE", 224, 16, "Epoch route mint cap."),
      field("gatewayConfigBump", "U8", 240, 1, "PDA bump for route config."),
      field("reserved3", "PADDING", 241, 15, "Reserved future-proof padding."),
    ],
  );
}

function guardianSetAccountLayout(): XXXLProductionRuntimeByteLayout {
  return layout(
    XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GuardianSetAccount,
    XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GuardianSet,
    [
      field("discriminator", "DISCRIMINATOR_8", 0, 8, "Account discriminator."),
      field("version", "VERSION_U16_LE", 8, 2, "Layout version."),
      field("status", "U8", 10, 1, "Guardian set status."),
      field("reserved0", "PADDING", 11, 1, "Reserved alignment padding."),
      field("quorumThreshold", "U16_LE", 12, 2, "Guardian quorum threshold."),
      field("guardianCount", "U8", 14, 1, "Active guardian public key count."),
      field("guardianSetBump", "U8", 15, 1, "PDA bump for guardian set."),
      field("guardianPubkeys", "PADDING", 16, 256, "Fixed capacity for 8 guardian pubkeys."),
      field("guardianKeyHash", "HASH_32", 272, 32, "Hash of active guardian public keys."),
      field("reserved1", "PADDING", 304, 16, "Reserved future-proof padding."),
    ],
  );
}

function processedEventAccountLayout(): XXXLProductionRuntimeByteLayout {
  return layout(
    XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
    XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.ProcessedEvent,
    [
      field("discriminator", "DISCRIMINATOR_8", 0, 8, "Account discriminator."),
      field("version", "VERSION_U16_LE", 8, 2, "Layout version."),
      field("consumed", "BOOL_U8", 10, 1, "Replay protection consumed flag."),
      field("reserved0", "PADDING", 11, 5, "Reserved alignment padding."),
      field("canonicalEventKey", "HASH_32", 16, 32, "Canonical gateway event key."),
      field("routeId", "FIXED_BYTES_32", 48, 32, "Route identifier."),
      field("recipient", "PUBKEY_32", 80, 32, "Recipient owner/account identifier."),
      field("consumedAmount", "U128_LE", 112, 16, "Consumed mint amount."),
      field("consumedSlot", "U64_LE", 128, 8, "Runtime slot where event was consumed."),
      field("reserved1", "PADDING", 136, 8, "Reserved future-proof padding."),
    ],
  );
}

function recipientBalanceAccountLayout(): XXXLProductionRuntimeByteLayout {
  return layout(
    XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
    XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.RecipientBalance,
    [
      field("discriminator", "DISCRIMINATOR_8", 0, 8, "Account discriminator."),
      field("version", "VERSION_U16_LE", 8, 2, "Layout version."),
      field("reserved0", "PADDING", 10, 6, "Reserved alignment padding."),
      field("owner", "PUBKEY_32", 16, 32, "Recipient owner."),
      field("mint", "PUBKEY_32", 48, 32, "XXXL SPL Token mint."),
      field("balance", "U128_LE", 80, 16, "Runtime mirror recipient balance."),
      field("lastCanonicalEventKey", "HASH_32", 96, 32, "Last consumed gateway event key."),
      field("reserved1", "PADDING", 128, 16, "Reserved future-proof padding."),
    ],
  );
}

function consumeGatewayMintInstructionLayout(): XXXLProductionRuntimeByteLayout {
  return layout(
    XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE.ConsumeGatewayMint,
    [
      field("discriminator", "DISCRIMINATOR_8", 0, 8, "Instruction discriminator."),
      field("version", "VERSION_U16_LE", 8, 2, "Instruction layout version."),
      field("accountMetaCount", "U8", 10, 1, "Expected account meta count."),
      field("routeAccountIndex", "U8", 11, 1, "Gateway config account index."),
      field("guardianSetAccountIndex", "U8", 12, 1, "Guardian set account index."),
      field("mintStateAccountIndex", "U8", 13, 1, "Mint state account index."),
      field("processedEventAccountIndex", "U8", 14, 1, "Processed event account index."),
      field("recipientBalanceAccountIndex", "U8", 15, 1, "Recipient balance account index."),
      field("routeId", "FIXED_BYTES_32", 16, 32, "Route identifier."),
      field("guardianSetId", "FIXED_BYTES_32", 48, 32, "Guardian set identifier."),
      field("mintId", "PUBKEY_32", 80, 32, "Target XXXL mint identifier."),
      field("canonicalEventKey", "HASH_32", 112, 32, "Canonical gateway event key."),
      field("recipient", "PUBKEY_32", 144, 32, "Recipient account/owner."),
      field("amount", "U128_LE", 176, 16, "Authorized mint amount."),
      field("sourceChainWeightBps", "U16_LE", 192, 2, "Route weight used upstream."),
      field("reserved0", "PADDING", 194, 14, "Reserved future-proof padding."),
    ],
  );
}

function layout(
  kind: XXXLProductionRuntimeByteLayoutKind,
  totalSize: number,
  fields: readonly XXXLProductionRuntimeByteField[],
): XXXLProductionRuntimeByteLayout {
  return {
    version: XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_VERSION,
    encoding: XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ENCODING,
    kind,
    totalSize,
    alignment: 8,
    fields,
  };
}

function field(
  name: string,
  type: XXXLProductionRuntimeByteFieldType,
  offset: number,
  size: number,
  description: string,
): XXXLProductionRuntimeByteField {
  return {
    name,
    type,
    offset,
    size,
    description,
  };
}

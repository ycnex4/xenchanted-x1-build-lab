import {
  XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND,
  xxxlProductionRuntimeByteLayouts,
  type XXXLProductionRuntimeByteField,
  type XXXLProductionRuntimeByteLayout,
  type XXXLProductionRuntimeByteLayoutKind,
} from "./runtime-production-byte-layout.js";
import {
  XXXL_X1_SVM_ACCOUNT_ROLE,
  XXXL_X1_SVM_HANDLER,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  deriveXXXLX1SvmGatewayMintAuthorityPdaModel,
  executeXXXLX1SvmConsumeGatewayMintSkeleton,
  xxxlX1SvmConsumeGatewayMintAccountMetas,
  xxxlX1SvmProgramSkeletonConfig,
} from "./x1-svm-program-skeleton.js";

export const XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_SET_VERSION = 1;

export const XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID = {
  MintStateAccount: "XXXL_SVM_SERIALIZED_MINT_STATE_ACCOUNT_V1",
  GatewayConfigAccount: "XXXL_SVM_SERIALIZED_GATEWAY_CONFIG_ACCOUNT_V1",
  GuardianSetAccount: "XXXL_SVM_SERIALIZED_GUARDIAN_SET_ACCOUNT_V1",
  ProcessedEventAccount: "XXXL_SVM_SERIALIZED_PROCESSED_EVENT_ACCOUNT_V1",
  RecipientBalanceAccount: "XXXL_SVM_SERIALIZED_RECIPIENT_BALANCE_ACCOUNT_V1",
  ConsumeGatewayMintInstruction:
    "XXXL_SVM_SERIALIZED_CONSUME_GATEWAY_MINT_INSTRUCTION_V1",
} as const;

export type XXXLSvmSerializedRuntimeVectorId =
  (typeof XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID)[keyof typeof XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID];

export const XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR = {
  MissingVector: "MISSING_VECTOR",
  DuplicateVector: "DUPLICATE_VECTOR",
  WrongByteLength: "WRONG_BYTE_LENGTH",
  WrongCanonicalHex: "WRONG_CANONICAL_HEX",
  WrongFieldOffset: "WRONG_FIELD_OFFSET",
  WrongSkeletonBoundary: "WRONG_SKELETON_BOUNDARY",
} as const;

export type XXXLSvmSerializedRuntimeVectorErrorCode =
  (typeof XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR)[keyof typeof XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR];

export type XXXLSvmSerializedRuntimeFieldProbe = {
  readonly fieldName: string;
  readonly offset: number;
  readonly size: number;
  readonly hex: string;
};

export type XXXLSvmSerializedRuntimeVector = {
  readonly version: typeof XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_SET_VERSION;
  readonly vectorId: XXXLSvmSerializedRuntimeVectorId;
  readonly layoutKind: XXXLProductionRuntimeByteLayoutKind;
  readonly description: string;
  readonly byteLength: number;
  readonly canonicalHex: string;
  readonly fieldProbes: readonly XXXLSvmSerializedRuntimeFieldProbe[];
};

export type XXXLSvmSerializedRuntimeVectorValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLSvmSerializedRuntimeVectorErrorCode[];
};

export type XXXLSvmSerializedRuntimeBundle = {
  readonly version: typeof XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_SET_VERSION;
  readonly vectors: readonly XXXLSvmSerializedRuntimeVector[];
  readonly handler: typeof XXXL_X1_SVM_HANDLER.ConsumeGatewayMint;
  readonly tokenProgramId: typeof XXXL_X1_SVM_TOKEN_PROGRAM_ID;
  readonly gatewayMintAuthorityPda: string;
  readonly accountMetaRoles: readonly string[];
  readonly cpiPrepared: boolean;
  readonly cpiAtomicWithParentTransaction: boolean;
};

const SAMPLE_VALUES = {
  version: 1,
  decimals: 18,
  authorityMode: 1,
  upgradeAuthorityStatus: 1,
  gatewayMintAuthorityBump: 201,
  status: 1,
  sourceChainWeightBps: 10_000,
  sourceChainId: 1n,
  quorumThreshold: 2,
  guardianCount: 3,
  guardianSetBump: 77,
  consumed: true,
  consumedAmount: 1_000n,
  consumedSlot: 123_456n,
  totalSupply: 500n,
  balance: 200n,
  amount: 1_000n,
  accountMetaCount: 9,
  routeAccountIndex: 1,
  guardianSetAccountIndex: 2,
  mintStateAccountIndex: 0,
  processedEventAccountIndex: 3,
  recipientBalanceAccountIndex: 4,
  perEventCap: 10_000n,
  dailyCap: 100_000n,
  epochCap: 1_000_000n,
  gatewayConfigBump: 88,
} as const;

export function xxxlSvmSerializedRuntimeVectors(): readonly XXXLSvmSerializedRuntimeVector[] {
  return [
    vectorFor(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.MintStateAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
      "Serialized Mint State account bytes.",
    ),
    vectorFor(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.GatewayConfigAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
      "Serialized Gateway Config account bytes.",
    ),
    vectorFor(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.GuardianSetAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GuardianSetAccount,
      "Serialized Guardian Set account bytes.",
    ),
    vectorFor(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.ProcessedEventAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
      "Serialized Processed Event account bytes.",
    ),
    vectorFor(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.RecipientBalanceAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
      "Serialized Recipient Balance account bytes.",
    ),
    vectorFor(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.ConsumeGatewayMintInstruction,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
      "Serialized consume_gateway_mint instruction bytes.",
    ),
  ];
}

export function xxxlSvmSerializedRuntimeBundle(): XXXLSvmSerializedRuntimeBundle {
  const config = xxxlX1SvmProgramSkeletonConfig();
  const result = executeXXXLX1SvmConsumeGatewayMintSkeleton();
  const pda = deriveXXXLX1SvmGatewayMintAuthorityPdaModel(config.programId);

  return {
    version: XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_SET_VERSION,
    vectors: xxxlSvmSerializedRuntimeVectors(),
    handler: XXXL_X1_SVM_HANDLER.ConsumeGatewayMint,
    tokenProgramId: XXXL_X1_SVM_TOKEN_PROGRAM_ID,
    gatewayMintAuthorityPda: pda.address,
    accountMetaRoles: xxxlX1SvmConsumeGatewayMintAccountMetas().map(
      (meta) => meta.role,
    ),
    cpiPrepared: result.cpiBoundary.prepared,
    cpiAtomicWithParentTransaction:
      result.cpiBoundary.atomicWithParentTransaction,
  };
}

export function validateXXXLSvmSerializedRuntimeVectors(
  vectors: readonly XXXLSvmSerializedRuntimeVector[] = xxxlSvmSerializedRuntimeVectors(),
): XXXLSvmSerializedRuntimeVectorValidationResult {
  const errors: XXXLSvmSerializedRuntimeVectorErrorCode[] = [];
  const ids = new Set<XXXLSvmSerializedRuntimeVectorId>();

  for (const vector of vectors) {
    if (ids.has(vector.vectorId)) {
      errors.push(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.DuplicateVector);
    }
    ids.add(vector.vectorId);

    const layout = layoutByKind(vector.layoutKind);

    if (vector.byteLength !== layout.totalSize) {
      errors.push(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.WrongByteLength);
    }

    if (vector.canonicalHex.length !== vector.byteLength * 2) {
      errors.push(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.WrongCanonicalHex);
    }

    if (vector.canonicalHex !== bytesToHex(serializeLayout(layout))) {
      errors.push(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.WrongCanonicalHex);
    }

    for (const probe of vector.fieldProbes) {
      const field = layout.fields.find((item) => item.name === probe.fieldName);

      if (
        field === undefined ||
        field.offset !== probe.offset ||
        field.size !== probe.size ||
        readHex(vector.canonicalHex, probe.offset, probe.size) !== probe.hex
      ) {
        errors.push(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.WrongFieldOffset);
      }
    }
  }

  for (const vectorId of Object.values(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID)) {
    if (!ids.has(vectorId)) {
      errors.push(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.MissingVector);
    }
  }

  const bundle = xxxlSvmSerializedRuntimeBundle();

  if (
    bundle.handler !== XXXL_X1_SVM_HANDLER.ConsumeGatewayMint ||
    bundle.tokenProgramId !== XXXL_X1_SVM_TOKEN_PROGRAM_ID ||
    bundle.gatewayMintAuthorityPda.length === 0 ||
    !bundle.cpiPrepared ||
    !bundle.cpiAtomicWithParentTransaction ||
    !sameStrings(bundle.accountMetaRoles, [
      XXXL_X1_SVM_ACCOUNT_ROLE.MintState,
      XXXL_X1_SVM_ACCOUNT_ROLE.GatewayConfig,
      XXXL_X1_SVM_ACCOUNT_ROLE.GuardianSet,
      XXXL_X1_SVM_ACCOUNT_ROLE.ProcessedEvent,
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientBalance,
      XXXL_X1_SVM_ACCOUNT_ROLE.SplTokenMint,
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientTokenAccount,
      XXXL_X1_SVM_ACCOUNT_ROLE.MintAuthorityPda,
      XXXL_X1_SVM_ACCOUNT_ROLE.TokenProgram,
    ])
  ) {
    errors.push(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.WrongSkeletonBoundary);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalSvmSerializedRuntimeBundleJson(
  bundle: XXXLSvmSerializedRuntimeBundle = xxxlSvmSerializedRuntimeBundle(),
): string {
  return JSON.stringify([
    ["version", bundle.version],
    ["handler", bundle.handler],
    ["tokenProgramId", bundle.tokenProgramId],
    ["gatewayMintAuthorityPda", bundle.gatewayMintAuthorityPda],
    ["accountMetaRoles", bundle.accountMetaRoles],
    ["cpiPrepared", bundle.cpiPrepared],
    ["cpiAtomicWithParentTransaction", bundle.cpiAtomicWithParentTransaction],
    [
      "vectors",
      bundle.vectors.map((vector) => [
        ["version", vector.version],
        ["vectorId", vector.vectorId],
        ["layoutKind", vector.layoutKind],
        ["description", vector.description],
        ["byteLength", vector.byteLength],
        ["canonicalHex", vector.canonicalHex],
        [
          "fieldProbes",
          vector.fieldProbes.map((probe) => [
            ["fieldName", probe.fieldName],
            ["offset", probe.offset],
            ["size", probe.size],
            ["hex", probe.hex],
          ]),
        ],
      ]),
    ],
  ]);
}

export function serializeXXXLLayoutByKind(
  kind: XXXLProductionRuntimeByteLayoutKind,
): Uint8Array {
  return serializeLayout(layoutByKind(kind));
}

function vectorFor(
  vectorId: XXXLSvmSerializedRuntimeVectorId,
  layoutKind: XXXLProductionRuntimeByteLayoutKind,
  description: string,
): XXXLSvmSerializedRuntimeVector {
  const layout = layoutByKind(layoutKind);
  const bytes = serializeLayout(layout);
  const canonicalHex = bytesToHex(bytes);

  return {
    version: XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_SET_VERSION,
    vectorId,
    layoutKind,
    description,
    byteLength: bytes.length,
    canonicalHex,
    fieldProbes: probesFor(layout, canonicalHex),
  };
}

function probesFor(
  layout: XXXLProductionRuntimeByteLayout,
  canonicalHex: string,
): readonly XXXLSvmSerializedRuntimeFieldProbe[] {
  const importantFields = [
    "discriminator",
    "version",
    "totalSupply",
    "gatewayMintAuthorityPda",
    "sourceChainWeightBps",
    "perEventCap",
    "guardianPubkeys",
    "guardianKeyHash",
    "consumed",
    "canonicalEventKey",
    "consumedAmount",
    "owner",
    "balance",
    "accountMetaCount",
    "routeId",
    "amount",
  ];

  return layout.fields
    .filter((field) => importantFields.includes(field.name))
    .map((field) => ({
      fieldName: field.name,
      offset: field.offset,
      size: field.size,
      hex: readHex(canonicalHex, field.offset, field.size),
    }));
}

function serializeLayout(layout: XXXLProductionRuntimeByteLayout): Uint8Array {
  const bytes = new Uint8Array(layout.totalSize);

  for (const field of layout.fields) {
    writeField(bytes, layout, field);
  }

  return bytes;
}

function writeField(
  bytes: Uint8Array,
  layout: XXXLProductionRuntimeByteLayout,
  field: XXXLProductionRuntimeByteField,
): void {
  switch (field.type) {
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Discriminator8:
      writeFixed(bytes, field, stableBytes(`${layout.kind}:discriminator`, 8));
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.VersionU16Le:
      writeU16Le(bytes, field.offset, SAMPLE_VALUES.version);
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U8:
      writeU8(bytes, field.offset, numericValueForField(field.name));
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.BoolU8:
      writeU8(bytes, field.offset, booleanValueForField(field.name) ? 1 : 0);
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U16Le:
      writeU16Le(bytes, field.offset, numericValueForField(field.name));
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U32Le:
      writeU32Le(bytes, field.offset, numericValueForField(field.name));
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U64Le:
      writeU64Le(bytes, field.offset, bigintValueForField(field.name));
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le:
      writeU128Le(bytes, field.offset, bigintValueForField(field.name));
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Pubkey32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Hash32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.FixedBytes32:
      writeFixed(bytes, field, stableBytes(`${layout.kind}:${field.name}`, 32));
      break;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Padding:
      writeFixed(bytes, field, new Uint8Array(field.size));
      break;
  }
}

function layoutByKind(
  kind: XXXLProductionRuntimeByteLayoutKind,
): XXXLProductionRuntimeByteLayout {
  const layout = xxxlProductionRuntimeByteLayouts().find(
    (item) => item.kind === kind,
  );

  if (!layout) {
    throw new Error(`Missing production runtime layout: ${kind}`);
  }

  return layout;
}

function numericValueForField(fieldName: string): number {
  switch (fieldName) {
    case "decimals":
      return SAMPLE_VALUES.decimals;
    case "authorityMode":
      return SAMPLE_VALUES.authorityMode;
    case "upgradeAuthorityStatus":
      return SAMPLE_VALUES.upgradeAuthorityStatus;
    case "gatewayMintAuthorityBump":
      return SAMPLE_VALUES.gatewayMintAuthorityBump;
    case "status":
      return SAMPLE_VALUES.status;
    case "sourceChainWeightBps":
      return SAMPLE_VALUES.sourceChainWeightBps;
    case "quorumThreshold":
      return SAMPLE_VALUES.quorumThreshold;
    case "guardianCount":
      return SAMPLE_VALUES.guardianCount;
    case "guardianSetBump":
      return SAMPLE_VALUES.guardianSetBump;
    case "accountMetaCount":
      return SAMPLE_VALUES.accountMetaCount;
    case "routeAccountIndex":
      return SAMPLE_VALUES.routeAccountIndex;
    case "guardianSetAccountIndex":
      return SAMPLE_VALUES.guardianSetAccountIndex;
    case "mintStateAccountIndex":
      return SAMPLE_VALUES.mintStateAccountIndex;
    case "processedEventAccountIndex":
      return SAMPLE_VALUES.processedEventAccountIndex;
    case "recipientBalanceAccountIndex":
      return SAMPLE_VALUES.recipientBalanceAccountIndex;
    case "gatewayConfigBump":
      return SAMPLE_VALUES.gatewayConfigBump;
    default:
      return 0;
  }
}

function booleanValueForField(fieldName: string): boolean {
  return fieldName === "consumed" ? SAMPLE_VALUES.consumed : false;
}

function bigintValueForField(fieldName: string): bigint {
  switch (fieldName) {
    case "sourceChainId":
      return SAMPLE_VALUES.sourceChainId;
    case "consumedAmount":
      return SAMPLE_VALUES.consumedAmount;
    case "consumedSlot":
      return SAMPLE_VALUES.consumedSlot;
    case "totalSupply":
      return SAMPLE_VALUES.totalSupply;
    case "balance":
      return SAMPLE_VALUES.balance;
    case "amount":
      return SAMPLE_VALUES.amount;
    case "perEventCap":
      return SAMPLE_VALUES.perEventCap;
    case "dailyCap":
      return SAMPLE_VALUES.dailyCap;
    case "epochCap":
      return SAMPLE_VALUES.epochCap;
    default:
      return 0n;
  }
}

function writeFixed(
  bytes: Uint8Array,
  field: XXXLProductionRuntimeByteField,
  value: Uint8Array,
): void {
  bytes.set(value.slice(0, field.size), field.offset);
}

function writeU8(bytes: Uint8Array, offset: number, value: number): void {
  bytes[offset] = value & 0xff;
}

function writeU16Le(bytes: Uint8Array, offset: number, value: number): void {
  bytes[offset] = value & 0xff;
  bytes[offset + 1] = (value >> 8) & 0xff;
}

function writeU32Le(bytes: Uint8Array, offset: number, value: number): void {
  bytes[offset] = value & 0xff;
  bytes[offset + 1] = (value >> 8) & 0xff;
  bytes[offset + 2] = (value >> 16) & 0xff;
  bytes[offset + 3] = (value >> 24) & 0xff;
}

function writeU64Le(bytes: Uint8Array, offset: number, value: bigint): void {
  writeBigIntLe(bytes, offset, value, 8);
}

function writeU128Le(bytes: Uint8Array, offset: number, value: bigint): void {
  writeBigIntLe(bytes, offset, value, 16);
}

function writeBigIntLe(
  bytes: Uint8Array,
  offset: number,
  value: bigint,
  byteLength: number,
): void {
  let remaining = value;

  for (let index = 0; index < byteLength; index += 1) {
    bytes[offset + index] = Number(remaining & 0xffn);
    remaining >>= 8n;
  }
}

function stableBytes(seed: string, length: number): Uint8Array {
  const output = new Uint8Array(length);
  let state = 0x811c9dc5;

  for (let index = 0; index < length; index += 1) {
    const code = seed.charCodeAt(index % seed.length);
    state = Math.imul(state ^ code ^ index, 0x01000193) >>> 0;
    output[index] = state & 0xff;
  }

  return output;
}

function bytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

function readHex(hex: string, offset: number, size: number): string {
  return hex.slice(offset * 2, (offset + size) * 2);
}

function sameStrings(left: readonly string[], right: readonly string[]): boolean {
  return (
    left.length === right.length &&
    left.every((item, index) => item === right[index])
  );
}

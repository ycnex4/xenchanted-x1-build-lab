import {
  XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND,
  xxxlProductionRuntimeByteLayouts,
  type XXXLProductionRuntimeByteField,
  type XXXLProductionRuntimeByteLayout,
  type XXXLProductionRuntimeByteLayoutKind,
} from "./runtime-production-byte-layout.js";
import {
  serializeXXXLLayoutByKind,
  validateXXXLSvmSerializedRuntimeVectors,
  xxxlSvmSerializedRuntimeVectors,
  type XXXLSvmSerializedRuntimeVector,
} from "./svm-serialized-runtime-vectors.js";
import {
  XXXL_X1_SVM_HANDLER,
  XXXL_X1_SVM_PROGRAM_SKELETON_STEP,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  executeXXXLX1SvmConsumeGatewayMintSkeleton,
} from "./x1-svm-program-skeleton.js";

export const XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION = 1;

export const XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR = {
  SerializedVectorValidationFailed: "SERIALIZED_VECTOR_VALIDATION_FAILED",
  ByteLengthMismatch: "BYTE_LENGTH_MISMATCH",
  CanonicalBytesMismatch: "CANONICAL_BYTES_MISMATCH",
  DiscriminatorMismatch: "DISCRIMINATOR_MISMATCH",
  VersionMismatch: "VERSION_MISMATCH",
  FieldOutOfRange: "FIELD_OUT_OF_RANGE",
  MissingRequiredAccount: "MISSING_REQUIRED_ACCOUNT",
  MissingInstruction: "MISSING_INSTRUCTION",
  WrongInstructionKind: "WRONG_INSTRUCTION_KIND",
  DecodedBytesInvalid: "DECODED_BYTES_INVALID",
  SkeletonExecutionRejected: "SKELETON_EXECUTION_REJECTED",
} as const;

export type XXXLSvmRuntimeDecoderHandlerErrorCode =
  (typeof XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR)[keyof typeof XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR];

export const XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP = {
  DecodeAccountBytes: "DECODE_ACCOUNT_BYTES",
  DecodeInstructionBytes: "DECODE_INSTRUCTION_BYTES",
  ValidateDecodedLayouts: "VALIDATE_DECODED_LAYOUTS",
  BuildHandlerInput: "BUILD_HANDLER_INPUT",
  ExecuteSkeletonBoundary: "EXECUTE_SKELETON_BOUNDARY",
  PrepareCpiBoundary: "PREPARE_CPI_BOUNDARY",
  ReturnResult: "RETURN_RESULT",
} as const;

export type XXXLSvmRuntimeDecoderHandlerStep =
  (typeof XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP)[keyof typeof XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP];

export type XXXLSvmRuntimeDecodedFieldValue = string | number | boolean;

export type XXXLSvmRuntimeDecodedField = {
  readonly name: string;
  readonly type: string;
  readonly offset: number;
  readonly size: number;
  readonly hex: string;
  readonly value: XXXLSvmRuntimeDecodedFieldValue;
};

export type XXXLSvmRuntimeDecodedBytes = {
  readonly version: typeof XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION;
  readonly layoutKind: XXXLProductionRuntimeByteLayoutKind;
  readonly byteLength: number;
  readonly expectedByteLength: number;
  readonly canonicalHex: string;
  readonly expectedCanonicalHex: string;
  readonly ok: boolean;
  readonly errors: readonly XXXLSvmRuntimeDecoderHandlerErrorCode[];
  readonly fields: readonly XXXLSvmRuntimeDecodedField[];
};

export type XXXLSvmRuntimeDecodedVectorBundle = {
  readonly version: typeof XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION;
  readonly ok: boolean;
  readonly errors: readonly XXXLSvmRuntimeDecoderHandlerErrorCode[];
  readonly decoded: readonly XXXLSvmRuntimeDecodedBytes[];
};

export type XXXLSvmRuntimeDecoderHandlerInput = {
  readonly sourceBundleOk: boolean;
  readonly decodedAccounts: readonly XXXLSvmRuntimeDecodedBytes[];
  readonly decodedInstruction?: XXXLSvmRuntimeDecodedBytes;
};

export type XXXLSvmRuntimeDecoderHandlerResult = {
  readonly version: typeof XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION;
  readonly ok: boolean;
  readonly errors: readonly XXXLSvmRuntimeDecoderHandlerErrorCode[];
  readonly steps: readonly XXXLSvmRuntimeDecoderHandlerStep[];
  readonly handler: typeof XXXL_X1_SVM_HANDLER.ConsumeGatewayMint;
  readonly tokenProgramId: typeof XXXL_X1_SVM_TOKEN_PROGRAM_ID;
  readonly skeletonSteps: readonly string[];
  readonly cpiPrepared: boolean;
  readonly cpiAtomicWithParentTransaction: boolean;
};

const REQUIRED_ACCOUNT_LAYOUT_KINDS: readonly XXXLProductionRuntimeByteLayoutKind[] = [
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GuardianSetAccount,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
] as const;

export function decodeXXXLSvmRuntimeBytes(
  layoutKind: XXXLProductionRuntimeByteLayoutKind,
  bytes: Uint8Array,
): XXXLSvmRuntimeDecodedBytes {
  const layout = layoutByKind(layoutKind);
  const expectedBytes = serializeXXXLLayoutByKind(layoutKind);
  const canonicalHex = xxxlSvmBytesToHex(bytes);
  const expectedCanonicalHex = xxxlSvmBytesToHex(expectedBytes);
  const errors: XXXLSvmRuntimeDecoderHandlerErrorCode[] = [];

  if (bytes.length !== layout.totalSize) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.ByteLengthMismatch);
  }

  if (bytes.length === expectedBytes.length && canonicalHex !== expectedCanonicalHex) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.CanonicalBytesMismatch);
  }

  if (
    readHex(bytes, 0, 8) !== readHex(expectedBytes, 0, 8)
  ) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.DiscriminatorMismatch);
  }

  const versionField = fieldByName(layout, "version");

  if (
    versionField === undefined ||
    !fieldInRange(bytes, versionField) ||
    readU16Le(bytes, versionField.offset) !==
      XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION
  ) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.VersionMismatch);
  }

  const fields = layout.fields.map((field) => {
    if (!fieldInRange(bytes, field)) {
      errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.FieldOutOfRange);
    }

    return decodeField(bytes, field);
  });

  return {
    version: XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION,
    layoutKind,
    byteLength: bytes.length,
    expectedByteLength: layout.totalSize,
    canonicalHex,
    expectedCanonicalHex,
    ok: errors.length === 0,
    errors,
    fields,
  };
}

export function decodeXXXLSvmSerializedRuntimeVector(
  vector: XXXLSvmSerializedRuntimeVector,
): XXXLSvmRuntimeDecodedBytes {
  return decodeXXXLSvmRuntimeBytes(
    vector.layoutKind,
    xxxlSvmHexToBytes(vector.canonicalHex),
  );
}

export function decodeXXXLSvmSerializedRuntimeVectorBundle(
  vectors: readonly XXXLSvmSerializedRuntimeVector[] = xxxlSvmSerializedRuntimeVectors(),
): XXXLSvmRuntimeDecodedVectorBundle {
  const vectorValidation = validateXXXLSvmSerializedRuntimeVectors(vectors);
  const decoded = vectors.map((vector) => decodeXXXLSvmSerializedRuntimeVector(vector));
  const errors: XXXLSvmRuntimeDecoderHandlerErrorCode[] = [];

  if (!vectorValidation.ok) {
    errors.push(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.SerializedVectorValidationFailed,
    );
  }

  if (decoded.some((item) => !item.ok)) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.DecodedBytesInvalid);
  }

  return {
    version: XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION,
    ok: errors.length === 0,
    errors,
    decoded,
  };
}

export function buildXXXLSvmRuntimeDecoderHandlerInputFromVectors(
  vectors: readonly XXXLSvmSerializedRuntimeVector[] = xxxlSvmSerializedRuntimeVectors(),
): XXXLSvmRuntimeDecoderHandlerInput {
  const bundle = decodeXXXLSvmSerializedRuntimeVectorBundle(vectors);

  const decodedInstruction = bundle.decoded.find(
    (decoded) =>
      decoded.layoutKind ===
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
  );

  return {
    sourceBundleOk: bundle.ok,
    decodedAccounts: bundle.decoded.filter(
      (decoded) =>
        decoded.layoutKind !==
        XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    ),
    ...(decodedInstruction === undefined ? {} : { decodedInstruction }),
  };
}

export function executeXXXLSvmRuntimeDecoderHandlerModel(
  input: XXXLSvmRuntimeDecoderHandlerInput = buildXXXLSvmRuntimeDecoderHandlerInputFromVectors(),
): XXXLSvmRuntimeDecoderHandlerResult {
  const steps: XXXLSvmRuntimeDecoderHandlerStep[] = [
    XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.DecodeAccountBytes,
    XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.DecodeInstructionBytes,
    XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.ValidateDecodedLayouts,
    XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.BuildHandlerInput,
  ];
  const errors: XXXLSvmRuntimeDecoderHandlerErrorCode[] = [];

  if (!input.sourceBundleOk) {
    errors.push(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.SerializedVectorValidationFailed,
    );
  }

  if (input.decodedInstruction === undefined) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.MissingInstruction);
  } else if (
    input.decodedInstruction.layoutKind !==
    XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction
  ) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.WrongInstructionKind);
  }

  for (const requiredKind of REQUIRED_ACCOUNT_LAYOUT_KINDS) {
    if (!input.decodedAccounts.some((account) => account.layoutKind === requiredKind)) {
      errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.MissingRequiredAccount);
    }
  }

  if (
    input.decodedAccounts.some((account) => !account.ok) ||
    input.decodedInstruction?.ok === false
  ) {
    errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.DecodedBytesInvalid);
  }

  let skeletonSteps: readonly string[] = [];
  let cpiPrepared = false;
  let cpiAtomicWithParentTransaction = false;

  if (errors.length === 0) {
    steps.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.ExecuteSkeletonBoundary);

    const skeletonResult = executeXXXLX1SvmConsumeGatewayMintSkeleton();

    skeletonSteps = skeletonResult.steps;

    if (!skeletonResult.ok) {
      errors.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.SkeletonExecutionRejected);
    } else {
      steps.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.PrepareCpiBoundary);
      cpiPrepared = skeletonResult.cpiBoundary.prepared;
      cpiAtomicWithParentTransaction =
        skeletonResult.cpiBoundary.atomicWithParentTransaction;
    }
  }

  steps.push(XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.ReturnResult);

  return {
    version: XXXL_SVM_RUNTIME_DECODER_HANDLER_MODEL_VERSION,
    ok: errors.length === 0,
    errors,
    steps,
    handler: XXXL_X1_SVM_HANDLER.ConsumeGatewayMint,
    tokenProgramId: XXXL_X1_SVM_TOKEN_PROGRAM_ID,
    skeletonSteps,
    cpiPrepared,
    cpiAtomicWithParentTransaction,
  };
}

export function findXXXLSvmRuntimeDecodedField(
  decoded: XXXLSvmRuntimeDecodedBytes,
  fieldName: string,
): XXXLSvmRuntimeDecodedField | undefined {
  return decoded.fields.find((field) => field.name === fieldName);
}

export function xxxlCanonicalSvmRuntimeDecoderHandlerReportJson(
  result: XXXLSvmRuntimeDecoderHandlerResult = executeXXXLSvmRuntimeDecoderHandlerModel(),
): string {
  return JSON.stringify([
    ["version", result.version],
    ["ok", result.ok],
    ["errors", result.errors],
    ["steps", result.steps],
    ["handler", result.handler],
    ["tokenProgramId", result.tokenProgramId],
    ["skeletonSteps", result.skeletonSteps],
    ["cpiPrepared", result.cpiPrepared],
    ["cpiAtomicWithParentTransaction", result.cpiAtomicWithParentTransaction],
  ]);
}

export function xxxlSvmBytesToHex(bytes: Uint8Array): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}

export function xxxlSvmHexToBytes(hex: string): Uint8Array {
  const normalized = hex.length % 2 === 0 ? hex : `0${hex}`;
  const bytes = new Uint8Array(normalized.length / 2);

  for (let index = 0; index < normalized.length; index += 2) {
    bytes[index / 2] = Number.parseInt(normalized.slice(index, index + 2), 16);
  }

  return bytes;
}

function decodeField(
  bytes: Uint8Array,
  field: XXXLProductionRuntimeByteField,
): XXXLSvmRuntimeDecodedField {
  return {
    name: field.name,
    type: field.type,
    offset: field.offset,
    size: field.size,
    hex: readHex(bytes, field.offset, field.size),
    value: decodeFieldValue(bytes, field),
  };
}

function decodeFieldValue(
  bytes: Uint8Array,
  field: XXXLProductionRuntimeByteField,
): XXXLSvmRuntimeDecodedFieldValue {
  if (!fieldInRange(bytes, field)) {
    return "OUT_OF_RANGE";
  }

  switch (field.type) {
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Discriminator8:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Pubkey32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Hash32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.FixedBytes32:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.Padding:
      return readHex(bytes, field.offset, field.size);
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.VersionU16Le:
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U16Le:
      return readU16Le(bytes, field.offset);
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U8:
      return bytes[field.offset] ?? 0;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.BoolU8:
      return (bytes[field.offset] ?? 0) === 1;
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U32Le:
      return readU32Le(bytes, field.offset);
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U64Le:
      return readBigIntLe(bytes, field.offset, 8).toString();
    case XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le:
      return readBigIntLe(bytes, field.offset, 16).toString();
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

function fieldByName(
  layout: XXXLProductionRuntimeByteLayout,
  fieldName: string,
): XXXLProductionRuntimeByteField | undefined {
  return layout.fields.find((field) => field.name === fieldName);
}

function fieldInRange(
  bytes: Uint8Array,
  field: XXXLProductionRuntimeByteField,
): boolean {
  return field.offset >= 0 && field.offset + field.size <= bytes.length;
}

function readHex(bytes: Uint8Array, offset: number, size: number): string {
  return xxxlSvmBytesToHex(bytes.slice(offset, offset + size));
}

function readU16Le(bytes: Uint8Array, offset: number): number {
  return (bytes[offset] ?? 0) | ((bytes[offset + 1] ?? 0) << 8);
}

function readU32Le(bytes: Uint8Array, offset: number): number {
  return (
    (bytes[offset] ?? 0) |
    ((bytes[offset + 1] ?? 0) << 8) |
    ((bytes[offset + 2] ?? 0) << 16) |
    ((bytes[offset + 3] ?? 0) << 24)
  ) >>> 0;
}

function readBigIntLe(
  bytes: Uint8Array,
  offset: number,
  byteLength: number,
): bigint {
  let value = 0n;

  for (let index = 0; index < byteLength; index += 1) {
    value |= BigInt(bytes[offset + index] ?? 0) << BigInt(index * 8);
  }

  return value;
}

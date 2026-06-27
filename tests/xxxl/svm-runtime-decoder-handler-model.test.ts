import { describe, expect, it } from "vitest";

import {
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND,
  XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR,
  XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP,
  XXXL_X1_SVM_HANDLER,
  XXXL_X1_SVM_PROGRAM_SKELETON_STEP,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  buildXXXLSvmRuntimeDecoderHandlerInputFromVectors,
  decodeXXXLSvmRuntimeBytes,
  decodeXXXLSvmSerializedRuntimeVectorBundle,
  executeXXXLSvmRuntimeDecoderHandlerModel,
  findXXXLSvmRuntimeDecodedField,
  serializeXXXLLayoutByKind,
  xxxlCanonicalSvmRuntimeDecoderHandlerReportJson,
  xxxlSvmBytesToHex,
  xxxlSvmHexToBytes,
  type XXXLSvmRuntimeDecodedBytes,
} from "../../src/index.js";

function decodedByKind(kind: string): XXXLSvmRuntimeDecodedBytes {
  const bundle = decodeXXXLSvmSerializedRuntimeVectorBundle();
  const decoded = bundle.decoded.find((item) => item.layoutKind === kind);

  expect(decoded).toBeDefined();

  return decoded as XXXLSvmRuntimeDecodedBytes;
}

function fieldValue(decoded: XXXLSvmRuntimeDecodedBytes, fieldName: string): unknown {
  const field = findXXXLSvmRuntimeDecodedField(decoded, fieldName);

  expect(field).toBeDefined();

  return field?.value;
}

describe("XXXL SVM runtime decoder handler model", () => {
  it("decodes serialized vector bundle successfully", () => {
    const bundle = decodeXXXLSvmSerializedRuntimeVectorBundle();

    expect(bundle.ok).toBe(true);
    expect(bundle.errors).toEqual([]);
    expect(bundle.decoded).toHaveLength(6);
  });

  it("decodes Mint State account fields", () => {
    const decoded = decodedByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    );

    expect(decoded.ok).toBe(true);
    expect(decoded.byteLength).toBe(176);
    expect(fieldValue(decoded, "version")).toBe(1);
    expect(fieldValue(decoded, "decimals")).toBe(18);
    expect(fieldValue(decoded, "totalSupply")).toBe("500");
  });

  it("decodes Mint State gateway mint authority PDA bytes", () => {
    const decoded = decodedByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    );
    const field = findXXXLSvmRuntimeDecodedField(
      decoded,
      "gatewayMintAuthorityPda",
    );

    expect(field?.offset).toBe(64);
    expect(field?.size).toBe(32);
    expect(String(field?.value)).toHaveLength(64);
  });

  it("decodes Gateway Config route policy fields", () => {
    const decoded = decodedByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
    );

    expect(decoded.byteLength).toBe(256);
    expect(fieldValue(decoded, "sourceChainWeightBps")).toBe(10_000);
    expect(fieldValue(decoded, "sourceChainId")).toBe("1");
    expect(fieldValue(decoded, "perEventCap")).toBe("10000");
  });

  it("decodes Guardian Set fixed-capacity fields", () => {
    const decoded = decodedByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GuardianSetAccount,
    );
    const guardianPubkeys = findXXXLSvmRuntimeDecodedField(
      decoded,
      "guardianPubkeys",
    );

    expect(decoded.byteLength).toBe(320);
    expect(fieldValue(decoded, "quorumThreshold")).toBe(2);
    expect(fieldValue(decoded, "guardianCount")).toBe(3);
    expect(guardianPubkeys?.size).toBe(256);
  });

  it("decodes Processed Event replay fields", () => {
    const decoded = decodedByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
    );

    expect(decoded.byteLength).toBe(144);
    expect(fieldValue(decoded, "consumed")).toBe(true);
    expect(fieldValue(decoded, "consumedAmount")).toBe("1000");
    expect(fieldValue(decoded, "consumedSlot")).toBe("123456");
  });

  it("decodes Recipient Balance mirror fields", () => {
    const decoded = decodedByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
    );

    expect(decoded.byteLength).toBe(144);
    expect(fieldValue(decoded, "balance")).toBe("200");
    expect(String(fieldValue(decoded, "owner"))).toHaveLength(64);
  });

  it("decodes consume_gateway_mint instruction fields", () => {
    const decoded = decodedByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );

    expect(decoded.byteLength).toBe(208);
    expect(fieldValue(decoded, "accountMetaCount")).toBe(9);
    expect(fieldValue(decoded, "routeAccountIndex")).toBe(1);
    expect(fieldValue(decoded, "amount")).toBe("1000");
  });

  it("roundtrips hex and bytes", () => {
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );
    const hex = xxxlSvmBytesToHex(bytes);

    expect(xxxlSvmBytesToHex(xxxlSvmHexToBytes(hex))).toBe(hex);
  });

  it("decodes direct bytes by layout kind", () => {
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
    );
    const decoded = decodeXXXLSvmRuntimeBytes(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
      bytes,
    );

    expect(decoded.ok).toBe(true);
    expect(fieldValue(decoded, "dailyCap")).toBe("100000");
  });

  it("rejects wrong byte length", () => {
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    ).slice(0, 100);
    const decoded = decodeXXXLSvmRuntimeBytes(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
      bytes,
    );

    expect(decoded.ok).toBe(false);
    expect(decoded.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.ByteLengthMismatch,
    );
    expect(decoded.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.FieldOutOfRange,
    );
  });

  it("rejects discriminator mismatch", () => {
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    );

    bytes[0] = (bytes[0] ?? 0) ^ 0xff;

    const decoded = decodeXXXLSvmRuntimeBytes(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
      bytes,
    );

    expect(decoded.ok).toBe(false);
    expect(decoded.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.DiscriminatorMismatch,
    );
    expect(decoded.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.CanonicalBytesMismatch,
    );
  });

  it("rejects version mismatch", () => {
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
    );

    bytes[8] = 2;

    const decoded = decodeXXXLSvmRuntimeBytes(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
      bytes,
    );

    expect(decoded.ok).toBe(false);
    expect(decoded.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.VersionMismatch,
    );
  });

  it("rejects canonical byte corruption outside discriminator and version", () => {
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );

    bytes[176] = (bytes[176] ?? 0) ^ 0x01;

    const decoded = decodeXXXLSvmRuntimeBytes(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
      bytes,
    );

    expect(decoded.ok).toBe(false);
    expect(decoded.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.CanonicalBytesMismatch,
    );
  });

  it("builds handler input from serialized vectors", () => {
    const input = buildXXXLSvmRuntimeDecoderHandlerInputFromVectors();

    expect(input.sourceBundleOk).toBe(true);
    expect(input.decodedAccounts).toHaveLength(5);
    expect(input.decodedInstruction?.layoutKind).toBe(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );
  });

  it("executes valid decoder handler model", () => {
    const result = executeXXXLSvmRuntimeDecoderHandlerModel();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.cpiPrepared).toBe(true);
    expect(result.cpiAtomicWithParentTransaction).toBe(true);
  });

  it("records decoder handler step order", () => {
    const result = executeXXXLSvmRuntimeDecoderHandlerModel();

    expect(result.steps).toEqual([
      XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.DecodeAccountBytes,
      XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.DecodeInstructionBytes,
      XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.ValidateDecodedLayouts,
      XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.BuildHandlerInput,
      XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.ExecuteSkeletonBoundary,
      XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.PrepareCpiBoundary,
      XXXL_SVM_RUNTIME_DECODER_HANDLER_STEP.ReturnResult,
    ]);
  });

  it("keeps skeleton boundary attached to decoder handler result", () => {
    const result = executeXXXLSvmRuntimeDecoderHandlerModel();

    expect(result.handler).toBe(XXXL_X1_SVM_HANDLER.ConsumeGatewayMint);
    expect(result.tokenProgramId).toBe(XXXL_X1_SVM_TOKEN_PROGRAM_ID);
    expect(result.skeletonSteps).toContain(
      XXXL_X1_SVM_PROGRAM_SKELETON_STEP.PrepareSplTokenMintToCpi,
    );
  });

  it("rejects missing required account", () => {
    const input = buildXXXLSvmRuntimeDecoderHandlerInputFromVectors();
    const result = executeXXXLSvmRuntimeDecoderHandlerModel({
      ...input,
      decodedAccounts: input.decodedAccounts.slice(1),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.MissingRequiredAccount,
    );
    expect(result.cpiPrepared).toBe(false);
  });

  it("rejects corrupted decoded account before CPI boundary", () => {
    const input = buildXXXLSvmRuntimeDecoderHandlerInputFromVectors();
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    );

    bytes[48] = (bytes[48] ?? 0) ^ 0x01;

    const corrupted = decodeXXXLSvmRuntimeBytes(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
      bytes,
    );

    const result = executeXXXLSvmRuntimeDecoderHandlerModel({
      ...input,
      sourceBundleOk: false,
      decodedAccounts: [
        corrupted,
        ...input.decodedAccounts.filter(
          (account) =>
            account.layoutKind !==
            XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
        ),
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.DecodedBytesInvalid,
    );
    expect(result.cpiPrepared).toBe(false);
  });

  it("rejects corrupted decoded instruction before CPI boundary", () => {
    const input = buildXXXLSvmRuntimeDecoderHandlerInputFromVectors();
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );

    bytes[176] = (bytes[176] ?? 0) ^ 0x01;

    const corrupted = decodeXXXLSvmRuntimeBytes(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
      bytes,
    );

    const result = executeXXXLSvmRuntimeDecoderHandlerModel({
      ...input,
      sourceBundleOk: false,
      decodedInstruction: corrupted,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_RUNTIME_DECODER_HANDLER_ERROR.DecodedBytesInvalid,
    );
    expect(result.cpiPrepared).toBe(false);
  });

  it("exports deterministic decoder handler report JSON", () => {
    const result = executeXXXLSvmRuntimeDecoderHandlerModel();
    const json = xxxlCanonicalSvmRuntimeDecoderHandlerReportJson(result);

    expect(json).toBe(xxxlCanonicalSvmRuntimeDecoderHandlerReportJson(result));
    expect(json).toContain('["handler","consume_gateway_mint"]');
    expect(json).toContain('["cpiPrepared",true]');
  });
});

import { describe, expect, it } from "vitest";

import {
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND,
  XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR,
  XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID,
  XXXL_X1_SVM_ACCOUNT_ROLE,
  XXXL_X1_SVM_HANDLER,
  XXXL_X1_SVM_TOKEN_PROGRAM_ID,
  serializeXXXLLayoutByKind,
  validateXXXLSvmSerializedRuntimeVectors,
  xxxlCanonicalSvmSerializedRuntimeBundleJson,
  xxxlSvmSerializedRuntimeBundle,
  xxxlSvmSerializedRuntimeVectors,
  type XXXLSvmSerializedRuntimeFieldProbe,
  type XXXLSvmSerializedRuntimeVector,
} from "../../src/index.js";

function vectorById(id: string): XXXLSvmSerializedRuntimeVector {
  const vector = xxxlSvmSerializedRuntimeVectors().find(
    (item) => item.vectorId === id,
  );

  expect(vector).toBeDefined();

  return vector as XXXLSvmSerializedRuntimeVector;
}

function probeByName(
  vector: XXXLSvmSerializedRuntimeVector,
  fieldName: string,
): XXXLSvmSerializedRuntimeFieldProbe {
  const probe = vector.fieldProbes.find((item) => item.fieldName === fieldName);

  expect(probe).toBeDefined();

  return probe as XXXLSvmSerializedRuntimeFieldProbe;
}

describe("XXXL SVM serialized runtime vectors", () => {
  it("exports serialized runtime vectors in canonical order", () => {
    expect(xxxlSvmSerializedRuntimeVectors().map((vector) => vector.vectorId)).toEqual([
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.MintStateAccount,
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.GatewayConfigAccount,
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.GuardianSetAccount,
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.ProcessedEventAccount,
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.RecipientBalanceAccount,
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.ConsumeGatewayMintInstruction,
    ]);
  });

  it("validates default serialized runtime vectors", () => {
    const result = validateXXXLSvmSerializedRuntimeVectors();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("serializes Mint State bytes with expected length and u128 total supply", () => {
    const vector = vectorById(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.MintStateAccount,
    );

    expect(vector.layoutKind).toBe(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    );
    expect(vector.byteLength).toBe(176);
    expect(vector.canonicalHex).toHaveLength(176 * 2);
    expect(probeByName(vector, "totalSupply")).toMatchObject({
      offset: 48,
      size: 16,
      hex: "f4010000000000000000000000000000",
    });
  });

  it("serializes Mint State with gateway mint authority PDA probe", () => {
    const vector = vectorById(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.MintStateAccount,
    );

    expect(probeByName(vector, "gatewayMintAuthorityPda")).toMatchObject({
      offset: 64,
      size: 32,
    });
  });

  it("serializes Gateway Config route policy fields", () => {
    const vector = vectorById(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.GatewayConfigAccount,
    );

    expect(vector.byteLength).toBe(256);
    expect(probeByName(vector, "sourceChainWeightBps")).toMatchObject({
      offset: 12,
      size: 2,
      hex: "1027",
    });
    expect(probeByName(vector, "perEventCap")).toMatchObject({
      offset: 192,
      size: 16,
      hex: "10270000000000000000000000000000",
    });
  });

  it("serializes Guardian Set fixed-capacity area and hash", () => {
    const vector = vectorById(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.GuardianSetAccount,
    );

    expect(vector.byteLength).toBe(320);
    expect(probeByName(vector, "guardianPubkeys")).toMatchObject({
      offset: 16,
      size: 256,
    });
    expect(probeByName(vector, "guardianKeyHash")).toMatchObject({
      offset: 272,
      size: 32,
    });
  });

  it("serializes Processed Event replay-protection fields", () => {
    const vector = vectorById(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.ProcessedEventAccount,
    );

    expect(vector.byteLength).toBe(144);
    expect(probeByName(vector, "consumed")).toMatchObject({
      offset: 10,
      size: 1,
      hex: "01",
    });
    expect(probeByName(vector, "consumedAmount")).toMatchObject({
      offset: 112,
      size: 16,
      hex: "e8030000000000000000000000000000",
    });
  });

  it("serializes Recipient Balance mirror fields", () => {
    const vector = vectorById(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.RecipientBalanceAccount,
    );

    expect(vector.byteLength).toBe(144);
    expect(probeByName(vector, "owner")).toMatchObject({
      offset: 16,
      size: 32,
    });
    expect(probeByName(vector, "balance")).toMatchObject({
      offset: 80,
      size: 16,
      hex: "c8000000000000000000000000000000",
    });
  });

  it("serializes consume_gateway_mint instruction bytes", () => {
    const vector = vectorById(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.ConsumeGatewayMintInstruction,
    );

    expect(vector.layoutKind).toBe(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );
    expect(vector.byteLength).toBe(208);
    expect(probeByName(vector, "accountMetaCount")).toMatchObject({
      offset: 10,
      size: 1,
      hex: "09",
    });
    expect(probeByName(vector, "amount")).toMatchObject({
      offset: 176,
      size: 16,
      hex: "e8030000000000000000000000000000",
    });
  });

  it("exposes raw serialization by layout kind", () => {
    const bytes = serializeXXXLLayoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );

    expect(bytes).toHaveLength(208);
    expect(Array.from(bytes.slice(176, 192))).toEqual([
      232, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
  });

  it("bundles serialized vectors with X1/SVM skeleton boundary", () => {
    const bundle = xxxlSvmSerializedRuntimeBundle();

    expect(bundle.handler).toBe(XXXL_X1_SVM_HANDLER.ConsumeGatewayMint);
    expect(bundle.tokenProgramId).toBe(XXXL_X1_SVM_TOKEN_PROGRAM_ID);
    expect(bundle.cpiPrepared).toBe(true);
    expect(bundle.cpiAtomicWithParentTransaction).toBe(true);
    expect(bundle.vectors).toHaveLength(6);
  });

  it("keeps account meta roles attached to bundle", () => {
    const bundle = xxxlSvmSerializedRuntimeBundle();

    expect(bundle.accountMetaRoles).toEqual([
      XXXL_X1_SVM_ACCOUNT_ROLE.MintState,
      XXXL_X1_SVM_ACCOUNT_ROLE.GatewayConfig,
      XXXL_X1_SVM_ACCOUNT_ROLE.GuardianSet,
      XXXL_X1_SVM_ACCOUNT_ROLE.ProcessedEvent,
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientBalance,
      XXXL_X1_SVM_ACCOUNT_ROLE.SplTokenMint,
      XXXL_X1_SVM_ACCOUNT_ROLE.RecipientTokenAccount,
      XXXL_X1_SVM_ACCOUNT_ROLE.MintAuthorityPda,
      XXXL_X1_SVM_ACCOUNT_ROLE.TokenProgram,
    ]);
  });

  it("exports deterministic canonical bundle JSON", () => {
    const bundle = xxxlSvmSerializedRuntimeBundle();
    const json = xxxlCanonicalSvmSerializedRuntimeBundleJson(bundle);

    expect(json).toBe(xxxlCanonicalSvmSerializedRuntimeBundleJson(bundle));
    expect(json).toContain('["handler","consume_gateway_mint"]');
    expect(json).toContain('["byteLength",208]');
  });

  it("detects missing serialized vector", () => {
    const vectors = xxxlSvmSerializedRuntimeVectors().filter(
      (vector) =>
        vector.vectorId !==
        XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.MintStateAccount,
    );

    const result = validateXXXLSvmSerializedRuntimeVectors(vectors);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.MissingVector,
    );
  });

  it("detects duplicate serialized vector", () => {
    const vectors = xxxlSvmSerializedRuntimeVectors();
    const result = validateXXXLSvmSerializedRuntimeVectors([
      ...vectors,
      vectorById(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.MintStateAccount),
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.DuplicateVector,
    );
  });

  it("detects wrong canonical hex", () => {
    const vectors = xxxlSvmSerializedRuntimeVectors();
    const broken = [
      {
        ...vectorById(XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ID.MintStateAccount),
        canonicalHex: "00",
      },
      ...vectors.slice(1),
    ];

    const result = validateXXXLSvmSerializedRuntimeVectors(broken);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_SVM_SERIALIZED_RUNTIME_VECTOR_ERROR.WrongCanonicalHex,
    );
  });
});

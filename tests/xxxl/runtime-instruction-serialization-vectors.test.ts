import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_CONSUME_GATEWAY_MINT_ACCOUNT_META_ORDER,
  XXXL_RUNTIME_CONSUME_GATEWAY_MINT_FIELD_ORDER,
  XXXL_RUNTIME_INSTRUCTION,
  XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE,
  XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ENCODING,
  XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR,
  XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_VERSION,
  xxxlCanonicalRuntimeInstructionJson,
  xxxlRuntimeConsumeGatewayMintExpectedAccountMetas,
  xxxlRuntimeInstructionSerializationLayouts,
  xxxlRuntimeInstructionSerializationVectors,
  validateXXXLRuntimeInstructionSerializationLayouts,
  validateXXXLRuntimeInstructionSerializationVectors,
  type XXXLRuntimeInstructionSerializationLayout,
} from "../../src/index.js";

describe("XXXL runtime instruction serialization vectors", () => {
  it("defines one canonical layout for CONSUME_GATEWAY_MINT", () => {
    const layouts = xxxlRuntimeInstructionSerializationLayouts();
    const result = validateXXXLRuntimeInstructionSerializationLayouts(layouts);

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
    expect(layouts.map((layout) => layout.instruction)).toEqual([
      XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
    ]);
  });

  it("uses canonical binary v1 and layout version 1", () => {
    const layout = xxxlRuntimeInstructionSerializationLayouts()[0];

    expect(layout).toBeDefined();
    if (!layout) {
      throw new Error("missing layout");
    }

    expect(layout.version).toBe(XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_VERSION);
    expect(layout.encoding).toBe(
      XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ENCODING.CanonicalBinaryV1,
    );
  });

  it("keeps account metas in deterministic SVM order", () => {
    const layout = xxxlRuntimeInstructionSerializationLayouts()[0];

    expect(layout).toBeDefined();
    if (!layout) {
      throw new Error("missing layout");
    }

    expect(layout.accountMetas.map((meta) => meta.role)).toEqual(
      XXXL_RUNTIME_CONSUME_GATEWAY_MINT_ACCOUNT_META_ORDER,
    );
    expect(layout.accountMetas.map((meta) => meta.position)).toEqual(
      layout.accountMetas.map((_, index) => index),
    );
  });

  it("keeps instruction and version as first serialized fields", () => {
    const layout = xxxlRuntimeInstructionSerializationLayouts()[0];

    expect(layout).toBeDefined();
    if (!layout) {
      throw new Error("missing layout");
    }

    expect(layout.fields[0]?.name).toBe("instruction");
    expect(layout.fields[1]?.name).toBe("version");
    expect(layout.fields.map((field) => field.name)).toEqual(
      XXXL_RUNTIME_CONSUME_GATEWAY_MINT_FIELD_ORDER,
    );
  });

  it("marks writable accounts and CPI signer boundary explicitly", () => {
    const metas = xxxlRuntimeConsumeGatewayMintExpectedAccountMetas();
    const mintAuthority = metas.find(
      (meta) =>
        meta.role === XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda,
    );
    const tokenProgram = metas.find(
      (meta) => meta.role === XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram,
    );

    expect(mintAuthority).toMatchObject({
      writable: false,
      parentInstructionSigner: false,
      cpiSigner: true,
    });

    expect(tokenProgram).toMatchObject({
      writable: false,
      parentInstructionSigner: false,
      cpiSigner: false,
    });
  });

  it("creates deterministic instruction serialization vectors", () => {
    const layouts = xxxlRuntimeInstructionSerializationLayouts();
    const vectors = xxxlRuntimeInstructionSerializationVectors();
    const result = validateXXXLRuntimeInstructionSerializationVectors(
      layouts,
      vectors,
    );

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
    expect(vectors).toHaveLength(1);
  });

  it("serializes amount bigint as decimal string in canonical JSON", () => {
    const vector = xxxlRuntimeInstructionSerializationVectors()[0];

    expect(vector).toBeDefined();
    if (!vector) {
      throw new Error("missing vector");
    }

    expect(vector.canonicalJson).toContain('"1000000000000"');
    expect(vector.canonicalJson).not.toContain("1000000000000n");
    expect(JSON.parse(vector.canonicalJson)[0]).toEqual([
      "instruction",
      XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
    ]);
  });

  it("canonical JSON is derived from layout field order", () => {
    const layout = xxxlRuntimeInstructionSerializationLayouts()[0];
    const vector = xxxlRuntimeInstructionSerializationVectors()[0];

    expect(layout).toBeDefined();
    expect(vector).toBeDefined();
    if (!layout || !vector) {
      throw new Error("missing layout or vector");
    }

    expect(xxxlCanonicalRuntimeInstructionJson(layout, vector.data)).toBe(
      vector.canonicalJson,
    );
  });

  it("rejects missing instruction layout", () => {
    const result = validateXXXLRuntimeInstructionSerializationLayouts([]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.MissingLayout,
    );
  });

  it("rejects duplicate instruction layouts", () => {
    const layouts = xxxlRuntimeInstructionSerializationLayouts();
    const result = validateXXXLRuntimeInstructionSerializationLayouts([
      ...layouts,
      layouts[0] as XXXLRuntimeInstructionSerializationLayout,
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.DuplicateLayout,
    );
  });

  it("rejects wrong account meta order", () => {
    const layout = xxxlRuntimeInstructionSerializationLayouts()[0];

    expect(layout).toBeDefined();
    if (!layout) {
      throw new Error("missing layout");
    }

    const mutatedLayout: XXXLRuntimeInstructionSerializationLayout = {
      ...layout,
      accountMetas: [
        layout.accountMetas[1]!,
        layout.accountMetas[0]!,
        ...layout.accountMetas.slice(2),
      ],
    };

    const result = validateXXXLRuntimeInstructionSerializationLayouts([
      mutatedLayout,
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongAccountMetaOrder,
    );
  });

  it("requires mint authority PDA to sign SPL Token CPI", () => {
    const layout = xxxlRuntimeInstructionSerializationLayouts()[0];

    expect(layout).toBeDefined();
    if (!layout) {
      throw new Error("missing layout");
    }

    const mutatedLayout: XXXLRuntimeInstructionSerializationLayout = {
      ...layout,
      accountMetas: layout.accountMetas.map((meta) =>
        meta.role === XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda
          ? { ...meta, cpiSigner: false }
          : meta,
      ),
    };

    const result = validateXXXLRuntimeInstructionSerializationLayouts([
      mutatedLayout,
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR
        .MintAuthorityPdaDoesNotSignCpi,
    );
  });

  it("rejects vectors with wrong canonical JSON", () => {
    const layouts = xxxlRuntimeInstructionSerializationLayouts();
    const vectors = xxxlRuntimeInstructionSerializationVectors();
    const result = validateXXXLRuntimeInstructionSerializationVectors(layouts, [
      {
        ...vectors[0]!,
        canonicalJson: "[]",
      },
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_INSTRUCTION_SERIALIZATION_ERROR.WrongCanonicalJson,
    );
  });
});

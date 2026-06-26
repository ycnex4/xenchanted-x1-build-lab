import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING,
  XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR,
  XXXL_RUNTIME_ACCOUNT_SERIALIZATION_FIELD_ORDER,
  XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION,
  xxxlCanonicalRuntimeAccountJson,
  xxxlRuntimeAccountSerializationLayouts,
  xxxlRuntimeAccountSerializationVectors,
  validateXXXLRuntimeAccountSerializationLayouts,
  validateXXXLRuntimeAccountSerializationVectors,
  type XXXLRuntimeAccountSerializationLayout,
} from "../../src/index.js";

describe("XXXL runtime account serialization vectors", () => {
  it("defines one canonical layout for each mandatory runtime account kind", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();
    const result = validateXXXLRuntimeAccountSerializationLayouts(layouts);

    expect(result.ok).toBe(true);
    expect(layouts.map((layout) => layout.accountKind)).toEqual([
      XXXL_RUNTIME_ACCOUNT_KIND.MintState,
      XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
      XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
      XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
      XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
    ]);
  });

  it("uses canonical binary v1 and layout version 1 for every account layout", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();

    for (const layout of layouts) {
      expect(layout.version).toBe(XXXL_RUNTIME_ACCOUNT_SERIALIZATION_VERSION);
      expect(layout.encoding).toBe(
        XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ENCODING.CanonicalBinaryV1,
      );
    }
  });

  it("keeps kind and version as the first two fields in every layout", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();

    for (const layout of layouts) {
      expect(layout.fields[0]?.name).toBe("kind");
      expect(layout.fields[1]?.name).toBe("version");
      expect(layout.fields.map((field) => field.position)).toEqual(
        layout.fields.map((_, index) => index),
      );
    }
  });

  it("matches the expected field order for every account kind", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();

    for (const layout of layouts) {
      expect(layout.fields.map((field) => field.name)).toEqual(
        XXXL_RUNTIME_ACCOUNT_SERIALIZATION_FIELD_ORDER[layout.accountKind],
      );
    }
  });

  it("creates deterministic account serialization vectors", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();
    const vectors = xxxlRuntimeAccountSerializationVectors();
    const result = validateXXXLRuntimeAccountSerializationVectors(
      layouts,
      vectors,
    );

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
    expect(vectors).toHaveLength(5);
  });

  it("serializes bigint values as decimal strings in canonical JSON", () => {
    const mintVector = xxxlRuntimeAccountSerializationVectors().find(
      (vector) => vector.accountKind === XXXL_RUNTIME_ACCOUNT_KIND.MintState,
    );

    expect(mintVector).toBeDefined();
    if (!mintVector) {
      throw new Error("missing mint vector");
    }

    expect(mintVector.canonicalJson).toContain('"1000000000000"');
    expect(mintVector.canonicalJson).not.toContain("1000000000000n");
    expect(JSON.parse(mintVector.canonicalJson)[0]).toEqual([
      "kind",
      XXXL_RUNTIME_ACCOUNT_KIND.MintState,
    ]);
  });

  it("canonical JSON is derived from layout field order", () => {
    const layout = xxxlRuntimeAccountSerializationLayouts()[0];
    const vector = xxxlRuntimeAccountSerializationVectors()[0];

    expect(layout).toBeDefined();
    expect(vector).toBeDefined();
    if (!layout || !vector) {
      throw new Error("missing layout or vector");
    }

    expect(xxxlCanonicalRuntimeAccountJson(layout, vector.account)).toBe(
      vector.canonicalJson,
    );
  });

  it("rejects a missing account layout", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts().filter(
      (layout) => layout.accountKind !== XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
    );

    const result = validateXXXLRuntimeAccountSerializationLayouts(layouts);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.MissingLayout,
    );
  });

  it("rejects duplicate account layouts", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();
    const result = validateXXXLRuntimeAccountSerializationLayouts([
      ...layouts,
      layouts[0] as XXXLRuntimeAccountSerializationLayout,
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.DuplicateLayout,
    );
  });

  it("rejects wrong field order", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();
    const mintLayout = layouts[0] as XXXLRuntimeAccountSerializationLayout;
    const mutatedLayout: XXXLRuntimeAccountSerializationLayout = {
      ...mintLayout,
      fields: [
        mintLayout.fields[1]!,
        mintLayout.fields[0]!,
        ...mintLayout.fields.slice(2),
      ],
    };

    const result = validateXXXLRuntimeAccountSerializationLayouts([
      mutatedLayout,
      ...layouts.slice(1),
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.WrongFieldOrder,
    );
  });

  it("rejects duplicate fields", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();
    const mintLayout = layouts[0] as XXXLRuntimeAccountSerializationLayout;
    const mutatedLayout: XXXLRuntimeAccountSerializationLayout = {
      ...mintLayout,
      fields: [...mintLayout.fields, mintLayout.fields[0]!],
    };

    const result = validateXXXLRuntimeAccountSerializationLayouts([
      mutatedLayout,
      ...layouts.slice(1),
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.DuplicateField,
    );
  });

  it("rejects vectors with wrong canonical JSON", () => {
    const layouts = xxxlRuntimeAccountSerializationLayouts();
    const vectors = xxxlRuntimeAccountSerializationVectors();
    const result = validateXXXLRuntimeAccountSerializationVectors(layouts, [
      {
        ...vectors[0]!,
        canonicalJson: "[]",
      },
      ...vectors.slice(1),
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_ACCOUNT_SERIALIZATION_ERROR.WrongCanonicalJson,
    );
  });
});

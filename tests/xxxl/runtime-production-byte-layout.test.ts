import { describe, expect, it } from "vitest";

import {
  XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE,
  XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ENCODING,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND,
  XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_VERSION,
  XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE,
  validateXXXLProductionRuntimeByteLayout,
  validateXXXLProductionRuntimeByteLayouts,
  xxxlCanonicalProductionRuntimeByteLayoutJson,
  xxxlProductionRuntimeAccountByteLayouts,
  xxxlProductionRuntimeByteLayouts,
  xxxlProductionRuntimeInstructionByteLayouts,
  type XXXLProductionRuntimeByteField,
  type XXXLProductionRuntimeByteLayout,
} from "../../src/index.js";

function layoutByKind(kind: string): XXXLProductionRuntimeByteLayout {
  const layout = xxxlProductionRuntimeByteLayouts().find(
    (item) => item.kind === kind,
  );

  expect(layout).toBeDefined();

  return layout as XXXLProductionRuntimeByteLayout;
}

function fieldByName(
  layout: XXXLProductionRuntimeByteLayout,
  name: string,
): XXXLProductionRuntimeByteField {
  const field = layout.fields.find((item) => item.name === name);

  expect(field).toBeDefined();

  return field as XXXLProductionRuntimeByteField;
}

describe("XXXL production runtime byte layout", () => {
  it("exports account byte layouts in canonical order", () => {
    expect(xxxlProductionRuntimeAccountByteLayouts().map((layout) => layout.kind)).toEqual([
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GuardianSetAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
    ]);
  });

  it("exports instruction byte layouts in canonical order", () => {
    expect(xxxlProductionRuntimeInstructionByteLayouts().map((layout) => layout.kind)).toEqual([
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    ]);
  });

  it("validates all production byte layouts", () => {
    const result = validateXXXLProductionRuntimeByteLayouts();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("uses fixed binary little-endian v1 metadata", () => {
    for (const layout of xxxlProductionRuntimeByteLayouts()) {
      expect(layout.version).toBe(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_VERSION);
      expect(layout.encoding).toBe(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ENCODING);
      expect(layout.alignment).toBe(8);
    }
  });

  it("defines exact account total sizes", () => {
    expect(
      layoutByKind(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount)
        .totalSize,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.MintState);
    expect(
      layoutByKind(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount)
        .totalSize,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GatewayConfig);
    expect(
      layoutByKind(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GuardianSetAccount)
        .totalSize,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GuardianSet);
    expect(
      layoutByKind(XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount)
        .totalSize,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.ProcessedEvent);
    expect(
      layoutByKind(
        XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
      ).totalSize,
    ).toBe(XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.RecipientBalance);
  });

  it("defines exact consume gateway mint instruction total size", () => {
    expect(
      layoutByKind(
        XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
      ).totalSize,
    ).toBe(
      XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE.ConsumeGatewayMint,
    );
  });

  it("defines mint state authority surfaces and PDA offset", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    );

    expect(fieldByName(layout, "totalSupply")).toMatchObject({
      offset: 48,
      size: 16,
      type: XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le,
    });
    expect(fieldByName(layout, "gatewayMintAuthorityPda")).toMatchObject({
      offset: 64,
      size: 32,
    });
    expect(fieldByName(layout, "programUpgradeAuthority")).toMatchObject({
      offset: 96,
      size: 32,
    });
    expect(fieldByName(layout, "splTokenMintAuthority")).toMatchObject({
      offset: 128,
      size: 32,
    });
  });

  it("defines gateway config route policy caps with u128 alignment", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GatewayConfigAccount,
    );

    expect(fieldByName(layout, "sourceChainWeightBps")).toMatchObject({
      offset: 12,
      size: 2,
    });
    expect(fieldByName(layout, "perEventCap")).toMatchObject({
      offset: 192,
      size: 16,
      type: XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le,
    });
    expect(fieldByName(layout, "dailyCap")).toMatchObject({
      offset: 208,
      size: 16,
    });
    expect(fieldByName(layout, "epochCap")).toMatchObject({
      offset: 224,
      size: 16,
    });
  });

  it("defines guardian set fixed pubkey capacity", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.GuardianSetAccount,
    );

    expect(fieldByName(layout, "guardianPubkeys")).toMatchObject({
      offset: 16,
      size: 256,
    });
    expect(fieldByName(layout, "guardianKeyHash")).toMatchObject({
      offset: 272,
      size: 32,
    });
  });

  it("defines processed event replay-protection fields", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
    );

    expect(fieldByName(layout, "consumed")).toMatchObject({
      offset: 10,
      size: 1,
      type: XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.BoolU8,
    });
    expect(fieldByName(layout, "canonicalEventKey")).toMatchObject({
      offset: 16,
      size: 32,
    });
    expect(fieldByName(layout, "consumedAmount")).toMatchObject({
      offset: 112,
      size: 16,
    });
  });

  it("defines recipient balance mirror fields", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
    );

    expect(fieldByName(layout, "owner")).toMatchObject({
      offset: 16,
      size: 32,
    });
    expect(fieldByName(layout, "mint")).toMatchObject({
      offset: 48,
      size: 32,
    });
    expect(fieldByName(layout, "balance")).toMatchObject({
      offset: 80,
      size: 16,
    });
  });

  it("defines consume gateway mint instruction account indices and data offsets", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );

    expect(fieldByName(layout, "accountMetaCount")).toMatchObject({
      offset: 10,
      size: 1,
    });
    expect(fieldByName(layout, "routeAccountIndex")).toMatchObject({
      offset: 11,
      size: 1,
    });
    expect(fieldByName(layout, "canonicalEventKey")).toMatchObject({
      offset: 112,
      size: 32,
    });
    expect(fieldByName(layout, "amount")).toMatchObject({
      offset: 176,
      size: 16,
      type: XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le,
    });
  });

  it("keeps all u128 fields 16-byte aligned", () => {
    for (const layout of xxxlProductionRuntimeByteLayouts()) {
      for (const field of layout.fields) {
        if (field.type === XXXL_PRODUCTION_RUNTIME_BYTE_FIELD_TYPE.U128Le) {
          expect(field.offset % 16).toBe(0);
        }
      }
    }
  });

  it("keeps all layouts contiguous and 8-byte total aligned", () => {
    for (const layout of xxxlProductionRuntimeByteLayouts()) {
      let expectedOffset = 0;

      for (const field of layout.fields) {
        expect(field.offset).toBe(expectedOffset);
        expectedOffset += field.size;
      }

      expect(expectedOffset).toBe(layout.totalSize);
      expect(layout.totalSize % 8).toBe(0);
    }
  });

  it("detects wrong total size", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.MintStateAccount,
    );

    const result = validateXXXLProductionRuntimeByteLayout({
      ...layout,
      totalSize: layout.totalSize + 1,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.WrongTotalSize,
    );
  });

  it("detects non-contiguous fields and invalid alignment", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ProcessedEventAccount,
    );

    const brokenFields = layout.fields.map((field) =>
      field.name === "canonicalEventKey"
        ? {
            ...field,
            offset: 17,
          }
        : field,
    );

    const result = validateXXXLProductionRuntimeByteLayout({
      ...layout,
      fields: brokenFields,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.NonContiguousFields,
    );
    expect(result.errors).toContain(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.InvalidAlignment,
    );
  });

  it("detects missing discriminator and duplicate field names", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.RecipientBalanceAccount,
    );

    const brokenFields = [
      {
        ...layout.fields[0]!,
        name: "notDiscriminator",
      },
      ...layout.fields.slice(1),
      fieldByName(layout, "balance"),
    ];

    const result = validateXXXLProductionRuntimeByteLayout({
      ...layout,
      fields: brokenFields,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.MissingDiscriminator,
    );
    expect(result.errors).toContain(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_ERROR.DuplicateFieldName,
    );
  });

  it("exports deterministic canonical layout JSON", () => {
    const layout = layoutByKind(
      XXXL_PRODUCTION_RUNTIME_BYTE_LAYOUT_KIND.ConsumeGatewayMintInstruction,
    );

    const json = xxxlCanonicalProductionRuntimeByteLayoutJson(layout);

    expect(json).toContain('["kind","CONSUME_GATEWAY_MINT_INSTRUCTION"]');
    expect(json).toContain('["totalSize",208]');
    expect(json).toContain('["name","amount"]');
    expect(json).toBe(xxxlCanonicalProductionRuntimeByteLayoutJson(layout));
  });
});

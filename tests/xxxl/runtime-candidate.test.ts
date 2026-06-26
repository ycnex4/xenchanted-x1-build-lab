import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_AUTHORITY_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_INSTRUCTION,
  XXXL_RUNTIME_ROUTE_STATUS,
  XXXL_RUNTIME_SCHEMA_ERROR,
  validateXXXLConsumeGatewayMintInstructionSchema,
  validateXXXLRuntimeAccountLayout,
  xxxlRuntimeAccountWriteSet,
  type XXXLConsumeGatewayMintInstructionSchema,
} from "../../src/index.js";

function validSchema(): XXXLConsumeGatewayMintInstructionSchema {
  return {
    accounts: {
      mintState: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.MintState,
        version: 1,
        mintId: "xxxl-mint",
        decimals: 18,
        totalSupply: 0n,
        authorityMode: XXXL_RUNTIME_AUTHORITY_MODE.GatewayOnly,
        upgradeAuthorityStatus: "TEMPORARY_STAGED_FINALIZATION",
      },
      gatewayConfig: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
        version: 1,
        routeId: "ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1",
        sourceChainId: 1n,
        sourceToken: "0xXNTD",
        targetMintToken: "XXXL",
        targetX1NetworkId: "x1-mainnet",
        targetMintCoreId: "xxxl-mint-core",
        guardianSetId: "guardian-set-1",
        quorumThreshold: 2,
        finalityRuleId: "ethereum-finalized",
        status: XXXL_RUNTIME_ROUTE_STATUS.Active,
      },
      guardianSet: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
        version: 1,
        guardianSetId: "guardian-set-1",
        guardianPublicKeys: ["guardian-1", "guardian-2", "guardian-3"],
        quorumThreshold: 2,
        status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active,
      },
      processedEvent: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
        version: 1,
        canonicalEventKey: "event-key-1",
        routeId: "ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1",
        consumed: false,
        consumedAmount: 1000n,
        recipient: "x1-recipient",
      },
      recipientBalance: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
        version: 1,
        mintId: "xxxl-mint",
        owner: "x1-recipient",
        balance: 0n,
      },
    },
    data: {
      instruction: XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
      routeId: "ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1",
      guardianSetId: "guardian-set-1",
      mintId: "xxxl-mint",
      canonicalEventKey: "event-key-1",
      recipient: "x1-recipient",
      amount: 1000n,
    },
  };
}

describe("XXXL runtime candidate account and instruction schema", () => {
  it("accepts a valid consume gateway mint account layout", () => {
    const result = validateXXXLRuntimeAccountLayout(validSchema().accounts);

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("accepts a valid consume gateway mint instruction schema", () => {
    const result = validateXXXLConsumeGatewayMintInstructionSchema(validSchema());

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("requires gateway-only mint authority mode during Genesis Phase", () => {
    const schema = validSchema();
    const result = validateXXXLRuntimeAccountLayout({
      ...schema.accounts,
      mintState: {
        ...schema.accounts.mintState,
        authorityMode: XXXL_RUNTIME_AUTHORITY_MODE.Frozen,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SCHEMA_ERROR.WrongMintAuthorityMode,
    );
  });

  it("requires active route and active guardian set", () => {
    const schema = validSchema();
    const result = validateXXXLRuntimeAccountLayout({
      ...schema.accounts,
      gatewayConfig: {
        ...schema.accounts.gatewayConfig,
        status: XXXL_RUNTIME_ROUTE_STATUS.Frozen,
      },
      guardianSet: {
        ...schema.accounts.guardianSet,
        status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Retired,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(XXXL_RUNTIME_SCHEMA_ERROR.WrongRouteStatus);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SCHEMA_ERROR.WrongGuardianSetStatus,
    );
  });

  it("requires guardian set id and quorum threshold to match route configuration", () => {
    const schema = validSchema();
    const result = validateXXXLRuntimeAccountLayout({
      ...schema.accounts,
      guardianSet: {
        ...schema.accounts.guardianSet,
        guardianSetId: "different-guardian-set",
        quorumThreshold: 3,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_SCHEMA_ERROR.GuardianSetMismatch,
      XXXL_RUNTIME_SCHEMA_ERROR.GuardianSetMismatch,
    ]);
  });

  it("rejects invalid quorum threshold", () => {
    const schema = validSchema();
    const result = validateXXXLRuntimeAccountLayout({
      ...schema.accounts,
      guardianSet: {
        ...schema.accounts.guardianSet,
        quorumThreshold: 4,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SCHEMA_ERROR.GuardianSetMismatch,
    );
    expect(result.errors).toContain(
      XXXL_RUNTIME_SCHEMA_ERROR.InvalidQuorumThreshold,
    );
  });

  it("detects instruction data mismatch against account layout", () => {
    const schema = validSchema();
    const result = validateXXXLConsumeGatewayMintInstructionSchema({
      ...schema,
      data: {
        ...schema.data,
        canonicalEventKey: "different-event-key",
        recipient: "different-recipient",
        amount: 2000n,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SCHEMA_ERROR.ProcessedEventMismatch,
    );
    expect(result.errors).toContain(XXXL_RUNTIME_SCHEMA_ERROR.RecipientMismatch);
  });

  it("documents the runtime write set for consume gateway mint", () => {
    expect(xxxlRuntimeAccountWriteSet(validSchema())).toEqual([
      XXXL_RUNTIME_ACCOUNT_KIND.MintState,
      XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
      XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
    ]);
  });
});

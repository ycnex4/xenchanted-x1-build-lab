import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_AUTHORITY_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_INSTRUCTION,
  XXXL_RUNTIME_ROUTE_STATUS,
  XXXL_RUNTIME_TRANSITION_ERROR,
  executeXXXLRuntimeConsumeGatewayMintCandidate,
  hexToBytes,
  stage1CanonicalEventKeyHex,
  type XXXLConsumeGatewayMintInstructionSchema,
  type XXXLStage1GatewayAuthorizationContract,
} from "../../src/index.js";

const CANONICAL_EVENT_KEY_BYTES = hexToBytes(
  `0x${"11".repeat(32)}`,
  32,
  "canonicalEventKey",
);

const OTHER_CANONICAL_EVENT_KEY_BYTES = hexToBytes(
  `0x${"22".repeat(32)}`,
  32,
  "otherCanonicalEventKey",
);

function validAuthorization(
  overrides: Partial<XXXLStage1GatewayAuthorizationContract> = {},
): XXXLStage1GatewayAuthorizationContract {
  return {
    authorizationOk: true,
    authorized: true,
    markedProcessed: true,
    canonicalEventKey: CANONICAL_EVENT_KEY_BYTES,
    amount: 1000n,
    ...overrides,
  };
}

function validSchema(
  overrides: Partial<XXXLConsumeGatewayMintInstructionSchema["data"]> = {},
): XXXLConsumeGatewayMintInstructionSchema {
  const canonicalEventKey = stage1CanonicalEventKeyHex(CANONICAL_EVENT_KEY_BYTES);
  const data = {
    instruction: XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
    routeId: "ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1",
    guardianSetId: "guardian-set-1",
    mintId: "xxxl-mint",
    canonicalEventKey,
    recipient: "x1-recipient",
    amount: 1000n,
    ...overrides,
  };

  return {
    accounts: {
      mintState: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.MintState,
        version: 1,
        mintId: "xxxl-mint",
        decimals: 18,
        totalSupply: 500n,
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
        canonicalEventKey: data.canonicalEventKey,
        routeId: "ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1",
        consumed: false,
        consumedAmount: data.amount,
        recipient: data.recipient,
      },
      recipientBalance: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
        version: 1,
        mintId: "xxxl-mint",
        owner: data.recipient,
        balance: 200n,
      },
    },
    data,
  };
}

describe("XXXL runtime candidate transition semantics", () => {
  it("executes a valid consume gateway mint transition atomically", () => {
    const schema = validSchema();
    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization(),
    });

    expect(result.ok).toBe(true);
    expect(result.executed).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.accounts.mintState.totalSupply).toBe(1500n);
    expect(result.accounts.recipientBalance.balance).toBe(1200n);
    expect(result.accounts.processedEvent.consumed).toBe(true);
    expect(result.accounts.processedEvent.consumedAmount).toBe(1000n);
  });

  it("does not mutate input accounts on successful transition", () => {
    const schema = validSchema();

    executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization(),
    });

    expect(schema.accounts.mintState.totalSupply).toBe(500n);
    expect(schema.accounts.recipientBalance.balance).toBe(200n);
    expect(schema.accounts.processedEvent.consumed).toBe(false);
  });

  it("rejects invalid instruction schema without mutation", () => {
    const schema = validSchema({
      routeId: "WRONG_ROUTE",
    });

    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization(),
    });

    expect(result.ok).toBe(false);
    expect(result.executed).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_TRANSITION_ERROR.InvalidInstructionSchema,
    ]);
    expect(result.accounts).toBe(schema.accounts);
    expect(schema.accounts.mintState.totalSupply).toBe(500n);
    expect(schema.accounts.processedEvent.consumed).toBe(false);
  });

  it("rejects Stage 1 authorization failure without mutation", () => {
    const schema = validSchema();

    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization({
        authorizationOk: false,
        authorized: false,
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_TRANSITION_ERROR.Stage1MintNotAuthorized,
    ]);
    expect(result.accounts).toBe(schema.accounts);
  });

  it("rejects unmarked Stage 1 processed state without mutation", () => {
    const schema = validSchema();

    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization({
        markedProcessed: false,
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_TRANSITION_ERROR.Stage1MintNotMarkedProcessed,
    ]);
    expect(result.accounts).toBe(schema.accounts);
  });

  it("rejects zero amount without mutation", () => {
    const schema = validSchema({
      amount: 0n,
    });

    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization({
        amount: 0n,
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([XXXL_RUNTIME_TRANSITION_ERROR.InvalidAmount]);
    expect(result.accounts).toBe(schema.accounts);
  });

  it("rejects authorization event key mismatch without mutation", () => {
    const schema = validSchema();

    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization({
        canonicalEventKey: OTHER_CANONICAL_EVENT_KEY_BYTES,
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_TRANSITION_ERROR.AuthorizationEventKeyMismatch,
    ]);
    expect(result.accounts).toBe(schema.accounts);
  });

  it("rejects authorization amount mismatch without mutation", () => {
    const schema = validSchema();

    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema,
      authorization: validAuthorization({
        amount: 999n,
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_TRANSITION_ERROR.AuthorizationAmountMismatch,
    ]);
    expect(result.accounts).toBe(schema.accounts);
  });

  it("rejects already consumed event without mutation", () => {
    const schema = validSchema();
    const consumedSchema = {
      ...schema,
      accounts: {
        ...schema.accounts,
        processedEvent: {
          ...schema.accounts.processedEvent,
          consumed: true,
        },
      },
    };

    const result = executeXXXLRuntimeConsumeGatewayMintCandidate({
      schema: consumedSchema,
      authorization: validAuthorization(),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_TRANSITION_ERROR.EventAlreadyConsumed,
    ]);
    expect(result.accounts).toBe(consumedSchema.accounts);
  });
});

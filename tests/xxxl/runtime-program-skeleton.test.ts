import { describe, expect, it } from "vitest";

import {
  AVALANCHE_MAINNET_CHAIN_ID,
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
  XXXL_MULTICHAIN_ROUTE_ID,
  XXXL_MULTICHAIN_ROUTE_STATUS,
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_AUTHORITY_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_INSTRUCTION,
  XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE,
  XXXL_RUNTIME_PROGRAM_SKELETON_ERROR,
  XXXL_RUNTIME_PROGRAM_SKELETON_GUARDIAN_SIGNATURE_SCOPE,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP,
  XXXL_RUNTIME_ROUTE_STATUS,
  executeXXXLRuntimeProgramSkeleton,
  hexToBytes,
  stage1CanonicalEventKeyHex,
  xxxlAvalancheLowWeightRouteCandidate,
  xxxlEthereumPrimaryRouteCandidate,
  type XXXLConsumeGatewayMintInstructionSchema,
  type XXXLStage1GatewayAuthorizationContract,
} from "../../src/index.js";

const CANONICAL_EVENT_KEY_BYTES = hexToBytes(
  `0x${"33".repeat(32)}`,
  32,
  "canonicalEventKey",
);

const OTHER_CANONICAL_EVENT_KEY_BYTES = hexToBytes(
  `0x${"44".repeat(32)}`,
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
  options: {
    readonly routeId?: string;
    readonly sourceChainId?: bigint;
    readonly sourceToken?: string;
    readonly guardianSetId?: string;
    readonly finalityRuleId?: string;
    readonly amount?: bigint;
    readonly recipient?: string;
  } = {},
): XXXLConsumeGatewayMintInstructionSchema {
  const routeId = options.routeId ?? XXXL_GATEWAY_ROUTE_ID;
  const guardianSetId = options.guardianSetId ?? "guardian-set-1";
  const amount = options.amount ?? 1000n;
  const recipient = options.recipient ?? "x1-recipient";
  const canonicalEventKey = stage1CanonicalEventKeyHex(
    CANONICAL_EVENT_KEY_BYTES,
  );

  const data = {
    instruction: XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
    routeId,
    guardianSetId,
    mintId: "xxxl-mint",
    canonicalEventKey,
    recipient,
    amount,
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
        routeId,
        sourceChainId: options.sourceChainId ?? BigInt(ETHEREUM_MAINNET_CHAIN_ID),
        sourceToken: options.sourceToken ?? "0xXNTD",
        targetMintToken: XXXL_MINT_TOKEN,
        targetX1NetworkId: "x1-mainnet",
        targetMintCoreId: "xxxl-mint-core",
        guardianSetId,
        quorumThreshold: 2,
        finalityRuleId: options.finalityRuleId ?? "ethereum-finalized",
        status: XXXL_RUNTIME_ROUTE_STATUS.Active,
      },
      guardianSet: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
        version: 1,
        guardianSetId,
        guardianPublicKeys: ["guardian-1", "guardian-2", "guardian-3"],
        quorumThreshold: 2,
        status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active,
      },
      processedEvent: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
        version: 1,
        canonicalEventKey: data.canonicalEventKey,
        routeId,
        consumed: false,
        consumedAmount: amount,
        recipient,
      },
      recipientBalance: {
        kind: XXXL_RUNTIME_ACCOUNT_KIND.RecipientBalance,
        version: 1,
        mintId: "xxxl-mint",
        owner: recipient,
        balance: 200n,
      },
    },
    data,
  };
}

describe("XXXL runtime program skeleton", () => {
  it("executes a valid Ethereum gateway mint skeleton path", () => {
    const schema = validSchema();
    const result = executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.ok).toBe(true);
    expect(result.executed).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.accountsAfter.mintState.totalSupply).toBe(1500n);
    expect(result.accountsAfter.recipientBalance.balance).toBe(1200n);
    expect(result.accountsAfter.processedEvent.consumed).toBe(true);
  });

  it("does not mutate input accounts on success", () => {
    const schema = validSchema();

    executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(schema.accounts.mintState.totalSupply).toBe(500n);
    expect(schema.accounts.recipientBalance.balance).toBe(200n);
    expect(schema.accounts.processedEvent.consumed).toBe(false);
  });

  it("models SPL Token mint_to CPI as atomic parent-transaction step", () => {
    const result = executeXXXLRuntimeProgramSkeleton({
      schema: validSchema(),
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.cpiStep.skipped).toBe(false);
    expect(result.cpiStep.atomicWithParentTransaction).toBe(true);
    expect(result.cpiStep.signerRole).toBe(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.MintAuthorityPda,
    );
    expect(result.cpiStep.mintRole).toBe(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.SplTokenMint,
    );
    expect(result.cpiStep.recipientTokenAccountRole).toBe(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.RecipientTokenAccount,
    );
    expect(result.cpiStep.tokenProgramRole).toBe(
      XXXL_RUNTIME_INSTRUCTION_ACCOUNT_META_ROLE.TokenProgram,
    );
  });

  it("records that guardian signature verification remains outside runtime skeleton", () => {
    const result = executeXXXLRuntimeProgramSkeleton({
      schema: validSchema(),
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.guardianSignatureVerificationScope).toBe(
      XXXL_RUNTIME_PROGRAM_SKELETON_GUARDIAN_SIGNATURE_SCOPE,
    );
  });

  it("audits supply and recipient balance after accepted gateway mint", () => {
    const result = executeXXXLRuntimeProgramSkeleton({
      schema: validSchema(),
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.supplyAudit.ok).toBe(true);
    expect(result.supplyAudit.totalSupplyBefore).toBe(500n);
    expect(result.supplyAudit.totalSupplyAfter).toBe(1500n);
    expect(result.supplyAudit.expectedTotalSupplyAfter).toBe(1500n);
    expect(result.supplyAudit.recipientBalanceBefore).toBe(200n);
    expect(result.supplyAudit.recipientBalanceAfter).toBe(1200n);
    expect(result.supplyAudit.expectedRecipientBalanceAfter).toBe(1200n);
  });

  it("rejects invalid instruction serialization boundary before execution", () => {
    const schema = validSchema();
    const result = executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
      instructionSerializationLayouts: [],
      instructionSerializationVectors: [],
    });

    expect(result.ok).toBe(false);
    expect(result.executed).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.InstructionSerializationInvalid,
    );
    expect(result.transition).toBeNull();
    expect(result.accountsAfter).toBe(schema.accounts);
    expect(result.cpiStep.skipped).toBe(true);
  });

  it("rejects invalid route policy before execution", () => {
    const schema = validSchema();
    const result = executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization(),
      routePolicy: [
        {
          ...xxxlEthereumPrimaryRouteCandidate(),
          sourceChainWeightBps: 9999,
        },
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.executed).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyInvalid,
    );
    expect(result.transition).toBeNull();
    expect(result.accountsAfter).toBe(schema.accounts);
  });

  it("rejects route ids not present in supplied route policy", () => {
    const schema = validSchema({
      routeId: "UNKNOWN_ROUTE",
    });

    const result = executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.ok).toBe(false);
    expect(result.executed).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyMissingRoute,
    );
    expect(result.transition).toBeNull();
    expect(result.accountsAfter).toBe(schema.accounts);
  });

  it("delegates Stage 1 authorization rejection without mutation", () => {
    const schema = validSchema();
    const result = executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization({
        authorizationOk: false,
        authorized: false,
      }),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.ok).toBe(false);
    expect(result.executed).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected,
    ]);
    expect(result.transition?.executed).toBe(false);
    expect(result.accountsAfter).toBe(schema.accounts);
    expect(result.cpiStep.skipped).toBe(true);
  });

  it("rejects replayed processed events without committing CPI boundary", () => {
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

    const result = executeXXXLRuntimeProgramSkeleton({
      schema: consumedSchema,
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.ok).toBe(false);
    expect(result.executed).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected,
    ]);
    expect(result.accountsAfter).toBe(consumedSchema.accounts);
    expect(result.cpiStep.skipped).toBe(true);
  });

  it("supports Avalanche low-weight route without Ethereum-only runtime assumptions", () => {
    const avalancheRoute = {
      ...xxxlAvalancheLowWeightRouteCandidate(10),
      status: XXXL_MULTICHAIN_ROUTE_STATUS.Active,
    };

    const schema = validSchema({
      routeId: XXXL_MULTICHAIN_ROUTE_ID.AvalancheLowWeight,
      sourceChainId: BigInt(AVALANCHE_MAINNET_CHAIN_ID),
      sourceToken: "0xAVAX_XNTD",
      guardianSetId: "guardian-set-avalanche-1",
      finalityRuleId: "avalanche-finalized",
    });

    const result = executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization(),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate(), avalancheRoute],
    });

    expect(result.ok).toBe(true);
    expect(result.executed).toBe(true);
    expect(result.accountsAfter.mintState.totalSupply).toBe(1500n);
    expect(result.steps).toContain(
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.SplTokenMintToCpi,
    );
  });

  it("keeps instruction route id as runtime input instead of hardcoding Ethereum", () => {
    const schema = validSchema({
      routeId: XXXL_MULTICHAIN_ROUTE_ID.AvalancheLowWeight,
      sourceChainId: BigInt(AVALANCHE_MAINNET_CHAIN_ID),
      guardianSetId: "guardian-set-avalanche-1",
    });

    expect(schema.data.routeId).toBe(XXXL_MULTICHAIN_ROUTE_ID.AvalancheLowWeight);
    expect(schema.data.routeId).not.toBe(XXXL_GATEWAY_ROUTE_ID);
  });

  it("rejects authorization event-key mismatch through transition layer", () => {
    const schema = validSchema();
    const result = executeXXXLRuntimeProgramSkeleton({
      schema,
      authorization: validAuthorization({
        canonicalEventKey: OTHER_CANONICAL_EVENT_KEY_BYTES,
      }),
      routePolicy: [xxxlEthereumPrimaryRouteCandidate()],
    });

    expect(result.ok).toBe(false);
    expect(result.executed).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected,
    ]);
    expect(result.accountsAfter).toBe(schema.accounts);
  });
});

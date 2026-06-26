import { describe, expect, it } from "vitest";

import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_GENESIS_PHASE,
  XXXL_MINT_TOKEN,
  XXXL_PROGRAM_VERSION,
  XXXL_TEMPORARY_UPGRADE_AUTHORITY_STATUS,
  XXXLProgramError,
  XXXLProgramErrorCode,
  type XXXLGatewayMintAuthorization,
  assertGatewaySupplyInvariant,
  createEmptyXXXLProgramState,
  processXXXLGatewayMintAuthorization,
  rejectManualXXXLMint,
} from "../../src/xxxl/program-v1.js";

function createValidAuthorization(
  overrides: Partial<XXXLGatewayMintAuthorization> = {},
): XXXLGatewayMintAuthorization {
  return {
    routeId: XXXL_GATEWAY_ROUTE_ID,
    sourceChainId: ETHEREUM_MAINNET_CHAIN_ID,
    sourceToken: "0xXNTD",
    sourceSender: "0xSourceSender",
    sourceBurnTxHash: "0xBurnTxHash",
    sourceBurnEventIndex: 0,
    sourceBlockNumber: 123456789n,
    sourceBlockHash: "0xSourceBlockHash",
    canonicalEventKey: "eth:1:xntd:burn:0xBurnTxHash:0",
    x1Recipient: "x1Recipient111111111111111111111111111111111",
    xxxlMintAmount: 1000n,
    mintToken: XXXL_MINT_TOKEN,
    ...overrides,
  };
}

function expectProgramError(
  action: () => unknown,
  code: XXXLProgramErrorCode,
): void {
  try {
    action();
    throw new Error("Expected XXXLProgramError");
  } catch (error) {
    expect(error).toBeInstanceOf(XXXLProgramError);
    expect((error as XXXLProgramError).code).toBe(code);
  }
}

describe("XXXL Program v1 deterministic boundary", () => {
  it("starts in gateway-only genesis phase with zero supply", () => {
    const state = createEmptyXXXLProgramState();

    expect(state.programVersion).toBe(XXXL_PROGRAM_VERSION);
    expect(state.genesisPhase).toBe(XXXL_GENESIS_PHASE);
    expect(state.upgradeAuthorityStatus).toBe(
      XXXL_TEMPORARY_UPGRADE_AUTHORITY_STATUS,
    );
    expect(state.totalSupply).toBe(0n);
    expect(state.processedGatewayEvents.size).toBe(0);
  });

  it("mints XXXL only from a valid gateway authorization", () => {
    const state = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization();

    const nextState = processXXXLGatewayMintAuthorization(state, authorization);

    expect(nextState.totalSupply).toBe(authorization.xxxlMintAmount);
    expect(nextState.processedGatewayEvents.has(authorization.canonicalEventKey))
      .toBe(true);
    expect(state.totalSupply).toBe(0n);
    expect(state.processedGatewayEvents.size).toBe(0);
  });

  it("keeps supply equal to the sum of accepted gateway mints", () => {
    const initialState = createEmptyXXXLProgramState();
    const firstAuthorization = createValidAuthorization({
      canonicalEventKey: "eth:1:xntd:burn:0xBurnTxHash:0",
      sourceBurnTxHash: "0xBurnTxHash",
      sourceBurnEventIndex: 0,
      xxxlMintAmount: 1000n,
    });
    const secondAuthorization = createValidAuthorization({
      canonicalEventKey: "eth:1:xntd:burn:0xSecondBurnTxHash:1",
      sourceBurnTxHash: "0xSecondBurnTxHash",
      sourceBurnEventIndex: 1,
      xxxlMintAmount: 2500n,
    });

    const afterFirst = processXXXLGatewayMintAuthorization(
      initialState,
      firstAuthorization,
    );
    const afterSecond = processXXXLGatewayMintAuthorization(
      afterFirst,
      secondAuthorization,
    );

    expect(afterSecond.totalSupply).toBe(3500n);
    expect(afterSecond.processedGatewayEvents.size).toBe(2);
  });

  it("proves the gateway supply delta invariant for an accepted mint", () => {
    const before = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization();

    const after = processXXXLGatewayMintAuthorization(before, authorization);

    expect(() =>
      assertGatewaySupplyInvariant(before, after, authorization),
    ).not.toThrow();
  });

  it("rejects manual mint attempts", () => {
    expectProgramError(
      () => rejectManualXXXLMint({ recipient: "x1Recipient", amount: 1n }),
      XXXLProgramErrorCode.MANUAL_MINT_FORBIDDEN,
    );
  });

  it("rejects replayed gateway events", () => {
    const state = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization();
    const afterFirst = processXXXLGatewayMintAuthorization(state, authorization);

    expectProgramError(
      () => processXXXLGatewayMintAuthorization(afterFirst, authorization),
      XXXLProgramErrorCode.REPLAYED_GATEWAY_EVENT,
    );

    expect(afterFirst.totalSupply).toBe(authorization.xxxlMintAmount);
    expect(afterFirst.processedGatewayEvents.size).toBe(1);
  });

  it("rejects wrong route id", () => {
    const state = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization({ routeId: "WRONG_ROUTE" });

    expectProgramError(
      () => processXXXLGatewayMintAuthorization(state, authorization),
      XXXLProgramErrorCode.INVALID_ROUTE_ID,
    );

    expect(state.totalSupply).toBe(0n);
    expect(state.processedGatewayEvents.size).toBe(0);
  });

  it("rejects wrong mint token", () => {
    const state = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization({ mintToken: "NOT_XXXL" });

    expectProgramError(
      () => processXXXLGatewayMintAuthorization(state, authorization),
      XXXLProgramErrorCode.INVALID_MINT_TOKEN,
    );

    expect(state.totalSupply).toBe(0n);
    expect(state.processedGatewayEvents.size).toBe(0);
  });

  it("rejects zero mint amount", () => {
    const state = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization({ xxxlMintAmount: 0n });

    expectProgramError(
      () => processXXXLGatewayMintAuthorization(state, authorization),
      XXXLProgramErrorCode.INVALID_MINT_AMOUNT,
    );

    expect(state.totalSupply).toBe(0n);
    expect(state.processedGatewayEvents.size).toBe(0);
  });

  it("rejects empty X1 recipient", () => {
    const state = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization({ x1Recipient: "   " });

    expectProgramError(
      () => processXXXLGatewayMintAuthorization(state, authorization),
      XXXLProgramErrorCode.INVALID_X1_RECIPIENT,
    );

    expect(state.totalSupply).toBe(0n);
    expect(state.processedGatewayEvents.size).toBe(0);
  });
});

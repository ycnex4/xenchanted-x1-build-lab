import { describe, expect, it } from "vitest";

import {
  XXXL_GENESIS_SUPPLY_INVARIANT_ERROR,
  XXXLGenesisSupplyInvariantError,
  assertGenesisGatewayMintSupplyDelta,
  assertGenesisSupplyEqualsAcceptedGatewayMintSum,
  assertNoUnauthorizedGenesisSupplyIncrease,
  assertRejectedGenesisTransitionPreservesState,
  createEmptyXXXLProgramState,
  processXXXLGatewayMintAuthorization,
  rejectManualXXXLMint,
  type XXXLGatewayMintAuthorization,
  XXXL_GATEWAY_ROUTE_ID,
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_MINT_TOKEN,
} from "../../src/index.js";

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

function expectInvariantError(
  action: () => unknown,
  code: string,
): void {
  try {
    action();
    throw new Error("Expected XXXLGenesisSupplyInvariantError");
  } catch (error) {
    expect(error).toBeInstanceOf(XXXLGenesisSupplyInvariantError);
    expect((error as XXXLGenesisSupplyInvariantError).code).toBe(code);
  }
}

describe("XXXL Genesis supply invariant hardening", () => {
  it("accepts a single gateway mint supply delta and processed-event delta", () => {
    const before = createEmptyXXXLProgramState();
    const authorization = createValidAuthorization();
    const after = processXXXLGatewayMintAuthorization(before, authorization);

    expect(() =>
      assertGenesisGatewayMintSupplyDelta(before, after, {
        canonicalEventKeyHex: authorization.canonicalEventKey,
        amount: authorization.xxxlMintAmount,
      }),
    ).not.toThrow();
  });

  it("keeps Genesis supply equal to the sum of accepted gateway mints", () => {
    const firstAuthorization = createValidAuthorization({
      canonicalEventKey: "eth:1:xntd:burn:0xFirstBurnTxHash:0",
      sourceBurnTxHash: "0xFirstBurnTxHash",
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
      createEmptyXXXLProgramState(),
      firstAuthorization,
    );
    const afterSecond = processXXXLGatewayMintAuthorization(
      afterFirst,
      secondAuthorization,
    );

    expect(() =>
      assertGenesisSupplyEqualsAcceptedGatewayMintSum(afterSecond, [
        firstAuthorization.xxxlMintAmount,
        secondAuthorization.xxxlMintAmount,
      ]),
    ).not.toThrow();
    expect(afterSecond.totalSupply).toBe(3500n);
  });

  it("rejects an unauthorized direct supply increase", () => {
    const before = createEmptyXXXLProgramState();
    const after = {
      ...before,
      totalSupply: 1n,
    };

    expectInvariantError(
      () => assertNoUnauthorizedGenesisSupplyIncrease(before, after),
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.UnauthorizedSupplyIncrease,
    );
  });

  it("requires rejected transitions to preserve supply and replay state", () => {
    const before = createEmptyXXXLProgramState();
    const after = createEmptyXXXLProgramState();

    expect(() =>
      assertRejectedGenesisTransitionPreservesState(before, after),
    ).not.toThrow();
  });

  it("detects rejected transition supply mutation", () => {
    const before = createEmptyXXXLProgramState();
    const after = {
      ...before,
      totalSupply: before.totalSupply + 1n,
    };

    expectInvariantError(
      () => assertRejectedGenesisTransitionPreservesState(before, after),
      XXXL_GENESIS_SUPPLY_INVARIANT_ERROR.RejectedTransitionMutatedSupply,
    );
  });

  it("manual mint remains forbidden and cannot satisfy Genesis supply invariant", () => {
    const before = createEmptyXXXLProgramState();

    expect(() =>
      rejectManualXXXLMint({
        recipient: "x1Recipient",
        amount: 1n,
      }),
    ).toThrow();

    expect(() =>
      assertGenesisSupplyEqualsAcceptedGatewayMintSum(before, []),
    ).not.toThrow();
    expect(before.totalSupply).toBe(0n);
  });
});

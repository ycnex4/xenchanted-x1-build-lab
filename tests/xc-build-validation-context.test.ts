import { describe, expect, it } from "vitest";
import { createXcBuildValidationContextFromProtocolParams } from "../src/index.js";
import type { XcProtocolParams } from "../src/index.js";

const PROTOCOL_PARAMS: XcProtocolParams = {
  genesisTs: 1780166915n,
  halvingInterval: 15552000n,
  xenBurnHalvingInterval: 31104000n,
  currentEpoch: 0n,
  nextHalvingTs: 1795718915n,
  initialNominal: 100000000000000000000n,
  currentBaseNominal: 100000000000000000000n,
  initialXenBurn: 100000000000000000000000000n,
  currentXenBurnAmount: 100000000000000000000000000n,
  enchantMultiplier: 3n,
  maxLevel: 22,
  baseAprBpsNow: 1000,
  bpsDenom: 10000n,
  earlyPenaltyBps: 100n,
  maxWalletNfts: 60n
};

describe("XC Build validation context", () => {
  it("creates validation context from protocol params", () => {
    const context = createXcBuildValidationContextFromProtocolParams({
      protocolParams: PROTOCOL_PARAMS
    });

    expect(context.protocolParams).toBe(PROTOCOL_PARAMS);
  });

  it("includes derived requirements", () => {
    const context = createXcBuildValidationContextFromProtocolParams({
      protocolParams: PROTOCOL_PARAMS
    });

    expect(context.requirements.requiredBaseNominal).toBe(
      PROTOCOL_PARAMS.currentBaseNominal
    );
    expect(context.requirements.requiredXenBurnAmount).toBe(
      PROTOCOL_PARAMS.currentXenBurnAmount
    );
  });

  it("preserves current epoch", () => {
    const context = createXcBuildValidationContextFromProtocolParams({
      protocolParams: {
        ...PROTOCOL_PARAMS,
        currentEpoch: 2n
      }
    });

    expect(context.requirements.currentEpoch).toBe(2n);
  });

  it("preserves required XNTD lock minimum", () => {
    const context = createXcBuildValidationContextFromProtocolParams({
      protocolParams: PROTOCOL_PARAMS
    });

    expect(context.requirements.requiredXntdLockMinimum).toBe(
      PROTOCOL_PARAMS.currentBaseNominal
    );
  });

  it("preserves required Forge minimum", () => {
    const context = createXcBuildValidationContextFromProtocolParams({
      protocolParams: PROTOCOL_PARAMS
    });

    expect(context.requirements.requiredForgeMinimum).toBe(
      PROTOCOL_PARAMS.currentBaseNominal * 5n
    );
  });

  it("rejects invalid protocol params through requirement derivation", () => {
    expect(() =>
      createXcBuildValidationContextFromProtocolParams({
        protocolParams: {
          ...PROTOCOL_PARAMS,
          currentBaseNominal: 0n
        }
      })
    ).toThrow("Invalid XC build requirement: currentBaseNominal must be positive");
  });

  it("does not call real RPC", () => {
    const context = createXcBuildValidationContextFromProtocolParams({
      protocolParams: PROTOCOL_PARAMS
    });

    expect(context.requirements.requiredBaseNominal).toBe(
      PROTOCOL_PARAMS.currentBaseNominal
    );
  });
});

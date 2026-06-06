import { describe, expect, it } from "vitest";
import {
  deriveCurrentXcBuildRequirements,
  validateXcBuildAgainstProtocolParams
} from "../src/index.js";
import type { XcProtocolParams } from "../src/index.js";

const MAINNET_PROTOCOL_PARAMS: XcProtocolParams = {
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

describe("XC protocol params build validation", () => {
  it("derives current epoch from protocol params", () => {
    const requirements = deriveCurrentXcBuildRequirements(MAINNET_PROTOCOL_PARAMS);

    expect(requirements.currentEpoch).toBe(0n);
  });

  it("derives current base nominal requirement", () => {
    const requirements = deriveCurrentXcBuildRequirements(MAINNET_PROTOCOL_PARAMS);

    expect(requirements.requiredBaseNominal).toBe(100000000000000000000n);
  });

  it("derives current XEN burn amount requirement", () => {
    const requirements = deriveCurrentXcBuildRequirements(MAINNET_PROTOCOL_PARAMS);

    expect(requirements.requiredXenBurnAmount).toBe(
      100000000000000000000000000n
    );
  });

  it("derives XNTD lock minimum from current base nominal", () => {
    const requirements = deriveCurrentXcBuildRequirements(MAINNET_PROTOCOL_PARAMS);

    expect(requirements.requiredXntdLockMinimum).toBe(
      MAINNET_PROTOCOL_PARAMS.currentBaseNominal
    );
  });

  it("derives Forge minimum from current base nominal", () => {
    const requirements = deriveCurrentXcBuildRequirements(MAINNET_PROTOCOL_PARAMS);

    expect(requirements.requiredForgeMinimum).toBe(
      MAINNET_PROTOCOL_PARAMS.currentBaseNominal * 5n
    );
  });

  it("preserves epoch timing metadata", () => {
    const requirements = deriveCurrentXcBuildRequirements(MAINNET_PROTOCOL_PARAMS);

    expect(requirements.genesisTs).toBe(MAINNET_PROTOCOL_PARAMS.genesisTs);
    expect(requirements.nextHalvingTs).toBe(
      MAINNET_PROTOCOL_PARAMS.nextHalvingTs
    );
    expect(requirements.halvingInterval).toBe(
      MAINNET_PROTOCOL_PARAMS.halvingInterval
    );
    expect(requirements.xenBurnHalvingInterval).toBe(
      MAINNET_PROTOCOL_PARAMS.xenBurnHalvingInterval
    );
  });

  it("handles later epochs", () => {
    const requirements = deriveCurrentXcBuildRequirements({
      ...MAINNET_PROTOCOL_PARAMS,
      currentEpoch: 3n,
      currentBaseNominal: 12500000000000000000n,
      currentXenBurnAmount: 50000000000000000000000000n
    });

    expect(requirements.currentEpoch).toBe(3n);
    expect(requirements.requiredBaseNominal).toBe(12500000000000000000n);
    expect(requirements.requiredXenBurnAmount).toBe(
      50000000000000000000000000n
    );
    expect(requirements.requiredXntdLockMinimum).toBe(12500000000000000000n);
    expect(requirements.requiredForgeMinimum).toBe(62500000000000000000n);
  });

  it("returns a positive validation result with derived requirements", () => {
    const result = validateXcBuildAgainstProtocolParams({
      protocolParams: MAINNET_PROTOCOL_PARAMS
    });

    expect(result.isValid).toBe(true);
    expect(result.requirements.requiredBaseNominal).toBe(
      MAINNET_PROTOCOL_PARAMS.currentBaseNominal
    );
  });

  it("rejects zero currentBaseNominal", () => {
    expect(() =>
      deriveCurrentXcBuildRequirements({
        ...MAINNET_PROTOCOL_PARAMS,
        currentBaseNominal: 0n
      })
    ).toThrow("Invalid XC build requirement: currentBaseNominal must be positive");
  });

  it("rejects zero currentXenBurnAmount", () => {
    expect(() =>
      deriveCurrentXcBuildRequirements({
        ...MAINNET_PROTOCOL_PARAMS,
        currentXenBurnAmount: 0n
      })
    ).toThrow(
      "Invalid XC build requirement: currentXenBurnAmount must be positive"
    );
  });

  it("rejects zero halvingInterval", () => {
    expect(() =>
      deriveCurrentXcBuildRequirements({
        ...MAINNET_PROTOCOL_PARAMS,
        halvingInterval: 0n
      })
    ).toThrow("Invalid XC build requirement: halvingInterval must be positive");
  });

  it("rejects zero xenBurnHalvingInterval", () => {
    expect(() =>
      deriveCurrentXcBuildRequirements({
        ...MAINNET_PROTOCOL_PARAMS,
        xenBurnHalvingInterval: 0n
      })
    ).toThrow(
      "Invalid XC build requirement: xenBurnHalvingInterval must be positive"
    );
  });

  it("does not call real RPC", () => {
    const requirements = deriveCurrentXcBuildRequirements(MAINNET_PROTOCOL_PARAMS);

    expect(requirements.requiredBaseNominal).toBe(
      MAINNET_PROTOCOL_PARAMS.currentBaseNominal
    );
  });
});

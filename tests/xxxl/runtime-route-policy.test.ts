import { describe, expect, it } from "vitest";

import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_FINALITY_KIND,
  XXXL_RUNTIME_FINALITY_STATUS,
  XXXL_RUNTIME_GUARDIAN_ROTATION_MODE,
  XXXL_RUNTIME_GUARDIAN_SET_STATUS,
  XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS,
  XXXL_RUNTIME_POLICY_ERROR,
  XXXL_RUNTIME_ROUTE_POLICY_VERSION,
  XXXL_RUNTIME_ROUTE_STATUS,
  buildXXXLRuntimeGatewayConfigAccount,
  buildXXXLRuntimeGuardianSetAccount,
  validateXXXLRuntimeRouteGuardianFinalityPolicy,
  type XXXLRuntimeRouteGuardianFinalityPolicyCandidate,
} from "../../src/index.js";

function validPolicy(): XXXLRuntimeRouteGuardianFinalityPolicyCandidate {
  return {
    route: {
      version: XXXL_RUNTIME_ROUTE_POLICY_VERSION,
      routeId: XXXL_GATEWAY_ROUTE_ID,
      sourceChainId: BigInt(ETHEREUM_MAINNET_CHAIN_ID),
      sourceToken: "0x1111111111111111111111111111111111111111",
      targetMintToken: XXXL_MINT_TOKEN,
      targetX1NetworkId: "x1-mainnet",
      targetMintCoreId: "xxxl-mint-core",
      guardianSetId: "guardian-set-1",
      quorumThreshold: 2,
      finalityRuleId: "ethereum-finalized",
      status: XXXL_RUNTIME_ROUTE_STATUS.Active,
    },
    guardian: {
      version: XXXL_RUNTIME_ROUTE_POLICY_VERSION,
      guardianSetId: "guardian-set-1",
      guardianPublicKeys: ["guardian-1", "guardian-2", "guardian-3"],
      quorumThreshold: 2,
      rotationMode: XXXL_RUNTIME_GUARDIAN_ROTATION_MODE.TimelockedMultisig,
      rotationTimelockSeconds: XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS,
      emergencyFreezeThreshold: 2,
      status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active,
    },
    finality: {
      version: XXXL_RUNTIME_ROUTE_POLICY_VERSION,
      finalityRuleId: "ethereum-finalized",
      sourceChainId: BigInt(ETHEREUM_MAINNET_CHAIN_ID),
      kind: XXXL_RUNTIME_FINALITY_KIND.EthereumFinalized,
      minConfirmations: 0,
      status: XXXL_RUNTIME_FINALITY_STATUS.Active,
    },
  };
}

describe("XXXL runtime route guardian finality policy candidate", () => {
  it("accepts a valid route guardian finality policy", () => {
    const result = validateXXXLRuntimeRouteGuardianFinalityPolicy(validPolicy());

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("builds runtime gateway config and guardian set accounts from policy", () => {
    const policy = validPolicy();

    expect(buildXXXLRuntimeGatewayConfigAccount(policy)).toEqual({
      kind: XXXL_RUNTIME_ACCOUNT_KIND.GatewayConfig,
      version: 1,
      routeId: XXXL_GATEWAY_ROUTE_ID,
      sourceChainId: BigInt(ETHEREUM_MAINNET_CHAIN_ID),
      sourceToken: "0x1111111111111111111111111111111111111111",
      targetMintToken: XXXL_MINT_TOKEN,
      targetX1NetworkId: "x1-mainnet",
      targetMintCoreId: "xxxl-mint-core",
      guardianSetId: "guardian-set-1",
      quorumThreshold: 2,
      finalityRuleId: "ethereum-finalized",
      status: XXXL_RUNTIME_ROUTE_STATUS.Active,
    });

    expect(buildXXXLRuntimeGuardianSetAccount(policy)).toEqual({
      kind: XXXL_RUNTIME_ACCOUNT_KIND.GuardianSet,
      version: 1,
      guardianSetId: "guardian-set-1",
      guardianPublicKeys: ["guardian-1", "guardian-2", "guardian-3"],
      quorumThreshold: 2,
      status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Active,
    });
  });

  it("rejects wrong route, wrong source chain, missing source token, and wrong target token", () => {
    const policy = validPolicy();
    const result = validateXXXLRuntimeRouteGuardianFinalityPolicy({
      ...policy,
      route: {
        ...policy.route,
        routeId: "WRONG_ROUTE",
        sourceChainId: 2n,
        sourceToken: "",
        targetMintToken: "NOT_XXXL",
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_POLICY_ERROR.WrongRouteId,
      XXXL_RUNTIME_POLICY_ERROR.WrongSourceChainId,
      XXXL_RUNTIME_POLICY_ERROR.MissingSourceToken,
      XXXL_RUNTIME_POLICY_ERROR.WrongTargetMintToken,
      XXXL_RUNTIME_POLICY_ERROR.FinalitySourceChainMismatch,
    ]);
  });

  it("rejects inactive route, guardian set, and finality rule", () => {
    const policy = validPolicy();
    const result = validateXXXLRuntimeRouteGuardianFinalityPolicy({
      route: {
        ...policy.route,
        status: XXXL_RUNTIME_ROUTE_STATUS.Frozen,
      },
      guardian: {
        ...policy.guardian,
        status: XXXL_RUNTIME_GUARDIAN_SET_STATUS.Retired,
      },
      finality: {
        ...policy.finality,
        status: XXXL_RUNTIME_FINALITY_STATUS.Retired,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_POLICY_ERROR.RouteNotActive,
      XXXL_RUNTIME_POLICY_ERROR.GuardianSetNotActive,
      XXXL_RUNTIME_POLICY_ERROR.FinalityRuleNotActive,
    ]);
  });

  it("rejects guardian set, finality rule, and quorum mismatches", () => {
    const policy = validPolicy();
    const result = validateXXXLRuntimeRouteGuardianFinalityPolicy({
      ...policy,
      guardian: {
        ...policy.guardian,
        guardianSetId: "guardian-set-2",
        quorumThreshold: 3,
        emergencyFreezeThreshold: 3,
      },
      finality: {
        ...policy.finality,
        finalityRuleId: "different-finality-rule",
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_POLICY_ERROR.GuardianSetIdMismatch,
      XXXL_RUNTIME_POLICY_ERROR.FinalityRuleIdMismatch,
      XXXL_RUNTIME_POLICY_ERROR.QuorumThresholdMismatch,
    ]);
  });

  it("rejects empty guardian set and invalid quorum threshold", () => {
    const policy = validPolicy();
    const result = validateXXXLRuntimeRouteGuardianFinalityPolicy({
      ...policy,
      guardian: {
        ...policy.guardian,
        guardianPublicKeys: [],
        quorumThreshold: 1,
        emergencyFreezeThreshold: 1,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(XXXL_RUNTIME_POLICY_ERROR.EmptyGuardianSet);
    expect(result.errors).toContain(
      XXXL_RUNTIME_POLICY_ERROR.InvalidQuorumThreshold,
    );
    expect(result.errors).toContain(
      XXXL_RUNTIME_POLICY_ERROR.InvalidEmergencyFreezeThreshold,
    );
  });

  it("rejects duplicate guardian public keys case-insensitively", () => {
    const policy = validPolicy();
    const result = validateXXXLRuntimeRouteGuardianFinalityPolicy({
      ...policy,
      guardian: {
        ...policy.guardian,
        guardianPublicKeys: ["Guardian-1", "guardian-1", "guardian-2"],
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_POLICY_ERROR.DuplicateGuardianPublicKey,
    ]);
  });

  it("rejects too short rotation timelock and weak emergency freeze threshold", () => {
    const policy = validPolicy();
    const result = validateXXXLRuntimeRouteGuardianFinalityPolicy({
      ...policy,
      guardian: {
        ...policy.guardian,
        rotationTimelockSeconds: XXXL_RUNTIME_MIN_ROTATION_TIMELOCK_SECONDS - 1,
        emergencyFreezeThreshold: 1,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_RUNTIME_POLICY_ERROR.RotationTimelockTooShort,
      XXXL_RUNTIME_POLICY_ERROR.InvalidEmergencyFreezeThreshold,
    ]);
  });

  it("supports safe-with-confirmations finality only above the minimum confirmation threshold", () => {
    const policy = validPolicy();

    expect(
      validateXXXLRuntimeRouteGuardianFinalityPolicy({
        ...policy,
        route: {
          ...policy.route,
          finalityRuleId: "ethereum-safe-64",
        },
        finality: {
          ...policy.finality,
          finalityRuleId: "ethereum-safe-64",
          kind: XXXL_RUNTIME_FINALITY_KIND.EthereumSafeWithConfirmations,
          minConfirmations: 64,
        },
      }).ok,
    ).toBe(true);

    const invalid = validateXXXLRuntimeRouteGuardianFinalityPolicy({
      ...policy,
      route: {
        ...policy.route,
        finalityRuleId: "ethereum-safe-63",
      },
      finality: {
        ...policy.finality,
        finalityRuleId: "ethereum-safe-63",
        kind: XXXL_RUNTIME_FINALITY_KIND.EthereumSafeWithConfirmations,
        minConfirmations: 63,
      },
    });

    expect(invalid.ok).toBe(false);
    expect(invalid.errors).toEqual([
      XXXL_RUNTIME_POLICY_ERROR.InvalidFinalityConfirmations,
    ]);
  });
});

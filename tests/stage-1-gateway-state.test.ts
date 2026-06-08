import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  createStage1GatewayState,
  hexToBytes,
  stage1CanonicalEventKeyHex,
  type Stage1GatewayRouteConfig,
} from "../src/index.js";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

type StringRecord = Record<string, string>;

type Stage1GeneratedFixture = {
  sampleInputs: StringRecord;
  validVector: {
    canonicalEventKey: string;
    guardianSignature: {
      guardianPublicKey: string;
    };
  };
};

function readFixture(): Stage1GeneratedFixture {
  return JSON.parse(
    readFileSync(VECTOR_PATH, "utf8"),
  ) as Stage1GeneratedFixture;
}

function required(record: StringRecord, key: string): string {
  const value = record[key];

  if (value === undefined) {
    throw new Error(`Missing fixture key: ${key}`);
  }

  return value;
}

function routeConfigFromFixture(fixture: Stage1GeneratedFixture): Stage1GatewayRouteConfig {
  return {
    sourceToken: required(fixture.sampleInputs, "sourceToken"),
    targetX1NetworkId: required(fixture.sampleInputs, "targetX1NetworkId"),
    targetMintCoreId: required(fixture.sampleInputs, "targetMintCoreId"),
  };
}

describe("Stage 1 gateway state model", () => {
  it("creates an empty Stage 1 gateway state from route and guardian quorum config", () => {
    const fixture = readFixture();
    const guardianPublicKey = hexToBytes(
      fixture.validVector.guardianSignature.guardianPublicKey,
      32,
      "guardianPublicKey",
    );
    const routeConfig = routeConfigFromFixture(fixture);

    const state = createStage1GatewayState({
      routeConfig,
      guardianQuorum: {
        guardianPublicKeys: [guardianPublicKey],
        threshold: 1,
      },
    });

    expect(state.routeConfig).toEqual(routeConfig);
    expect(state.guardianQuorum.threshold).toBe(1);
    expect(state.guardianQuorum.guardianPublicKeys).toEqual([guardianPublicKey]);
    expect(state.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(0);
    expect(state.mintCoreState.balancesByX1Recipient.size).toBe(0);
    expect(state.mintCoreState.totalMinted).toBe(0n);
  });

  it("preloads processed burns, balances, and totalMinted", () => {
    const fixture = readFixture();
    const guardianPublicKey = hexToBytes(
      fixture.validVector.guardianSignature.guardianPublicKey,
      32,
      "guardianPublicKey",
    );
    const canonicalEventKey = hexToBytes(
      fixture.validVector.canonicalEventKey,
      32,
      "canonicalEventKey",
    );
    const recipientHex = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    const state = createStage1GatewayState({
      routeConfig: routeConfigFromFixture(fixture),
      guardianQuorum: {
        guardianPublicKeys: [guardianPublicKey],
        threshold: 1,
      },
      processedCanonicalEventKeyHexes: [
        stage1CanonicalEventKeyHex(canonicalEventKey).toUpperCase(),
      ],
      initialBalancesByX1Recipient: {
        [recipientHex.toUpperCase()]: 123n,
      },
      totalMinted: 456n,
    });

    expect(
      state.processedBurnRegistry.processedCanonicalEventKeys.has(
        stage1CanonicalEventKeyHex(canonicalEventKey),
      ),
    ).toBe(true);
    expect(state.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(123n);
    expect(state.mintCoreState.totalMinted).toBe(456n);
  });

  it("keeps processed burn registry and mint core state as independent mutable sub-states", () => {
    const fixture = readFixture();
    const guardianPublicKey = hexToBytes(
      fixture.validVector.guardianSignature.guardianPublicKey,
      32,
      "guardianPublicKey",
    );
    const state = createStage1GatewayState({
      routeConfig: routeConfigFromFixture(fixture),
      guardianQuorum: {
        guardianPublicKeys: [guardianPublicKey],
        threshold: 1,
      },
    });

    const recipientHex = "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    state.processedBurnRegistry.processedCanonicalEventKeys.add(
      "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
    );
    state.mintCoreState.balancesByX1Recipient.set(recipientHex, 11n);
    state.mintCoreState.totalMinted = 11n;

    expect(state.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(1);
    expect(state.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(11n);
    expect(state.mintCoreState.totalMinted).toBe(11n);
  });
});

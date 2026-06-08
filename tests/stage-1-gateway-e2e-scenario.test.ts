import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_MINT_AUTHORIZATION_ERROR,
  STAGE1_MINT_CORE_ERROR,
  bytes32,
  createStage1GatewayState,
  executeStage1MintCore,
  hexToBytes,
  stage1CanonicalEventKeyHex,
  stage1MintAmountFromFields,
  stage1X1RecipientHex,
  type Stage1GatewayMintMessageFields,
  type Stage1GatewayRouteConfig,
} from "../src/index.js";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

type StringRecord = Record<string, string>;

type Stage1GeneratedFixture = {
  sampleInputs: StringRecord;
  validVector: {
    x1RecipientBytes: string;
    domainSeparator: string;
    messageHash: string;
    encodedFields: StringRecord;
    guardianSignature: {
      guardianPublicKey: string;
      guardianSignature: string;
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

function fieldsFromFixture(
  fixture: Stage1GeneratedFixture,
): Stage1GatewayMintMessageFields {
  return {
    messageType: bytes32(required(fixture.validVector.encodedFields, "messageType")),
    schemaVersion: bytes32(
      required(fixture.validVector.encodedFields, "schemaVersion"),
    ),
    routeId: bytes32(required(fixture.validVector.encodedFields, "routeId")),
    sourceChainId: bytes32(
      required(fixture.validVector.encodedFields, "sourceChainId"),
    ),
    sourceToken: bytes32(required(fixture.validVector.encodedFields, "sourceToken")),
    sourceSender: bytes32(
      required(fixture.validVector.encodedFields, "sourceSender"),
    ),
    sourceBurnTxHash: bytes32(
      required(fixture.validVector.encodedFields, "sourceBurnTxHash"),
    ),
    sourceBurnEventIndex: bytes32(
      required(fixture.validVector.encodedFields, "sourceBurnEventIndex"),
    ),
    sourceBlockNumber: bytes32(
      required(fixture.validVector.encodedFields, "sourceBlockNumber"),
    ),
    sourceBlockHash: bytes32(
      required(fixture.validVector.encodedFields, "sourceBlockHash"),
    ),
    sourceNonce: bytes32(required(fixture.validVector.encodedFields, "sourceNonce")),
    canonicalEventKey: bytes32(
      required(fixture.validVector.encodedFields, "canonicalEventKey"),
    ),
    x1RecipientHash: bytes32(
      required(fixture.validVector.encodedFields, "x1RecipientHash"),
    ),
    burnedAmount: bytes32(required(fixture.validVector.encodedFields, "burnedAmount")),
    sourceChainWeightBps: bytes32(
      required(fixture.validVector.encodedFields, "sourceChainWeightBps"),
    ),
    xxxlMintAmount: bytes32(
      required(fixture.validVector.encodedFields, "xxxlMintAmount"),
    ),
    mintToken: bytes32(required(fixture.validVector.encodedFields, "mintToken")),
    deadlineOrFinalityBlock: bytes32(
      required(fixture.validVector.encodedFields, "deadlineOrFinalityBlock"),
    ),
    messageNonce: bytes32(
      required(fixture.validVector.encodedFields, "messageNonce"),
    ),
  };
}

function routeConfigFromFixture(fixture: Stage1GeneratedFixture): Stage1GatewayRouteConfig {
  return {
    sourceToken: required(fixture.sampleInputs, "sourceToken"),
    targetX1NetworkId: required(fixture.sampleInputs, "targetX1NetworkId"),
    targetMintCoreId: required(fixture.sampleInputs, "targetMintCoreId"),
  };
}

function validStage1GatewayE2eFixtureInput() {
  const fixture = readFixture();
  const fields = fieldsFromFixture(fixture);
  const x1RecipientBytes = hexToBytes(
    fixture.validVector.x1RecipientBytes,
    32,
    "x1RecipientBytes",
  );
  const guardianPublicKey = hexToBytes(
    fixture.validVector.guardianSignature.guardianPublicKey,
    32,
    "guardianPublicKey",
  );
  const guardianSignature = hexToBytes(
    fixture.validVector.guardianSignature.guardianSignature,
    64,
    "guardianSignature",
  );
  const state = createStage1GatewayState({
    routeConfig: routeConfigFromFixture(fixture),
    guardianQuorum: {
      guardianPublicKeys: [guardianPublicKey],
      threshold: 1,
    },
  });

  return {
    fixture,
    state,
    fields,
    x1RecipientBytes,
    guardianPublicKey,
    guardianSignature,
    input: {
      fields,
      x1RecipientBytes,
      domainSeparator: bytes32(fixture.validVector.domainSeparator),
      messageHash: bytes32(fixture.validVector.messageHash),
      routeConfig: state.routeConfig,
      quorum: state.guardianQuorum,
      approvals: [
        {
          guardianPublicKey,
          guardianSignature,
        },
      ],
      processedBurnRegistry: state.processedBurnRegistry,
      mintCoreState: state.mintCoreState,
    },
  };
}

describe("Stage 1 gateway end-to-end scenario", () => {
  it("executes the generated fixture through state-backed mint core and rejects replay", async () => {
    const { state, input, fields, x1RecipientBytes } =
      validStage1GatewayE2eFixtureInput();
    const recipientHex = stage1X1RecipientHex(x1RecipientBytes);
    const canonicalEventKeyHex = stage1CanonicalEventKeyHex(fields.canonicalEventKey);
    const amount = stage1MintAmountFromFields(fields);

    expect(state.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(0);
    expect(state.mintCoreState.balancesByX1Recipient.size).toBe(0);
    expect(state.mintCoreState.totalMinted).toBe(0n);

    const first = await executeStage1MintCore(input);

    expect(first.ok).toBe(true);
    expect(first.minted).toBe(true);
    expect(first.errors).toEqual([]);
    expect(first.authorization.ok).toBe(true);
    expect(first.authorization.authorized).toBe(true);
    expect(first.authorization.markedProcessed).toBe(true);
    expect(first.authorization.quorum.ok).toBe(true);
    expect(first.authorization.quorum.validApprovalCount).toBe(1);
    expect(first.authorization.processedBurn.ok).toBe(true);
    expect(first.recipientHex).toBe(recipientHex);
    expect(first.amount).toBe(amount);
    expect(first.balanceAfter).toBe(amount);
    expect(first.totalMintedAfter).toBe(amount);

    expect(state.processedBurnRegistry.processedCanonicalEventKeys.has(
      canonicalEventKeyHex,
    )).toBe(true);
    expect(state.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(amount);
    expect(state.mintCoreState.totalMinted).toBe(amount);

    const second = await executeStage1MintCore(input);

    expect(second.ok).toBe(false);
    expect(second.minted).toBe(false);
    expect(second.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(second.authorization.ok).toBe(false);
    expect(second.authorization.authorized).toBe(false);
    expect(second.authorization.markedProcessed).toBe(false);
    expect(second.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.BurnAlreadyProcessed,
    ]);
    expect(second.authorization.quorum.ok).toBe(true);
    expect(second.balanceAfter).toBe(amount);
    expect(second.totalMintedAfter).toBe(amount);

    expect(state.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(1);
    expect(state.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(amount);
    expect(state.mintCoreState.totalMinted).toBe(amount);
  });
});

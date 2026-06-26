import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import {
  XXXL_STAGE1_GATEWAY_CONSUMER_ERROR,
  authorizeStage1Mint,
  bytes32,
  createEmptyXXXLProgramState,
  createStage1ProcessedBurnRegistry,
  hexToBytes,
  processXXXLStage1GatewayMintAuthorization,
  stage1MintAmountFromFields,
  type Stage1GatewayMintMessageFields,
} from "../../src/index.js";

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
  invalidVectors: {
    id: string;
    guardianSignature?: string;
  }[];
};

function readFixture(): Stage1GeneratedFixture {
  return JSON.parse(readFileSync(VECTOR_PATH, "utf8")) as Stage1GeneratedFixture;
}

function required(record: StringRecord, key: string): string {
  const value = record[key];

  if (value === undefined) {
    throw new Error(`Missing fixture key: ${key}`);
  }

  return value;
}

function invalidVectorById(
  fixture: Stage1GeneratedFixture,
  id: string,
): Stage1GeneratedFixture["invalidVectors"][number] {
  const vector = fixture.invalidVectors.find((candidate) => candidate.id === id);

  if (vector === undefined) {
    throw new Error(`Missing invalid vector: ${id}`);
  }

  return vector;
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

function routeConfigFromFixture(fixture: Stage1GeneratedFixture) {
  return {
    sourceToken: required(fixture.sampleInputs, "sourceToken"),
    targetX1NetworkId: required(fixture.sampleInputs, "targetX1NetworkId"),
    targetMintCoreId: required(fixture.sampleInputs, "targetMintCoreId"),
  };
}

function validAuthorizationInputFromFixture() {
  const fixture = readFixture();
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
  const fields = fieldsFromFixture(fixture);
  const x1RecipientBytes = hexToBytes(
    fixture.validVector.x1RecipientBytes,
    32,
    "x1RecipientBytes",
  );

  return {
    fixture,
    guardianPublicKey,
    guardianSignature,
    fields,
    x1RecipientBytes,
    input: {
      fields,
      x1RecipientBytes,
      domainSeparator: bytes32(fixture.validVector.domainSeparator),
      messageHash: bytes32(fixture.validVector.messageHash),
      routeConfig: routeConfigFromFixture(fixture),
      quorum: {
        guardianPublicKeys: [guardianPublicKey],
        threshold: 1,
      },
      approvals: [
        {
          guardianPublicKey,
          guardianSignature,
        },
      ],
      processedBurnRegistry: createStage1ProcessedBurnRegistry(),
    },
  };
}

describe("XXXL Stage 1 gateway authorization consumer", () => {
  it("mints XXXL only after a valid Stage 1 mint authorization", async () => {
    const { input, fields, x1RecipientBytes } = validAuthorizationInputFromFixture();
    const authorization = await authorizeStage1Mint(input);
    const state = createEmptyXXXLProgramState();

    const result = processXXXLStage1GatewayMintAuthorization({
      state,
      fields,
      x1RecipientBytes,
      authorization,
    });

    expect(result.ok).toBe(true);
    expect(result.minted).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.amount).toBe(stage1MintAmountFromFields(fields));
    expect(result.totalSupplyAfter).toBe(result.amount);
    expect(result.state.totalSupply).toBe(result.amount);
    expect(result.state.processedGatewayEvents.has(result.canonicalEventKeyHex))
      .toBe(true);
    expect(state.totalSupply).toBe(0n);
    expect(state.processedGatewayEvents.size).toBe(0);
  });

  it("keeps XXXL supply equal to the Stage 1 authorized mint amount", async () => {
    const { input, fields, x1RecipientBytes } = validAuthorizationInputFromFixture();
    const authorization = await authorizeStage1Mint(input);
    const state = createEmptyXXXLProgramState();
    const amount = stage1MintAmountFromFields(fields);

    const result = processXXXLStage1GatewayMintAuthorization({
      state,
      fields,
      x1RecipientBytes,
      authorization,
    });

    expect(result.state.totalSupply).toBe(amount);
    expect(result.totalSupplyAfter).toBe(amount);
  });

  it("rejects a Stage 1 authorization that failed quorum verification", async () => {
    const {
      fixture,
      input,
      fields,
      x1RecipientBytes,
      guardianPublicKey,
    } = validAuthorizationInputFromFixture();
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );

    const authorization = await authorizeStage1Mint({
      ...input,
      approvals: [
        {
          guardianPublicKey,
          guardianSignature: hexToBytes(
            alteredSignatureVector.guardianSignature!,
            64,
            "alteredGuardianSignature",
          ),
        },
      ],
    });
    const state = createEmptyXXXLProgramState();

    const result = processXXXLStage1GatewayMintAuthorization({
      state,
      fields,
      x1RecipientBytes,
      authorization,
    });

    expect(result.ok).toBe(false);
    expect(result.minted).toBe(false);
    expect(result.errors).toEqual([
      XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.Stage1MintNotAuthorized,
      XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.Stage1MintNotMarkedProcessed,
    ]);
    expect(result.state.totalSupply).toBe(0n);
    expect(result.state.processedGatewayEvents.size).toBe(0);
  });

  it("rejects a Stage 1 replay authorization result", async () => {
    const { input, fields, x1RecipientBytes } = validAuthorizationInputFromFixture();

    const firstAuthorization = await authorizeStage1Mint(input);
    const secondAuthorization = await authorizeStage1Mint(input);
    const afterFirst = processXXXLStage1GatewayMintAuthorization({
      state: createEmptyXXXLProgramState(),
      fields,
      x1RecipientBytes,
      authorization: firstAuthorization,
    });

    const afterSecond = processXXXLStage1GatewayMintAuthorization({
      state: afterFirst.state,
      fields,
      x1RecipientBytes,
      authorization: secondAuthorization,
    });

    expect(afterFirst.ok).toBe(true);
    expect(afterSecond.ok).toBe(false);
    expect(afterSecond.minted).toBe(false);
    expect(afterSecond.errors).toEqual([
      XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.Stage1MintNotAuthorized,
      XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.Stage1MintNotMarkedProcessed,
      XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.ReplayedGatewayEvent,
    ]);
    expect(afterSecond.state.totalSupply).toBe(afterFirst.state.totalSupply);
  });

  it("rejects local XXXL replay even if the same Stage 1 authorization object is reused", async () => {
    const { input, fields, x1RecipientBytes } = validAuthorizationInputFromFixture();
    const authorization = await authorizeStage1Mint(input);
    const first = processXXXLStage1GatewayMintAuthorization({
      state: createEmptyXXXLProgramState(),
      fields,
      x1RecipientBytes,
      authorization,
    });

    const replay = processXXXLStage1GatewayMintAuthorization({
      state: first.state,
      fields,
      x1RecipientBytes,
      authorization,
    });

    expect(first.ok).toBe(true);
    expect(replay.ok).toBe(false);
    expect(replay.minted).toBe(false);
    expect(replay.errors).toEqual([
      XXXL_STAGE1_GATEWAY_CONSUMER_ERROR.ReplayedGatewayEvent,
    ]);
    expect(replay.state.totalSupply).toBe(first.state.totalSupply);
    expect(replay.state.processedGatewayEvents.size).toBe(1);
  });
});

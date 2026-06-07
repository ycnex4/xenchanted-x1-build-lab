import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_GUARDIAN_QUORUM_ERROR,
  STAGE1_MINT_AUTHORIZATION_ERROR,
  STAGE1_PROCESSED_BURN_REGISTRY_ERROR,
  authorizeStage1Mint,
  bytes32,
  createStage1ProcessedBurnRegistry,
  hexToBytes,
  stage1CanonicalEventKeyHex,
  type Stage1GatewayMintMessageFields,
} from "../src/index.js";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

type StringRecord = Record<string, string>;

type Stage1GeneratedFixture = {
  sampleInputs: StringRecord;
  validVector: {
    x1RecipientBytes: string;
    domainSeparator: string;
    messageHash: string;
    canonicalEventKey: string;
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

  return {
    fixture,
    guardianPublicKey,
    guardianSignature,
    input: {
      fields: fieldsFromFixture(fixture),
      x1RecipientBytes: hexToBytes(
        fixture.validVector.x1RecipientBytes,
        32,
        "x1RecipientBytes",
      ),
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

describe("Stage 1 mint authorization model", () => {
  it("authorizes a valid quorum for an unprocessed canonicalEventKey and marks it processed", async () => {
    const { input } = validAuthorizationInputFromFixture();

    const result = await authorizeStage1Mint(input);

    expect(result.ok).toBe(true);
    expect(result.authorized).toBe(true);
    expect(result.markedProcessed).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.quorum.ok).toBe(true);
    expect(result.processedBurn.ok).toBe(true);
    expect(result.processedBurn.errors).toEqual([]);
    expect(
      input.processedBurnRegistry.processedCanonicalEventKeys.has(
        stage1CanonicalEventKeyHex(input.fields.canonicalEventKey),
      ),
    ).toBe(true);
  });

  it("rejects duplicate canonicalEventKey replay and does not mark again", async () => {
    const { input } = validAuthorizationInputFromFixture();

    const first = await authorizeStage1Mint(input);
    const second = await authorizeStage1Mint(input);

    expect(first.ok).toBe(true);
    expect(second.ok).toBe(false);
    expect(second.authorized).toBe(false);
    expect(second.markedProcessed).toBe(false);
    expect(second.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.BurnAlreadyProcessed,
    ]);
    expect(second.quorum.ok).toBe(true);
    expect(second.processedBurn.errors).toEqual([
      STAGE1_PROCESSED_BURN_REGISTRY_ERROR.AlreadyProcessed,
    ]);
    expect(input.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(1);
  });

  it("rejects invalid quorum and does not mark processed", async () => {
    const { fixture, input, guardianPublicKey } = validAuthorizationInputFromFixture();
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );

    const result = await authorizeStage1Mint({
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

    expect(result.ok).toBe(false);
    expect(result.authorized).toBe(false);
    expect(result.markedProcessed).toBe(false);
    expect(result.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum,
    ]);
    expect(result.quorum.ok).toBe(false);
    expect(result.quorum.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached,
    ]);
    expect(result.processedBurn.ok).toBe(true);
    expect(input.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(0);
  });

  it("rejects when both quorum is invalid and burn is already processed", async () => {
    const { fixture, input, guardianPublicKey } = validAuthorizationInputFromFixture();
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );

    input.processedBurnRegistry.processedCanonicalEventKeys.add(
      stage1CanonicalEventKeyHex(input.fields.canonicalEventKey),
    );

    const result = await authorizeStage1Mint({
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

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum,
      STAGE1_MINT_AUTHORIZATION_ERROR.BurnAlreadyProcessed,
    ]);
    expect(result.markedProcessed).toBe(false);
    expect(input.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(1);
  });
});

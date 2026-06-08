import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_MINT_AUTHORIZATION_ERROR,
  STAGE1_MINT_CORE_ERROR,
  bytes32,
  createStage1MintCoreState,
  createStage1ProcessedBurnRegistry,
  executeStage1MintCore,
  hexToBytes,
  stage1CanonicalEventKeyHex,
  stage1MintAmountFromFields,
  stage1X1RecipientHex,
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

function validMintCoreInputFromFixture() {
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
  const x1RecipientBytes = hexToBytes(
    fixture.validVector.x1RecipientBytes,
    32,
    "x1RecipientBytes",
  );
  const fields = fieldsFromFixture(fixture);

  return {
    fixture,
    guardianPublicKey,
    guardianSignature,
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
      mintCoreState: createStage1MintCoreState(),
    },
  };
}

describe("Stage 1 mint core model", () => {
  it("mints XXXL to the recipient and increases totalMinted after valid authorization", async () => {
    const { input } = validMintCoreInputFromFixture();
    const amount = stage1MintAmountFromFields(input.fields);
    const recipientHex = stage1X1RecipientHex(input.x1RecipientBytes);

    const result = await executeStage1MintCore(input);

    expect(result.ok).toBe(true);
    expect(result.minted).toBe(true);
    expect(result.errors).toEqual([]);
    expect(result.authorization.ok).toBe(true);
    expect(result.authorization.markedProcessed).toBe(true);
    expect(result.recipientHex).toBe(recipientHex);
    expect(result.amount).toBe(amount);
    expect(result.balanceAfter).toBe(amount);
    expect(result.totalMintedAfter).toBe(amount);
    expect(input.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(amount);
    expect(input.mintCoreState.totalMinted).toBe(amount);
    expect(
      input.processedBurnRegistry.processedCanonicalEventKeys.has(
        stage1CanonicalEventKeyHex(input.fields.canonicalEventKey),
      ),
    ).toBe(true);
  });

  it("adds to an existing recipient balance and totalMinted", async () => {
    const { input } = validMintCoreInputFromFixture();
    const amount = stage1MintAmountFromFields(input.fields);
    const recipientHex = stage1X1RecipientHex(input.x1RecipientBytes);

    input.mintCoreState = createStage1MintCoreState(
      {
        [recipientHex]: 11n,
      },
      111n,
    );

    const result = await executeStage1MintCore(input);

    expect(result.ok).toBe(true);
    expect(result.balanceAfter).toBe(amount + 11n);
    expect(result.totalMintedAfter).toBe(amount + 111n);
    expect(input.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(
      amount + 11n,
    );
    expect(input.mintCoreState.totalMinted).toBe(amount + 111n);
  });

  it("rejects replay without changing balance or totalMinted", async () => {
    const { input } = validMintCoreInputFromFixture();
    const amount = stage1MintAmountFromFields(input.fields);
    const recipientHex = stage1X1RecipientHex(input.x1RecipientBytes);

    const first = await executeStage1MintCore(input);
    const second = await executeStage1MintCore(input);

    expect(first.ok).toBe(true);
    expect(second.ok).toBe(false);
    expect(second.minted).toBe(false);
    expect(second.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(second.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.BurnAlreadyProcessed,
    ]);
    expect(second.balanceAfter).toBe(amount);
    expect(second.totalMintedAfter).toBe(amount);
    expect(input.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(amount);
    expect(input.mintCoreState.totalMinted).toBe(amount);
  });

  it("rejects invalid authorization without changing balance, totalMinted, or processed registry", async () => {
    const { fixture, input, guardianPublicKey } = validMintCoreInputFromFixture();
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );
    const recipientHex = stage1X1RecipientHex(input.x1RecipientBytes);

    const result = await executeStage1MintCore({
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
    expect(result.minted).toBe(false);
    expect(result.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(result.authorization.ok).toBe(false);
    expect(result.balanceAfter).toBe(0n);
    expect(result.totalMintedAfter).toBe(0n);
    expect(input.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(
      undefined,
    );
    expect(input.mintCoreState.totalMinted).toBe(0n);
    expect(input.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(0);
  });
});

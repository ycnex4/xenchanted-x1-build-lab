import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_ED25519_VERIFICATION_ERROR,
  STAGE1_VERIFICATION_ERROR,
  bytes32,
  hexToBytes,
  uint256Be,
  verifyStage1GatewayApproval,
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
    messageHash?: string;
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

function validApprovalInputFromFixture() {
  const fixture = readFixture();

  return {
    fixture,
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
      guardianPublicKey: hexToBytes(
        fixture.validVector.guardianSignature.guardianPublicKey,
        32,
        "guardianPublicKey",
      ),
      guardianSignature: hexToBytes(
        fixture.validVector.guardianSignature.guardianSignature,
        64,
        "guardianSignature",
      ),
    },
  };
}

function cloneFields(
  fields: Stage1GatewayMintMessageFields,
): Stage1GatewayMintMessageFields {
  return Object.fromEntries(
    Object.entries(fields).map(([key, value]) => [key, new Uint8Array(value)]),
  ) as Stage1GatewayMintMessageFields;
}

describe("Stage 1 gateway approval verifier", () => {
  it("accepts a valid message with a valid guardian signature", async () => {
    const { input } = validApprovalInputFromFixture();

    const result = await verifyStage1GatewayApproval(input);

    expect(result).toEqual({
      ok: true,
      message: {
        ok: true,
        errors: [],
      },
      signature: {
        ok: true,
        errors: [],
      },
    });
  });

  it("rejects an invalid message even when the signature is valid for the supplied hash", async () => {
    const { input } = validApprovalInputFromFixture();
    const fields = cloneFields(input.fields);

    fields.sourceChainId = uint256Be(2);

    const result = await verifyStage1GatewayApproval({
      ...input,
      fields,
    });

    expect(result.ok).toBe(false);
    expect(result.message.ok).toBe(false);
    expect(result.message.errors).toEqual(
      expect.arrayContaining([
        STAGE1_VERIFICATION_ERROR.WrongSourceChainId,
        STAGE1_VERIFICATION_ERROR.WrongMessageHash,
      ]),
    );
    expect(result.signature).toEqual({
      ok: true,
      errors: [],
    });
  });

  it("rejects an invalid signature even when the message is valid", async () => {
    const { fixture, input } = validApprovalInputFromFixture();
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );

    const result = await verifyStage1GatewayApproval({
      ...input,
      guardianSignature: hexToBytes(
        alteredSignatureVector.guardianSignature!,
        64,
        "alteredGuardianSignature",
      ),
    });

    expect(result.ok).toBe(false);
    expect(result.message).toEqual({
      ok: true,
      errors: [],
    });
    expect(result.signature).toEqual({
      ok: false,
      errors: [STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed],
    });
  });

  it("rejects when both message verification and signature verification fail", async () => {
    const { fixture, input } = validApprovalInputFromFixture();
    const fields = cloneFields(input.fields);
    const wrongMessageHashVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_MESSAGE_HASH",
    );

    fields.burnedAmount = uint256Be(0);

    const result = await verifyStage1GatewayApproval({
      ...input,
      fields,
      messageHash: hexToBytes(
        wrongMessageHashVector.messageHash!,
        32,
        "wrongMessageHash",
      ),
    });

    expect(result.ok).toBe(false);
    expect(result.message.ok).toBe(false);
    expect(result.message.errors).toEqual(
      expect.arrayContaining([
        STAGE1_VERIFICATION_ERROR.BurnedAmountZero,
        STAGE1_VERIFICATION_ERROR.WrongMessageHash,
      ]),
    );
    expect(result.signature).toEqual({
      ok: false,
      errors: [STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed],
    });
  });

  it("rejects malformed Ed25519 approval inputs", async () => {
    const { input } = validApprovalInputFromFixture();

    const result = await verifyStage1GatewayApproval({
      ...input,
      guardianPublicKey: new Uint8Array(31),
      guardianSignature: new Uint8Array(63),
    });

    expect(result.ok).toBe(false);
    expect(result.message).toEqual({
      ok: true,
      errors: [],
    });
    expect(result.signature).toEqual({
      ok: false,
      errors: [
        STAGE1_ED25519_VERIFICATION_ERROR.InvalidGuardianPublicKeyLength,
        STAGE1_ED25519_VERIFICATION_ERROR.InvalidGuardianSignatureLength,
      ],
    });
  });
});

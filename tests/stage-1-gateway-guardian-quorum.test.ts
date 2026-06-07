import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_ED25519_VERIFICATION_ERROR,
  STAGE1_GUARDIAN_QUORUM_ERROR,
  bytes32,
  hexToBytes,
  verifyStage1GuardianQuorum,
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

function validQuorumInputFromFixture() {
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
    },
  };
}

describe("Stage 1 gateway guardian quorum model", () => {
  it("accepts a valid one-of-one guardian quorum", async () => {
    const { input, guardianPublicKey } = validQuorumInputFromFixture();

    const result = await verifyStage1GuardianQuorum(input);

    expect(result.ok).toBe(true);
    expect(result.threshold).toBe(1);
    expect(result.validApprovalCount).toBe(1);
    expect(result.acceptedGuardianPublicKeyHexes).toEqual([
      `0x${Buffer.from(guardianPublicKey).toString("hex")}`,
    ]);
    expect(result.errors).toEqual([]);
    expect(result.approvals).toHaveLength(1);
    expect(result.approvals[0]?.ok).toBe(true);
    expect(result.approvals[0]?.errors).toEqual([]);
  });

  it("rejects an empty guardian set and invalid threshold", async () => {
    const { input } = validQuorumInputFromFixture();

    const result = await verifyStage1GuardianQuorum({
      ...input,
      quorum: {
        guardianPublicKeys: [],
        threshold: 1,
      },
      approvals: [],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual(
      expect.arrayContaining([
        STAGE1_GUARDIAN_QUORUM_ERROR.EmptyGuardianSet,
        STAGE1_GUARDIAN_QUORUM_ERROR.InvalidThreshold,
        STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached,
      ]),
    );
    expect(result.validApprovalCount).toBe(0);
  });

  it("rejects quorum not reached", async () => {
    const { input, guardianPublicKey } = validQuorumInputFromFixture();
    const secondGuardian = new Uint8Array(guardianPublicKey);
    secondGuardian[0]! ^= 1;

    const result = await verifyStage1GuardianQuorum({
      ...input,
      quorum: {
        guardianPublicKeys: [guardianPublicKey, secondGuardian],
        threshold: 2,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(1);
    expect(result.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached,
    ]);
  });

  it("rejects duplicate guardian approvals", async () => {
    const { input, guardianPublicKey, guardianSignature } =
      validQuorumInputFromFixture();

    const result = await verifyStage1GuardianQuorum({
      ...input,
      approvals: [
        {
          guardianPublicKey,
          guardianSignature,
        },
        {
          guardianPublicKey,
          guardianSignature,
        },
      ],
    });

    expect(result.ok).toBe(true);
    expect(result.validApprovalCount).toBe(1);
    expect(result.approvals[0]?.ok).toBe(true);
    expect(result.approvals[1]?.ok).toBe(false);
    expect(result.approvals[1]?.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.DuplicateGuardianApproval,
    ]);
  });

  it("rejects unknown guardians", async () => {
    const { input, guardianPublicKey, guardianSignature } =
      validQuorumInputFromFixture();
    const unknownGuardian = new Uint8Array(guardianPublicKey);
    unknownGuardian[0]! ^= 1;

    const result = await verifyStage1GuardianQuorum({
      ...input,
      approvals: [
        {
          guardianPublicKey: unknownGuardian,
          guardianSignature,
        },
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(0);
    expect(result.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached,
    ]);
    expect(result.approvals[0]?.errors).toEqual(
      expect.arrayContaining([
        STAGE1_GUARDIAN_QUORUM_ERROR.UnknownGuardian,
        STAGE1_GUARDIAN_QUORUM_ERROR.InvalidApproval,
      ]),
    );
    expect(result.approvals[0]?.approval?.signature.errors).toEqual([
      STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed,
    ]);
  });

  it("rejects invalid approval signatures", async () => {
    const { fixture, input, guardianPublicKey } = validQuorumInputFromFixture();
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );

    const result = await verifyStage1GuardianQuorum({
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
    expect(result.validApprovalCount).toBe(0);
    expect(result.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached,
    ]);
    expect(result.approvals[0]?.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.InvalidApproval,
    ]);
    expect(result.approvals[0]?.approval?.signature.errors).toEqual([
      STAGE1_ED25519_VERIFICATION_ERROR.SignatureVerificationFailed,
    ]);
  });
});

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_GUARDIAN_QUORUM_ERROR,
  STAGE1_MINT_AUTHORIZATION_ERROR,
  STAGE1_MINT_CORE_ERROR,
  bytes32,
  createStage1GatewayState,
  executeStage1MintCore,
  hexToBytes,
  stage1CanonicalEventKeyHex,
  stage1MintAmountFromFields,
  stage1X1RecipientHex,
  uint256Be,
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

function cloneFields(
  fields: Stage1GatewayMintMessageFields,
): Stage1GatewayMintMessageFields {
  return Object.fromEntries(
    Object.entries(fields).map(([key, value]) => [key, new Uint8Array(value)]),
  ) as Stage1GatewayMintMessageFields;
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

function expectNoMintMutation(
  state: ReturnType<typeof createStage1GatewayState>,
  recipientHex: string,
): void {
  expect(state.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(0);
  expect(state.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(
    undefined,
  );
  expect(state.mintCoreState.totalMinted).toBe(0n);
}

describe("Stage 1 gateway negative end-to-end matrix", () => {
  it("rejects malformed message fields without mutating gateway state", async () => {
    const { state, input, fields, x1RecipientBytes } =
      validStage1GatewayE2eFixtureInput();
    const recipientHex = stage1X1RecipientHex(x1RecipientBytes);
    const malformedFields = cloneFields(fields);
    malformedFields.sourceChainId = uint256Be(2);

    const result = await executeStage1MintCore({
      ...input,
      fields: malformedFields,
    });

    expect(result.ok).toBe(false);
    expect(result.minted).toBe(false);
    expect(result.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(result.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum,
    ]);
    expect(result.authorization.quorum.ok).toBe(false);
    expect(result.authorization.markedProcessed).toBe(false);
    expect(result.balanceAfter).toBe(0n);
    expect(result.totalMintedAfter).toBe(0n);
    expectNoMintMutation(state, recipientHex);
  });

  it("rejects route mismatch without mutating gateway state", async () => {
    const { state, input, x1RecipientBytes } = validStage1GatewayE2eFixtureInput();
    const recipientHex = stage1X1RecipientHex(x1RecipientBytes);

    const result = await executeStage1MintCore({
      ...input,
      routeConfig: {
        ...input.routeConfig,
        targetMintCoreId:
          "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
      },
    });

    expect(result.ok).toBe(false);
    expect(result.minted).toBe(false);
    expect(result.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(result.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum,
    ]);
    expect(result.authorization.quorum.ok).toBe(false);
    expect(result.authorization.markedProcessed).toBe(false);
    expect(result.balanceAfter).toBe(0n);
    expect(result.totalMintedAfter).toBe(0n);
    expectNoMintMutation(state, recipientHex);
  });

  it("rejects invalid guardian signature without mutating gateway state", async () => {
    const { fixture, state, input, guardianPublicKey, x1RecipientBytes } =
      validStage1GatewayE2eFixtureInput();
    const recipientHex = stage1X1RecipientHex(x1RecipientBytes);
    const alteredSignatureVector = invalidVectorById(
      fixture,
      "INVALID_WRONG_ED25519_SIGNATURE",
    );

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
    expect(result.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum,
    ]);
    expect(result.authorization.quorum.ok).toBe(false);
    expect(result.authorization.markedProcessed).toBe(false);
    expect(result.balanceAfter).toBe(0n);
    expect(result.totalMintedAfter).toBe(0n);
    expectNoMintMutation(state, recipientHex);
  });

  it("rejects unknown guardian approval without mutating gateway state", async () => {
    const { state, input, guardianPublicKey, x1RecipientBytes } =
      validStage1GatewayE2eFixtureInput();
    const recipientHex = stage1X1RecipientHex(x1RecipientBytes);
    const unknownAllowedGuardian = new Uint8Array(guardianPublicKey);
    unknownAllowedGuardian[0]! ^= 1;

    const result = await executeStage1MintCore({
      ...input,
      quorum: {
        guardianPublicKeys: [unknownAllowedGuardian],
        threshold: 1,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.minted).toBe(false);
    expect(result.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(result.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum,
    ]);
    expect(result.authorization.quorum.ok).toBe(false);
    expect(result.authorization.quorum.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached,
    ]);
    expect(result.authorization.quorum.approvals[0]?.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.UnknownGuardian,
    ]);
    expect(result.authorization.markedProcessed).toBe(false);
    expect(result.balanceAfter).toBe(0n);
    expect(result.totalMintedAfter).toBe(0n);
    expectNoMintMutation(state, recipientHex);
  });

  it("rejects quorum failure without mutating gateway state", async () => {
    const { state, input, guardianPublicKey, x1RecipientBytes } =
      validStage1GatewayE2eFixtureInput();
    const recipientHex = stage1X1RecipientHex(x1RecipientBytes);
    const secondGuardian = new Uint8Array(guardianPublicKey);
    secondGuardian[0]! ^= 1;

    const result = await executeStage1MintCore({
      ...input,
      quorum: {
        guardianPublicKeys: [guardianPublicKey, secondGuardian],
        threshold: 2,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.minted).toBe(false);
    expect(result.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(result.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.InvalidQuorum,
    ]);
    expect(result.authorization.quorum.ok).toBe(false);
    expect(result.authorization.quorum.validApprovalCount).toBe(1);
    expect(result.authorization.quorum.errors).toEqual([
      STAGE1_GUARDIAN_QUORUM_ERROR.QuorumNotReached,
    ]);
    expect(result.authorization.markedProcessed).toBe(false);
    expect(result.balanceAfter).toBe(0n);
    expect(result.totalMintedAfter).toBe(0n);
    expectNoMintMutation(state, recipientHex);
  });

  it("rejects preprocessed canonicalEventKey replay without minting", async () => {
    const { state, input, fields, x1RecipientBytes } =
      validStage1GatewayE2eFixtureInput();
    const recipientHex = stage1X1RecipientHex(x1RecipientBytes);
    const canonicalEventKeyHex = stage1CanonicalEventKeyHex(fields.canonicalEventKey);
    const amount = stage1MintAmountFromFields(fields);

    state.processedBurnRegistry.processedCanonicalEventKeys.add(canonicalEventKeyHex);

    const result = await executeStage1MintCore(input);

    expect(result.ok).toBe(false);
    expect(result.minted).toBe(false);
    expect(result.errors).toEqual([STAGE1_MINT_CORE_ERROR.MintNotAuthorized]);
    expect(result.authorization.errors).toEqual([
      STAGE1_MINT_AUTHORIZATION_ERROR.BurnAlreadyProcessed,
    ]);
    expect(result.authorization.quorum.ok).toBe(true);
    expect(result.authorization.markedProcessed).toBe(false);
    expect(result.amount).toBe(amount);
    expect(result.balanceAfter).toBe(0n);
    expect(result.totalMintedAfter).toBe(0n);
    expect(state.processedBurnRegistry.processedCanonicalEventKeys.size).toBe(1);
    expect(state.mintCoreState.balancesByX1Recipient.get(recipientHex)).toBe(
      undefined,
    );
    expect(state.mintCoreState.totalMinted).toBe(0n);
  });
});

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_VERIFICATION_ERROR,
  bytes32,
  bytesToHex,
  computeStage1CanonicalEventKey,
  computeStage1DomainSeparator,
  computeStage1GatewayValues,
  computeStage1MessageHash,
  computeStage1X1RecipientHash,
  hexToBytes,
  uint256Be,
  verifyStage1GatewayMintMessage,
  type Stage1GatewayMintMessageFields,
} from "../src/index.js";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

type StringRecord = Record<string, string>;

type Stage1GeneratedFixture = {
  sampleInputs: StringRecord;
  constants: StringRecord;
  validVector: {
    x1RecipientBytes: string;
    x1RecipientHash: string;
    canonicalEventKey: string;
    domainSeparator: string;
    encodedFields: StringRecord;
    messageHash: string;
    messageHashPreimage: string;
    encodedGatewayMintMessage: string;
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

function routeConfigFromFixture(fixture: Stage1GeneratedFixture) {
  return {
    sourceToken: required(fixture.sampleInputs, "sourceToken"),
    targetX1NetworkId: required(fixture.sampleInputs, "targetX1NetworkId"),
    targetMintCoreId: required(fixture.sampleInputs, "targetMintCoreId"),
  };
}

function validInputFromFixture() {
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

describe("Stage 1 gateway verifier helpers", () => {
  it("computes the locked fixture values", () => {
    const { fixture, input } = validInputFromFixture();

    expect(bytesToHex(computeStage1CanonicalEventKey(input.fields))).toBe(
      fixture.validVector.canonicalEventKey,
    );
    expect(bytesToHex(computeStage1X1RecipientHash(input.x1RecipientBytes))).toBe(
      fixture.validVector.x1RecipientHash,
    );
    expect(bytesToHex(computeStage1DomainSeparator(input.routeConfig))).toBe(
      fixture.validVector.domainSeparator,
    );
    expect(bytesToHex(computeStage1MessageHash(input.fields, input.domainSeparator))).toBe(
      fixture.validVector.messageHash,
    );

    const computed = computeStage1GatewayValues({
      fields: input.fields,
      x1RecipientBytes: input.x1RecipientBytes,
      domainSeparator: input.domainSeparator,
    });

    expect(bytesToHex(computed.encodedGatewayMintMessage)).toBe(
      fixture.validVector.encodedGatewayMintMessage,
    );
    expect(bytesToHex(computed.messageHashPreimage)).toBe(
      fixture.validVector.messageHashPreimage,
    );
    expect(bytesToHex(computed.messageHash)).toBe(fixture.validVector.messageHash);
  });

  it("accepts the valid Stage 1 fixture", () => {
    const { input } = validInputFromFixture();
    const result = verifyStage1GatewayMintMessage(input);

    expect(result).toEqual({
      ok: true,
      errors: [],
    });
  });

  it("rejects wrong route constants", () => {
    const { input } = validInputFromFixture();
    const fields = cloneFields(input.fields);

    fields.messageType = bytes32(
      "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );
    fields.routeId = bytes32(
      "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    );
    fields.mintToken = bytes32(
      "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
    );

    const result = verifyStage1GatewayMintMessage({
      ...input,
      fields,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual(
      expect.arrayContaining([
        STAGE1_VERIFICATION_ERROR.WrongMessageType,
        STAGE1_VERIFICATION_ERROR.WrongRouteId,
        STAGE1_VERIFICATION_ERROR.WrongMintToken,
        STAGE1_VERIFICATION_ERROR.WrongMessageHash,
      ]),
    );
  });

  it("rejects wrong source route and amount rules", () => {
    const { input } = validInputFromFixture();
    const fields = cloneFields(input.fields);

    fields.sourceChainId = uint256Be(2);
    fields.sourceToken = bytes32(
      "0x0000000000000000000000003333333333333333333333333333333333333333",
    );
    fields.burnedAmount = uint256Be(0);
    fields.sourceChainWeightBps = uint256Be(9999);
    fields.xxxlMintAmount = uint256Be(123);

    const result = verifyStage1GatewayMintMessage({
      ...input,
      fields,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual(
      expect.arrayContaining([
        STAGE1_VERIFICATION_ERROR.WrongSourceChainId,
        STAGE1_VERIFICATION_ERROR.WrongSourceToken,
        STAGE1_VERIFICATION_ERROR.BurnedAmountZero,
        STAGE1_VERIFICATION_ERROR.WrongSourceChainWeightBps,
        STAGE1_VERIFICATION_ERROR.XxxlMintAmountMismatch,
        STAGE1_VERIFICATION_ERROR.WrongMessageHash,
      ]),
    );
  });

  it("rejects wrong evidence binding and recipient binding", () => {
    const { input } = validInputFromFixture();
    const fields = cloneFields(input.fields);

    fields.canonicalEventKey = bytes32(
      "0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
    );
    fields.x1RecipientHash = bytes32(
      "0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
    );

    const result = verifyStage1GatewayMintMessage({
      ...input,
      fields,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual(
      expect.arrayContaining([
        STAGE1_VERIFICATION_ERROR.WrongCanonicalEventKey,
        STAGE1_VERIFICATION_ERROR.WrongX1RecipientHash,
        STAGE1_VERIFICATION_ERROR.WrongMessageHash,
      ]),
    );
  });

  it("rejects invalid recipient bytes", () => {
    const { input } = validInputFromFixture();

    expect(
      verifyStage1GatewayMintMessage({
        ...input,
        x1RecipientBytes: new Uint8Array(),
      }).errors,
    ).toContain(STAGE1_VERIFICATION_ERROR.InvalidX1RecipientLength);

    expect(
      verifyStage1GatewayMintMessage({
        ...input,
        x1RecipientBytes: new Uint8Array(32),
      }).errors,
    ).toContain(STAGE1_VERIFICATION_ERROR.ZeroX1Recipient);
  });

  it("rejects wrong domain separator and message hash", () => {
    const { input } = validInputFromFixture();

    const wrongDomainSeparator = bytes32(
      "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    );
    const wrongMessageHash = bytes32(
      "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    );

    const result = verifyStage1GatewayMintMessage({
      ...input,
      domainSeparator: wrongDomainSeparator,
      messageHash: wrongMessageHash,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual(
      expect.arrayContaining([
        STAGE1_VERIFICATION_ERROR.WrongDomainSeparator,
        STAGE1_VERIFICATION_ERROR.WrongMessageHash,
      ]),
    );
  });
});

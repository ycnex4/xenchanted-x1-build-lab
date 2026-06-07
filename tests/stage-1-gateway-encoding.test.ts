import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import {
  STAGE1_GATEWAY_FIELD_ORDER,
  addressToBytes32LeftPadded,
  buildStage1CanonicalEventKeyPreimage,
  buildStage1DomainSeparatorPreimage,
  buildStage1MessageHashPreimage,
  bytes32,
  bytesToHex,
  encodeStage1GatewayMintMessage,
  hexToBytes,
  keccakBytes,
  keccakUtf8Label,
  uint256Be,
} from "../src/index.js";

const VECTOR_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

type StringRecord = Record<string, string>;

type Stage1GeneratedFixture = {
  fieldOrder: string[];
  sampleInputs: StringRecord;
  constants: StringRecord;
  validVector: {
    x1RecipientBytes: string;
    x1RecipientHash: string;
    canonicalEventKeyPreimage: string;
    canonicalEventKey: string;
    domainSeparatorPreimage: string;
    domainSeparator: string;
    encodedFields: StringRecord;
    encodedGatewayMintMessage: string;
    messageHashPreimage: string;
    messageHash: string;
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

describe("Stage 1 gateway encoding helpers", () => {
  it("encodes uint256 values as 32-byte big-endian words", () => {
    expect(bytesToHex(uint256Be(0))).toBe(
      "0x0000000000000000000000000000000000000000000000000000000000000000",
    );
    expect(bytesToHex(uint256Be(1))).toBe(
      "0x0000000000000000000000000000000000000000000000000000000000000001",
    );
    expect(bytesToHex(uint256Be(19000000))).toBe(
      "0x000000000000000000000000000000000000000000000000000000000121eac0",
    );
    expect(bytesToHex(uint256Be("1000000000000000000000"))).toBe(
      "0x00000000000000000000000000000000000000000000003635c9adc5dea00000",
    );
  });

  it("left-pads Ethereum addresses to 32 bytes", () => {
    expect(
      bytesToHex(addressToBytes32LeftPadded("0x1111111111111111111111111111111111111111")),
    ).toBe(
      "0x0000000000000000000000001111111111111111111111111111111111111111",
    );

    expect(() =>
      addressToBytes32LeftPadded("0x111111111111111111111111111111111111111100"),
    ).toThrow("Ethereum address must be 20 bytes");
  });

  it("reproduces the generated Stage 1 fixture hashes and preimages", () => {
    const fixture = readFixture();
    const sampleInputs = fixture.sampleInputs;
    const validVector = fixture.validVector;

    expect(fixture.fieldOrder).toEqual([...STAGE1_GATEWAY_FIELD_ORDER]);

    const messageType = bytes32(
      keccakUtf8Label(required(sampleInputs, "messageTypeLabel")),
      "messageType",
    );
    const routeId = bytes32(
      keccakUtf8Label(required(sampleInputs, "routeIdLabel")),
      "routeId",
    );
    const mintToken = bytes32(
      keccakUtf8Label(required(sampleInputs, "mintTokenLabel")),
      "mintToken",
    );
    const protocolNameHash = bytes32(
      keccakUtf8Label(required(sampleInputs, "protocolNameLabel")),
      "protocolNameHash",
    );
    const gatewayVersionHash = bytes32(
      keccakUtf8Label(required(sampleInputs, "gatewayVersionLabel")),
      "gatewayVersionHash",
    );
    const messageTypeFamilyHash = bytes32(
      keccakUtf8Label(required(sampleInputs, "messageTypeFamilyLabel")),
      "messageTypeFamilyHash",
    );

    expect(bytesToHex(messageType)).toBe(required(fixture.constants, "messageType"));
    expect(bytesToHex(routeId)).toBe(required(fixture.constants, "routeId"));
    expect(bytesToHex(mintToken)).toBe(required(fixture.constants, "mintToken"));
    expect(bytesToHex(protocolNameHash)).toBe(
      required(fixture.constants, "protocolNameHash"),
    );
    expect(bytesToHex(gatewayVersionHash)).toBe(
      required(fixture.constants, "gatewayVersionHash"),
    );
    expect(bytesToHex(messageTypeFamilyHash)).toBe(
      required(fixture.constants, "messageTypeFamilyHash"),
    );

    const sourceChainId = uint256Be(required(sampleInputs, "sourceChainId"));
    const sourceToken = addressToBytes32LeftPadded(
      required(sampleInputs, "sourceToken"),
    );
    const sourceBurnTxHash = bytes32(
      required(sampleInputs, "sourceBurnTxHash"),
      "sourceBurnTxHash",
    );
    const sourceBurnEventIndex = uint256Be(
      required(sampleInputs, "sourceBurnEventIndex"),
    );

    const canonicalEventKeyPreimage = buildStage1CanonicalEventKeyPreimage({
      sourceChainId,
      sourceToken,
      sourceBurnTxHash,
      sourceBurnEventIndex,
    });
    const canonicalEventKey = bytes32(
      keccakBytes(canonicalEventKeyPreimage),
      "canonicalEventKey",
    );

    expect(canonicalEventKeyPreimage).toHaveLength(128);
    expect(bytesToHex(canonicalEventKeyPreimage)).toBe(
      validVector.canonicalEventKeyPreimage,
    );
    expect(bytesToHex(canonicalEventKey)).toBe(validVector.canonicalEventKey);

    const x1RecipientBytes = hexToBytes(
      required(sampleInputs, "x1RecipientBytes"),
      32,
      "x1RecipientBytes",
    );
    const x1RecipientHash = bytes32(
      keccakBytes(x1RecipientBytes),
      "x1RecipientHash",
    );

    expect(bytesToHex(x1RecipientBytes)).toBe(validVector.x1RecipientBytes);
    expect(bytesToHex(x1RecipientHash)).toBe(validVector.x1RecipientHash);

    const domainSeparatorPreimage = buildStage1DomainSeparatorPreimage({
      protocolNameHash,
      gatewayVersionHash,
      targetX1NetworkId: uint256Be(required(sampleInputs, "targetX1NetworkId")),
      targetMintCoreId: bytes32(
        required(sampleInputs, "targetMintCoreId"),
        "targetMintCoreId",
      ),
      messageTypeFamilyHash,
    });
    const domainSeparator = bytes32(
      keccakBytes(domainSeparatorPreimage),
      "domainSeparator",
    );

    expect(domainSeparatorPreimage).toHaveLength(160);
    expect(bytesToHex(domainSeparatorPreimage)).toBe(
      validVector.domainSeparatorPreimage,
    );
    expect(bytesToHex(domainSeparator)).toBe(validVector.domainSeparator);

    const fields = {
      messageType,
      schemaVersion: uint256Be(required(sampleInputs, "schemaVersion")),
      routeId,
      sourceChainId,
      sourceToken,
      sourceSender: addressToBytes32LeftPadded(
        required(sampleInputs, "sourceSender"),
      ),
      sourceBurnTxHash,
      sourceBurnEventIndex,
      sourceBlockNumber: uint256Be(required(sampleInputs, "sourceBlockNumber")),
      sourceBlockHash: bytes32(
        required(sampleInputs, "sourceBlockHash"),
        "sourceBlockHash",
      ),
      sourceNonce: uint256Be(required(sampleInputs, "sourceNonce")),
      canonicalEventKey,
      x1RecipientHash,
      burnedAmount: uint256Be(required(sampleInputs, "burnedAmount")),
      sourceChainWeightBps: uint256Be(
        required(sampleInputs, "sourceChainWeightBps"),
      ),
      xxxlMintAmount: uint256Be(required(sampleInputs, "xxxlMintAmount")),
      mintToken,
      deadlineOrFinalityBlock: uint256Be(
        required(sampleInputs, "deadlineOrFinalityBlock"),
      ),
      messageNonce: uint256Be(required(sampleInputs, "messageNonce")),
    };

    const encodedGatewayMintMessage = encodeStage1GatewayMintMessage(fields);
    const messageHashPreimage = buildStage1MessageHashPreimage(
      domainSeparator,
      encodedGatewayMintMessage,
    );
    const messageHash = keccakBytes(messageHashPreimage);

    expect(encodedGatewayMintMessage).toHaveLength(608);
    expect(messageHashPreimage).toHaveLength(640);
    expect(bytesToHex(encodedGatewayMintMessage)).toBe(
      validVector.encodedGatewayMintMessage,
    );
    expect(bytesToHex(messageHashPreimage)).toBe(validVector.messageHashPreimage);
    expect(messageHash).toBe(validVector.messageHash);

    for (const fieldName of STAGE1_GATEWAY_FIELD_ORDER) {
      expect(bytesToHex(fields[fieldName])).toBe(
        required(validVector.encodedFields, fieldName),
      );
    }
  });

  it("rejects malformed fixed-width inputs", () => {
    const validFields = Object.fromEntries(
      STAGE1_GATEWAY_FIELD_ORDER.map((fieldName) => [
        fieldName,
        new Uint8Array(32),
      ]),
    ) as Record<(typeof STAGE1_GATEWAY_FIELD_ORDER)[number], Uint8Array>;

    validFields.sourceNonce = new Uint8Array(31);

    expect(() => encodeStage1GatewayMintMessage(validFields)).toThrow(
      "sourceNonce must be exactly 32 bytes",
    );

    expect(() =>
      buildStage1MessageHashPreimage(new Uint8Array(31), new Uint8Array(608)),
    ).toThrow("domainSeparator must be exactly 32 bytes");
  });
});

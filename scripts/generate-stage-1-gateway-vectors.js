import { mkdir, writeFile } from "node:fs/promises";
import { dirname, resolve } from "node:path";
import { keccak256 } from "viem";
import * as ed25519 from "@noble/ed25519";

const OUTPUT_PATH = "docs/gateway/generated/stage-1-gateway-vectors.json";

const FIELD_ORDER = [
  "messageType",
  "schemaVersion",
  "routeId",
  "sourceChainId",
  "sourceToken",
  "sourceSender",
  "sourceBurnTxHash",
  "sourceBurnEventIndex",
  "sourceBlockNumber",
  "sourceBlockHash",
  "sourceNonce",
  "canonicalEventKey",
  "x1RecipientHash",
  "burnedAmount",
  "sourceChainWeightBps",
  "xxxlMintAmount",
  "mintToken",
  "deadlineOrFinalityBlock",
  "messageNonce",
];

const TEST_ONLY_GUARDIAN_PRIVATE_KEY_SEED =
  "0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";

function strip0x(value) {
  if (typeof value !== "string") {
    throw new TypeError("Expected hex string");
  }

  return value.startsWith("0x") ? value.slice(2) : value;
}

function assertEvenHex(hex, label) {
  if (hex.length % 2 !== 0) {
    throw new Error(`${label} must have an even number of hex characters`);
  }

  if (!/^[0-9a-fA-F]*$/.test(hex)) {
    throw new Error(`${label} must be hex`);
  }
}

function hexToBytes(value, expectedLength, label = "hex value") {
  const hex = strip0x(value);
  assertEvenHex(hex, label);

  const bytes = Uint8Array.from(Buffer.from(hex, "hex"));

  if (expectedLength !== undefined && bytes.length !== expectedLength) {
    throw new Error(
      `${label} must be ${expectedLength} bytes, got ${bytes.length}`,
    );
  }

  return bytes;
}

function bytesToHex(bytes) {
  return `0x${Buffer.from(bytes).toString("hex")}`;
}

function concatBytes(...parts) {
  const totalLength = parts.reduce((sum, part) => sum + part.length, 0);
  const out = new Uint8Array(totalLength);

  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }

  return out;
}

function utf8Bytes(value) {
  return new TextEncoder().encode(value);
}

function keccakBytes(bytes) {
  return keccak256(bytesToHex(bytes));
}

function keccakLabel(label) {
  return keccakBytes(utf8Bytes(label));
}

function uint256Be(value) {
  const number = BigInt(value);
  const maxUint256 = (1n << 256n) - 1n;

  if (number < 0n || number > maxUint256) {
    throw new Error(`uint256 out of range: ${value}`);
  }

  return hexToBytes(`0x${number.toString(16).padStart(64, "0")}`, 32, "uint256");
}

function bytes32(value, label = "bytes32") {
  return hexToBytes(value, 32, label);
}

function addressToBytes32LeftPadded(address) {
  const addressBytes = hexToBytes(address, 20, "Ethereum address");
  return concatBytes(new Uint8Array(12), addressBytes);
}

function asciiDecimalStringPaddedTo32(value) {
  const ascii = utf8Bytes(String(value));
  if (ascii.length > 32) {
    throw new Error("ASCII decimal string is too long for this invalid vector");
  }

  const out = new Uint8Array(32);
  out.set(ascii, 0);
  return out;
}

function encodeGatewayMintMessage(fields, order = FIELD_ORDER) {
  const encodedFields = order.map((fieldName) => {
    const field = fields[fieldName];

    if (!field) {
      throw new Error(`Missing field: ${fieldName}`);
    }

    if (field.length !== 32) {
      throw new Error(`${fieldName} must be exactly 32 bytes`);
    }

    return field;
  });

  return concatBytes(...encodedFields);
}

function encodeGatewayMintMessageAllowingOmission(fields, order) {
  return concatBytes(...order.map((fieldName) => fields[fieldName]));
}

function buildCanonicalEventKeyPreimage({
  sourceChainId,
  sourceToken,
  sourceBurnTxHash,
  sourceBurnEventIndex,
}) {
  return concatBytes(
    sourceChainId,
    sourceToken,
    sourceBurnTxHash,
    sourceBurnEventIndex,
  );
}

function buildDomainSeparatorPreimage({
  protocolNameHash,
  gatewayVersionHash,
  targetX1NetworkId,
  targetMintCoreId,
  messageTypeFamilyHash,
}) {
  return concatBytes(
    protocolNameHash,
    gatewayVersionHash,
    targetX1NetworkId,
    targetMintCoreId,
    messageTypeFamilyHash,
  );
}

function buildMessageHashPreimage(domainSeparator, encodedGatewayMintMessage) {
  return concatBytes(domainSeparator, encodedGatewayMintMessage);
}

function cloneFields(fields) {
  return Object.fromEntries(
    Object.entries(fields).map(([key, value]) => [key, new Uint8Array(value)]),
  );
}

function fieldHexMap(fields) {
  return Object.fromEntries(
    FIELD_ORDER.map((fieldName) => [fieldName, bytesToHex(fields[fieldName])]),
  );
}

function makeMessageInvalidVector({
  id,
  description,
  expectedRejectionReason,
  fields,
  order = FIELD_ORDER,
  domainSeparator,
  extra = {},
}) {
  const encodedGatewayMintMessage = encodeGatewayMintMessage(fields, order);
  const messageHashPreimage = buildMessageHashPreimage(
    domainSeparator,
    encodedGatewayMintMessage,
  );
  const messageHash = keccakBytes(messageHashPreimage);

  return {
    id,
    description,
    expectedRejectionReason,
    fieldOrder: order,
    encodedGatewayMintMessage: bytesToHex(encodedGatewayMintMessage),
    encodedGatewayMintMessageLengthBytes: encodedGatewayMintMessage.length,
    messageHashPreimage: bytesToHex(messageHashPreimage),
    messageHashPreimageLengthBytes: messageHashPreimage.length,
    messageHash,
    ...extra,
  };
}

function makeMutatedFieldVector({
  id,
  description,
  expectedRejectionReason,
  validFields,
  fieldName,
  value,
  domainSeparator,
  extra = {},
}) {
  const fields = cloneFields(validFields);
  fields[fieldName] = value;

  return makeMessageInvalidVector({
    id,
    description,
    expectedRejectionReason,
    fields,
    domainSeparator,
    extra: {
      mutatedField: fieldName,
      mutatedFieldValue: bytesToHex(value),
      ...extra,
    },
  });
}

function requireLength(bytes, expectedLength, label) {
  if (bytes.length !== expectedLength) {
    throw new Error(`${label} length expected ${expectedLength}, got ${bytes.length}`);
  }
}

async function main() {
  const labels = {
    messageTypeLabel: "X1_GATEWAY_MINT",
    routeIdLabel: "ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1",
    mintTokenLabel: "XXXL",
    protocolNameLabel: "xEnchanted XNTD-to-XXXL Gateway",
    gatewayVersionLabel: "Stage1",
    messageTypeFamilyLabel: "X1GatewayMintMessage",
  };

  const sampleInputs = {
    vectorId: "STAGE1_GATEWAY_VALID_001",
    ...labels,
    schemaVersion: "1",
    sourceChainId: "1",
    sourceToken: "0x1111111111111111111111111111111111111111",
    sourceSender: "0x2222222222222222222222222222222222222222",
    sourceBurnTxHash:
      "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    sourceBurnEventIndex: "7",
    sourceBlockNumber: "19000000",
    sourceBlockHash:
      "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    sourceNonce: "42",
    x1RecipientBytes:
      "0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20",
    burnedAmount: "1000000000000000000000",
    sourceChainWeightBps: "10000",
    xxxlMintAmount: "1000000000000000000000",
    deadlineOrFinalityBlock: "0",
    messageNonce: "0",
    targetX1NetworkId: "1001",
    targetMintCoreId:
      "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
  };

  const constants = {
    messageType: bytes32(keccakLabel(labels.messageTypeLabel), "messageType"),
    routeId: bytes32(keccakLabel(labels.routeIdLabel), "routeId"),
    mintToken: bytes32(keccakLabel(labels.mintTokenLabel), "mintToken"),
    protocolNameHash: bytes32(
      keccakLabel(labels.protocolNameLabel),
      "protocolNameHash",
    ),
    gatewayVersionHash: bytes32(
      keccakLabel(labels.gatewayVersionLabel),
      "gatewayVersionHash",
    ),
    messageTypeFamilyHash: bytes32(
      keccakLabel(labels.messageTypeFamilyLabel),
      "messageTypeFamilyHash",
    ),
  };

  const sourceChainId = uint256Be(sampleInputs.sourceChainId);
  const sourceToken = addressToBytes32LeftPadded(sampleInputs.sourceToken);
  const sourceBurnTxHash = bytes32(
    sampleInputs.sourceBurnTxHash,
    "sourceBurnTxHash",
  );
  const sourceBurnEventIndex = uint256Be(sampleInputs.sourceBurnEventIndex);

  const canonicalEventKeyPreimage = buildCanonicalEventKeyPreimage({
    sourceChainId,
    sourceToken,
    sourceBurnTxHash,
    sourceBurnEventIndex,
  });
  const canonicalEventKey = bytes32(
    keccakBytes(canonicalEventKeyPreimage),
    "canonicalEventKey",
  );

  const x1RecipientBytes = hexToBytes(
    sampleInputs.x1RecipientBytes,
    32,
    "x1RecipientBytes",
  );
  const x1RecipientHash = bytes32(
    keccakBytes(x1RecipientBytes),
    "x1RecipientHash",
  );

  const domainSeparatorPreimage = buildDomainSeparatorPreimage({
    protocolNameHash: constants.protocolNameHash,
    gatewayVersionHash: constants.gatewayVersionHash,
    targetX1NetworkId: uint256Be(sampleInputs.targetX1NetworkId),
    targetMintCoreId: bytes32(sampleInputs.targetMintCoreId, "targetMintCoreId"),
    messageTypeFamilyHash: constants.messageTypeFamilyHash,
  });
  const domainSeparator = bytes32(
    keccakBytes(domainSeparatorPreimage),
    "domainSeparator",
  );

  const validFields = {
    messageType: constants.messageType,
    schemaVersion: uint256Be(sampleInputs.schemaVersion),
    routeId: constants.routeId,
    sourceChainId,
    sourceToken,
    sourceSender: addressToBytes32LeftPadded(sampleInputs.sourceSender),
    sourceBurnTxHash,
    sourceBurnEventIndex,
    sourceBlockNumber: uint256Be(sampleInputs.sourceBlockNumber),
    sourceBlockHash: bytes32(sampleInputs.sourceBlockHash, "sourceBlockHash"),
    sourceNonce: uint256Be(sampleInputs.sourceNonce),
    canonicalEventKey,
    x1RecipientHash,
    burnedAmount: uint256Be(sampleInputs.burnedAmount),
    sourceChainWeightBps: uint256Be(sampleInputs.sourceChainWeightBps),
    xxxlMintAmount: uint256Be(sampleInputs.xxxlMintAmount),
    mintToken: constants.mintToken,
    deadlineOrFinalityBlock: uint256Be(sampleInputs.deadlineOrFinalityBlock),
    messageNonce: uint256Be(sampleInputs.messageNonce),
  };

  const encodedGatewayMintMessage = encodeGatewayMintMessage(validFields);
  const messageHashPreimage = buildMessageHashPreimage(
    domainSeparator,
    encodedGatewayMintMessage,
  );
  const messageHash = bytes32(keccakBytes(messageHashPreimage), "messageHash");

  requireLength(canonicalEventKeyPreimage, 128, "canonicalEventKeyPreimage");
  requireLength(domainSeparatorPreimage, 160, "domainSeparatorPreimage");
  requireLength(encodedGatewayMintMessage, 608, "encodedGatewayMintMessage");
  requireLength(messageHashPreimage, 640, "messageHashPreimage");

  const guardianPrivateKeySeedBytes = hexToBytes(
    TEST_ONLY_GUARDIAN_PRIVATE_KEY_SEED,
    32,
    "TEST_ONLY_GUARDIAN_PRIVATE_KEY_SEED",
  );
  const guardianPublicKeyBytes = await ed25519.getPublicKeyAsync(
    guardianPrivateKeySeedBytes,
  );
  const guardianSignatureBytes = await ed25519.signAsync(
    messageHash,
    guardianPrivateKeySeedBytes,
  );

  const signatureVerifies = await ed25519.verifyAsync(
    guardianSignatureBytes,
    messageHash,
    guardianPublicKeyBytes,
  );

  if (!signatureVerifies) {
    throw new Error("Valid Ed25519 signature failed verification");
  }

  const wrongMessageHash = bytes32(
    keccakBytes(utf8Bytes("wrong Stage 1 message hash test vector")),
    "wrongMessageHash",
  );
  const wrongGuardianPrivateKeySeedBytes = hexToBytes(
    "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    32,
    "wrong test-only seed",
  );
  const wrongGuardianPublicKeyBytes = await ed25519.getPublicKeyAsync(
    wrongGuardianPrivateKeySeedBytes,
  );
  const alteredSignatureBytes = new Uint8Array(guardianSignatureBytes);
  alteredSignatureBytes[0] ^= 1;

  const validSignatureOverDifferentMessageHashBytes = await ed25519.signAsync(
    wrongMessageHash,
    guardianPrivateKeySeedBytes,
  );

  const verificationChecks = {
    validSignatureOverMessageHash: signatureVerifies,
    validSignatureWrongMessageHashFails: !(await ed25519.verifyAsync(
      guardianSignatureBytes,
      wrongMessageHash,
      guardianPublicKeyBytes,
    )),
    validSignatureWrongPublicKeyFails: !(await ed25519.verifyAsync(
      guardianSignatureBytes,
      messageHash,
      wrongGuardianPublicKeyBytes,
    )),
    alteredSignatureFails: !(await ed25519.verifyAsync(
      alteredSignatureBytes,
      messageHash,
      guardianPublicKeyBytes,
    )),
    validSignatureOverDifferentMessageHashFailsForOriginalMessageHash: !(await ed25519.verifyAsync(
      validSignatureOverDifferentMessageHashBytes,
      messageHash,
      guardianPublicKeyBytes,
    )),
  };

  if (!Object.values(verificationChecks).every(Boolean)) {
    throw new Error(`Unexpected Ed25519 verification result: ${JSON.stringify(verificationChecks)}`);
  }

  const invalidVectors = [];

  const wrongFieldOrder = [...FIELD_ORDER];
  const routeIdIndex = wrongFieldOrder.indexOf("routeId");
  const sourceChainIdIndex = wrongFieldOrder.indexOf("sourceChainId");
  [wrongFieldOrder[routeIdIndex], wrongFieldOrder[sourceChainIdIndex]] = [
    wrongFieldOrder[sourceChainIdIndex],
    wrongFieldOrder[routeIdIndex],
  ];

  invalidVectors.push(
    makeMessageInvalidVector({
      id: "INVALID_WRONG_FIELD_ORDER_ROUTE_ID_SOURCE_CHAIN_ID_SWAPPED",
      description: "routeId and sourceChainId are swapped in the encoded message.",
      expectedRejectionReason:
        "Message hash must not match the canonical field-order hash.",
      fields: validFields,
      order: wrongFieldOrder,
      domainSeparator,
    }),
  );

  for (const omittedFieldName of ["deadlineOrFinalityBlock", "messageNonce"]) {
    const omittedOrder = FIELD_ORDER.filter((fieldName) => fieldName !== omittedFieldName);
    const encoded = encodeGatewayMintMessageAllowingOmission(validFields, omittedOrder);
    const preimage = buildMessageHashPreimage(domainSeparator, encoded);

    invalidVectors.push({
      id: `INVALID_${omittedFieldName.toUpperCase()}_OMITTED`,
      description: `${omittedFieldName} is omitted instead of being included as a zero-filled 32-byte field.`,
      expectedRejectionReason:
        "All 19 fields are mandatory; unused optional fields must be zero-filled, not omitted.",
      omittedField: omittedFieldName,
      fieldOrder: omittedOrder,
      encodedGatewayMintMessage: bytesToHex(encoded),
      encodedGatewayMintMessageLengthBytes: encoded.length,
      messageHashPreimage: bytesToHex(preimage),
      messageHashPreimageLengthBytes: preimage.length,
      messageHash: keccakBytes(preimage),
    });
  }

  invalidVectors.push(
    makeMutatedFieldVector({
      id: "INVALID_BURNED_AMOUNT_DECIMAL_STRING_ENCODING",
      description:
        "burnedAmount is encoded as ASCII decimal string bytes instead of uint256 big-endian 32-byte word.",
      expectedRejectionReason:
        "Amounts must be uint256 big-endian 32-byte words, never decimal strings.",
      validFields,
      fieldName: "burnedAmount",
      value: asciiDecimalStringPaddedTo32(sampleInputs.burnedAmount),
      domainSeparator,
    }),
  );

  invalidVectors.push(
    makeMutatedFieldVector({
      id: "INVALID_WRONG_SOURCE_CHAIN_ID",
      description: "sourceChainId is 2 instead of Ethereum mainnet chainId 1.",
      expectedRejectionReason:
        "Immutable Stage 1 route requires sourceChainId = 1.",
      validFields,
      fieldName: "sourceChainId",
      value: uint256Be(2),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_SOURCE_TOKEN",
      description: "sourceToken is not the configured Ethereum XNTD token address.",
      expectedRejectionReason:
        "Immutable Stage 1 route requires the exact configured sourceToken.",
      validFields,
      fieldName: "sourceToken",
      value: addressToBytes32LeftPadded("0x3333333333333333333333333333333333333333"),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_SOURCE_BURN_TX_HASH",
      description: "sourceBurnTxHash differs from the verified Ethereum burn transaction.",
      expectedRejectionReason:
        "canonicalEventKey/evidence binding must match the verified source burn tx hash.",
      validFields,
      fieldName: "sourceBurnTxHash",
      value: bytes32("0xabababababababababababababababababababababababababababababababab"),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_SOURCE_BURN_EVENT_INDEX",
      description: "sourceBurnEventIndex differs from the verified Ethereum burn event index.",
      expectedRejectionReason:
        "canonicalEventKey/evidence binding must match the verified source burn event index.",
      validFields,
      fieldName: "sourceBurnEventIndex",
      value: uint256Be(8),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_SOURCE_BLOCK_NUMBER",
      description: "sourceBlockNumber differs from the finalized Ethereum source block.",
      expectedRejectionReason:
        "Finality evidence must bind the exact sourceBlockNumber.",
      validFields,
      fieldName: "sourceBlockNumber",
      value: uint256Be(19000001),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_SOURCE_BLOCK_HASH",
      description: "sourceBlockHash differs from the finalized Ethereum source block hash.",
      expectedRejectionReason:
        "Finality evidence must bind the exact sourceBlockHash.",
      validFields,
      fieldName: "sourceBlockHash",
      value: bytes32("0xbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbcbc"),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_CANONICAL_EVENT_KEY",
      description: "canonicalEventKey does not match the canonical event key preimage.",
      expectedRejectionReason:
        "canonicalEventKey must equal keccak256(sourceChainId || sourceToken || sourceBurnTxHash || sourceBurnEventIndex).",
      validFields,
      fieldName: "canonicalEventKey",
      value: bytes32("0xdddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_X1_RECIPIENT_HASH",
      description: "x1RecipientHash does not match the supplied raw 32-byte x1RecipientBytes.",
      expectedRejectionReason:
        "x1RecipientHash must equal keccak256(x1RecipientBytes).",
      validFields,
      fieldName: "x1RecipientHash",
      value: bytes32("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"),
      domainSeparator,
    }),
  );

  for (const recipientCase of [
    {
      id: "INVALID_EMPTY_X1_RECIPIENT_BYTES",
      bytes: new Uint8Array(),
      reason: "x1RecipientBytes must be exactly 32 bytes and non-zero.",
    },
    {
      id: "INVALID_NON_32_BYTE_X1_RECIPIENT_BYTES",
      bytes: hexToBytes("0x010203", 3, "non-32-byte recipient test"),
      reason: "x1RecipientBytes must be exactly 32 bytes.",
    },
    {
      id: "INVALID_ZERO_X1_RECIPIENT_BYTES",
      bytes: new Uint8Array(32),
      reason: "x1RecipientBytes must not be 32 zero bytes.",
    },
  ]) {
    invalidVectors.push({
      id: recipientCase.id,
      description: "Invalid raw recipient bytes supplied outside the signed payload.",
      expectedRejectionReason: recipientCase.reason,
      x1RecipientBytes: bytesToHex(recipientCase.bytes),
      x1RecipientBytesLengthBytes: recipientCase.bytes.length,
      computedX1RecipientHash: keccakBytes(recipientCase.bytes),
    });
  }

  invalidVectors.push(
    makeMutatedFieldVector({
      id: "INVALID_BURNED_AMOUNT_ZERO",
      description: "burnedAmount is zero.",
      expectedRejectionReason: "burnedAmount must be greater than zero.",
      validFields,
      fieldName: "burnedAmount",
      value: uint256Be(0),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_XXXL_MINT_AMOUNT_DIFFERS_FROM_BURNED_AMOUNT",
      description: "xxxlMintAmount differs from burnedAmount.",
      expectedRejectionReason:
        "Stage 1 full-weight accounting requires xxxlMintAmount = burnedAmount.",
      validFields,
      fieldName: "xxxlMintAmount",
      value: uint256Be(999),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_SOURCE_CHAIN_WEIGHT_BPS_NOT_10000",
      description: "sourceChainWeightBps is 9999 instead of 10000.",
      expectedRejectionReason:
        "Stage 1 full-weight accounting requires sourceChainWeightBps = 10000.",
      validFields,
      fieldName: "sourceChainWeightBps",
      value: uint256Be(9999),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_MINT_TOKEN",
      description: "mintToken is not keccak256('XXXL').",
      expectedRejectionReason:
        "Immutable Stage 1 route requires mintToken = keccak256('XXXL').",
      validFields,
      fieldName: "mintToken",
      value: bytes32(keccakLabel("XNTD"), "wrong mintToken"),
      domainSeparator,
    }),
    makeMutatedFieldVector({
      id: "INVALID_WRONG_ROUTE_ID",
      description: "routeId is not keccak256('ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1').",
      expectedRejectionReason:
        "Immutable Stage 1 route requires the exact routeId.",
      validFields,
      fieldName: "routeId",
      value: bytes32(keccakLabel("WRONG_ROUTE"), "wrong routeId"),
      domainSeparator,
    }),
  );

  const wrongDomainSeparator = bytes32(
    keccakBytes(utf8Bytes("wrong domain separator")),
    "wrongDomainSeparator",
  );

  invalidVectors.push(
    makeMessageInvalidVector({
      id: "INVALID_WRONG_DOMAIN_SEPARATOR",
      description: "The message hash is computed with a wrong domainSeparator.",
      expectedRejectionReason:
        "domainSeparator must be derived from the exact protocol, version, target network, target mint core, and message family.",
      fields: validFields,
      domainSeparator: wrongDomainSeparator,
      extra: {
        mutatedDomainSeparator: bytesToHex(wrongDomainSeparator),
      },
    }),
  );

  for (const domainCase of [
    {
      id: "INVALID_WRONG_TARGET_X1_NETWORK_ID",
      description: "targetX1NetworkId is 1002 instead of 1001.",
      targetX1NetworkId: uint256Be(1002),
      targetMintCoreId: bytes32(sampleInputs.targetMintCoreId, "targetMintCoreId"),
    },
    {
      id: "INVALID_WRONG_TARGET_MINT_CORE_ID",
      description: "targetMintCoreId differs from the configured mint core id.",
      targetX1NetworkId: uint256Be(sampleInputs.targetX1NetworkId),
      targetMintCoreId: bytes32(
        "0xcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcdcd",
        "wrong targetMintCoreId",
      ),
    },
  ]) {
    const mutatedDomainPreimage = buildDomainSeparatorPreimage({
      protocolNameHash: constants.protocolNameHash,
      gatewayVersionHash: constants.gatewayVersionHash,
      targetX1NetworkId: domainCase.targetX1NetworkId,
      targetMintCoreId: domainCase.targetMintCoreId,
      messageTypeFamilyHash: constants.messageTypeFamilyHash,
    });
    const mutatedDomainSeparator = bytes32(
      keccakBytes(mutatedDomainPreimage),
      "mutatedDomainSeparator",
    );

    invalidVectors.push(
      makeMessageInvalidVector({
        id: domainCase.id,
        description: domainCase.description,
        expectedRejectionReason:
          "Domain separator must bind the exact target X1 network and target mint core.",
        fields: validFields,
        domainSeparator: mutatedDomainSeparator,
        extra: {
          mutatedDomainSeparatorPreimage: bytesToHex(mutatedDomainPreimage),
          mutatedDomainSeparator: bytesToHex(mutatedDomainSeparator),
        },
      }),
    );
  }

  invalidVectors.push(
    {
      id: "INVALID_WRONG_MESSAGE_HASH",
      description: "A guardian signature is checked against the wrong messageHash.",
      expectedRejectionReason:
        "Ed25519 signature must verify against the exact canonical messageHash.",
      messageHash: bytesToHex(wrongMessageHash),
      guardianPublicKey: bytesToHex(guardianPublicKeyBytes),
      guardianSignature: bytesToHex(guardianSignatureBytes),
      signatureVerifies: await ed25519.verifyAsync(
        guardianSignatureBytes,
        wrongMessageHash,
        guardianPublicKeyBytes,
      ),
    },
    {
      id: "INVALID_WRONG_ED25519_SIGNATURE",
      description: "The valid signature is altered by flipping one bit.",
      expectedRejectionReason:
        "Altered Ed25519 signature must fail verification.",
      messageHash: bytesToHex(messageHash),
      guardianPublicKey: bytesToHex(guardianPublicKeyBytes),
      guardianSignature: bytesToHex(alteredSignatureBytes),
      signatureVerifies: await ed25519.verifyAsync(
        alteredSignatureBytes,
        messageHash,
        guardianPublicKeyBytes,
      ),
    },
    {
      id: "INVALID_VALID_SIGNATURE_OVER_DIFFERENT_MESSAGE_HASH",
      description:
        "The signature is valid for a different messageHash, but invalid for this vector's canonical messageHash.",
      expectedRejectionReason:
        "A signature over any different messageHash must not authorize this canonical message.",
      canonicalMessageHash: bytesToHex(messageHash),
      differentMessageHash: bytesToHex(wrongMessageHash),
      guardianPublicKey: bytesToHex(guardianPublicKeyBytes),
      guardianSignature: bytesToHex(validSignatureOverDifferentMessageHashBytes),
      verifiesAgainstDifferentMessageHash: await ed25519.verifyAsync(
        validSignatureOverDifferentMessageHashBytes,
        wrongMessageHash,
        guardianPublicKeyBytes,
      ),
      verifiesAgainstCanonicalMessageHash: await ed25519.verifyAsync(
        validSignatureOverDifferentMessageHashBytes,
        messageHash,
        guardianPublicKeyBytes,
      ),
    },
    {
      id: "INVALID_DUPLICATE_CANONICAL_EVENT_KEY_ALREADY_PROCESSED",
      description:
        "The same canonicalEventKey is submitted after it has already been processed.",
      expectedRejectionReason:
        "Processed burn registry must reject duplicate canonicalEventKey before minting.",
      canonicalEventKey: bytesToHex(canonicalEventKey),
      assumedProcessedBeforeSubmission: true,
    },
  );

  const output = {
    metadata: {
      generatedBy: "scripts/generate-stage-1-gateway-vectors.js",
      warning:
        "This file contains deterministic test-only Ed25519 key material for public test vectors. It must never be used for production signing.",
      vectorProfile:
        "Stage 1 XNTD-to-XXXL Gateway fixed-width custom big-endian signed payload encoding",
      encoding:
        "Every signed field is exactly 32 bytes. Unsigned integers are uint256 big-endian 32-byte words. Ethereum addresses are 20 bytes left-padded with zero bytes to 32 bytes. Optional fields are zero-filled, never omitted.",
      hashFunction: "keccak256",
      signatureStandard: "Ed25519",
      signaturePayload: "messageHash",
      x1RecipientType: "32 raw bytes X1/SVM public key",
      processedRegistryKey: "canonicalEventKey",
    },
    fieldOrder: FIELD_ORDER,
    lengths: {
      canonicalEventKeyPreimageBytes: canonicalEventKeyPreimage.length,
      domainSeparatorPreimageBytes: domainSeparatorPreimage.length,
      encodedGatewayMintMessageBytes: encodedGatewayMintMessage.length,
      messageHashPreimageBytes: messageHashPreimage.length,
      ed25519PrivateKeySeedBytes: guardianPrivateKeySeedBytes.length,
      ed25519PublicKeyBytes: guardianPublicKeyBytes.length,
      ed25519SignatureBytes: guardianSignatureBytes.length,
    },
    sampleInputs,
    constants: {
      messageType: bytesToHex(constants.messageType),
      routeId: bytesToHex(constants.routeId),
      mintToken: bytesToHex(constants.mintToken),
      protocolNameHash: bytesToHex(constants.protocolNameHash),
      gatewayVersionHash: bytesToHex(constants.gatewayVersionHash),
      messageTypeFamilyHash: bytesToHex(constants.messageTypeFamilyHash),
    },
    validVector: {
      id: sampleInputs.vectorId,
      sourceChainId: bytesToHex(sourceChainId),
      sourceToken: bytesToHex(sourceToken),
      sourceBurnTxHash: bytesToHex(sourceBurnTxHash),
      sourceBurnEventIndex: bytesToHex(sourceBurnEventIndex),
      x1RecipientBytes: bytesToHex(x1RecipientBytes),
      x1RecipientHash: bytesToHex(x1RecipientHash),
      canonicalEventKeyPreimage: bytesToHex(canonicalEventKeyPreimage),
      canonicalEventKey: bytesToHex(canonicalEventKey),
      domainSeparatorPreimage: bytesToHex(domainSeparatorPreimage),
      domainSeparator: bytesToHex(domainSeparator),
      encodedFields: fieldHexMap(validFields),
      encodedGatewayMintMessage: bytesToHex(encodedGatewayMintMessage),
      messageHashPreimage: bytesToHex(messageHashPreimage),
      messageHash: bytesToHex(messageHash),
      guardianSignature: {
        warning:
          "Deterministic test-only Ed25519 vector material. Never use this seed outside public test vectors.",
        guardianPrivateKeySeed: TEST_ONLY_GUARDIAN_PRIVATE_KEY_SEED,
        guardianPublicKey: bytesToHex(guardianPublicKeyBytes),
        messageHash: bytesToHex(messageHash),
        guardianSignature: bytesToHex(guardianSignatureBytes),
        signatureVerifies,
      },
    },
    signatureVerificationChecks: verificationChecks,
    invalidVectors,
  };

  await mkdir(dirname(resolve(OUTPUT_PATH)), { recursive: true });
  await writeFile(`${OUTPUT_PATH}`, `${JSON.stringify(output, null, 2)}\n`);

  console.log(`Wrote ${OUTPUT_PATH}`);
  console.log(`Valid vector: ${sampleInputs.vectorId}`);
  console.log(`Invalid vectors: ${invalidVectors.length}`);
  console.log(`messageHash: ${bytesToHex(messageHash)}`);
  console.log(`guardianPublicKey: ${bytesToHex(guardianPublicKeyBytes)}`);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});

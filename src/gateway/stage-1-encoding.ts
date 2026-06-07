import { keccak256, type Hex } from "viem";

export const STAGE1_GATEWAY_FIELD_ORDER = [
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
] as const;

export type Stage1GatewayFieldName =
  (typeof STAGE1_GATEWAY_FIELD_ORDER)[number];

export type Stage1GatewayMintMessageFields = Record<
  Stage1GatewayFieldName,
  Uint8Array
>;

export function strip0x(value: string): string {
  return value.startsWith("0x") ? value.slice(2) : value;
}

export function bytesToHex(bytes: Uint8Array): Hex {
  return `0x${Buffer.from(bytes).toString("hex")}` as Hex;
}

export function hexToBytes(
  value: string,
  expectedLength?: number,
  label = "hex value",
): Uint8Array {
  const hex = strip0x(value);

  if (hex.length % 2 !== 0) {
    throw new Error(`${label} must have an even number of hex characters`);
  }

  if (!/^[0-9a-fA-F]*$/.test(hex)) {
    throw new Error(`${label} must be hex`);
  }

  const bytes = Uint8Array.from(Buffer.from(hex, "hex"));

  if (expectedLength !== undefined && bytes.length !== expectedLength) {
    throw new Error(
      `${label} must be ${expectedLength} bytes, got ${bytes.length}`,
    );
  }

  return bytes;
}

export function concatBytes(...parts: Uint8Array[]): Uint8Array {
  const totalLength = parts.reduce((sum, part) => sum + part.length, 0);
  const out = new Uint8Array(totalLength);

  let offset = 0;

  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }

  return out;
}

export function utf8Bytes(value: string): Uint8Array {
  return new TextEncoder().encode(value);
}

export function keccakBytes(bytes: Uint8Array): Hex {
  return keccak256(bytesToHex(bytes));
}

export function keccakUtf8Label(label: string): Hex {
  return keccakBytes(utf8Bytes(label));
}

export function bytes32(value: string, label = "bytes32"): Uint8Array {
  return hexToBytes(value, 32, label);
}

export function uint256Be(value: string | number | bigint): Uint8Array {
  const number = BigInt(value);
  const maxUint256 = (1n << 256n) - 1n;

  if (number < 0n || number > maxUint256) {
    throw new Error(`uint256 out of range: ${value.toString()}`);
  }

  return hexToBytes(
    `0x${number.toString(16).padStart(64, "0")}`,
    32,
    "uint256",
  );
}

export function addressToBytes32LeftPadded(address: string): Uint8Array {
  const addressBytes = hexToBytes(address, 20, "Ethereum address");

  return concatBytes(new Uint8Array(12), addressBytes);
}

export function buildStage1CanonicalEventKeyPreimage(input: {
  sourceChainId: Uint8Array;
  sourceToken: Uint8Array;
  sourceBurnTxHash: Uint8Array;
  sourceBurnEventIndex: Uint8Array;
}): Uint8Array {
  return concatBytes(
    input.sourceChainId,
    input.sourceToken,
    input.sourceBurnTxHash,
    input.sourceBurnEventIndex,
  );
}

export function buildStage1DomainSeparatorPreimage(input: {
  protocolNameHash: Uint8Array;
  gatewayVersionHash: Uint8Array;
  targetX1NetworkId: Uint8Array;
  targetMintCoreId: Uint8Array;
  messageTypeFamilyHash: Uint8Array;
}): Uint8Array {
  return concatBytes(
    input.protocolNameHash,
    input.gatewayVersionHash,
    input.targetX1NetworkId,
    input.targetMintCoreId,
    input.messageTypeFamilyHash,
  );
}

export function encodeStage1GatewayMintMessage(
  fields: Stage1GatewayMintMessageFields,
): Uint8Array {
  return concatBytes(
    ...STAGE1_GATEWAY_FIELD_ORDER.map((fieldName) => {
      const value = fields[fieldName];

      if (value.length !== 32) {
        throw new Error(`${fieldName} must be exactly 32 bytes`);
      }

      return value;
    }),
  );
}

export function buildStage1MessageHashPreimage(
  domainSeparator: Uint8Array,
  encodedGatewayMintMessage: Uint8Array,
): Uint8Array {
  if (domainSeparator.length !== 32) {
    throw new Error(
      `domainSeparator must be exactly 32 bytes, got ${domainSeparator.length}`,
    );
  }

  return concatBytes(domainSeparator, encodedGatewayMintMessage);
}

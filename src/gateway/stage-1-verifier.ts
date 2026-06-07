import {
  addressToBytes32LeftPadded,
  buildStage1CanonicalEventKeyPreimage,
  buildStage1DomainSeparatorPreimage,
  buildStage1MessageHashPreimage,
  bytes32,
  encodeStage1GatewayMintMessage,
  keccakBytes,
  keccakUtf8Label,
  uint256Be,
  type Stage1GatewayMintMessageFields,
} from "./stage-1-encoding.js";

export const STAGE1_GATEWAY_MESSAGE_TYPE_LABEL = "X1_GATEWAY_MINT";
export const STAGE1_GATEWAY_ROUTE_ID_LABEL =
  "ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1";
export const STAGE1_GATEWAY_MINT_TOKEN_LABEL = "XXXL";
export const STAGE1_GATEWAY_PROTOCOL_NAME_LABEL =
  "xEnchanted XNTD-to-XXXL Gateway";
export const STAGE1_GATEWAY_VERSION_LABEL = "Stage1";
export const STAGE1_GATEWAY_MESSAGE_TYPE_FAMILY_LABEL =
  "X1GatewayMintMessage";

export const STAGE1_ETHEREUM_MAINNET_CHAIN_ID = 1n;
export const STAGE1_SOURCE_CHAIN_WEIGHT_BPS = 10000n;

export const STAGE1_VERIFICATION_ERROR = {
  InvalidX1RecipientLength: "INVALID_X1_RECIPIENT_LENGTH",
  ZeroX1Recipient: "ZERO_X1_RECIPIENT",
  WrongX1RecipientHash: "WRONG_X1_RECIPIENT_HASH",
  WrongCanonicalEventKey: "WRONG_CANONICAL_EVENT_KEY",
  WrongDomainSeparator: "WRONG_DOMAIN_SEPARATOR",
  WrongMessageHash: "WRONG_MESSAGE_HASH",
  WrongMessageType: "WRONG_MESSAGE_TYPE",
  WrongRouteId: "WRONG_ROUTE_ID",
  WrongMintToken: "WRONG_MINT_TOKEN",
  WrongSourceChainId: "WRONG_SOURCE_CHAIN_ID",
  WrongSourceToken: "WRONG_SOURCE_TOKEN",
  BurnedAmountZero: "BURNED_AMOUNT_ZERO",
  WrongSourceChainWeightBps: "WRONG_SOURCE_CHAIN_WEIGHT_BPS",
  XxxlMintAmountMismatch: "XXXL_MINT_AMOUNT_MISMATCH",
} as const;

export type Stage1VerificationErrorCode =
  (typeof STAGE1_VERIFICATION_ERROR)[keyof typeof STAGE1_VERIFICATION_ERROR];

export type Stage1RouteVerificationConfig = {
  sourceToken: string;
  targetX1NetworkId: string | number | bigint;
  targetMintCoreId: string;
};

export type Stage1GatewayVerificationInput = {
  fields: Stage1GatewayMintMessageFields;
  x1RecipientBytes: Uint8Array;
  domainSeparator: Uint8Array;
  messageHash: Uint8Array;
  routeConfig: Stage1RouteVerificationConfig;
};

export type Stage1GatewayVerificationResult = {
  ok: boolean;
  errors: Stage1VerificationErrorCode[];
};

export type Stage1GatewayComputedValues = {
  canonicalEventKey: Uint8Array;
  x1RecipientHash: Uint8Array;
  domainSeparator: Uint8Array;
  encodedGatewayMintMessage: Uint8Array;
  messageHashPreimage: Uint8Array;
  messageHash: Uint8Array;
};

export function bytesEqual(left: Uint8Array, right: Uint8Array): boolean {
  if (left.length !== right.length) {
    return false;
  }

  let diff = 0;

  for (let index = 0; index < left.length; index += 1) {
    diff |= left[index]! ^ right[index]!;
  }

  return diff === 0;
}

export function isZeroBytes(bytes: Uint8Array): boolean {
  return bytes.every((byte) => byte === 0);
}

export function uint256BeToBigInt(bytes: Uint8Array): bigint {
  if (bytes.length !== 32) {
    throw new Error(`uint256 must be exactly 32 bytes, got ${bytes.length}`);
  }

  return BigInt(`0x${Buffer.from(bytes).toString("hex")}`);
}

export function stage1GatewayMessageType(): Uint8Array {
  return bytes32(
    keccakUtf8Label(STAGE1_GATEWAY_MESSAGE_TYPE_LABEL),
    "messageType",
  );
}

export function stage1GatewayRouteId(): Uint8Array {
  return bytes32(keccakUtf8Label(STAGE1_GATEWAY_ROUTE_ID_LABEL), "routeId");
}

export function stage1GatewayMintToken(): Uint8Array {
  return bytes32(
    keccakUtf8Label(STAGE1_GATEWAY_MINT_TOKEN_LABEL),
    "mintToken",
  );
}

export function stage1GatewayProtocolNameHash(): Uint8Array {
  return bytes32(
    keccakUtf8Label(STAGE1_GATEWAY_PROTOCOL_NAME_LABEL),
    "protocolNameHash",
  );
}

export function stage1GatewayVersionHash(): Uint8Array {
  return bytes32(
    keccakUtf8Label(STAGE1_GATEWAY_VERSION_LABEL),
    "gatewayVersionHash",
  );
}

export function stage1GatewayMessageTypeFamilyHash(): Uint8Array {
  return bytes32(
    keccakUtf8Label(STAGE1_GATEWAY_MESSAGE_TYPE_FAMILY_LABEL),
    "messageTypeFamilyHash",
  );
}

export function computeStage1CanonicalEventKey(
  fields: Stage1GatewayMintMessageFields,
): Uint8Array {
  return bytes32(
    keccakBytes(
      buildStage1CanonicalEventKeyPreimage({
        sourceChainId: fields.sourceChainId,
        sourceToken: fields.sourceToken,
        sourceBurnTxHash: fields.sourceBurnTxHash,
        sourceBurnEventIndex: fields.sourceBurnEventIndex,
      }),
    ),
    "canonicalEventKey",
  );
}

export function computeStage1X1RecipientHash(
  x1RecipientBytes: Uint8Array,
): Uint8Array {
  return bytes32(keccakBytes(x1RecipientBytes), "x1RecipientHash");
}

export function computeStage1DomainSeparator(
  routeConfig: Pick<
    Stage1RouteVerificationConfig,
    "targetX1NetworkId" | "targetMintCoreId"
  >,
): Uint8Array {
  return bytes32(
    keccakBytes(
      buildStage1DomainSeparatorPreimage({
        protocolNameHash: stage1GatewayProtocolNameHash(),
        gatewayVersionHash: stage1GatewayVersionHash(),
        targetX1NetworkId: uint256Be(routeConfig.targetX1NetworkId),
        targetMintCoreId: bytes32(
          routeConfig.targetMintCoreId,
          "targetMintCoreId",
        ),
        messageTypeFamilyHash: stage1GatewayMessageTypeFamilyHash(),
      }),
    ),
    "domainSeparator",
  );
}

export function computeStage1MessageHash(
  fields: Stage1GatewayMintMessageFields,
  domainSeparator: Uint8Array,
): Uint8Array {
  const encodedGatewayMintMessage = encodeStage1GatewayMintMessage(fields);

  return bytes32(
    keccakBytes(
      buildStage1MessageHashPreimage(domainSeparator, encodedGatewayMintMessage),
    ),
    "messageHash",
  );
}

export function computeStage1GatewayValues(
  input: Pick<
    Stage1GatewayVerificationInput,
    "fields" | "x1RecipientBytes" | "domainSeparator"
  >,
): Stage1GatewayComputedValues {
  const encodedGatewayMintMessage = encodeStage1GatewayMintMessage(input.fields);
  const messageHashPreimage = buildStage1MessageHashPreimage(
    input.domainSeparator,
    encodedGatewayMintMessage,
  );

  return {
    canonicalEventKey: computeStage1CanonicalEventKey(input.fields),
    x1RecipientHash: computeStage1X1RecipientHash(input.x1RecipientBytes),
    domainSeparator: input.domainSeparator,
    encodedGatewayMintMessage,
    messageHashPreimage,
    messageHash: bytes32(keccakBytes(messageHashPreimage), "messageHash"),
  };
}

export function verifyStage1GatewayMintMessage(
  input: Stage1GatewayVerificationInput,
): Stage1GatewayVerificationResult {
  const errors: Stage1VerificationErrorCode[] = [];
  const fields = input.fields;

  if (input.x1RecipientBytes.length !== 32) {
    errors.push(STAGE1_VERIFICATION_ERROR.InvalidX1RecipientLength);
  } else if (isZeroBytes(input.x1RecipientBytes)) {
    errors.push(STAGE1_VERIFICATION_ERROR.ZeroX1Recipient);
  }

  if (!bytesEqual(fields.messageType, stage1GatewayMessageType())) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongMessageType);
  }

  if (!bytesEqual(fields.routeId, stage1GatewayRouteId())) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongRouteId);
  }

  if (!bytesEqual(fields.mintToken, stage1GatewayMintToken())) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongMintToken);
  }

  if (
    uint256BeToBigInt(fields.sourceChainId) !== STAGE1_ETHEREUM_MAINNET_CHAIN_ID
  ) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongSourceChainId);
  }

  if (
    !bytesEqual(
      fields.sourceToken,
      addressToBytes32LeftPadded(input.routeConfig.sourceToken),
    )
  ) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongSourceToken);
  }

  const burnedAmount = uint256BeToBigInt(fields.burnedAmount);
  const xxxlMintAmount = uint256BeToBigInt(fields.xxxlMintAmount);

  if (burnedAmount === 0n) {
    errors.push(STAGE1_VERIFICATION_ERROR.BurnedAmountZero);
  }

  if (uint256BeToBigInt(fields.sourceChainWeightBps) !== STAGE1_SOURCE_CHAIN_WEIGHT_BPS) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongSourceChainWeightBps);
  }

  if (xxxlMintAmount !== burnedAmount) {
    errors.push(STAGE1_VERIFICATION_ERROR.XxxlMintAmountMismatch);
  }

  if (!bytesEqual(fields.canonicalEventKey, computeStage1CanonicalEventKey(fields))) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongCanonicalEventKey);
  }

  if (
    input.x1RecipientBytes.length === 32 &&
    !bytesEqual(fields.x1RecipientHash, computeStage1X1RecipientHash(input.x1RecipientBytes))
  ) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongX1RecipientHash);
  }

  if (
    !bytesEqual(
      input.domainSeparator,
      computeStage1DomainSeparator(input.routeConfig),
    )
  ) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongDomainSeparator);
  }

  if (!bytesEqual(input.messageHash, computeStage1MessageHash(fields, input.domainSeparator))) {
    errors.push(STAGE1_VERIFICATION_ERROR.WrongMessageHash);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

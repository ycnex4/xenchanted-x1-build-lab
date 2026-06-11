import { describe, expect, it } from "vitest";
import { concatHex, keccak256, padHex, stringToHex, toHex } from "viem";

type Hex = `0x${string}`;

type Stage2GatewayMessageContext = {
  messageType: string;
  routeId: string;
  sourceChainId: bigint;
  sourceToken: string;
  canonicalEventKey: Hex;
  x1Recipient: Hex;
  mintedAmount: bigint;
  guardianSetVersion: bigint;
  deadlineOrFinalityBlock: bigint;
  messageNonce: Hex;
};

function bytes32(byteHex: string): Hex {
  if (!/^[0-9a-fA-F]{2}$/.test(byteHex)) {
    throw new Error("byteHex must be exactly one byte");
  }

  return `0x${byteHex.repeat(32)}` as Hex;
}

function u64be(value: bigint): Hex {
  if (value < 0n || value > 0xffffffffffffffffn) {
    throw new Error("value does not fit into u64");
  }

  return padHex(toHex(value), { size: 8 });
}

function labelHash(label: string): Hex {
  return keccak256(stringToHex(label));
}

function deriveStage2GatewayMessageHash(
  context: Stage2GatewayMessageContext,
): Hex {
  return keccak256(
    concatHex([
      labelHash(context.messageType),
      labelHash(context.routeId),
      u64be(context.sourceChainId),
      labelHash(context.sourceToken),
      context.canonicalEventKey,
      context.x1Recipient,
      u64be(context.mintedAmount),
      u64be(context.guardianSetVersion),
      u64be(context.deadlineOrFinalityBlock),
      context.messageNonce,
    ]),
  );
}

function baseContext(): Stage2GatewayMessageContext {
  return {
    messageType: "XEC_STAGE2_DIRECT_MINT_APPROVAL_V1",
    routeId: "ETHEREUM_XNTD_TO_X1_XXXL",
    sourceChainId: 1n,
    sourceToken: "ETHEREUM_XNTD",
    canonicalEventKey: bytes32("11"),
    x1Recipient: bytes32("22"),
    mintedAmount: 1_000_000n,
    guardianSetVersion: 7n,
    deadlineOrFinalityBlock: 20_000_000n,
    messageNonce: bytes32("33"),
  };
}

describe("Stage 2 message hash binding", () => {
  it("derives the same hash for the same canonical context", () => {
    const first = deriveStage2GatewayMessageHash(baseContext());
    const second = deriveStage2GatewayMessageHash(baseContext());

    expect(first).toBe(second);
  });

  it("binds the hash to canonical_event_key", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      canonicalEventKey: bytes32("44"),
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to x1_recipient", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      x1Recipient: bytes32("55"),
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to minted_amount", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      mintedAmount: base.mintedAmount + 1n,
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to route_id", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      routeId: "ETHEREUM_XNTD_TO_X1_PROBE",
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to source_chain_id", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      sourceChainId: 11155111n,
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to source_token", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      sourceToken: "ETHEREUM_PROBE_TOKEN",
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to guardian_set_version", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      guardianSetVersion: base.guardianSetVersion + 1n,
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to deadline_or_finality_block", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      deadlineOrFinalityBlock: base.deadlineOrFinalityBlock + 1n,
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to message_nonce", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      messageNonce: bytes32("66"),
    });

    expect(changed).not.toBe(signedHash);
  });

  it("binds the hash to message_type", () => {
    const base = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(base);

    const changed = deriveStage2GatewayMessageHash({
      ...base,
      messageType: "XEC_STAGE2_OTHER_MESSAGE_TYPE",
    });

    expect(changed).not.toBe(signedHash);
  });

  it("prevents relayer reuse of a signature for a different event and recipient", () => {
    const legitimate = baseContext();
    const signedHash = deriveStage2GatewayMessageHash(legitimate);

    const relayerAttempt = deriveStage2GatewayMessageHash({
      ...legitimate,
      canonicalEventKey: bytes32("77"),
      x1Recipient: bytes32("88"),
    });

    expect(relayerAttempt).not.toBe(signedHash);
  });

  it("rejects numeric values outside u64 range in the reference encoder", () => {
    expect(() => u64be(-1n)).toThrow("value does not fit into u64");
    expect(() => u64be(0x10000000000000000n)).toThrow(
      "value does not fit into u64",
    );
  });
});

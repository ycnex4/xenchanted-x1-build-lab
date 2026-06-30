import { readFileSync } from "node:fs";
import * as ed25519 from "@noble/ed25519";
import { describe, expect, it } from "vitest";

import {
  XXXL_GATEWAY_AUTHORIZATION_ERROR,
  XXXL_GATEWAY_AUTHORIZATION_STATUS,
  XXXL_GUARDIAN_APPROVAL_ERROR,
  XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS,
  authorizeXxxlGatewayMintBoundary,
  bytesToHex,
  hashXxxlGuardianPayload,
  hexToBytes,
  type XXXLGatewayAuthorizationInput,
  type XXXLGatewayProcessedRegistrySnapshot,
  type XXXLGatewayRuntimeBindingExpectations,
  type XXXLGuardianApproval,
  type XXXLGuardianPayloadFields,
} from "../../src/index.js";

const TEST_GUARDIAN_SEEDS: readonly Uint8Array[] = [
  new Uint8Array(32).fill(0x11),
  new Uint8Array(32).fill(0x22),
  new Uint8Array(32).fill(0x33),
];

type TestGuardian = {
  readonly seed: Uint8Array;
  readonly publicKey: Uint8Array;
};

function cloneFields(
  overrides: Partial<XXXLGuardianPayloadFields> = {},
): XXXLGuardianPayloadFields {
  return {
    ...XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS,
    route_id: new Uint8Array(XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.route_id),
    source_token: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_token,
    ),
    source_sender: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_sender,
    ),
    source_burn_tx_hash: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_burn_tx_hash,
    ),
    source_block_hash: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.source_block_hash,
    ),
    canonical_event_key: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.canonical_event_key,
    ),
    x1_recipient: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.x1_recipient,
    ),
    target_mint: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.target_mint,
    ),
    guardian_set_id: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.guardian_set_id,
    ),
    message_nonce: new Uint8Array(
      XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS.message_nonce,
    ),
    ...overrides,
  };
}

async function testGuardian(index: number): Promise<TestGuardian> {
  const seed = TEST_GUARDIAN_SEEDS[index];

  if (seed === undefined) {
    throw new Error(`Missing test guardian seed ${index}`);
  }

  return {
    seed,
    publicKey: new Uint8Array(await ed25519.getPublicKeyAsync(seed)),
  };
}

function payloadHashBytes(fields: XXXLGuardianPayloadFields): Uint8Array {
  return hexToBytes(hashXxxlGuardianPayload(fields), 32, "payloadHash");
}

async function signPayloadHash(
  fields: XXXLGuardianPayloadFields,
  guardian: TestGuardian,
): Promise<Uint8Array> {
  return new Uint8Array(
    await ed25519.signAsync(payloadHashBytes(fields), guardian.seed),
  );
}

async function approvalForGuardian(
  index: number,
  fields: XXXLGuardianPayloadFields,
): Promise<XXXLGuardianApproval & { readonly guardian: TestGuardian }> {
  const guardian = await testGuardian(index);

  return {
    guardian,
    guardianPublicKey: guardian.publicKey,
    guardianSignature: await signPayloadHash(fields, guardian),
  };
}

function expectedBindings(
  fields: XXXLGuardianPayloadFields,
  overrides: Partial<XXXLGatewayRuntimeBindingExpectations> = {},
): XXXLGatewayRuntimeBindingExpectations {
  return {
    route_id: new Uint8Array(fields.route_id),
    source_chain_id: fields.source_chain_id,
    target_mint: new Uint8Array(fields.target_mint),
    guardian_set_id: new Uint8Array(fields.guardian_set_id),
    source_chain_weight_bps: fields.source_chain_weight_bps,
    ...overrides,
  };
}

function canonicalEventKeyHex(fields: XXXLGuardianPayloadFields): string {
  return bytesToHex(fields.canonical_event_key).toLowerCase();
}

async function validAuthorizationInput(
  fields = cloneFields(),
): Promise<XXXLGatewayAuthorizationInput & { readonly approval: XXXLGuardianApproval }> {
  const approval = await approvalForGuardian(0, fields);

  return {
    approval,
    fields,
    quorum: {
      guardianPublicKeys: [approval.guardianPublicKey],
      threshold: 1,
    },
    approvals: [approval],
    currentTimeOrSlot: BigInt(fields.expiration_slot_or_unix_ts) - 1n,
    processedRegistry: {
      processedCanonicalEventKeyHexes: [],
    },
    expectedBindings: expectedBindings(fields),
  };
}

function tamperSignature(signature: Uint8Array): Uint8Array {
  const tampered = new Uint8Array(signature);
  tampered[0] = tampered[0]! ^ 0x01;
  return tampered;
}

describe("XXXL gateway authorization boundary model", () => {
  it("accepts valid authorization when quorum passes, event is new, not expired, and bindings match", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary(input);

    expect(result.ok).toBe(true);
    expect(result.authorizationStatus).toBe(
      XXXL_GATEWAY_AUTHORIZATION_STATUS.Authorized,
    );
    expect(result.payloadHash).toBe(hashXxxlGuardianPayload(input.fields));
    expect(result.errors).toEqual([]);
    expect(result.quorum.ok).toBe(true);
    expect(result.replayCheck.ok).toBe(true);
    expect(result.expirationCheck.ok).toBe(true);
    expect(result.routeBindingCheck.ok).toBe(true);
  });

  it("rejects insufficient guardian quorum", async () => {
    const input = await validAuthorizationInput();
    const secondGuardian = await testGuardian(1);
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      quorum: {
        guardianPublicKeys: [
          input.approval.guardianPublicKey,
          secondGuardian.publicKey,
        ],
        threshold: 2,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.InvalidGuardianQuorum,
    ]);
    expect(result.quorum.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
  });

  it("rejects invalid signature through quorum result", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      approvals: [
        {
          guardianPublicKey: input.approval.guardianPublicKey,
          guardianSignature: tamperSignature(input.approval.guardianSignature),
        },
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.InvalidGuardianQuorum,
    ]);
    expect(result.quorum.approvals[0]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
    ]);
  });

  it("rejects already processed canonical_event_key", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      processedRegistry: {
        processedCanonicalEventKeyHexes: [canonicalEventKeyHex(input.fields)],
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.CanonicalEventAlreadyProcessed,
    ]);
    expect(result.replayCheck.alreadyProcessed).toBe(true);
  });

  it("rejects expired payload", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      currentTimeOrSlot: BigInt(input.fields.expiration_slot_or_unix_ts) + 1n,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.PayloadExpired,
    ]);
    expect(result.expirationCheck.expired).toBe(true);
  });

  it("accepts at exact expiration boundary", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      currentTimeOrSlot: input.fields.expiration_slot_or_unix_ts,
    });

    expect(result.ok).toBe(true);
    expect(result.expirationCheck.expired).toBe(false);
    expect(result.expirationCheck.currentTimeOrSlot).toBe(
      result.expirationCheck.expirationSlotOrUnixTs,
    );
  });

  it("rejects route_id mismatch", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      expectedBindings: expectedBindings(input.fields, {
        route_id: new Uint8Array(32).fill(0x12),
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.RouteIdMismatch,
    ]);
  });

  it("rejects source_chain_id mismatch", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      expectedBindings: expectedBindings(input.fields, {
        source_chain_id: 2n,
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainIdMismatch,
    ]);
  });

  it("rejects target_mint mismatch", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      expectedBindings: expectedBindings(input.fields, {
        target_mint: new Uint8Array(32).fill(0x34),
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.TargetMintMismatch,
    ]);
  });

  it("rejects guardian_set_id mismatch", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      expectedBindings: expectedBindings(input.fields, {
        guardian_set_id: new Uint8Array(32).fill(0x23),
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.GuardianSetIdMismatch,
    ]);
  });

  it("rejects source_chain_weight_bps mismatch", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      expectedBindings: expectedBindings(input.fields, {
        source_chain_weight_bps: 9_999,
      }),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainWeightMismatch,
    ]);
  });

  it("rejects malformed payload fail-closed", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      fields: cloneFields({ burned_amount: 0n }),
    });

    expect(result.ok).toBe(false);
    expect(result.payloadHash).toBeNull();
    expect(result.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.InvalidGuardianQuorum,
      XXXL_GATEWAY_AUTHORIZATION_ERROR.InvalidPayload,
    ]);
    expect(result.quorum.errors).toEqual(
      expect.arrayContaining([XXXL_GUARDIAN_APPROVAL_ERROR.InvalidPayload]),
    );
  });

  it("verifies payloadHash equals Phase 23 hashXxxlGuardianPayload(fields)", async () => {
    const input = await validAuthorizationInput();
    const result = await authorizeXxxlGatewayMintBoundary(input);

    expect(result.payloadHash).toBe(hashXxxlGuardianPayload(input.fields));
  });

  it("changing canonical_event_key changes authorization replay identity", async () => {
    const originalFields = cloneFields();
    const changedFields = cloneFields({
      canonical_event_key: new Uint8Array(32).fill(0x77),
    });
    const originalInput = await validAuthorizationInput(originalFields);
    const changedInput = await validAuthorizationInput(changedFields);
    const processedRegistry = {
      processedCanonicalEventKeyHexes: [canonicalEventKeyHex(originalFields)],
    };

    const originalResult = await authorizeXxxlGatewayMintBoundary({
      ...originalInput,
      processedRegistry,
    });
    const changedResult = await authorizeXxxlGatewayMintBoundary({
      ...changedInput,
      processedRegistry,
    });

    expect(originalResult.ok).toBe(false);
    expect(originalResult.errors).toEqual([
      XXXL_GATEWAY_AUTHORIZATION_ERROR.CanonicalEventAlreadyProcessed,
    ]);
    expect(changedResult.ok).toBe(true);
    expect(changedResult.replayCheck.canonicalEventKeyHex).not.toBe(
      originalResult.replayCheck.canonicalEventKeyHex,
    );
  });

  it("authorization does not mutate processed registry snapshot", async () => {
    const input = await validAuthorizationInput();
    const snapshot: XXXLGatewayProcessedRegistrySnapshot = {
      processedCanonicalEventKeyHexes: [],
    };
    const before = JSON.stringify(snapshot);

    await authorizeXxxlGatewayMintBoundary({
      ...input,
      processedRegistry: snapshot,
    });

    expect(JSON.stringify(snapshot)).toBe(before);
    expect(snapshot.processedCanonicalEventKeyHexes).toEqual([]);
  });

  it("does not accept caller-supplied payloadHash as authoritative", async () => {
    const input = await validAuthorizationInput();
    const callerPayloadHash = `0x${"00".repeat(32)}`;
    const result = await authorizeXxxlGatewayMintBoundary({
      ...input,
      payloadHash: callerPayloadHash,
    } as XXXLGatewayAuthorizationInput);

    expect(result.ok).toBe(true);
    expect(result.payloadHash).toBe(hashXxxlGuardianPayload(input.fields));
    expect(result.payloadHash).not.toBe(callerPayloadHash);
  });

  it("module and test source avoid prohibited execution concepts", () => {
    const blockedTerms = [
      ["live", "route"].join(" "),
      ["S", "PL", " C", "PI"].join(""),
      ["invoke", "signed"].join("_"),
      ["mint", "to"].join("_"),
      ["process", "instruction"].join("_"),
      ["programs", ["xxxl", "svm"].join("-")].join("/"),
    ];
    const sourcePaths = [
      ["src", "xxxl", "gateway-authorization-boundary.ts"].join("/"),
      ["tests", "xxxl", "gateway-authorization-boundary.test.ts"].join("/"),
    ];

    for (const sourcePath of sourcePaths) {
      const source = readFileSync(sourcePath, "utf8");

      for (const blockedTerm of blockedTerms) {
        expect(source).not.toContain(blockedTerm);
      }
    }
  });
});

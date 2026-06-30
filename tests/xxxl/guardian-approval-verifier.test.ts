import { readFileSync } from "node:fs";
import * as ed25519 from "@noble/ed25519";
import { describe, expect, it } from "vitest";

import {
  XXXL_GUARDIAN_APPROVAL_ERROR,
  XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS,
  buildXxxlGuardianPayloadHashPreimage,
  bytesToHex,
  hashXxxlGuardianPayload,
  hexToBytes,
  validateXxxlGuardianQuorumConfig,
  verifyXxxlGuardianPayloadApproval,
  verifyXxxlGuardianPayloadQuorum,
  type XXXLGuardianApproval,
  type XXXLGuardianPayloadFields,
} from "../../src/index.js";

const TEST_GUARDIAN_SEEDS: readonly Uint8Array[] = [
  new Uint8Array(32).fill(0x01),
  new Uint8Array(32).fill(0x02),
  new Uint8Array(32).fill(0x03),
  new Uint8Array(32).fill(0x04),
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
  return new Uint8Array(await ed25519.signAsync(payloadHashBytes(fields), guardian.seed));
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

function tamperSignature(signature: Uint8Array): Uint8Array {
  const tampered = new Uint8Array(signature);
  tampered[0] = tampered[0]! ^ 0x01;
  return tampered;
}

async function expectOldSignatureInvalidAfterFieldChange(
  mutate: (fields: XXXLGuardianPayloadFields) => XXXLGuardianPayloadFields,
): Promise<void> {
  const originalFields = cloneFields();
  const changedFields = mutate(cloneFields());
  const approval = await approvalForGuardian(0, originalFields);
  const result = await verifyXxxlGuardianPayloadQuorum({
    fields: changedFields,
    quorum: {
      guardianPublicKeys: [approval.guardianPublicKey],
      threshold: 1,
    },
    approvals: [approval],
  });

  expect(result.ok).toBe(false);
  expect(result.payloadHash).toBe(hashXxxlGuardianPayload(changedFields));
  expect(result.payloadHash).not.toBe(hashXxxlGuardianPayload(originalFields));
  expect(result.validApprovalCount).toBe(0);
  expect(result.errors).toEqual([
    XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
  ]);
  expect(result.approvals[0]?.errors).toEqual([
    XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
  ]);
}

describe("XXXL guardian payload approval verifier", () => {
  it("accepts valid 1-of-1 quorum", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [approval.guardianPublicKey],
        threshold: 1,
      },
      approvals: [approval],
    });

    expect(result.ok).toBe(true);
    expect(result.payloadHash).toBe(hashXxxlGuardianPayload(fields));
    expect(result.threshold).toBe(1);
    expect(result.validApprovalCount).toBe(1);
    expect(result.acceptedGuardianPublicKeyHexes).toEqual([
      bytesToHex(approval.guardianPublicKey),
    ]);
    expect(result.errors).toEqual([]);
    expect(result.approvals[0]).toMatchObject({
      ok: true,
      errors: [],
      payloadHash: hashXxxlGuardianPayload(fields),
    });
  });

  it("accepts valid 2-of-3 quorum", async () => {
    const fields = cloneFields();
    const approval0 = await approvalForGuardian(0, fields);
    const approval1 = await approvalForGuardian(1, fields);
    const guardian2 = await testGuardian(2);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [
          approval0.guardianPublicKey,
          approval1.guardianPublicKey,
          guardian2.publicKey,
        ],
        threshold: 2,
      },
      approvals: [approval1, approval0],
    });

    expect(result.ok).toBe(true);
    expect(result.threshold).toBe(2);
    expect(result.validApprovalCount).toBe(2);
    expect(result.acceptedGuardianPublicKeyHexes).toEqual([
      bytesToHex(approval1.guardianPublicKey),
      bytesToHex(approval0.guardianPublicKey),
    ]);
    expect(result.errors).toEqual([]);
  });

  it("rejects empty guardian set", async () => {
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields: cloneFields(),
      quorum: {
        guardianPublicKeys: [],
        threshold: 1,
      },
      approvals: [],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(0);
    expect(result.errors).toEqual(
      expect.arrayContaining([
        XXXL_GUARDIAN_APPROVAL_ERROR.EmptyGuardianSet,
        XXXL_GUARDIAN_APPROVAL_ERROR.InvalidThreshold,
        XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
      ]),
    );
  });

  it("rejects invalid threshold 0", async () => {
    const guardian = await testGuardian(0);

    expect(
      validateXxxlGuardianQuorumConfig({
        guardianPublicKeys: [guardian.publicKey],
        threshold: 0,
      }),
    ).toEqual([XXXL_GUARDIAN_APPROVAL_ERROR.InvalidThreshold]);
  });

  it("rejects threshold greater than guardian count", async () => {
    const guardian = await testGuardian(0);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields: cloneFields(),
      quorum: {
        guardianPublicKeys: [guardian.publicKey],
        threshold: 2,
      },
      approvals: [],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidThreshold,
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
  });

  it("rejects duplicate guardian public keys in config", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [
          approval.guardianPublicKey,
          new Uint8Array(approval.guardianPublicKey),
        ],
        threshold: 1,
      },
      approvals: [approval],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(1);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianPublicKey,
    ]);
  });

  it("rejects unknown guardian approval", async () => {
    const fields = cloneFields();
    const knownGuardian = await testGuardian(0);
    const unknownApproval = await approvalForGuardian(1, fields);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [knownGuardian.publicKey],
        threshold: 1,
      },
      approvals: [unknownApproval],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(0);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
    expect(result.approvals[0]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.UnknownGuardian,
    ]);
  });

  it("rejects duplicate guardian approval while allowing the first valid approval to count", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const guardian1 = await testGuardian(1);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [approval.guardianPublicKey, guardian1.publicKey],
        threshold: 2,
      },
      approvals: [approval, approval],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(1);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
    expect(result.approvals[0]?.ok).toBe(true);
    expect(result.approvals[1]?.ok).toBe(false);
    expect(result.approvals[1]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianApproval,
    ]);
  });

  it("rejects invalid signature", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [approval.guardianPublicKey],
        threshold: 1,
      },
      approvals: [
        {
          guardianPublicKey: approval.guardianPublicKey,
          guardianSignature: tamperSignature(approval.guardianSignature),
        },
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(0);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
    expect(result.approvals[0]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
    ]);
  });

  it("rejects signature made for a different payload", async () => {
    const fields = cloneFields();
    const differentFields = cloneFields({ source_chain_id: 2n });
    const guardian = await testGuardian(0);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [guardian.publicKey],
        threshold: 1,
      },
      approvals: [
        {
          guardianPublicKey: guardian.publicKey,
          guardianSignature: await signPayloadHash(differentFields, guardian),
        },
      ],
    });

    expect(hashXxxlGuardianPayload(differentFields)).not.toBe(
      hashXxxlGuardianPayload(fields),
    );
    expect(result.ok).toBe(false);
    expect(result.approvals[0]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
    ]);
  });

  it("rejects signature made over the full hash preimage instead of the payload hash", async () => {
    const fields = cloneFields();
    const guardian = await testGuardian(0);
    const signatureOverPreimage = new Uint8Array(
      await ed25519.signAsync(
        buildXxxlGuardianPayloadHashPreimage(fields),
        guardian.seed,
      ),
    );
    const result = await verifyXxxlGuardianPayloadApproval({
      fields,
      guardianPublicKey: guardian.publicKey,
      guardianSignature: signatureOverPreimage,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
    ]);
  });

  it("rejects malformed public key length", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [approval.guardianPublicKey],
        threshold: 1,
      },
      approvals: [
        {
          guardianPublicKey: new Uint8Array(31),
          guardianSignature: approval.guardianSignature,
        },
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(0);
    expect(result.approvals[0]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidGuardianPublicKeyLength,
    ]);
  });

  it("rejects malformed signature length", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [approval.guardianPublicKey],
        threshold: 1,
      },
      approvals: [
        {
          guardianPublicKey: approval.guardianPublicKey,
          guardianSignature: new Uint8Array(63),
        },
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(0);
    expect(result.approvals[0]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidGuardianSignatureLength,
    ]);
  });

  it("rejects quorum not reached", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const guardian1 = await testGuardian(1);
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [approval.guardianPublicKey, guardian1.publicKey],
        threshold: 2,
      },
      approvals: [approval],
    });

    expect(result.ok).toBe(false);
    expect(result.validApprovalCount).toBe(1);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
  });

  it("changing source_chain_id changes payload hash and invalidates old signature", async () => {
    await expectOldSignatureInvalidAfterFieldChange((fields) => ({
      ...fields,
      source_chain_id: 2n,
    }));
  });

  it("changing canonical_event_key changes payload hash and invalidates old signature", async () => {
    await expectOldSignatureInvalidAfterFieldChange((fields) => ({
      ...fields,
      canonical_event_key: new Uint8Array(32).fill(0x45),
    }));
  });

  it("changing message_nonce changes payload hash and invalidates old signature", async () => {
    await expectOldSignatureInvalidAfterFieldChange((fields) => ({
      ...fields,
      message_nonce: new Uint8Array(32).fill(0x67),
    }));
  });

  it("result payloadHash equals hashXxxlGuardianPayload(fields)", async () => {
    const fields = cloneFields();
    const approval = await approvalForGuardian(0, fields);
    const callerSuppliedPayloadHash = `0x${"00".repeat(32)}`;
    const result = await verifyXxxlGuardianPayloadApproval({
      fields,
      guardianPublicKey: approval.guardianPublicKey,
      guardianSignature: approval.guardianSignature,
      payloadHash: callerSuppliedPayloadHash,
    } as Parameters<typeof verifyXxxlGuardianPayloadApproval>[0]);

    expect(result.ok).toBe(true);
    expect(result.payloadHash).toBe(hashXxxlGuardianPayload(fields));
    expect(result.payloadHash).not.toBe(callerSuppliedPayloadHash);
  });

  it("captures invalid payload field errors without throwing", async () => {
    const fields = cloneFields({ burned_amount: 0n });
    const approval = await approvalForGuardian(0, cloneFields());
    const result = await verifyXxxlGuardianPayloadQuorum({
      fields,
      quorum: {
        guardianPublicKeys: [approval.guardianPublicKey],
        threshold: 1,
      },
      approvals: [approval],
    });

    expect(result.ok).toBe(false);
    expect(result.payloadHash).toBeNull();
    expect(result.validApprovalCount).toBe(0);
    expect(result.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidPayload,
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
    expect(result.approvals[0]?.errors).toEqual([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidPayload,
    ]);
  });

  it("does not import or reference live route, SVM, or SPL CPI concepts", () => {
    const source = readFileSync(
      "src/xxxl/guardian-approval-verifier.ts",
      "utf8",
    );

    expect(source).not.toMatch(/programs\/xxxl-svm/i);
    expect(source).not.toMatch(/\bsvm\b/i);
    expect(source).not.toMatch(/\bspl\b/i);
    expect(source).not.toMatch(/\bcpi\b/i);
    expect(source).not.toMatch(/invoke_signed/i);
    expect(source).not.toMatch(/mint_to/i);
    expect(source).not.toMatch(/live route/i);
  });
});

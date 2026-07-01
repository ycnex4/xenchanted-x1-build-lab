import {
  concatBytes,
  hexToBytes,
  keccakBytes,
  keccakUtf8Label,
  bytesToHex,
} from "../gateway/stage-1-encoding.js";
import {
  XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER,
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
  type XXXLGuardianPayloadVector,
} from "./guardian-payload-encoding.js";
import {
  XXXL_TS_SVM_PARITY_CASE_MATRIX,
  XXXL_TS_SVM_PARITY_EXPECTED_DECISION,
  XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS,
  XXXL_TS_SVM_PARITY_VALID_CANONICAL_VECTOR,
  type XXXLTsSvmParityCaseId,
  type XXXLTsSvmParityExpectedDecision,
} from "./ts-svm-parity-vector-suite.js";

export const XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_ID =
  "XXXL_TS_SVM_PARITY_INVALID_FIXTURES_PHASE_28";

export const XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_VERSION = 1;

export const XXXL_TS_SVM_PARITY_INVALID_FIXTURE_BOUNDARY =
  "CONCRETE_INVALID_FIXTURES_ONLY_NO_RUNTIME_EXECUTION";

export type XXXLTsSvmParityInvalidCaseId = Exclude<
  XXXLTsSvmParityCaseId,
  "valid-canonical-payload"
>;

export type XXXLTsSvmParityExpectedFailureLayer =
  | "PAYLOAD_CANONICALIZATION"
  | "PAYLOAD_VALIDATION"
  | "PAYLOAD_HASH_DOMAIN"
  | "ROUTE_BINDING"
  | "GUARDIAN_APPROVAL"
  | "GUARDIAN_QUORUM"
  | "EXPIRATION"
  | "REPLAY"
  | "SOURCE_PROOF"
  | "AMOUNT_CONTROL"
  | "TARGET_MINT_LEGITIMACY";

export type XXXLTsSvmParityValidControlFixture = {
  readonly caseId: "valid-canonical-payload";
  readonly fixtureId: string;
  readonly phase27VectorId: string;
  readonly expectedDecision: Extract<
    XXXLTsSvmParityExpectedDecision,
    "ACCEPT_CANONICAL_PAYLOAD_FOR_PARITY"
  >;
  readonly canonicalPayloadVector: XXXLGuardianPayloadVector;
  readonly baselinePayloadHash: string;
  readonly baselineEncodedPayloadHex: string;
};

export type XXXLTsSvmParityInvalidFixture = {
  readonly caseId: XXXLTsSvmParityInvalidCaseId;
  readonly fixtureId: string;
  readonly phase27VectorId: string;
  readonly expectedDecision: Extract<
    XXXLTsSvmParityExpectedDecision,
    "REJECT_BEFORE_EXECUTION"
  >;
  readonly expectedFailureLayer: XXXLTsSvmParityExpectedFailureLayer;
  readonly futureRuntimeCheck: string;
  readonly sourceBoundary: string;
  readonly baselinePayloadHash: string;
  readonly baselineEncodedPayloadHex: string;
  readonly tamperedInput: Record<string, unknown>;
};

export type XXXLTsSvmParityInvalidFixtureSuite = {
  readonly suiteId: typeof XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_ID;
  readonly suiteVersion: typeof XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_VERSION;
  readonly boundary: typeof XXXL_TS_SVM_PARITY_INVALID_FIXTURE_BOUNDARY;
  readonly phase27RequiredCaseIds: readonly XXXLTsSvmParityCaseId[];
  readonly validControlFixture: XXXLTsSvmParityValidControlFixture;
  readonly invalidFixtures: readonly XXXLTsSvmParityInvalidFixture[];
};

export type XXXLTsSvmParityInvalidFixtureSuiteValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly string[];
};

const BASELINE_VECTOR = XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR;
const BASELINE_FIELDS = BASELINE_VECTOR.fields;
const BASELINE_PAYLOAD_HASH = BASELINE_VECTOR.payloadHash;
const BASELINE_ENCODED_PAYLOAD_HEX = BASELINE_VECTOR.encodedPayloadHex;
const PHASE_28_TEST_GUARDIAN_PUBLIC_KEY_HEX =
  "0xd04ab232742bb4ab3a1368bd4615e4e6d0224ab71a016baf8520a332c9778737";
const PHASE_28_SECOND_TEST_GUARDIAN_PUBLIC_KEY_HEX =
  "0xa09aa5f47a6759802ff955f8dc2d2a14a5c99d23be97f864127ff9383455a4f0";
const PHASE_28_TEST_GUARDIAN_SIGNATURE_HEX =
  "0x833e28683b7d3f1c04a25c92aa770d8dad6987f626edc829938aa06abb81852c4135c2ffd81c3f80e84208b857e6e99e5f4c83da8fc22cce9ae0da3971c02e04";
const PHASE_28_TAMPERED_GUARDIAN_SIGNATURE_HEX =
  "0x823e28683b7d3f1c04a25c92aa770d8dad6987f626edc829938aa06abb81852c4135c2ffd81c3f80e84208b857e6e99e5f4c83da8fc22cce9ae0da3971c02e04";
const WRONG_HASH_DOMAIN_LABEL = "XXXL_GUARDIAN_PAYLOAD_HASH_V1_PHASE_28_WRONG";

function phase27Case(caseId: XXXLTsSvmParityCaseId) {
  const parityCase = XXXL_TS_SVM_PARITY_CASE_MATRIX.find(
    (candidate) => candidate.caseId === caseId,
  );

  if (parityCase === undefined) {
    throw new Error(`Missing Phase 27 parity case: ${caseId}`);
  }

  return parityCase;
}

function fixtureId(caseId: XXXLTsSvmParityInvalidCaseId): string {
  return `xxxl-phase-28-concrete-invalid-${caseId}`;
}

function byteHex(bytes: Uint8Array): string {
  return bytesToHex(bytes).toLowerCase();
}

function repeatedHex(byte: string, count: number): string {
  return `0x${byte.repeat(count)}`;
}

function replaceHexByte(hex: string, byteOffset: number, replacement: string): string {
  const start = 2 + byteOffset * 2;
  return `${hex.slice(0, start)}${replacement}${hex.slice(start + 2)}`;
}

function wrongDomainFixtureInput(): Record<string, unknown> {
  const wrongDomainSeparatorHex = keccakUtf8Label(WRONG_HASH_DOMAIN_LABEL);
  const wrongHashPreimage = concatBytes(
    hexToBytes(wrongDomainSeparatorHex, 32, "wrong domain separator"),
    hexToBytes(BASELINE_ENCODED_PAYLOAD_HEX, undefined, "encoded payload"),
  );

  return {
    expectedHashDomainLabel: BASELINE_VECTOR.hashDomainLabel,
    wrongHashDomainLabel: WRONG_HASH_DOMAIN_LABEL,
    expectedDomainSeparatorHex: BASELINE_VECTOR.hashDomainSeparatorHex,
    wrongDomainSeparatorHex,
    wrongHashPreimageHex: byteHex(wrongHashPreimage),
    wrongPayloadHash: keccakBytes(wrongHashPreimage),
  };
}

function invalidFixture(
  caseId: XXXLTsSvmParityInvalidCaseId,
  expectedFailureLayer: XXXLTsSvmParityExpectedFailureLayer,
  tamperedInput: Record<string, unknown>,
): XXXLTsSvmParityInvalidFixture {
  const parityCase = phase27Case(caseId);

  return {
    caseId,
    fixtureId: fixtureId(caseId),
    phase27VectorId: parityCase.vectorId,
    expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
    expectedFailureLayer,
    futureRuntimeCheck: parityCase.futureRuntimeCheck,
    sourceBoundary: parityCase.sourceBoundary,
    baselinePayloadHash: BASELINE_PAYLOAD_HASH,
    baselineEncodedPayloadHex: BASELINE_ENCODED_PAYLOAD_HEX,
    tamperedInput,
  };
}

export const XXXL_TS_SVM_PARITY_VALID_CONTROL_FIXTURE: XXXLTsSvmParityValidControlFixture =
  {
    caseId: "valid-canonical-payload",
    fixtureId: "xxxl-phase-28-valid-control-canonical-payload",
    phase27VectorId: phase27Case("valid-canonical-payload").vectorId,
    expectedDecision:
      XXXL_TS_SVM_PARITY_EXPECTED_DECISION.AcceptCanonicalPayload,
    canonicalPayloadVector: XXXL_TS_SVM_PARITY_VALID_CANONICAL_VECTOR,
    baselinePayloadHash: BASELINE_PAYLOAD_HASH,
    baselineEncodedPayloadHex: BASELINE_ENCODED_PAYLOAD_HEX,
  };

export const XXXL_TS_SVM_PARITY_INVALID_FIXTURES: readonly XXXLTsSvmParityInvalidFixture[] =
  [
    invalidFixture("wrong-field-order", "PAYLOAD_CANONICALIZATION", {
      expectedFieldOrder: [...XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER],
      actualFieldOrder: [
        ...XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER.slice(0, 4),
        "source_token",
        "source_chain_id",
        ...XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER.slice(6),
      ],
      swappedFields: ["source_chain_id", "source_token"],
    }),
    invalidFixture("wrong-byte-encoding", "PAYLOAD_CANONICALIZATION", {
      byteOffset: 0,
      originalByteHex: BASELINE_ENCODED_PAYLOAD_HEX.slice(2, 4),
      tamperedByteHex: "12",
      tamperedEncodedPayloadHex: replaceHexByte(
        BASELINE_ENCODED_PAYLOAD_HEX,
        0,
        "12",
      ),
    }),
    invalidFixture("wrong-hash-domain", "PAYLOAD_HASH_DOMAIN", wrongDomainFixtureInput()),
    invalidFixture("malformed-bytes32-field", "PAYLOAD_VALIDATION", {
      fieldName: "route_id",
      expectedLength: 32,
      actualLength: 31,
      valueHex: repeatedHex("11", 31),
    }),
    invalidFixture("malformed-var-bytes-field", "PAYLOAD_VALIDATION", {
      fieldName: "source_token",
      expectedLength: "1..65535",
      actualLength: 0,
      valueHex: "0x",
    }),
    invalidFixture("invalid-source-chain-id", "ROUTE_BINDING", {
      payloadSourceChainId: BASELINE_FIELDS.source_chain_id.toString(),
      expectedRuntimeSourceChainId: "2",
      mismatch: true,
    }),
    invalidFixture("wrong-target-mint", "ROUTE_BINDING", {
      payloadTargetMintHex: byteHex(BASELINE_FIELDS.target_mint),
      expectedRuntimeTargetMintHex: repeatedHex("34", 32),
    }),
    invalidFixture("wrong-guardian-set-id", "ROUTE_BINDING", {
      payloadGuardianSetIdHex: byteHex(BASELINE_FIELDS.guardian_set_id),
      expectedRuntimeGuardianSetIdHex: repeatedHex("23", 32),
    }),
    invalidFixture("wrong-source-chain-weight", "ROUTE_BINDING", {
      payloadSourceChainWeightBps: BASELINE_FIELDS.source_chain_weight_bps,
      expectedRuntimeSourceChainWeightBps: 9_999,
    }),
    invalidFixture("invalid-signature", "GUARDIAN_APPROVAL", {
      guardianPublicKeyHex: PHASE_28_TEST_GUARDIAN_PUBLIC_KEY_HEX,
      payloadHash: BASELINE_PAYLOAD_HASH,
      validSignatureHex: PHASE_28_TEST_GUARDIAN_SIGNATURE_HEX,
      tamperedSignatureHex: PHASE_28_TAMPERED_GUARDIAN_SIGNATURE_HEX,
      tamperedByteOffset: 0,
    }),
    invalidFixture("duplicate-guardian-approval", "GUARDIAN_QUORUM", {
      approvals: [
        {
          guardianPublicKeyHex: PHASE_28_TEST_GUARDIAN_PUBLIC_KEY_HEX,
          signatureHex: PHASE_28_TEST_GUARDIAN_SIGNATURE_HEX,
        },
        {
          guardianPublicKeyHex: PHASE_28_TEST_GUARDIAN_PUBLIC_KEY_HEX,
          signatureHex: PHASE_28_TEST_GUARDIAN_SIGNATURE_HEX,
        },
      ],
      duplicateGuardianPublicKeyHex: PHASE_28_TEST_GUARDIAN_PUBLIC_KEY_HEX,
    }),
    invalidFixture("insufficient-quorum", "GUARDIAN_QUORUM", {
      threshold: 2,
      guardianPublicKeyHexes: [
        PHASE_28_TEST_GUARDIAN_PUBLIC_KEY_HEX,
        PHASE_28_SECOND_TEST_GUARDIAN_PUBLIC_KEY_HEX,
      ],
      approvalGuardianPublicKeyHexes: [PHASE_28_TEST_GUARDIAN_PUBLIC_KEY_HEX],
      validApprovalCount: 1,
    }),
    invalidFixture("expired-payload", "EXPIRATION", {
      expirationSlot: BASELINE_FIELDS.expiration_slot_or_unix_ts.toString(),
      currentSlot: (
        BigInt(BASELINE_FIELDS.expiration_slot_or_unix_ts) + 1n
      ).toString(),
      boundaryRule: "currentSlot > expirationSlot",
    }),
    invalidFixture("duplicate-canonical-event-key", "REPLAY", {
      payloadCanonicalEventKeyHex: byteHex(BASELINE_FIELDS.canonical_event_key),
      processedCanonicalEventKeyHexes: [
        byteHex(BASELINE_FIELDS.canonical_event_key),
      ],
    }),
    invalidFixture("wrong-canonical-event-key-preimage", "SOURCE_PROOF", {
      payloadCanonicalEventKeyHex: byteHex(BASELINE_FIELDS.canonical_event_key),
      preimageDerivedCanonicalEventKeyHex: repeatedHex("77", 32),
      sourceChainId: BASELINE_FIELDS.source_chain_id.toString(),
      sourceTokenHex: byteHex(BASELINE_FIELDS.source_token),
      sourceBurnTxHashHex: byteHex(BASELINE_FIELDS.source_burn_tx_hash),
      sourceBurnEventIndex: BASELINE_FIELDS.source_burn_event_index.toString(),
    }),
    invalidFixture("wrong-source-burn-tx-hash", "SOURCE_PROOF", {
      payloadSourceBurnTxHashHex: byteHex(BASELINE_FIELDS.source_burn_tx_hash),
      proofSourceBurnTxHashHex: repeatedHex("ab", 32),
    }),
    invalidFixture("wrong-source-burn-event-index", "SOURCE_PROOF", {
      payloadSourceBurnEventIndex:
        BASELINE_FIELDS.source_burn_event_index.toString(),
      proofSourceBurnEventIndex: (
        BigInt(BASELINE_FIELDS.source_burn_event_index) + 1n
      ).toString(),
    }),
    invalidFixture("amount-over-route-cap", "AMOUNT_CONTROL", {
      payloadXxxlMintAmount: BASELINE_FIELDS.xxxl_mint_amount.toString(),
      routeMaxAmount: (
        BigInt(BASELINE_FIELDS.xxxl_mint_amount) - 1n
      ).toString(),
      comparison: "payloadXxxlMintAmount > routeMaxAmount",
    }),
    invalidFixture("invalid-target-mint", "TARGET_MINT_LEGITIMACY", {
      targetMintHex: byteHex(BASELINE_FIELDS.target_mint),
      routeRecognized: false,
      expectedMintAuthorityHex: repeatedHex("90", 32),
      actualMintAuthorityHex: repeatedHex("91", 32),
    }),
  ];

export const XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE: XXXLTsSvmParityInvalidFixtureSuite =
  {
    suiteId: XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_ID,
    suiteVersion: XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_VERSION,
    boundary: XXXL_TS_SVM_PARITY_INVALID_FIXTURE_BOUNDARY,
    phase27RequiredCaseIds: XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS,
    validControlFixture: XXXL_TS_SVM_PARITY_VALID_CONTROL_FIXTURE,
    invalidFixtures: XXXL_TS_SVM_PARITY_INVALID_FIXTURES,
  };

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

function hasConcreteTamperedInput(input: Record<string, unknown>): boolean {
  return Object.keys(input).length > 0;
}

export function validateXxxlTsSvmParityInvalidFixtureSuite(
  suite: XXXLTsSvmParityInvalidFixtureSuite = XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE,
): XXXLTsSvmParityInvalidFixtureSuiteValidationResult {
  const errors: string[] = [];
  const phase27CaseIds = [...XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS];
  const requiredInvalidCaseIds = phase27CaseIds.filter(
    (caseId): caseId is XXXLTsSvmParityInvalidCaseId =>
      caseId !== "valid-canonical-payload",
  );
  const fixtureCaseIds = suite.invalidFixtures.map(
    (fixture) => fixture.caseId as string,
  );
  const fixtureIds = suite.invalidFixtures.map((fixture) => fixture.fixtureId);
  const phase27VectorIds = new Map(
    XXXL_TS_SVM_PARITY_CASE_MATRIX.map((testCase) => [
      testCase.caseId,
      testCase.vectorId,
    ]),
  );

  if (suite.suiteId !== XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_ID) {
    errors.push("wrong suite id");
  }

  if (suite.suiteVersion !== XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_VERSION) {
    errors.push("wrong suite version");
  }

  if (suite.boundary !== XXXL_TS_SVM_PARITY_INVALID_FIXTURE_BOUNDARY) {
    errors.push("wrong boundary marker");
  }

  if (
    JSON.stringify(suite.phase27RequiredCaseIds) !==
    JSON.stringify(XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS)
  ) {
    errors.push("wrong Phase 27 case dependency");
  }

  if (
    suite.validControlFixture.canonicalPayloadVector !==
    XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR
  ) {
    errors.push("valid control fixture does not reuse Phase 23 vector");
  }

  if (
    suite.validControlFixture.baselinePayloadHash !==
    XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash
  ) {
    errors.push("valid control fixture payload hash mismatch");
  }

  if (
    suite.validControlFixture.baselineEncodedPayloadHex !==
    XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex
  ) {
    errors.push("valid control fixture encoded payload mismatch");
  }

  if (fixtureCaseIds.includes("valid-canonical-payload")) {
    errors.push("valid canonical payload must not be an invalid fixture");
  }

  if (hasDuplicates(fixtureCaseIds)) {
    errors.push("duplicate invalid case id");
  }

  if (hasDuplicates(fixtureIds)) {
    errors.push("duplicate fixture id");
  }

  for (const requiredCaseId of requiredInvalidCaseIds) {
    if (!fixtureCaseIds.includes(requiredCaseId)) {
      errors.push(`missing invalid fixture: ${requiredCaseId}`);
    }
  }

  for (const fixture of suite.invalidFixtures) {
    if (!requiredInvalidCaseIds.includes(fixture.caseId)) {
      errors.push(`unexpected invalid fixture: ${fixture.caseId}`);
    }

    if (
      fixture.expectedDecision !==
      XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution
    ) {
      errors.push(`invalid fixture does not reject: ${fixture.caseId}`);
    }

    if (fixture.phase27VectorId !== phase27VectorIds.get(fixture.caseId)) {
      errors.push(`wrong Phase 27 vector id: ${fixture.caseId}`);
    }

    if (fixture.baselinePayloadHash !== BASELINE_PAYLOAD_HASH) {
      errors.push(`baseline payload hash mismatch: ${fixture.caseId}`);
    }

    if (fixture.baselineEncodedPayloadHex !== BASELINE_ENCODED_PAYLOAD_HEX) {
      errors.push(`baseline encoded payload mismatch: ${fixture.caseId}`);
    }

    if (!hasConcreteTamperedInput(fixture.tamperedInput)) {
      errors.push(`missing concrete tampered input: ${fixture.caseId}`);
    }

    if (fixture.futureRuntimeCheck.trim().length === 0) {
      errors.push(`missing future runtime check: ${fixture.caseId}`);
    }

    if (fixture.sourceBoundary.trim().length === 0) {
      errors.push(`missing source boundary: ${fixture.caseId}`);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

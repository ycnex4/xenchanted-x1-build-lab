import {
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
  type XXXLGuardianPayloadVector,
} from "./guardian-payload-encoding.js";

export const XXXL_TS_SVM_PARITY_VECTOR_SUITE_ID =
  "XXXL_TS_SVM_PARITY_VECTOR_SUITE_PHASE_27";

export const XXXL_TS_SVM_PARITY_VECTOR_SUITE_VERSION = 1;

export const XXXL_TS_SVM_PARITY_ONLY_BOUNDARY =
  "PARITY_ONLY_PRE_RUNTIME_VECTOR_SUITE";

export const XXXL_TS_SVM_PARITY_EXPECTED_DECISION = {
  AcceptCanonicalPayload: "ACCEPT_CANONICAL_PAYLOAD_FOR_PARITY",
  RejectBeforeExecution: "REJECT_BEFORE_EXECUTION",
} as const;

export type XXXLTsSvmParityExpectedDecision =
  (typeof XXXL_TS_SVM_PARITY_EXPECTED_DECISION)[keyof typeof XXXL_TS_SVM_PARITY_EXPECTED_DECISION];

export const XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS = [
  "valid-canonical-payload",
  "wrong-field-order",
  "wrong-byte-encoding",
  "wrong-hash-domain",
  "malformed-bytes32-field",
  "malformed-var-bytes-field",
  "invalid-source-chain-id",
  "wrong-target-mint",
  "wrong-guardian-set-id",
  "wrong-source-chain-weight",
  "invalid-signature",
  "duplicate-guardian-approval",
  "insufficient-quorum",
  "expired-payload",
  "duplicate-canonical-event-key",
  "wrong-canonical-event-key-preimage",
  "wrong-source-burn-tx-hash",
  "wrong-source-burn-event-index",
  "amount-over-route-cap",
  "invalid-target-mint",
] as const;

export type XXXLTsSvmParityCaseId =
  (typeof XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS)[number];

export type XXXLTsSvmParityVectorCase = {
  readonly caseId: XXXLTsSvmParityCaseId;
  readonly vectorId: string;
  readonly expectedDecision: XXXLTsSvmParityExpectedDecision;
  readonly futureRuntimeCheck: string;
  readonly sourceBoundary: string;
  readonly canonicalPayloadVector?: XXXLGuardianPayloadVector;
};

export type XXXLTsSvmParityVectorSuite = {
  readonly suiteId: typeof XXXL_TS_SVM_PARITY_VECTOR_SUITE_ID;
  readonly suiteVersion: typeof XXXL_TS_SVM_PARITY_VECTOR_SUITE_VERSION;
  readonly parityOnlyBoundary: typeof XXXL_TS_SVM_PARITY_ONLY_BOUNDARY;
  readonly requiredCaseIds: readonly XXXLTsSvmParityCaseId[];
  readonly validCanonicalVector: XXXLGuardianPayloadVector;
  readonly cases: readonly XXXLTsSvmParityVectorCase[];
};

export type XXXLTsSvmParityVectorSuiteValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly string[];
};

export const XXXL_TS_SVM_PARITY_VALID_CANONICAL_VECTOR =
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR;

export const XXXL_TS_SVM_PARITY_CASE_MATRIX: readonly XXXLTsSvmParityVectorCase[] =
  [
    {
      caseId: "valid-canonical-payload",
      vectorId: "xxxl-ts-svm-parity-valid-canonical-payload",
      expectedDecision:
        XXXL_TS_SVM_PARITY_EXPECTED_DECISION.AcceptCanonicalPayload,
      futureRuntimeCheck:
        "Recompute Phase 23 canonical payload bytes and payload hash, then continue verifier checks only if every canonical field matches.",
      sourceBoundary: "Phase 22 field order and Phase 23 canonical vector",
      canonicalPayloadVector: XXXL_TS_SVM_PARITY_VALID_CANONICAL_VECTOR,
    },
    {
      caseId: "wrong-field-order",
      vectorId: "xxxl-ts-svm-parity-wrong-field-order",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject payloads whose decoded field order differs from the Phase 22 canonical order.",
      sourceBoundary: "Phase 22 payload structure",
    },
    {
      caseId: "wrong-byte-encoding",
      vectorId: "xxxl-ts-svm-parity-wrong-byte-encoding",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject bytes that do not exactly match Phase 23 canonical binary encoding v1.",
      sourceBoundary: "Phase 23 payload byte encoding",
    },
    {
      caseId: "wrong-hash-domain",
      vectorId: "xxxl-ts-svm-parity-wrong-hash-domain",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject payload hashes built from any domain separator other than the Phase 23 hash domain label.",
      sourceBoundary: "Phase 23 payload hash domain",
    },
    {
      caseId: "malformed-bytes32-field",
      vectorId: "xxxl-ts-svm-parity-malformed-bytes32-field",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject fixed-width fields that are not exactly 32 bytes.",
      sourceBoundary: "Phase 23 payload validation",
    },
    {
      caseId: "malformed-var-bytes-field",
      vectorId: "xxxl-ts-svm-parity-malformed-var-bytes-field",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject length-prefixed source fields that are empty or exceed the Phase 23 length bound.",
      sourceBoundary: "Phase 23 payload validation",
    },
    {
      caseId: "invalid-source-chain-id",
      vectorId: "xxxl-ts-svm-parity-invalid-source-chain-id",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject source chain ids outside the canonical u64 range or mismatched with route configuration.",
      sourceBoundary: "Phase 23 validation and Phase 25 bindings",
    },
    {
      caseId: "wrong-target-mint",
      vectorId: "xxxl-ts-svm-parity-wrong-target-mint",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject target mint values that do not match the expected route binding.",
      sourceBoundary: "Phase 25 binding expectations",
    },
    {
      caseId: "wrong-guardian-set-id",
      vectorId: "xxxl-ts-svm-parity-wrong-guardian-set-id",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject guardian set ids that do not match the active route guardian set binding.",
      sourceBoundary: "Phase 25 binding expectations",
    },
    {
      caseId: "wrong-source-chain-weight",
      vectorId: "xxxl-ts-svm-parity-wrong-source-chain-weight",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject source chain weight values that do not match the expected route binding.",
      sourceBoundary: "Phase 25 binding expectations",
    },
    {
      caseId: "invalid-signature",
      vectorId: "xxxl-ts-svm-parity-invalid-signature",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject guardian approvals whose Ed25519 signature does not verify over the Phase 23 payload hash.",
      sourceBoundary: "Phase 24 guardian approval verifier",
    },
    {
      caseId: "duplicate-guardian-approval",
      vectorId: "xxxl-ts-svm-parity-duplicate-guardian-approval",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject duplicate guardian approvals after counting only the first accepted guardian key.",
      sourceBoundary: "Phase 24 guardian quorum verifier",
    },
    {
      caseId: "insufficient-quorum",
      vectorId: "xxxl-ts-svm-parity-insufficient-quorum",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject proof bundles whose accepted valid guardian approvals do not reach threshold.",
      sourceBoundary: "Phase 24 guardian quorum verifier",
    },
    {
      caseId: "expired-payload",
      vectorId: "xxxl-ts-svm-parity-expired-payload",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject payloads when the runtime slot is greater than the signed expiration slot.",
      sourceBoundary: "Phase 25 expiration boundary and Phase 26 reviewed default",
    },
    {
      caseId: "duplicate-canonical-event-key",
      vectorId: "xxxl-ts-svm-parity-duplicate-canonical-event-key",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject canonical event keys already present in replay storage.",
      sourceBoundary: "Phase 25 replay check and Phase 26 reviewed default",
    },
    {
      caseId: "wrong-canonical-event-key-preimage",
      vectorId: "xxxl-ts-svm-parity-wrong-canonical-event-key-preimage",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject canonical event keys that do not match the reviewed source burn identity preimage.",
      sourceBoundary: "Phase 22 event identity and Phase 26 replay default",
    },
    {
      caseId: "wrong-source-burn-tx-hash",
      vectorId: "xxxl-ts-svm-parity-wrong-source-burn-tx-hash",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject source burn transaction hashes that do not match the source proof identity.",
      sourceBoundary: "Phase 22 source burn evidence",
    },
    {
      caseId: "wrong-source-burn-event-index",
      vectorId: "xxxl-ts-svm-parity-wrong-source-burn-event-index",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject source burn event indexes that do not match the source proof identity.",
      sourceBoundary: "Phase 22 source burn evidence",
    },
    {
      caseId: "amount-over-route-cap",
      vectorId: "xxxl-ts-svm-parity-amount-over-route-cap",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject payload amounts greater than the route max amount per message.",
      sourceBoundary: "Phase 26 reviewed amount-control default",
    },
    {
      caseId: "invalid-target-mint",
      vectorId: "xxxl-ts-svm-parity-invalid-target-mint",
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      futureRuntimeCheck:
        "Reject target mint accounts not recognized by runtime route configuration.",
      sourceBoundary: "Phase 26 target mint legitimacy default",
    },
  ];

export const XXXL_TS_SVM_PARITY_VECTOR_SUITE: XXXLTsSvmParityVectorSuite = {
  suiteId: XXXL_TS_SVM_PARITY_VECTOR_SUITE_ID,
  suiteVersion: XXXL_TS_SVM_PARITY_VECTOR_SUITE_VERSION,
  parityOnlyBoundary: XXXL_TS_SVM_PARITY_ONLY_BOUNDARY,
  requiredCaseIds: XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS,
  validCanonicalVector: XXXL_TS_SVM_PARITY_VALID_CANONICAL_VECTOR,
  cases: XXXL_TS_SVM_PARITY_CASE_MATRIX,
};

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

export function validateXxxlTsSvmParityVectorSuite(
  suite: XXXLTsSvmParityVectorSuite = XXXL_TS_SVM_PARITY_VECTOR_SUITE,
): XXXLTsSvmParityVectorSuiteValidationResult {
  const errors: string[] = [];
  const requiredCaseIds = [...XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS];
  const suiteCaseIds = suite.cases.map((testCase) => testCase.caseId);
  const vectorIds = suite.cases.map((testCase) => testCase.vectorId);
  const acceptedCases = suite.cases.filter(
    (testCase) =>
      testCase.expectedDecision ===
      XXXL_TS_SVM_PARITY_EXPECTED_DECISION.AcceptCanonicalPayload,
  );
  const invalidCases = suite.cases.filter(
    (testCase) => testCase.caseId !== "valid-canonical-payload",
  );

  if (suite.suiteId !== XXXL_TS_SVM_PARITY_VECTOR_SUITE_ID) {
    errors.push("wrong suite id");
  }

  if (suite.suiteVersion !== XXXL_TS_SVM_PARITY_VECTOR_SUITE_VERSION) {
    errors.push("wrong suite version");
  }

  if (suite.parityOnlyBoundary !== XXXL_TS_SVM_PARITY_ONLY_BOUNDARY) {
    errors.push("wrong parity-only boundary marker");
  }

  if (
    JSON.stringify(suite.requiredCaseIds) !==
    JSON.stringify(XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS)
  ) {
    errors.push("wrong required case ids");
  }

  if (suite.validCanonicalVector !== XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR) {
    errors.push("canonical Phase 23 vector is not reused by reference");
  }

  if (hasDuplicates(suiteCaseIds)) {
    errors.push("duplicate case id");
  }

  if (hasDuplicates(vectorIds)) {
    errors.push("duplicate vector id");
  }

  for (const requiredCaseId of requiredCaseIds) {
    if (!suiteCaseIds.includes(requiredCaseId)) {
      errors.push(`missing case id: ${requiredCaseId}`);
    }
  }

  for (const caseId of suiteCaseIds) {
    if (!requiredCaseIds.includes(caseId)) {
      errors.push(`unexpected case id: ${caseId}`);
    }
  }

  if (
    acceptedCases.length !== 1 ||
    acceptedCases[0]?.caseId !== "valid-canonical-payload"
  ) {
    errors.push("only the canonical payload may be accepted");
  }

  for (const invalidCase of invalidCases) {
    if (
      invalidCase.expectedDecision !==
      XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution
    ) {
      errors.push(`invalid case does not reject: ${invalidCase.caseId}`);
    }
  }

  for (const testCase of suite.cases) {
    if (testCase.futureRuntimeCheck.trim().length === 0) {
      errors.push(`missing future runtime check: ${testCase.caseId}`);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import {
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
  XXXL_TS_SVM_PARITY_CASE_MATRIX,
  XXXL_TS_SVM_PARITY_EXPECTED_DECISION,
  XXXL_TS_SVM_PARITY_ONLY_BOUNDARY,
  XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS,
  XXXL_TS_SVM_PARITY_VALID_CANONICAL_VECTOR,
  XXXL_TS_SVM_PARITY_VECTOR_SUITE,
  XXXL_TS_SVM_PARITY_VECTOR_SUITE_ID,
  XXXL_TS_SVM_PARITY_VECTOR_SUITE_VERSION,
  validateXxxlTsSvmParityVectorSuite,
} from "../../src/index.js";

const REQUIRED_PHASE_27_CASE_IDS = [
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

describe("XXXL TS/SVM parity vector suite", () => {
  it("exposes the expected suite id, version, and parity-only boundary", () => {
    expect(XXXL_TS_SVM_PARITY_VECTOR_SUITE_ID).toBe(
      "XXXL_TS_SVM_PARITY_VECTOR_SUITE_PHASE_27",
    );
    expect(XXXL_TS_SVM_PARITY_VECTOR_SUITE_VERSION).toBe(1);
    expect(XXXL_TS_SVM_PARITY_ONLY_BOUNDARY).toBe(
      "PARITY_ONLY_PRE_RUNTIME_VECTOR_SUITE",
    );
    expect(XXXL_TS_SVM_PARITY_VECTOR_SUITE).toMatchObject({
      suiteId: XXXL_TS_SVM_PARITY_VECTOR_SUITE_ID,
      suiteVersion: XXXL_TS_SVM_PARITY_VECTOR_SUITE_VERSION,
      parityOnlyBoundary: XXXL_TS_SVM_PARITY_ONLY_BOUNDARY,
    });
  });

  it("reuses the Phase 23 canonical vector as the valid TS/SVM vector", () => {
    const validCase = XXXL_TS_SVM_PARITY_CASE_MATRIX.find(
      (testCase) => testCase.caseId === "valid-canonical-payload",
    );

    expect(XXXL_TS_SVM_PARITY_VALID_CANONICAL_VECTOR).toBe(
      XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
    );
    expect(XXXL_TS_SVM_PARITY_VECTOR_SUITE.validCanonicalVector).toBe(
      XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
    );
    expect(validCase?.canonicalPayloadVector).toBe(
      XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
    );
    expect(validCase?.canonicalPayloadVector?.encodedPayloadHex).toBe(
      XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex,
    );
    expect(validCase?.canonicalPayloadVector?.payloadHash).toBe(
      XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash,
    );
  });

  it("covers required Phase 27 case ids exactly once", () => {
    expect(XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS).toEqual(
      REQUIRED_PHASE_27_CASE_IDS,
    );
    expect(
      XXXL_TS_SVM_PARITY_VECTOR_SUITE.cases.map((testCase) => testCase.caseId),
    ).toEqual(REQUIRED_PHASE_27_CASE_IDS);

    for (const caseId of REQUIRED_PHASE_27_CASE_IDS) {
      expect(
        XXXL_TS_SVM_PARITY_VECTOR_SUITE.cases.filter(
          (testCase) => testCase.caseId === caseId,
        ),
      ).toHaveLength(1);
    }
  });

  it("keeps vector ids unique", () => {
    const vectorIds = XXXL_TS_SVM_PARITY_VECTOR_SUITE.cases.map(
      (testCase) => testCase.vectorId,
    );

    expect(new Set(vectorIds).size).toBe(vectorIds.length);
  });

  it("accepts only the canonical payload case", () => {
    const acceptedCases = XXXL_TS_SVM_PARITY_VECTOR_SUITE.cases.filter(
      (testCase) =>
        testCase.expectedDecision ===
        XXXL_TS_SVM_PARITY_EXPECTED_DECISION.AcceptCanonicalPayload,
    );

    expect(acceptedCases.map((testCase) => testCase.caseId)).toEqual([
      "valid-canonical-payload",
    ]);
  });

  it("marks all invalid cases as reject-before-execution", () => {
    const invalidCases = XXXL_TS_SVM_PARITY_VECTOR_SUITE.cases.filter(
      (testCase) => testCase.caseId !== "valid-canonical-payload",
    );

    expect(invalidCases).toHaveLength(REQUIRED_PHASE_27_CASE_IDS.length - 1);
    for (const testCase of invalidCases) {
      expect(testCase.expectedDecision).toBe(
        XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      );
    }
  });

  it("gives every case explicit future runtime check text", () => {
    for (const testCase of XXXL_TS_SVM_PARITY_VECTOR_SUITE.cases) {
      expect(testCase.futureRuntimeCheck.trim().length).toBeGreaterThan(32);
      expect(testCase.sourceBoundary.trim().length).toBeGreaterThan(8);
    }
  });

  it("validates the suite", () => {
    expect(validateXxxlTsSvmParityVectorSuite()).toEqual({
      ok: true,
      errors: [],
    });
  });

  it("does not introduce execution-enabling concepts in the source module", () => {
    const blockedTerms = [
      ["live", "route"].join(" "),
      ["S", "PL", " C", "PI"].join(""),
      ["invoke", "signed"].join("_"),
      ["mint", "to"].join("_"),
      ["process", "instruction"].join("_"),
    ];
    const source = readFileSync(
      ["src", "xxxl", "ts-svm-parity-vector-suite.ts"].join("/"),
      "utf8",
    );

    for (const blockedTerm of blockedTerms) {
      expect(source).not.toContain(blockedTerm);
    }
  });
});

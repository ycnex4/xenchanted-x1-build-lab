import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import {
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
  XXXL_TS_SVM_PARITY_CASE_MATRIX,
  XXXL_TS_SVM_PARITY_EXPECTED_DECISION,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURE_BOUNDARY,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_ID,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_VERSION,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURES,
  XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS,
  XXXL_TS_SVM_PARITY_VALID_CONTROL_FIXTURE,
  validateXxxlTsSvmParityInvalidFixtureSuite,
} from "../../src/index.js";

const REQUIRED_INVALID_CASE_IDS = [
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

describe("XXXL TS/SVM concrete invalid parity fixtures", () => {
  it("exposes the expected suite id, version, and boundary marker", () => {
    expect(XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_ID).toBe(
      "XXXL_TS_SVM_PARITY_INVALID_FIXTURES_PHASE_28",
    );
    expect(XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_VERSION).toBe(1);
    expect(XXXL_TS_SVM_PARITY_INVALID_FIXTURE_BOUNDARY).toBe(
      "CONCRETE_INVALID_FIXTURES_ONLY_NO_RUNTIME_EXECUTION",
    );
    expect(XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE).toMatchObject({
      suiteId: XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_ID,
      suiteVersion: XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE_VERSION,
      boundary: XXXL_TS_SVM_PARITY_INVALID_FIXTURE_BOUNDARY,
    });
  });

  it("reuses the Phase 23 canonical vector for the valid control fixture", () => {
    expect(XXXL_TS_SVM_PARITY_VALID_CONTROL_FIXTURE.caseId).toBe(
      "valid-canonical-payload",
    );
    expect(
      XXXL_TS_SVM_PARITY_VALID_CONTROL_FIXTURE.canonicalPayloadVector,
    ).toBe(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR);
    expect(XXXL_TS_SVM_PARITY_VALID_CONTROL_FIXTURE.baselinePayloadHash).toBe(
      XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash,
    );
    expect(
      XXXL_TS_SVM_PARITY_VALID_CONTROL_FIXTURE.baselineEncodedPayloadHex,
    ).toBe(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex);
  });

  it("covers exactly the 19 invalid Phase 27 cases", () => {
    const invalidCaseIds = XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map(
      (fixture) => fixture.caseId,
    );

    expect(invalidCaseIds).toEqual(REQUIRED_INVALID_CASE_IDS);
    expect(invalidCaseIds).toHaveLength(19);
    expect(invalidCaseIds).not.toContain("valid-canonical-payload");
    expect(
      XXXL_TS_SVM_PARITY_REQUIRED_CASE_IDS.filter(
        (caseId) => caseId !== "valid-canonical-payload",
      ),
    ).toEqual(REQUIRED_INVALID_CASE_IDS);
  });

  it("marks every invalid fixture as reject-before-execution", () => {
    for (const fixture of XXXL_TS_SVM_PARITY_INVALID_FIXTURES) {
      expect(fixture.expectedDecision).toBe(
        XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      );
    }
  });

  it("keeps fixture ids unique", () => {
    const fixtureIds = XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map(
      (fixture) => fixture.fixtureId,
    );

    expect(new Set(fixtureIds).size).toBe(fixtureIds.length);
  });

  it("links every fixture to an existing Phase 27 case id and vector id", () => {
    const phase27Cases = new Map(
      XXXL_TS_SVM_PARITY_CASE_MATRIX.map((testCase) => [
        testCase.caseId,
        testCase.vectorId,
      ]),
    );

    for (const fixture of XXXL_TS_SVM_PARITY_INVALID_FIXTURES) {
      expect(phase27Cases.has(fixture.caseId)).toBe(true);
      expect(fixture.phase27VectorId).toBe(phase27Cases.get(fixture.caseId));
    }
  });

  it("gives every fixture concrete tampered input", () => {
    for (const fixture of XXXL_TS_SVM_PARITY_INVALID_FIXTURES) {
      expect(Object.keys(fixture.tamperedInput).length).toBeGreaterThan(0);
      expect(JSON.stringify(fixture.tamperedInput)).not.toBe("{}");
    }
  });

  it("keeps baseline encoded payload and hash tied to the Phase 23 valid vector", () => {
    for (const fixture of XXXL_TS_SVM_PARITY_INVALID_FIXTURES) {
      expect(fixture.baselineEncodedPayloadHex).toBe(
        XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.encodedPayloadHex,
      );
      expect(fixture.baselinePayloadHash).toBe(
        XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.payloadHash,
      );
    }
  });

  it("validates the suite", () => {
    expect(validateXxxlTsSvmParityInvalidFixtureSuite()).toEqual({
      ok: true,
      errors: [],
    });
  });

  it("does not introduce execution-enabling concepts in source files", () => {
    const blockedTerms = [
      ["live", "route"].join(" "),
      ["S", "PL", " C", "PI"].join(""),
      ["invoke", "signed"].join("_"),
      ["mint", "to"].join("_"),
      ["process", "instruction"].join("_"),
    ];
    const sourcePaths = [
      ["src", "xxxl", "ts-svm-parity-invalid-fixtures.ts"].join("/"),
      ["tests", "xxxl", "ts-svm-parity-invalid-fixtures.test.ts"].join("/"),
    ];

    for (const sourcePath of sourcePaths) {
      const source = readFileSync(sourcePath, "utf8");

      for (const blockedTerm of blockedTerms) {
        expect(source).not.toContain(blockedTerm);
      }
    }
  });
});

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

import {
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
  XXXL_TS_SVM_PARITY_CASE_MATRIX,
  XXXL_TS_SVM_PARITY_EXPECTED_DECISION,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURES,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_BOUNDARY,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_ID,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_VERSION,
  validateXxxlTsSvmParityVerifierValidationSuite,
} from "../../src/index.js";

const TS_MODEL_VALIDATED_CASE_IDS = [
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
] as const;

const FUTURE_RUNTIME_REQUIRED_CASE_IDS = [
  "wrong-field-order",
  "wrong-byte-encoding",
  "wrong-canonical-event-key-preimage",
  "wrong-source-burn-tx-hash",
  "wrong-source-burn-event-index",
  "amount-over-route-cap",
  "invalid-target-mint",
] as const;

describe("XXXL TS/SVM verifier-oriented parity validation", () => {
  it("exposes the expected suite id, version, and boundary marker", () => {
    expect(XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_ID).toBe(
      "XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_PHASE_29",
    );
    expect(XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_VERSION).toBe(1);
    expect(XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_BOUNDARY).toBe(
      "VERIFIER_ORIENTED_VALIDATION_ONLY_NO_RUNTIME_EXECUTION",
    );
    expect(XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE).toMatchObject({
      suiteId: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_ID,
      suiteVersion: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_VERSION,
      boundary: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_BOUNDARY,
    });
  });

  it("keeps Phase 23 canonical payload as a control vector only", () => {
    expect(
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE.phase23CanonicalPayloadVector,
    ).toBe(XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR);
    expect(
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX.map(
        (entry) => entry.caseId,
      ),
    ).not.toContain("valid-canonical-payload");
  });

  it("covers exactly the Phase 28 invalid fixtures", () => {
    const phase28CaseIds = XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map(
      (fixture) => fixture.caseId,
    );
    const matrixCaseIds = XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX.map(
      (entry) => entry.caseId,
    );

    expect(matrixCaseIds).toEqual(phase28CaseIds);
    expect(matrixCaseIds).toHaveLength(19);
    expect(new Set(matrixCaseIds).size).toBe(matrixCaseIds.length);
    expect(matrixCaseIds).not.toContain("valid-canonical-payload");
  });

  it("links every validation entry to Phase 28 and Phase 27", () => {
    const fixtureByCaseId = new Map(
      XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map((fixture) => [
        fixture.caseId,
        fixture,
      ]),
    );
    const vectorIdByCaseId = new Map(
      XXXL_TS_SVM_PARITY_CASE_MATRIX.map((testCase) => [
        testCase.caseId,
        testCase.vectorId,
      ]),
    );

    expect(
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE.phase28FixtureSuiteId,
    ).toBe(XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE.suiteId);

    for (const entry of XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX) {
      const fixture = fixtureByCaseId.get(entry.caseId);

      expect(fixture).toBeDefined();
      expect(entry.fixtureId).toBe(fixture?.fixtureId);
      expect(entry.phase28ExpectedFailureLayer).toBe(
        fixture?.expectedFailureLayer,
      );
      expect(entry.phase27VectorId).toBe(vectorIdByCaseId.get(entry.caseId));
      expect(entry.phase27VectorId).toBe(fixture?.phase27VectorId);
    }
  });

  it("preserves reject-before-execution for every invalid fixture", () => {
    for (const entry of XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX) {
      expect(entry.expectedDecision).toBe(
        XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      );
    }
  });

  it("partitions cases between current TS model checks and future runtime requirements", () => {
    const tsModelCaseIds = XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX.filter(
      (entry) =>
        entry.validationStatus ===
        XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.TsModelValidatedRejection,
    ).map((entry) => entry.caseId);
    const futureRuntimeCaseIds =
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX.filter(
        (entry) =>
          entry.validationStatus ===
          XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.FutureRuntimeValidationRequired,
      ).map((entry) => entry.caseId);

    expect(tsModelCaseIds).toEqual(TS_MODEL_VALIDATED_CASE_IDS);
    expect(futureRuntimeCaseIds).toEqual(FUTURE_RUNTIME_REQUIRED_CASE_IDS);
    expect(tsModelCaseIds.length).toBeGreaterThan(0);
    expect(futureRuntimeCaseIds.length).toBeGreaterThan(0);
  });

  it("keeps status flags internally consistent", () => {
    for (const entry of XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX) {
      if (
        entry.validationStatus ===
        XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.TsModelValidatedRejection
      ) {
        expect(entry.usesExistingTsModel).toBe(true);
        expect(entry.runtimeImplementationRequired).toBe(false);
        expect(entry.existingTsModelReference).not.toBeNull();
        expect(entry.existingTsModelChecks.length).toBeGreaterThan(0);
      } else {
        expect(entry.usesExistingTsModel).toBe(false);
        expect(entry.runtimeImplementationRequired).toBe(true);
        expect(entry.existingTsModelReference).toBeNull();
        expect(entry.existingTsModelChecks).toEqual([]);
      }

      expect(entry.forbiddenExecutionTouched).toBe(false);
      expect(entry.liveRouteTouched).toBe(false);
      expect(entry.splCpiTouched).toBe(false);
      expect(entry.invokeSignedTouched).toBe(false);
      expect(entry.mintToTouched).toBe(false);
      expect(entry.runtimeStateMutationTouched).toBe(false);
      expect(entry.processedEventMarkingTouched).toBe(false);
      expect(entry.productionProgramIdTouched).toBe(false);
      expect(entry.blockerRemovalTouched).toBe(false);
    }
  });

  it("gives every entry explicit verifier reasoning", () => {
    for (const entry of XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX) {
      expect(entry.reason.trim().length).toBeGreaterThan(32);
      expect(entry.futureRuntimeCheck.trim().length).toBeGreaterThan(32);
      expect(entry.sourceBoundary.trim().length).toBeGreaterThan(8);
    }
  });

  it("validates the suite", () => {
    expect(validateXxxlTsSvmParityVerifierValidationSuite()).toEqual({
      ok: true,
      errors: [],
    });
  });

  it("does not introduce exact execution-enabling terms in source files", () => {
    const blockedTerms = [
      ["live", "route"].join(" "),
      ["S", "PL", " C", "PI"].join(""),
      ["invoke", "signed"].join("_"),
      ["mint", "to"].join("_"),
      ["process", "instruction"].join("_"),
    ];
    const sourcePaths = [
      ["src", "xxxl", "ts-svm-parity-verifier-validation.ts"].join("/"),
      ["tests", "xxxl", "ts-svm-parity-verifier-validation.test.ts"].join("/"),
    ];

    for (const sourcePath of sourcePaths) {
      const source = readFileSync(sourcePath, "utf8");

      for (const blockedTerm of blockedTerms) {
        expect(source).not.toContain(blockedTerm);
      }
    }
  });
});

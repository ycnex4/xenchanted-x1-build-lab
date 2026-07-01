import { readFileSync } from "node:fs";
import { beforeAll, describe, expect, it } from "vitest";

import {
  XXXL_GUARDIAN_APPROVAL_ERROR,
  XXXL_GATEWAY_AUTHORIZATION_ERROR,
  XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS,
  XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_BOUNDARY,
  XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_ID,
  XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_VERSION,
  XXXL_TS_SVM_PARITY_EXPECTED_DECISION,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURES,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS,
  buildXxxlTsSvmParityExecutionBackedValidationSuite,
  validateXxxlTsSvmParityExecutionBackedValidationSuite,
  type XXXLTsSvmParityExecutionBackedValidationSuite,
} from "../../src/index.js";

const FUTURE_RUNTIME_REQUIRED_CASE_IDS = [
  "wrong-field-order",
  "wrong-byte-encoding",
  "wrong-canonical-event-key-preimage",
  "wrong-source-burn-tx-hash",
  "wrong-source-burn-event-index",
  "amount-over-route-cap",
  "invalid-target-mint",
] as const;

describe("XXXL TS/SVM execution-backed parity validation", () => {
  let suite: XXXLTsSvmParityExecutionBackedValidationSuite;

  beforeAll(async () => {
    suite = await buildXxxlTsSvmParityExecutionBackedValidationSuite();
  });

  it("exposes the expected suite id, version, and boundary marker", () => {
    expect(XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_ID).toBe(
      "XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_PHASE_30",
    );
    expect(XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_VERSION).toBe(1);
    expect(XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_BOUNDARY).toBe(
      "EXECUTION_BACKED_TS_PARITY_ONLY_NO_RUNTIME_EXECUTION",
    );
    expect(suite).toMatchObject({
      suiteId: XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_ID,
      suiteVersion:
        XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_VERSION,
      boundary: XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_BOUNDARY,
    });
  });

  it("covers exactly all Phase 28 invalid fixtures", () => {
    const phase28CaseIds = XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map(
      (fixture) => fixture.caseId,
    );
    const suiteCaseIds = suite.entries.map((entry) => entry.caseId);

    expect(suiteCaseIds).toEqual(phase28CaseIds);
    expect(suiteCaseIds).toHaveLength(19);
    expect(new Set(suiteCaseIds).size).toBe(suiteCaseIds.length);
    expect(suiteCaseIds).not.toContain("valid-canonical-payload");
  });

  it("links every entry to Phase 28 fixtures and Phase 29 entries", () => {
    const fixtureByCaseId = new Map(
      XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map((fixture) => [
        fixture.caseId,
        fixture,
      ]),
    );
    const phase29ByCaseId = new Map(
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX.map((entry) => [
        entry.caseId,
        entry,
      ]),
    );

    for (const entry of suite.entries) {
      const fixture = fixtureByCaseId.get(entry.caseId);
      const phase29 = phase29ByCaseId.get(entry.caseId);

      expect(fixture).toBeDefined();
      expect(phase29).toBeDefined();
      expect(entry.fixtureId).toBe(fixture?.fixtureId);
      expect(entry.phase27VectorId).toBe(fixture?.phase27VectorId);
      expect(entry.phase29ValidationStatus).toBe(phase29?.validationStatus);
      expect(entry.existingTsModelReference).toBe(
        phase29?.existingTsModelReference,
      );
    }
  });

  it("preserves reject-before-execution for every entry", () => {
    for (const entry of suite.entries) {
      expect(entry.expectedDecision).toBe(
        XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      );
    }
  });

  it("keeps all Phase 29 future-runtime-required cases future-required", () => {
    const futureCaseIds = suite.entries
      .filter(
        (entry) =>
          entry.executionStatus ===
          XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.FutureRuntimeValidationRequired,
      )
      .map((entry) => entry.caseId);

    expect(futureCaseIds).toEqual(FUTURE_RUNTIME_REQUIRED_CASE_IDS);

    for (const entry of suite.entries.filter((candidate) =>
      FUTURE_RUNTIME_REQUIRED_CASE_IDS.includes(
        candidate.caseId as (typeof FUTURE_RUNTIME_REQUIRED_CASE_IDS)[number],
      ),
    )) {
      expect(entry.phase29ValidationStatus).toBe(
        XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.FutureRuntimeValidationRequired,
      );
      expect(entry.expectedError).toBeNull();
      expect(entry.actualError).toBeNull();
      expect(entry.matchesExpected).toBeNull();
      expect(entry.runtimeImplementationRequired).toBe(true);
    }
  });

  it("execution-backed entries have concrete expected and actual errors", () => {
    const executionBackedEntries = suite.entries.filter(
      (entry) =>
        entry.executionStatus ===
        XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.TsExecutionBackedRejection,
    );

    expect(executionBackedEntries).toHaveLength(12);

    for (const entry of executionBackedEntries) {
      expect(entry.phase29ValidationStatus).toBe(
        XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.TsModelValidatedRejection,
      );
      expect(entry.expectedError).not.toBeNull();
      expect(entry.actualError).toBe(entry.expectedError);
      expect(entry.actualErrors).toContain(entry.expectedError);
      expect(entry.matchesExpected).toBe(true);
      expect(entry.runtimeImplementationRequired).toBe(false);
    }
  });

  it("backs every Phase 29 TS-model case with execution", () => {
    const phase29TsModelCaseIds =
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX.filter(
        (entry) =>
          entry.validationStatus ===
          XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.TsModelValidatedRejection,
      ).map((entry) => entry.caseId);
    const phase30BackedCaseIds = suite.entries
      .filter(
        (entry) =>
          entry.executionStatus ===
          XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.TsExecutionBackedRejection,
      )
      .map((entry) => entry.caseId);
    const unavailableCaseIds = suite.entries
      .filter(
        (entry) =>
          entry.executionStatus ===
          XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.TsExecutionPathUnavailable,
      )
      .map((entry) => entry.caseId);

    expect(phase30BackedCaseIds).toEqual(phase29TsModelCaseIds);
    expect(unavailableCaseIds).toEqual([]);
  });

  it("includes guardian verifier and authorization boundary execution-backed cases", () => {
    const guardianErrors = new Set<string>([
      XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
      XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianApproval,
      XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    ]);
    const authorizationErrors = new Set<string>(
      Object.values(XXXL_GATEWAY_AUTHORIZATION_ERROR),
    );
    const executionBackedEntries = suite.entries.filter(
      (entry) =>
        entry.executionStatus ===
        XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.TsExecutionBackedRejection,
    );

    expect(
      executionBackedEntries.some(
        (entry) =>
          entry.expectedError !== null && guardianErrors.has(entry.expectedError),
      ),
    ).toBe(true);
    expect(
      executionBackedEntries.some(
        (entry) =>
          entry.expectedError !== null &&
          authorizationErrors.has(entry.expectedError),
      ),
    ).toBe(true);
  });

  it("keeps all forbidden boundary flags false", () => {
    for (const entry of suite.entries) {
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

  it("validates the execution-backed suite", async () => {
    await expect(
      validateXxxlTsSvmParityExecutionBackedValidationSuite(suite),
    ).resolves.toEqual({
      ok: true,
      errors: [],
    });
    await expect(
      validateXxxlTsSvmParityExecutionBackedValidationSuite(),
    ).resolves.toEqual({
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
      ["src", "xxxl", "ts-svm-parity-execution-backed-validation.ts"].join("/"),
      ["tests", "xxxl", "ts-svm-parity-execution-backed-validation.test.ts"].join("/"),
    ];

    for (const sourcePath of sourcePaths) {
      const source = readFileSync(sourcePath, "utf8");

      for (const blockedTerm of blockedTerms) {
        expect(source).not.toContain(blockedTerm);
      }
    }
  });
});

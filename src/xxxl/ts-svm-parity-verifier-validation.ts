import { XXXL_GATEWAY_AUTHORIZATION_ERROR } from "./gateway-authorization-boundary.js";
import { XXXL_GUARDIAN_APPROVAL_ERROR } from "./guardian-approval-verifier.js";
import {
  XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL,
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
} from "./guardian-payload-encoding.js";
import {
  XXXL_TS_SVM_PARITY_INVALID_FIXTURES,
  XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE,
  type XXXLTsSvmParityExpectedFailureLayer,
  type XXXLTsSvmParityInvalidCaseId,
} from "./ts-svm-parity-invalid-fixtures.js";
import {
  XXXL_TS_SVM_PARITY_CASE_MATRIX,
  XXXL_TS_SVM_PARITY_EXPECTED_DECISION,
} from "./ts-svm-parity-vector-suite.js";

export const XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_ID =
  "XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_PHASE_29";

export const XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_VERSION = 1;

export const XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_BOUNDARY =
  "VERIFIER_ORIENTED_VALIDATION_ONLY_NO_RUNTIME_EXECUTION";

export const XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS = {
  TsModelValidatedRejection: "TS_MODEL_VALIDATED_REJECTION",
  FutureRuntimeValidationRequired: "FUTURE_RUNTIME_VALIDATION_REQUIRED",
} as const;

export type XXXLTsSvmParityVerifierValidationStatus =
  (typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS)[keyof typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS];

export const XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER = {
  Phase23PayloadValidation: "PHASE_23_PAYLOAD_VALIDATION",
  Phase23HashDomain: "PHASE_23_HASH_DOMAIN",
  Phase24GuardianVerifier: "PHASE_24_GUARDIAN_VERIFIER",
  Phase25AuthorizationBoundary: "PHASE_25_AUTHORIZATION_BOUNDARY",
  FutureRawPayloadDecoder: "FUTURE_RAW_PAYLOAD_DECODER",
  FutureSourceProofVerifier: "FUTURE_SOURCE_PROOF_VERIFIER",
  FutureAmountCapVerifier: "FUTURE_AMOUNT_CAP_VERIFIER",
  FutureTargetMintVerifier: "FUTURE_TARGET_MINT_VERIFIER",
} as const;

export type XXXLTsSvmParityVerifierValidationLayer =
  (typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER)[keyof typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER];

export type XXXLTsSvmParityVerifierValidationEntry = {
  readonly caseId: XXXLTsSvmParityInvalidCaseId;
  readonly fixtureId: string;
  readonly phase27VectorId: string;
  readonly phase28ExpectedFailureLayer: XXXLTsSvmParityExpectedFailureLayer;
  readonly expectedDecision: typeof XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution;
  readonly validationStatus: XXXLTsSvmParityVerifierValidationStatus;
  readonly validationLayer: XXXLTsSvmParityVerifierValidationLayer;
  readonly reason: string;
  readonly futureRuntimeCheck: string;
  readonly sourceBoundary: string;
  readonly usesExistingTsModel: boolean;
  readonly existingTsModelReference: string | null;
  readonly existingTsModelChecks: readonly string[];
  readonly runtimeImplementationRequired: boolean;
  readonly forbiddenExecutionTouched: false;
  readonly liveRouteTouched: false;
  readonly splCpiTouched: false;
  readonly invokeSignedTouched: false;
  readonly mintToTouched: false;
  readonly runtimeStateMutationTouched: false;
  readonly processedEventMarkingTouched: false;
  readonly productionProgramIdTouched: false;
  readonly blockerRemovalTouched: false;
};

export type XXXLTsSvmParityVerifierValidationSuite = {
  readonly suiteId: typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_ID;
  readonly suiteVersion: typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_VERSION;
  readonly boundary: typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_BOUNDARY;
  readonly phase23CanonicalPayloadVector: typeof XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR;
  readonly phase28FixtureSuiteId: typeof XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE.suiteId;
  readonly validationMatrix: readonly XXXLTsSvmParityVerifierValidationEntry[];
};

export type XXXLTsSvmParityVerifierValidationSuiteValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly string[];
};

type ValidationPlan = {
  readonly validationStatus: XXXLTsSvmParityVerifierValidationStatus;
  readonly validationLayer: XXXLTsSvmParityVerifierValidationLayer;
  readonly reason: string;
  readonly existingTsModelReference: string | null;
  readonly existingTsModelChecks: readonly string[];
};

const TS_MODEL_VALIDATED_REJECTION =
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.TsModelValidatedRejection;
const FUTURE_RUNTIME_VALIDATION_REQUIRED =
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.FutureRuntimeValidationRequired;

const VALIDATION_PLAN_BY_CASE_ID: {
  readonly [caseId in XXXLTsSvmParityInvalidCaseId]: ValidationPlan;
} = {
  "wrong-field-order": {
    validationStatus: FUTURE_RUNTIME_VALIDATION_REQUIRED,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.FutureRawPayloadDecoder,
    reason:
      "Phase 28 fixture records the wrong decoded order, but the existing TypeScript model starts after canonical field construction.",
    existingTsModelReference: null,
    existingTsModelChecks: [],
  },
  "wrong-byte-encoding": {
    validationStatus: FUTURE_RUNTIME_VALIDATION_REQUIRED,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.FutureRawPayloadDecoder,
    reason:
      "Phase 28 fixture records corrupted encoded bytes, while the current TypeScript model only emits canonical bytes from structured fields.",
    existingTsModelReference: null,
    existingTsModelChecks: [],
  },
  "wrong-hash-domain": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase23HashDomain,
    reason:
      "Phase 23 fixes the payload hash domain, so a hash built with any other domain is rejected before verifier parity can continue.",
    existingTsModelReference: "XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL",
    existingTsModelChecks: [XXXL_GUARDIAN_PAYLOAD_HASH_DOMAIN_LABEL],
  },
  "malformed-bytes32-field": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase23PayloadValidation,
    reason:
      "Phase 23 payload validation rejects fixed-width fields that are not exactly 32 bytes.",
    existingTsModelReference: "encodeXxxlGuardianPayload",
    existingTsModelChecks: ["bytes32 length check"],
  },
  "malformed-var-bytes-field": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase23PayloadValidation,
    reason:
      "Phase 23 payload validation rejects empty length-prefixed source fields.",
    existingTsModelReference: "encodeXxxlGuardianPayload",
    existingTsModelChecks: ["variable bytes length check"],
  },
  "invalid-source-chain-id": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase25AuthorizationBoundary,
    reason:
      "Phase 25 binding checks reject a payload source chain id that does not match the expected route binding.",
    existingTsModelReference: "XXXL_GATEWAY_AUTHORIZATION_ERROR",
    existingTsModelChecks: [XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainIdMismatch],
  },
  "wrong-target-mint": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase25AuthorizationBoundary,
    reason:
      "Phase 25 binding checks reject a target mint value that differs from the route expectation.",
    existingTsModelReference: "XXXL_GATEWAY_AUTHORIZATION_ERROR",
    existingTsModelChecks: [XXXL_GATEWAY_AUTHORIZATION_ERROR.TargetMintMismatch],
  },
  "wrong-guardian-set-id": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase25AuthorizationBoundary,
    reason:
      "Phase 25 binding checks reject a guardian set id that differs from the active route expectation.",
    existingTsModelReference: "XXXL_GATEWAY_AUTHORIZATION_ERROR",
    existingTsModelChecks: [XXXL_GATEWAY_AUTHORIZATION_ERROR.GuardianSetIdMismatch],
  },
  "wrong-source-chain-weight": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase25AuthorizationBoundary,
    reason:
      "Phase 25 binding checks reject a source chain weight that differs from the expected route value.",
    existingTsModelReference: "XXXL_GATEWAY_AUTHORIZATION_ERROR",
    existingTsModelChecks: [
      XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainWeightMismatch,
    ],
  },
  "invalid-signature": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase24GuardianVerifier,
    reason:
      "Phase 24 guardian approval verification rejects a tampered Ed25519 signature over the Phase 23 payload hash.",
    existingTsModelReference: "XXXL_GUARDIAN_APPROVAL_ERROR",
    existingTsModelChecks: [XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature],
  },
  "duplicate-guardian-approval": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase24GuardianVerifier,
    reason:
      "Phase 24 guardian quorum verification rejects duplicate approvals from the same guardian key.",
    existingTsModelReference: "XXXL_GUARDIAN_APPROVAL_ERROR",
    existingTsModelChecks: [XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianApproval],
  },
  "insufficient-quorum": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase24GuardianVerifier,
    reason:
      "Phase 24 guardian quorum verification rejects approval sets below the configured threshold.",
    existingTsModelReference: "XXXL_GUARDIAN_APPROVAL_ERROR",
    existingTsModelChecks: [XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached],
  },
  "expired-payload": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase25AuthorizationBoundary,
    reason:
      "Phase 25 authorization checks reject a payload after the signed expiration slot.",
    existingTsModelReference: "XXXL_GATEWAY_AUTHORIZATION_ERROR",
    existingTsModelChecks: [XXXL_GATEWAY_AUTHORIZATION_ERROR.PayloadExpired],
  },
  "duplicate-canonical-event-key": {
    validationStatus: TS_MODEL_VALIDATED_REJECTION,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.Phase25AuthorizationBoundary,
    reason:
      "Phase 25 replay checks reject a canonical event key that already appears in the processed registry snapshot.",
    existingTsModelReference: "XXXL_GATEWAY_AUTHORIZATION_ERROR",
    existingTsModelChecks: [
      XXXL_GATEWAY_AUTHORIZATION_ERROR.CanonicalEventAlreadyProcessed,
    ],
  },
  "wrong-canonical-event-key-preimage": {
    validationStatus: FUTURE_RUNTIME_VALIDATION_REQUIRED,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.FutureSourceProofVerifier,
    reason:
      "Phase 28 records the source identity mismatch, but the current TypeScript model has no source proof preimage recomputation helper.",
    existingTsModelReference: null,
    existingTsModelChecks: [],
  },
  "wrong-source-burn-tx-hash": {
    validationStatus: FUTURE_RUNTIME_VALIDATION_REQUIRED,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.FutureSourceProofVerifier,
    reason:
      "Phase 28 records the source transaction hash mismatch, but source proof verification remains a future runtime requirement.",
    existingTsModelReference: null,
    existingTsModelChecks: [],
  },
  "wrong-source-burn-event-index": {
    validationStatus: FUTURE_RUNTIME_VALIDATION_REQUIRED,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.FutureSourceProofVerifier,
    reason:
      "Phase 28 records the source event index mismatch, but source proof verification remains a future runtime requirement.",
    existingTsModelReference: null,
    existingTsModelChecks: [],
  },
  "amount-over-route-cap": {
    validationStatus: FUTURE_RUNTIME_VALIDATION_REQUIRED,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.FutureAmountCapVerifier,
    reason:
      "Phase 28 records the route amount cap violation, but the current TypeScript authorization boundary has no amount cap helper.",
    existingTsModelReference: null,
    existingTsModelChecks: [],
  },
  "invalid-target-mint": {
    validationStatus: FUTURE_RUNTIME_VALIDATION_REQUIRED,
    validationLayer:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_LAYER.FutureTargetMintVerifier,
    reason:
      "Phase 28 records an unrecognized target mint account, but account legitimacy checks remain a future runtime requirement.",
    existingTsModelReference: null,
    existingTsModelChecks: [],
  },
};

function phase27VectorId(caseId: XXXLTsSvmParityInvalidCaseId): string {
  const parityCase = XXXL_TS_SVM_PARITY_CASE_MATRIX.find(
    (candidate) => candidate.caseId === caseId,
  );

  if (parityCase === undefined) {
    throw new Error(`Missing Phase 27 parity case: ${caseId}`);
  }

  return parityCase.vectorId;
}

export const XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX: readonly XXXLTsSvmParityVerifierValidationEntry[] =
  XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map((fixture) => {
    const plan = VALIDATION_PLAN_BY_CASE_ID[fixture.caseId];
    const usesExistingTsModel =
      plan.validationStatus === TS_MODEL_VALIDATED_REJECTION;

    return {
      caseId: fixture.caseId,
      fixtureId: fixture.fixtureId,
      phase27VectorId: phase27VectorId(fixture.caseId),
      phase28ExpectedFailureLayer: fixture.expectedFailureLayer,
      expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
      validationStatus: plan.validationStatus,
      validationLayer: plan.validationLayer,
      reason: plan.reason,
      futureRuntimeCheck: fixture.futureRuntimeCheck,
      sourceBoundary: fixture.sourceBoundary,
      usesExistingTsModel,
      existingTsModelReference: plan.existingTsModelReference,
      existingTsModelChecks: plan.existingTsModelChecks,
      runtimeImplementationRequired: !usesExistingTsModel,
      forbiddenExecutionTouched: false,
      liveRouteTouched: false,
      splCpiTouched: false,
      invokeSignedTouched: false,
      mintToTouched: false,
      runtimeStateMutationTouched: false,
      processedEventMarkingTouched: false,
      productionProgramIdTouched: false,
      blockerRemovalTouched: false,
    };
  });

export const XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE: XXXLTsSvmParityVerifierValidationSuite =
  {
    suiteId: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_ID,
    suiteVersion: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_VERSION,
    boundary: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_BOUNDARY,
    phase23CanonicalPayloadVector: XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
    phase28FixtureSuiteId: XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE.suiteId,
    validationMatrix: XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX,
  };

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

function expectedFixtureByCaseId(): Map<XXXLTsSvmParityInvalidCaseId, {
  readonly fixtureId: string;
  readonly phase27VectorId: string;
}> {
  return new Map(
    XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map((fixture) => [
      fixture.caseId,
      {
        fixtureId: fixture.fixtureId,
        phase27VectorId: fixture.phase27VectorId,
      },
    ]),
  );
}

export function validateXxxlTsSvmParityVerifierValidationSuite(
  suite: XXXLTsSvmParityVerifierValidationSuite = XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE,
): XXXLTsSvmParityVerifierValidationSuiteValidationResult {
  const errors: string[] = [];
  const expectedFixtures = expectedFixtureByCaseId();
  const caseIds = suite.validationMatrix.map((entry) => entry.caseId as string);
  const fixtureIds = suite.validationMatrix.map((entry) => entry.fixtureId);
  const expectedCaseIds = XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map(
    (fixture) => fixture.caseId,
  );

  if (suite.suiteId !== XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_ID) {
    errors.push("wrong suite id");
  }

  if (suite.suiteVersion !== XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE_VERSION) {
    errors.push("wrong suite version");
  }

  if (suite.boundary !== XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_BOUNDARY) {
    errors.push("wrong boundary marker");
  }

  if (suite.phase23CanonicalPayloadVector !== XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR) {
    errors.push("Phase 23 canonical payload vector is not reused");
  }

  if (suite.phase28FixtureSuiteId !== XXXL_TS_SVM_PARITY_INVALID_FIXTURE_SUITE.suiteId) {
    errors.push("wrong Phase 28 fixture suite dependency");
  }

  if (caseIds.includes("valid-canonical-payload")) {
    errors.push("valid canonical payload must not be part of Phase 29 invalid validation");
  }

  if (hasDuplicates(caseIds)) {
    errors.push("duplicate validation case id");
  }

  if (hasDuplicates(fixtureIds)) {
    errors.push("duplicate validation fixture id");
  }

  for (const expectedCaseId of expectedCaseIds) {
    if (!caseIds.includes(expectedCaseId)) {
      errors.push(`missing verifier validation case: ${expectedCaseId}`);
    }
  }

  for (const entry of suite.validationMatrix) {
    const expectedFixture = expectedFixtures.get(entry.caseId);

    if (expectedFixture === undefined) {
      errors.push(`unexpected verifier validation case: ${entry.caseId}`);
      continue;
    }

    if (entry.fixtureId !== expectedFixture.fixtureId) {
      errors.push(`wrong Phase 28 fixture id: ${entry.caseId}`);
    }

    if (entry.phase27VectorId !== expectedFixture.phase27VectorId) {
      errors.push(`wrong Phase 27 vector id: ${entry.caseId}`);
    }

    if (
      entry.expectedDecision !==
      XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution
    ) {
      errors.push(`entry does not reject before execution: ${entry.caseId}`);
    }

    if (entry.reason.trim().length === 0) {
      errors.push(`missing validation reason: ${entry.caseId}`);
    }

    if (entry.futureRuntimeCheck.trim().length === 0) {
      errors.push(`missing future runtime check: ${entry.caseId}`);
    }

    if (entry.sourceBoundary.trim().length === 0) {
      errors.push(`missing source boundary: ${entry.caseId}`);
    }

    if (entry.usesExistingTsModel) {
      if (entry.validationStatus !== TS_MODEL_VALIDATED_REJECTION) {
        errors.push(`TS model entry has wrong status: ${entry.caseId}`);
      }

      if (entry.runtimeImplementationRequired) {
        errors.push(`TS model entry wrongly requires runtime implementation: ${entry.caseId}`);
      }

      if (entry.existingTsModelReference === null) {
        errors.push(`TS model entry lacks reference: ${entry.caseId}`);
      }

      if (entry.existingTsModelChecks.length === 0) {
        errors.push(`TS model entry lacks checks: ${entry.caseId}`);
      }
    } else {
      if (entry.validationStatus !== FUTURE_RUNTIME_VALIDATION_REQUIRED) {
        errors.push(`future runtime entry has wrong status: ${entry.caseId}`);
      }

      if (!entry.runtimeImplementationRequired) {
        errors.push(`future runtime entry does not require implementation: ${entry.caseId}`);
      }

      if (entry.existingTsModelReference !== null) {
        errors.push(`future runtime entry should not cite TS model reference: ${entry.caseId}`);
      }

      if (entry.existingTsModelChecks.length !== 0) {
        errors.push(`future runtime entry should not cite TS model checks: ${entry.caseId}`);
      }
    }

    if (
      entry.forbiddenExecutionTouched ||
      entry.liveRouteTouched ||
      entry.splCpiTouched ||
      entry.invokeSignedTouched ||
      entry.mintToTouched ||
      entry.runtimeStateMutationTouched ||
      entry.processedEventMarkingTouched ||
      entry.productionProgramIdTouched ||
      entry.blockerRemovalTouched
    ) {
      errors.push(`entry touches forbidden execution boundary: ${entry.caseId}`);
    }
  }

  if (
    !suite.validationMatrix.some(
      (entry) => entry.validationStatus === TS_MODEL_VALIDATED_REJECTION,
    )
  ) {
    errors.push("missing TS model validated rejection entries");
  }

  if (
    !suite.validationMatrix.some(
      (entry) => entry.validationStatus === FUTURE_RUNTIME_VALIDATION_REQUIRED,
    )
  ) {
    errors.push("missing future runtime validation entries");
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

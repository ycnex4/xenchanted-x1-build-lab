import * as ed25519 from "@noble/ed25519";

import { hexToBytes } from "../gateway/stage-1-encoding.js";
import {
  XXXL_GATEWAY_AUTHORIZATION_ERROR,
  XXXL_GATEWAY_AUTHORIZATION_STATUS,
  authorizeXxxlGatewayMintBoundary,
  type XXXLGatewayAuthorizationInput,
  type XXXLGatewayRuntimeBindingExpectations,
} from "./gateway-authorization-boundary.js";
import {
  XXXL_GUARDIAN_APPROVAL_ERROR,
  verifyXxxlGuardianPayloadQuorum,
  type XXXLGuardianApproval,
} from "./guardian-approval-verifier.js";
import {
  XXXL_GUARDIAN_PAYLOAD_VALID_FIELDS,
  XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
  encodeXxxlGuardianPayload,
  hashXxxlGuardianPayload,
  validateXxxlGuardianPayloadVector,
  type XXXLGuardianPayloadFields,
} from "./guardian-payload-encoding.js";
import {
  XXXL_TS_SVM_PARITY_INVALID_FIXTURES,
  type XXXLTsSvmParityInvalidCaseId,
} from "./ts-svm-parity-invalid-fixtures.js";
import {
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS,
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE,
  type XXXLTsSvmParityVerifierValidationEntry,
  type XXXLTsSvmParityVerifierValidationStatus,
} from "./ts-svm-parity-verifier-validation.js";
import {
  XXXL_TS_SVM_PARITY_EXPECTED_DECISION,
  type XXXLTsSvmParityExpectedDecision,
} from "./ts-svm-parity-vector-suite.js";

export const XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_ID =
  "XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_PHASE_30";

export const XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_VERSION = 1;

export const XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_BOUNDARY =
  "EXECUTION_BACKED_TS_PARITY_ONLY_NO_RUNTIME_EXECUTION";

export const XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS = {
  TsExecutionBackedRejection: "TS_EXECUTION_BACKED_REJECTION",
  TsExecutionPathUnavailable: "TS_EXECUTION_PATH_UNAVAILABLE",
  FutureRuntimeValidationRequired: "FUTURE_RUNTIME_VALIDATION_REQUIRED",
} as const;

export type XXXLTsSvmParityExecutionBackedStatus =
  (typeof XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS)[keyof typeof XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS];

export type XXXLTsSvmParityExecutionBackedValidationEntry = {
  readonly caseId: XXXLTsSvmParityInvalidCaseId;
  readonly fixtureId: string;
  readonly phase27VectorId: string;
  readonly phase29ValidationStatus: XXXLTsSvmParityVerifierValidationStatus;
  readonly expectedDecision: Extract<
    XXXLTsSvmParityExpectedDecision,
    "REJECT_BEFORE_EXECUTION"
  >;
  readonly executionStatus: XXXLTsSvmParityExecutionBackedStatus;
  readonly existingTsModelReference: string | null;
  readonly expectedError: string | null;
  readonly actualError: string | null;
  readonly actualErrors: readonly string[];
  readonly matchesExpected: boolean | null;
  readonly reason: string;
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

export type XXXLTsSvmParityExecutionBackedValidationSuite = {
  readonly suiteId: typeof XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_ID;
  readonly suiteVersion: typeof XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_VERSION;
  readonly boundary: typeof XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_BOUNDARY;
  readonly phase23CanonicalPayloadVector: typeof XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR;
  readonly phase29VerifierValidationSuiteId: typeof XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE.suiteId;
  readonly entries: readonly XXXLTsSvmParityExecutionBackedValidationEntry[];
};

export type XXXLTsSvmParityExecutionBackedValidationSuiteValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly string[];
};

type ExecutionAttempt = Pick<
  XXXLTsSvmParityExecutionBackedValidationEntry,
  | "executionStatus"
  | "expectedError"
  | "actualError"
  | "actualErrors"
  | "matchesExpected"
  | "reason"
  | "runtimeImplementationRequired"
>;

type TestGuardian = {
  readonly seed: Uint8Array;
  readonly publicKey: Uint8Array;
};

const TEST_GUARDIAN_SEEDS: readonly Uint8Array[] = [
  new Uint8Array(32).fill(0x31),
  new Uint8Array(32).fill(0x32),
];

const TS_MODEL_VALIDATED_REJECTION =
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.TsModelValidatedRejection;
const FUTURE_RUNTIME_VALIDATION_REQUIRED =
  XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_STATUS.FutureRuntimeValidationRequired;
const TS_EXECUTION_BACKED_REJECTION =
  XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.TsExecutionBackedRejection;
const TS_EXECUTION_PATH_UNAVAILABLE =
  XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.TsExecutionPathUnavailable;
const PHASE_30_FUTURE_RUNTIME_VALIDATION_REQUIRED =
  XXXL_TS_SVM_PARITY_EXECUTION_BACKED_STATUS.FutureRuntimeValidationRequired;

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

function stringInput(input: Record<string, unknown>, key: string): string {
  const value = input[key];

  if (typeof value !== "string") {
    throw new Error(`Phase 28 fixture ${key} must be a string`);
  }

  return value;
}

function numberInput(input: Record<string, unknown>, key: string): number {
  const value = input[key];

  if (typeof value !== "number") {
    throw new Error(`Phase 28 fixture ${key} must be a number`);
  }

  return value;
}

function stringArrayInput(
  input: Record<string, unknown>,
  key: string,
): readonly string[] {
  const value = input[key];

  if (!Array.isArray(value) || !value.every((item) => typeof item === "string")) {
    throw new Error(`Phase 28 fixture ${key} must be a string array`);
  }

  return value;
}

function fixtureFor(
  caseId: XXXLTsSvmParityInvalidCaseId,
): (typeof XXXL_TS_SVM_PARITY_INVALID_FIXTURES)[number] {
  const fixture = XXXL_TS_SVM_PARITY_INVALID_FIXTURES.find(
    (candidate) => candidate.caseId === caseId,
  );

  if (fixture === undefined) {
    throw new Error(`Missing Phase 28 fixture: ${caseId}`);
  }

  return fixture;
}

function executionBacked(
  expectedError: string,
  actualErrors: readonly string[],
  reason: string,
): ExecutionAttempt {
  const actualError = actualErrors.includes(expectedError)
    ? expectedError
    : (actualErrors[0] ?? null);

  return {
    executionStatus: TS_EXECUTION_BACKED_REJECTION,
    expectedError,
    actualError,
    actualErrors,
    matchesExpected: actualErrors.includes(expectedError),
    reason,
    runtimeImplementationRequired: false,
  };
}

function unavailable(reason: string): ExecutionAttempt {
  return {
    executionStatus: TS_EXECUTION_PATH_UNAVAILABLE,
    expectedError: null,
    actualError: null,
    actualErrors: [],
    matchesExpected: null,
    reason,
    runtimeImplementationRequired: true,
  };
}

function futureRequired(reason: string): ExecutionAttempt {
  return {
    executionStatus: PHASE_30_FUTURE_RUNTIME_VALIDATION_REQUIRED,
    expectedError: null,
    actualError: null,
    actualErrors: [],
    matchesExpected: null,
    reason,
    runtimeImplementationRequired: true,
  };
}

function captureThrownError(run: () => void): readonly string[] {
  try {
    run();
    return [];
  } catch (error) {
    return [error instanceof Error ? error.message : String(error)];
  }
}

async function testGuardian(index: number): Promise<TestGuardian> {
  const seed = TEST_GUARDIAN_SEEDS[index];

  if (seed === undefined) {
    throw new Error(`Missing Phase 30 test guardian seed: ${index}`);
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

async function executeWrongHashDomain(): Promise<ExecutionAttempt> {
  const fixture = fixtureFor("wrong-hash-domain");
  const wrongHashDomainLabel = stringInput(
    fixture.tamperedInput,
    "wrongHashDomainLabel",
  );
  const result = validateXxxlGuardianPayloadVector({
    ...XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
    hashDomainLabel:
      wrongHashDomainLabel as typeof XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR.hashDomainLabel,
    hashDomainSeparatorHex: stringInput(
      fixture.tamperedInput,
      "wrongDomainSeparatorHex",
    ),
    hashPreimageHex: stringInput(fixture.tamperedInput, "wrongHashPreimageHex"),
    payloadHash: stringInput(fixture.tamperedInput, "wrongPayloadHash"),
  });

  return executionBacked(
    "wrong hash domain label",
    result.errors,
    "Executed Phase 23 vector validation against the Phase 28 wrong hash-domain material.",
  );
}

async function executeMalformedBytes32Field(): Promise<ExecutionAttempt> {
  const fixture = fixtureFor("malformed-bytes32-field");
  const actualErrors = captureThrownError(() =>
    encodeXxxlGuardianPayload(
      cloneFields({
        route_id: hexToBytes(
          stringInput(fixture.tamperedInput, "valueHex"),
          31,
          "route_id",
        ),
      }),
    ),
  );

  return executionBacked(
    "route_id must be exactly 32 bytes, got 31",
    actualErrors,
    "Executed Phase 23 payload encoder validation against the malformed bytes32 field.",
  );
}

async function executeMalformedVarBytesField(): Promise<ExecutionAttempt> {
  const fixture = fixtureFor("malformed-var-bytes-field");
  const actualErrors = captureThrownError(() =>
    encodeXxxlGuardianPayload(
      cloneFields({
        source_token: hexToBytes(
          stringInput(fixture.tamperedInput, "valueHex"),
          undefined,
          "source_token",
        ),
      }),
    ),
  );

  return executionBacked(
    "source_token must be non-empty",
    actualErrors,
    "Executed Phase 23 payload encoder validation against the empty variable bytes field.",
  );
}

async function executeInvalidSignature(): Promise<ExecutionAttempt> {
  const fixture = fixtureFor("invalid-signature");
  const result = await verifyXxxlGuardianPayloadQuorum({
    fields: cloneFields(),
    quorum: {
      guardianPublicKeys: [
        hexToBytes(
          stringInput(fixture.tamperedInput, "guardianPublicKeyHex"),
          32,
          "guardian public key",
        ),
      ],
      threshold: 1,
    },
    approvals: [
      {
        guardianPublicKey: hexToBytes(
          stringInput(fixture.tamperedInput, "guardianPublicKeyHex"),
          32,
          "guardian public key",
        ),
        guardianSignature: hexToBytes(
          stringInput(fixture.tamperedInput, "tamperedSignatureHex"),
          64,
          "guardian signature",
        ),
      },
    ],
  });

  return executionBacked(
    XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
    result.approvals.flatMap((approval) => approval.errors),
    "Executed Phase 24 guardian quorum verification against the Phase 28 tampered signature.",
  );
}

async function executeDuplicateGuardianApproval(): Promise<ExecutionAttempt> {
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

  return executionBacked(
    XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianApproval,
    result.approvals.flatMap((approvalResult) => approvalResult.errors),
    "Executed Phase 24 guardian quorum verification against duplicate approvals.",
  );
}

async function executeInsufficientQuorum(): Promise<ExecutionAttempt> {
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

  return executionBacked(
    XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
    result.errors,
    "Executed Phase 24 guardian quorum verification with one approval below a 2-of-2 threshold.",
  );
}

async function executeAuthorizationBoundaryCase(
  caseId: XXXLTsSvmParityInvalidCaseId,
): Promise<ExecutionAttempt> {
  const fixture = fixtureFor(caseId);
  const input = await validAuthorizationInput();
  const processedRegistry = input.processedRegistry;
  let authorizationInput: XXXLGatewayAuthorizationInput = input;
  let expectedError: string;
  let reason: string;

  switch (caseId) {
    case "invalid-source-chain-id":
      expectedError = XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainIdMismatch;
      authorizationInput = {
        ...input,
        expectedBindings: expectedBindings(input.fields, {
          source_chain_id: BigInt(
            stringInput(fixture.tamperedInput, "expectedRuntimeSourceChainId"),
          ),
        }),
      };
      reason =
        "Executed Phase 25 authorization boundary against a source chain id mismatch.";
      break;
    case "wrong-target-mint":
      expectedError = XXXL_GATEWAY_AUTHORIZATION_ERROR.TargetMintMismatch;
      authorizationInput = {
        ...input,
        expectedBindings: expectedBindings(input.fields, {
          target_mint: hexToBytes(
            stringInput(fixture.tamperedInput, "expectedRuntimeTargetMintHex"),
            32,
            "expected target mint",
          ),
        }),
      };
      reason =
        "Executed Phase 25 authorization boundary against a target mint mismatch.";
      break;
    case "wrong-guardian-set-id":
      expectedError = XXXL_GATEWAY_AUTHORIZATION_ERROR.GuardianSetIdMismatch;
      authorizationInput = {
        ...input,
        expectedBindings: expectedBindings(input.fields, {
          guardian_set_id: hexToBytes(
            stringInput(
              fixture.tamperedInput,
              "expectedRuntimeGuardianSetIdHex",
            ),
            32,
            "expected guardian set id",
          ),
        }),
      };
      reason =
        "Executed Phase 25 authorization boundary against a guardian set id mismatch.";
      break;
    case "wrong-source-chain-weight":
      expectedError = XXXL_GATEWAY_AUTHORIZATION_ERROR.SourceChainWeightMismatch;
      authorizationInput = {
        ...input,
        expectedBindings: expectedBindings(input.fields, {
          source_chain_weight_bps: numberInput(
            fixture.tamperedInput,
            "expectedRuntimeSourceChainWeightBps",
          ),
        }),
      };
      reason =
        "Executed Phase 25 authorization boundary against a source chain weight mismatch.";
      break;
    case "expired-payload":
      expectedError = XXXL_GATEWAY_AUTHORIZATION_ERROR.PayloadExpired;
      authorizationInput = {
        ...input,
        currentTimeOrSlot: BigInt(stringInput(fixture.tamperedInput, "currentSlot")),
      };
      reason =
        "Executed Phase 25 authorization boundary against an expired payload slot.";
      break;
    case "duplicate-canonical-event-key": {
      expectedError =
        XXXL_GATEWAY_AUTHORIZATION_ERROR.CanonicalEventAlreadyProcessed;
      const processedCanonicalEventKeyHexes = stringArrayInput(
        fixture.tamperedInput,
        "processedCanonicalEventKeyHexes",
      );
      authorizationInput = {
        ...input,
        processedRegistry: {
          processedCanonicalEventKeyHexes,
        },
      };
      reason =
        "Executed Phase 25 authorization boundary against a processed registry snapshot containing the canonical event key.";
      break;
    }
    default:
      return unavailable(`No Phase 25 authorization executor for ${caseId}.`);
  }

  const beforeRegistry = JSON.stringify(authorizationInput.processedRegistry);
  const result = await authorizeXxxlGatewayMintBoundary(authorizationInput);
  const afterRegistry = JSON.stringify(authorizationInput.processedRegistry);
  const actualErrors =
    beforeRegistry === afterRegistry
      ? result.errors
      : [...result.errors, "processed registry snapshot mutated"];

  if (result.authorizationStatus === XXXL_GATEWAY_AUTHORIZATION_STATUS.Authorized) {
    return executionBacked(expectedError, actualErrors, reason);
  }

  if (processedRegistry.processedCanonicalEventKeyHexes.length !== 0) {
    return executionBacked(
      expectedError,
      [...actualErrors, "baseline registry snapshot mutated"],
      reason,
    );
  }

  return executionBacked(expectedError, actualErrors, reason);
}

async function executeTsModelCase(
  entry: XXXLTsSvmParityVerifierValidationEntry,
): Promise<ExecutionAttempt> {
  switch (entry.caseId) {
    case "wrong-hash-domain":
      return executeWrongHashDomain();
    case "malformed-bytes32-field":
      return executeMalformedBytes32Field();
    case "malformed-var-bytes-field":
      return executeMalformedVarBytesField();
    case "invalid-source-chain-id":
    case "wrong-target-mint":
    case "wrong-guardian-set-id":
    case "wrong-source-chain-weight":
    case "expired-payload":
    case "duplicate-canonical-event-key":
      return executeAuthorizationBoundaryCase(entry.caseId);
    case "invalid-signature":
      return executeInvalidSignature();
    case "duplicate-guardian-approval":
      return executeDuplicateGuardianApproval();
    case "insufficient-quorum":
      return executeInsufficientQuorum();
    default:
      return unavailable(
        `No existing TypeScript execution path was found for ${entry.caseId}.`,
      );
  }
}

function futureRequiredFromPhase29(
  entry: XXXLTsSvmParityVerifierValidationEntry,
): ExecutionAttempt {
  return futureRequired(
    `Phase 29 classifies ${entry.caseId} as future runtime validation required: ${entry.reason}`,
  );
}

function entryFromAttempt(
  entry: XXXLTsSvmParityVerifierValidationEntry,
  attempt: ExecutionAttempt,
): XXXLTsSvmParityExecutionBackedValidationEntry {
  return {
    caseId: entry.caseId,
    fixtureId: entry.fixtureId,
    phase27VectorId: entry.phase27VectorId,
    phase29ValidationStatus: entry.validationStatus,
    expectedDecision: XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution,
    executionStatus: attempt.executionStatus,
    existingTsModelReference: entry.existingTsModelReference,
    expectedError: attempt.expectedError,
    actualError: attempt.actualError,
    actualErrors: attempt.actualErrors,
    matchesExpected: attempt.matchesExpected,
    reason: attempt.reason,
    runtimeImplementationRequired: attempt.runtimeImplementationRequired,
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
}

export async function buildXxxlTsSvmParityExecutionBackedValidationMatrix(): Promise<
  readonly XXXLTsSvmParityExecutionBackedValidationEntry[]
> {
  const entries: XXXLTsSvmParityExecutionBackedValidationEntry[] = [];

  for (const entry of XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_MATRIX) {
    const attempt =
      entry.validationStatus === TS_MODEL_VALIDATED_REJECTION
        ? await executeTsModelCase(entry)
        : futureRequiredFromPhase29(entry);

    entries.push(entryFromAttempt(entry, attempt));
  }

  return entries;
}

export async function buildXxxlTsSvmParityExecutionBackedValidationSuite(): Promise<XXXLTsSvmParityExecutionBackedValidationSuite> {
  return {
    suiteId: XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_ID,
    suiteVersion: XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_VERSION,
    boundary: XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_BOUNDARY,
    phase23CanonicalPayloadVector: XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR,
    phase29VerifierValidationSuiteId:
      XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE.suiteId,
    entries: await buildXxxlTsSvmParityExecutionBackedValidationMatrix(),
  };
}

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

export async function validateXxxlTsSvmParityExecutionBackedValidationSuite(
  suite?: XXXLTsSvmParityExecutionBackedValidationSuite,
): Promise<XXXLTsSvmParityExecutionBackedValidationSuiteValidationResult> {
  const actualSuite =
    suite ?? (await buildXxxlTsSvmParityExecutionBackedValidationSuite());
  const errors: string[] = [];
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
  const expectedCaseIds = XXXL_TS_SVM_PARITY_INVALID_FIXTURES.map(
    (fixture) => fixture.caseId,
  );
  const caseIds = actualSuite.entries.map((entry) => entry.caseId as string);
  const fixtureIds = actualSuite.entries.map((entry) => entry.fixtureId);
  const tsModelEntries = actualSuite.entries.filter(
    (entry) => entry.phase29ValidationStatus === TS_MODEL_VALIDATED_REJECTION,
  );
  const futureEntries = actualSuite.entries.filter(
    (entry) => entry.phase29ValidationStatus === FUTURE_RUNTIME_VALIDATION_REQUIRED,
  );

  if (actualSuite.suiteId !== XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_ID) {
    errors.push("wrong suite id");
  }

  if (
    actualSuite.suiteVersion !==
    XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_SUITE_VERSION
  ) {
    errors.push("wrong suite version");
  }

  if (actualSuite.boundary !== XXXL_TS_SVM_PARITY_EXECUTION_BACKED_VALIDATION_BOUNDARY) {
    errors.push("wrong boundary marker");
  }

  if (actualSuite.phase23CanonicalPayloadVector !== XXXL_GUARDIAN_PAYLOAD_VALID_VECTOR) {
    errors.push("Phase 23 canonical payload vector is not reused");
  }

  if (
    actualSuite.phase29VerifierValidationSuiteId !==
    XXXL_TS_SVM_PARITY_VERIFIER_VALIDATION_SUITE.suiteId
  ) {
    errors.push("wrong Phase 29 validation suite dependency");
  }

  if (caseIds.includes("valid-canonical-payload")) {
    errors.push("valid canonical payload must not be part of Phase 30 invalid validation");
  }

  if (hasDuplicates(caseIds)) {
    errors.push("duplicate execution-backed case id");
  }

  if (hasDuplicates(fixtureIds)) {
    errors.push("duplicate execution-backed fixture id");
  }

  for (const expectedCaseId of expectedCaseIds) {
    if (!caseIds.includes(expectedCaseId)) {
      errors.push(`missing execution-backed case: ${expectedCaseId}`);
    }
  }

  for (const entry of actualSuite.entries) {
    const fixture = fixtureByCaseId.get(entry.caseId);
    const phase29 = phase29ByCaseId.get(entry.caseId);

    if (fixture === undefined) {
      errors.push(`unexpected execution-backed case: ${entry.caseId}`);
      continue;
    }

    if (phase29 === undefined) {
      errors.push(`missing Phase 29 entry: ${entry.caseId}`);
      continue;
    }

    if (entry.fixtureId !== fixture.fixtureId) {
      errors.push(`wrong Phase 28 fixture id: ${entry.caseId}`);
    }

    if (entry.phase27VectorId !== phase29.phase27VectorId) {
      errors.push(`wrong Phase 27 vector id: ${entry.caseId}`);
    }

    if (entry.phase29ValidationStatus !== phase29.validationStatus) {
      errors.push(`wrong Phase 29 validation status: ${entry.caseId}`);
    }

    if (
      entry.expectedDecision !==
      XXXL_TS_SVM_PARITY_EXPECTED_DECISION.RejectBeforeExecution
    ) {
      errors.push(`entry does not reject before execution: ${entry.caseId}`);
    }

    if (entry.reason.trim().length === 0) {
      errors.push(`missing reason: ${entry.caseId}`);
    }

    if (entry.executionStatus === TS_EXECUTION_BACKED_REJECTION) {
      if (entry.expectedError === null) {
        errors.push(`execution-backed entry lacks expected error: ${entry.caseId}`);
      }

      if (entry.actualError === null) {
        errors.push(`execution-backed entry lacks actual error: ${entry.caseId}`);
      }

      if (entry.matchesExpected !== true) {
        errors.push(`execution-backed entry does not match expected: ${entry.caseId}`);
      }

      if (entry.runtimeImplementationRequired) {
        errors.push(`execution-backed entry wrongly requires runtime implementation: ${entry.caseId}`);
      }
    }

    if (
      entry.executionStatus === PHASE_30_FUTURE_RUNTIME_VALIDATION_REQUIRED &&
      entry.phase29ValidationStatus !== FUTURE_RUNTIME_VALIDATION_REQUIRED
    ) {
      errors.push(`non-future Phase 29 case marked future-required: ${entry.caseId}`);
    }

    if (
      entry.executionStatus === TS_EXECUTION_PATH_UNAVAILABLE &&
      entry.matchesExpected === true
    ) {
      errors.push(`unavailable entry pretends to be execution-backed: ${entry.caseId}`);
    }

    if (
      entry.executionStatus !== TS_EXECUTION_BACKED_REJECTION &&
      (entry.expectedError !== null ||
        entry.actualError !== null ||
        entry.matchesExpected !== null)
    ) {
      errors.push(`non-execution-backed entry carries execution result: ${entry.caseId}`);
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

  if (tsModelEntries.length !== 12) {
    errors.push("wrong Phase 29 TS-model case count");
  }

  for (const entry of tsModelEntries) {
    if (
      entry.executionStatus !== TS_EXECUTION_BACKED_REJECTION &&
      entry.executionStatus !== TS_EXECUTION_PATH_UNAVAILABLE
    ) {
      errors.push(`TS-model case is not execution-backed or unavailable: ${entry.caseId}`);
    }
  }

  if (futureEntries.length !== 7) {
    errors.push("wrong Phase 29 future-runtime case count");
  }

  for (const entry of futureEntries) {
    if (entry.executionStatus !== PHASE_30_FUTURE_RUNTIME_VALIDATION_REQUIRED) {
      errors.push(`future-runtime case did not remain future-required: ${entry.caseId}`);
    }
  }

  if (
    !actualSuite.entries.some(
      (entry) =>
        entry.executionStatus === TS_EXECUTION_BACKED_REJECTION &&
        entry.expectedError !== null &&
        ([
          XXXL_GUARDIAN_APPROVAL_ERROR.InvalidSignature,
          XXXL_GUARDIAN_APPROVAL_ERROR.DuplicateGuardianApproval,
          XXXL_GUARDIAN_APPROVAL_ERROR.QuorumNotReached,
        ] as readonly string[]).includes(entry.expectedError),
    )
  ) {
    errors.push("missing execution-backed guardian verifier case");
  }

  if (
    !actualSuite.entries.some(
      (entry) =>
        entry.executionStatus === TS_EXECUTION_BACKED_REJECTION &&
        entry.expectedError !== null &&
        (Object.values(XXXL_GATEWAY_AUTHORIZATION_ERROR) as readonly string[]).includes(
          entry.expectedError,
        ),
    )
  ) {
    errors.push("missing execution-backed authorization boundary case");
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

import {
  executeXXXLRuntimeProgramSkeleton,
  type XXXLRuntimeProgramSkeletonErrorCode,
  type XXXLRuntimeProgramSkeletonStep,
} from "./runtime-program-skeleton.js";
import {
  XXXL_RUNTIME_EXECUTION_VECTOR_ID,
  validateXXXLRuntimeExecutionVectors,
  xxxlRuntimeExecutionVectorScenarios,
  xxxlRuntimeExecutionVectors,
  type XXXLRuntimeExecutionVector,
  type XXXLRuntimeExecutionVectorId,
  type XXXLRuntimeExecutionVectorScenario,
  type XXXLRuntimeExecutionVectorValidationResult,
} from "./runtime-execution-vectors.js";

export const XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION = 1;

export const XXXL_RUNTIME_DRY_RUN_FIXTURE_ID = {
  AllExecutionVectors: "XXXL_RUNTIME_DRY_RUN_ALL_EXECUTION_VECTORS",
  SuccessfulRoutes: "XXXL_RUNTIME_DRY_RUN_SUCCESSFUL_ROUTES",
  PreflightRejections: "XXXL_RUNTIME_DRY_RUN_PREFLIGHT_REJECTIONS",
  TransitionRejections: "XXXL_RUNTIME_DRY_RUN_TRANSITION_REJECTIONS",
} as const;

export type XXXLRuntimeDryRunFixtureId =
  (typeof XXXL_RUNTIME_DRY_RUN_FIXTURE_ID)[keyof typeof XXXL_RUNTIME_DRY_RUN_FIXTURE_ID];

export const XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR = {
  DuplicateFixture: "DUPLICATE_FIXTURE",
  MissingFixture: "MISSING_FIXTURE",
  EmptyFixture: "EMPTY_FIXTURE",
  DuplicateFixtureVector: "DUPLICATE_FIXTURE_VECTOR",
  MissingScenario: "MISSING_SCENARIO",
  MissingExecutionVector: "MISSING_EXECUTION_VECTOR",
  ExecutionVectorValidationFailed: "EXECUTION_VECTOR_VALIDATION_FAILED",
  ExecutionMismatch: "EXECUTION_MISMATCH",
} as const;

export type XXXLRuntimeDryRunFixtureErrorCode =
  (typeof XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR)[keyof typeof XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR];

export type XXXLRuntimeDryRunFixture = {
  readonly version: typeof XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION;
  readonly fixtureId: XXXLRuntimeDryRunFixtureId;
  readonly description: string;
  readonly vectorIds: readonly XXXLRuntimeExecutionVectorId[];
};

export type XXXLRuntimeDryRunVectorReport = {
  readonly vectorId: XXXLRuntimeExecutionVectorId;
  readonly description: string;
  readonly ok: boolean;
  readonly scenarioFound: boolean;
  readonly executionVectorFound: boolean;
  readonly matchedExecutionVector: boolean;
  readonly expectedOk: boolean;
  readonly actualOk: boolean;
  readonly expectedExecuted: boolean;
  readonly actualExecuted: boolean;
  readonly expectedErrors: readonly XXXLRuntimeProgramSkeletonErrorCode[];
  readonly actualErrors: readonly XXXLRuntimeProgramSkeletonErrorCode[];
  readonly expectedSteps: readonly XXXLRuntimeProgramSkeletonStep[];
  readonly actualSteps: readonly XXXLRuntimeProgramSkeletonStep[];
  readonly cpiSkipped: boolean;
  readonly supplyAuditOk: boolean;
};

export type XXXLRuntimeDryRunReport = {
  readonly version: typeof XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION;
  readonly fixtureId: XXXLRuntimeDryRunFixtureId;
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimeDryRunFixtureErrorCode[];
  readonly requestedVectorIds: readonly XXXLRuntimeExecutionVectorId[];
  readonly vectorValidation: XXXLRuntimeExecutionVectorValidationResult;
  readonly reports: readonly XXXLRuntimeDryRunVectorReport[];
};

export function xxxlRuntimeDryRunFixtures(): readonly XXXLRuntimeDryRunFixture[] {
  return [
    {
      version: XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION,
      fixtureId: XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.AllExecutionVectors,
      description:
        "Full runtime dry-run fixture covering all deterministic execution vectors.",
      vectorIds: Object.values(XXXL_RUNTIME_EXECUTION_VECTOR_ID),
    },
    {
      version: XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION,
      fixtureId: XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.SuccessfulRoutes,
      description:
        "Successful route-aware executions: Ethereum primary and Avalanche low-weight candidate.",
      vectorIds: [
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
      ],
    },
    {
      version: XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION,
      fixtureId: XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.PreflightRejections,
      description:
        "Preflight rejection dry-run fixture before transition execution.",
      vectorIds: [
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.InvalidRoutePolicyRejected,
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.MissingRouteRejected,
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.InstructionSerializationRejected,
      ],
    },
    {
      version: XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION,
      fixtureId: XXXL_RUNTIME_DRY_RUN_FIXTURE_ID.TransitionRejections,
      description:
        "Transition rejection dry-run fixture after preflight validation.",
      vectorIds: [
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.Stage1AuthorizationRejected,
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
        XXXL_RUNTIME_EXECUTION_VECTOR_ID.EventKeyMismatchRejected,
      ],
    },
  ];
}

export function validateXXXLRuntimeDryRunFixtures(
  fixtures: readonly XXXLRuntimeDryRunFixture[],
): readonly XXXLRuntimeDryRunFixtureErrorCode[] {
  const errors: XXXLRuntimeDryRunFixtureErrorCode[] = [];
  const fixtureIds = new Set<XXXLRuntimeDryRunFixtureId>();

  for (const fixture of fixtures) {
    if (fixtureIds.has(fixture.fixtureId)) {
      errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.DuplicateFixture);
    }
    fixtureIds.add(fixture.fixtureId);

    if (fixture.vectorIds.length === 0) {
      errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.EmptyFixture);
    }

    if (hasDuplicates(fixture.vectorIds)) {
      errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.DuplicateFixtureVector);
    }
  }

  for (const fixtureId of Object.values(XXXL_RUNTIME_DRY_RUN_FIXTURE_ID)) {
    if (!fixtureIds.has(fixtureId)) {
      errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.MissingFixture);
    }
  }

  return errors;
}

export function executeXXXLRuntimeDryRunFixture(
  fixture: XXXLRuntimeDryRunFixture,
  options: {
    readonly scenarios?: readonly XXXLRuntimeExecutionVectorScenario[];
    readonly executionVectors?: readonly XXXLRuntimeExecutionVector[];
  } = {},
): XXXLRuntimeDryRunReport {
  const scenarios =
    options.scenarios ?? xxxlRuntimeExecutionVectorScenarios();
  const executionVectors =
    options.executionVectors ?? xxxlRuntimeExecutionVectors();

  const vectorValidation =
    validateXXXLRuntimeExecutionVectors(executionVectors);

  const errors: XXXLRuntimeDryRunFixtureErrorCode[] = [];

  if (!vectorValidation.ok) {
    errors.push(
      XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.ExecutionVectorValidationFailed,
    );
  }

  if (fixture.vectorIds.length === 0) {
    errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.EmptyFixture);
  }

  if (hasDuplicates(fixture.vectorIds)) {
    errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.DuplicateFixtureVector);
  }

  const scenarioById = new Map<
    XXXLRuntimeExecutionVectorId,
    XXXLRuntimeExecutionVectorScenario
  >();
  const vectorById = new Map<
    XXXLRuntimeExecutionVectorId,
    XXXLRuntimeExecutionVector
  >();

  for (const scenario of scenarios) {
    scenarioById.set(scenario.vectorId, scenario);
  }

  for (const vector of executionVectors) {
    vectorById.set(vector.vectorId, vector);
  }

  const reports: XXXLRuntimeDryRunVectorReport[] = [];

  for (const vectorId of fixture.vectorIds) {
    const scenario = scenarioById.get(vectorId);
    const vector = vectorById.get(vectorId);

    if (!scenario) {
      errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.MissingScenario);
    }

    if (!vector) {
      errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.MissingExecutionVector);
    }

    if (!scenario || !vector) {
      reports.push(missingVectorReport(vectorId, scenario, vector));
      continue;
    }

    const result = executeXXXLRuntimeProgramSkeleton(scenario.input);

    const matchedExecutionVector =
      result.ok === vector.actualOk &&
      result.executed === vector.actualExecuted &&
      result.cpiStep.skipped === vector.actualCpiSkipped &&
      result.supplyAudit.ok === vector.actualSupplyAuditOk &&
      sameStrings(result.errors, vector.actualErrors) &&
      sameStrings(result.steps, vector.actualSteps);

    if (!matchedExecutionVector) {
      errors.push(XXXL_RUNTIME_DRY_RUN_FIXTURE_ERROR.ExecutionMismatch);
    }

    reports.push({
      vectorId,
      description: scenario.description,
      ok: matchedExecutionVector,
      scenarioFound: true,
      executionVectorFound: true,
      matchedExecutionVector,
      expectedOk: scenario.expectedOk,
      actualOk: result.ok,
      expectedExecuted: scenario.expectedExecuted,
      actualExecuted: result.executed,
      expectedErrors: scenario.expectedErrors,
      actualErrors: result.errors,
      expectedSteps: scenario.expectedSteps,
      actualSteps: result.steps,
      cpiSkipped: result.cpiStep.skipped,
      supplyAuditOk: result.supplyAudit.ok,
    });
  }

  const ok =
    vectorValidation.ok &&
    errors.length === 0 &&
    reports.every((report) => report.ok);

  return {
    version: XXXL_RUNTIME_DRY_RUN_FIXTURE_VERSION,
    fixtureId: fixture.fixtureId,
    ok,
    errors,
    requestedVectorIds: fixture.vectorIds,
    vectorValidation,
    reports,
  };
}

export function executeXXXLRuntimeDryRunFixtureSet(
  fixtures: readonly XXXLRuntimeDryRunFixture[] = xxxlRuntimeDryRunFixtures(),
): readonly XXXLRuntimeDryRunReport[] {
  return fixtures.map((fixture) => executeXXXLRuntimeDryRunFixture(fixture));
}

function missingVectorReport(
  vectorId: XXXLRuntimeExecutionVectorId,
  scenario: XXXLRuntimeExecutionVectorScenario | undefined,
  vector: XXXLRuntimeExecutionVector | undefined,
): XXXLRuntimeDryRunVectorReport {
  return {
    vectorId,
    description:
      scenario?.description ?? vector?.description ?? "Missing dry-run vector input.",
    ok: false,
    scenarioFound: scenario !== undefined,
    executionVectorFound: vector !== undefined,
    matchedExecutionVector: false,
    expectedOk: scenario?.expectedOk ?? false,
    actualOk: vector?.actualOk ?? false,
    expectedExecuted: scenario?.expectedExecuted ?? false,
    actualExecuted: vector?.actualExecuted ?? false,
    expectedErrors: scenario?.expectedErrors ?? [],
    actualErrors: vector?.actualErrors ?? [],
    expectedSteps: scenario?.expectedSteps ?? [],
    actualSteps: vector?.actualSteps ?? [],
    cpiSkipped: vector?.actualCpiSkipped ?? true,
    supplyAuditOk: vector?.actualSupplyAuditOk ?? false,
  };
}

function hasDuplicates(items: readonly string[]): boolean {
  return new Set(items).size !== items.length;
}

function sameStrings(left: readonly string[], right: readonly string[]): boolean {
  return (
    left.length === right.length &&
    left.every((item, index) => item === right[index])
  );
}

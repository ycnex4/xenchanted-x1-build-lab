export const XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_VERSION = 1;

export const XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_STATUS = {
  PlanOnlyNotLiveRoute: "ATOMIC_EXECUTION_PLAN_FIXTURE_ONLY_NOT_LIVE_ROUTE",
} as const;

export const XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR = {
  WrongStatus: "WRONG_STATUS",
  WrongStepOrder: "WRONG_STEP_ORDER",
  MissingCpiPreparation: "MISSING_CPI_PREPARATION",
  MissingProcessedEventMutation: "MISSING_PROCESSED_EVENT_MUTATION",
  MissingRecipientBalanceMutation: "MISSING_RECIPIENT_BALANCE_MUTATION",
  MissingAtomicPrecheck: "MISSING_ATOMIC_PRECHECK",
  LiveRouteActivated: "LIVE_ROUTE_ACTIVATED",
  MissingNonGoal: "MISSING_NON_GOAL",
} as const;

export type XXXLAtomicExecutionPlanStep =
  | "VALIDATE_AND_PREPARE_CPI"
  | "MARK_PROCESSED_EVENT_CONSUMED"
  | "CREDIT_RECIPIENT_BALANCE"
  | "KEEP_LIVE_ROUTE_DISABLED";

export type XXXLAtomicExecutionPlanFixtureErrorCode =
  (typeof XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR)[keyof typeof XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR];

export type XXXLAtomicExecutionPlanFixture = {
  readonly version: typeof XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_VERSION;
  readonly status: typeof XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_STATUS.PlanOnlyNotLiveRoute;
  readonly rustModule: "programs/xxxl-svm/src/execution_plan.rs";
  readonly stepOrder: readonly XXXLAtomicExecutionPlanStep[];
  readonly atomicPrechecks: {
    readonly rejectReplayBeforeCredit: true;
    readonly rejectBalanceOverflowBeforeProcessedMark: true;
    readonly rejectWrongRecipientBalanceOwnerBeforeMutation: true;
    readonly rejectPreparedCpiAmountMismatch: true;
  };
  readonly preparedBoundaries: {
    readonly cpiBoundaryPrepared: true;
    readonly processedEventMutationPrepared: true;
    readonly recipientBalanceMutationPrepared: true;
  };
  readonly liveExecution: {
    readonly processInstructionCallsMintToCpi: false;
    readonly processInstructionMarksProcessedEvent: false;
    readonly processInstructionCreditsRecipientBalance: false;
    readonly routeActivationEnabled: false;
  };
  readonly guarantees: readonly string[];
  readonly nonGoals: readonly string[];
};

export type XXXLAtomicExecutionPlanFixtureValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLAtomicExecutionPlanFixtureErrorCode[];
};

export function xxxlAtomicExecutionPlanFixture(): XXXLAtomicExecutionPlanFixture {
  return {
    version: XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_VERSION,
    status: XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_STATUS.PlanOnlyNotLiveRoute,
    rustModule: "programs/xxxl-svm/src/execution_plan.rs",
    stepOrder: [
      "VALIDATE_AND_PREPARE_CPI",
      "MARK_PROCESSED_EVENT_CONSUMED",
      "CREDIT_RECIPIENT_BALANCE",
      "KEEP_LIVE_ROUTE_DISABLED",
    ],
    atomicPrechecks: {
      rejectReplayBeforeCredit: true,
      rejectBalanceOverflowBeforeProcessedMark: true,
      rejectWrongRecipientBalanceOwnerBeforeMutation: true,
      rejectPreparedCpiAmountMismatch: true,
    },
    preparedBoundaries: {
      cpiBoundaryPrepared: true,
      processedEventMutationPrepared: true,
      recipientBalanceMutationPrepared: true,
    },
    liveExecution: {
      processInstructionCallsMintToCpi: false,
      processInstructionMarksProcessedEvent: false,
      processInstructionCreditsRecipientBalance: false,
      routeActivationEnabled: false,
    },
    guarantees: [
      "ATOMIC_STEP_ORDER_IS_FIXED",
      "CPI_PREPARATION_IS_BOUND_TO_STATE_MUTATION_PLAN",
      "REPLAY_IS_REJECTED_BEFORE_RECIPIENT_CREDIT",
      "BALANCE_OVERFLOW_IS_REJECTED_BEFORE_PROCESSED_EVENT_MARK",
      "LIVE_ROUTE_EXECUTION_REMAINS_DISABLED",
    ],
    nonGoals: [
      "NO_LIVE_ROUTE_ACTIVATION",
      "NO_MINT_TO_INVOCATION_FROM_PROCESS_INSTRUCTION",
      "NO_PROCESS_INSTRUCTION_PROCESSED_EVENT_MUTATION",
      "NO_PROCESS_INSTRUCTION_RECIPIENT_BALANCE_MUTATION",
      "NO_DEPLOYMENT",
      "NO_AUTHORITY_FREEZE_EXECUTION",
    ],
  };
}

export function validateXXXLAtomicExecutionPlanFixture(
  fixture: XXXLAtomicExecutionPlanFixture = xxxlAtomicExecutionPlanFixture(),
): XXXLAtomicExecutionPlanFixtureValidationResult {
  const errors: XXXLAtomicExecutionPlanFixtureErrorCode[] = [];

  const expectedStepOrder: readonly XXXLAtomicExecutionPlanStep[] = [
    "VALIDATE_AND_PREPARE_CPI",
    "MARK_PROCESSED_EVENT_CONSUMED",
    "CREDIT_RECIPIENT_BALANCE",
    "KEEP_LIVE_ROUTE_DISABLED",
  ];

  if (
    fixture.status !==
    XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_STATUS.PlanOnlyNotLiveRoute
  ) {
    errors.push(XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.WrongStatus);
  }

  if (fixture.stepOrder.join(">") !== expectedStepOrder.join(">")) {
    errors.push(XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.WrongStepOrder);
  }

  if (!fixture.preparedBoundaries.cpiBoundaryPrepared) {
    errors.push(XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.MissingCpiPreparation);
  }

  if (!fixture.preparedBoundaries.processedEventMutationPrepared) {
    errors.push(
      XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.MissingProcessedEventMutation,
    );
  }

  if (!fixture.preparedBoundaries.recipientBalanceMutationPrepared) {
    errors.push(
      XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.MissingRecipientBalanceMutation,
    );
  }

  if (
    !fixture.atomicPrechecks.rejectReplayBeforeCredit ||
    !fixture.atomicPrechecks.rejectBalanceOverflowBeforeProcessedMark ||
    !fixture.atomicPrechecks.rejectWrongRecipientBalanceOwnerBeforeMutation ||
    !fixture.atomicPrechecks.rejectPreparedCpiAmountMismatch
  ) {
    errors.push(XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.MissingAtomicPrecheck);
  }

  if (
    fixture.liveExecution.processInstructionCallsMintToCpi ||
    fixture.liveExecution.processInstructionMarksProcessedEvent ||
    fixture.liveExecution.processInstructionCreditsRecipientBalance ||
    fixture.liveExecution.routeActivationEnabled
  ) {
    errors.push(XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.LiveRouteActivated);
  }

  for (const nonGoal of [
    "NO_LIVE_ROUTE_ACTIVATION",
    "NO_MINT_TO_INVOCATION_FROM_PROCESS_INSTRUCTION",
    "NO_DEPLOYMENT",
  ]) {
    if (!fixture.nonGoals.includes(nonGoal)) {
      errors.push(XXXL_ATOMIC_EXECUTION_PLAN_FIXTURE_ERROR.MissingNonGoal);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalAtomicExecutionPlanFixtureJson(
  fixture: XXXLAtomicExecutionPlanFixture = xxxlAtomicExecutionPlanFixture(),
): string {
  return JSON.stringify([
    ["version", fixture.version],
    ["status", fixture.status],
    ["rustModule", fixture.rustModule],
    ["stepOrder", fixture.stepOrder],
    ["atomicPrechecks", fixture.atomicPrechecks],
    ["preparedBoundaries", fixture.preparedBoundaries],
    ["liveExecution", fixture.liveExecution],
    ["guarantees", fixture.guarantees],
    ["nonGoals", fixture.nonGoals],
  ]);
}

export function xxxlAtomicExecutionPlanFixtureMarkdown(
  fixture: XXXLAtomicExecutionPlanFixture = xxxlAtomicExecutionPlanFixture(),
): string {
  return [
    "# XXXL Atomic Execution Plan Fixture",
    "",
    `Status: ${fixture.status}`,
    "",
    "This fixture fixes the atomic execution-plan order without activating the live route.",
    "",
    "Step order:",
    ...fixture.stepOrder.map((step, index) => `${index + 1}. ${step}`),
    "",
    "Atomic prechecks:",
    `- rejectReplayBeforeCredit: ${fixture.atomicPrechecks.rejectReplayBeforeCredit}`,
    `- rejectBalanceOverflowBeforeProcessedMark: ${fixture.atomicPrechecks.rejectBalanceOverflowBeforeProcessedMark}`,
    `- rejectPreparedCpiAmountMismatch: ${fixture.atomicPrechecks.rejectPreparedCpiAmountMismatch}`,
    "",
    "Non-goals:",
    ...fixture.nonGoals.map((item) => `- ${item}`),
    "",
  ].join("\n");
}

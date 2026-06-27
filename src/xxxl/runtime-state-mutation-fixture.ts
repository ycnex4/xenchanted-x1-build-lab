export const XXXL_RUNTIME_STATE_MUTATION_FIXTURE_VERSION = 1;

export const XXXL_RUNTIME_STATE_MUTATION_FIXTURE_STATUS = {
  MutationFixtureOnly: "RUNTIME_STATE_MUTATION_FIXTURE_ONLY_NOT_LIVE_ROUTE",
} as const;

export const XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR = {
  WrongStatus: "WRONG_STATUS",
  MissingProcessedEventMutation: "MISSING_PROCESSED_EVENT_MUTATION",
  MissingRecipientBalanceMutation: "MISSING_RECIPIENT_BALANCE_MUTATION",
  MissingReplayProtection: "MISSING_REPLAY_PROTECTION",
  MissingOverflowProtection: "MISSING_OVERFLOW_PROTECTION",
  LiveRouteActivated: "LIVE_ROUTE_ACTIVATED",
  MissingNonGoal: "MISSING_NON_GOAL",
} as const;

export type XXXLRuntimeStateMutationFixtureErrorCode =
  (typeof XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR)[keyof typeof XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR];

export type XXXLRuntimeStateMutationFixture = {
  readonly version: typeof XXXL_RUNTIME_STATE_MUTATION_FIXTURE_VERSION;
  readonly status: typeof XXXL_RUNTIME_STATE_MUTATION_FIXTURE_STATUS.MutationFixtureOnly;
  readonly rustModule: "programs/xxxl-svm/src/state.rs";
  readonly processedEventMutation: {
    readonly function: "mark_processed_event_consumed";
    readonly writesConsumedFlag: true;
    readonly writesConsumedAmount: true;
    readonly writesConsumedSlot: true;
    readonly rejectsReplay: true;
    readonly requiresCanonicalEventKeyMatch: true;
    readonly requiresRouteMatch: true;
    readonly requiresRecipientMatch: true;
  };
  readonly recipientBalanceMutation: {
    readonly function: "credit_recipient_balance";
    readonly usesCheckedAdd: true;
    readonly writesBalance: true;
    readonly writesLastCanonicalEventKey: true;
    readonly requiresOwnerMatch: true;
    readonly requiresMintMatch: true;
  };
  readonly liveExecution: {
    readonly processInstructionMutatesProcessedEvent: false;
    readonly processInstructionMutatesRecipientBalance: false;
    readonly routeActivationEnabled: false;
    readonly mintToInvocationEnabled: false;
  };
  readonly guarantees: readonly string[];
  readonly nonGoals: readonly string[];
};

export type XXXLRuntimeStateMutationFixtureValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimeStateMutationFixtureErrorCode[];
};

export function xxxlRuntimeStateMutationFixture(): XXXLRuntimeStateMutationFixture {
  return {
    version: XXXL_RUNTIME_STATE_MUTATION_FIXTURE_VERSION,
    status: XXXL_RUNTIME_STATE_MUTATION_FIXTURE_STATUS.MutationFixtureOnly,
    rustModule: "programs/xxxl-svm/src/state.rs",
    processedEventMutation: {
      function: "mark_processed_event_consumed",
      writesConsumedFlag: true,
      writesConsumedAmount: true,
      writesConsumedSlot: true,
      rejectsReplay: true,
      requiresCanonicalEventKeyMatch: true,
      requiresRouteMatch: true,
      requiresRecipientMatch: true,
    },
    recipientBalanceMutation: {
      function: "credit_recipient_balance",
      usesCheckedAdd: true,
      writesBalance: true,
      writesLastCanonicalEventKey: true,
      requiresOwnerMatch: true,
      requiresMintMatch: true,
    },
    liveExecution: {
      processInstructionMutatesProcessedEvent: false,
      processInstructionMutatesRecipientBalance: false,
      routeActivationEnabled: false,
      mintToInvocationEnabled: false,
    },
    guarantees: [
      "PROCESSED_EVENT_MUTATION_IS_DETERMINISTIC",
      "PROCESSED_EVENT_REPLAY_IS_REJECTED",
      "RECIPIENT_BALANCE_CREDIT_USES_CHECKED_ADD",
      "LAST_CANONICAL_EVENT_KEY_IS_WRITTEN",
      "LIVE_HANDLER_ROUTE_EXECUTION_REMAINS_DISABLED",
    ],
    nonGoals: [
      "NO_LIVE_ROUTE_ACTIVATION",
      "NO_MINT_TO_INVOCATION_FROM_HANDLER",
      "NO_PROCESS_INSTRUCTION_STATE_MUTATION",
      "NO_DEPLOYMENT",
      "NO_AUTHORITY_FREEZE_EXECUTION",
    ],
  };
}

export function validateXXXLRuntimeStateMutationFixture(
  fixture: XXXLRuntimeStateMutationFixture = xxxlRuntimeStateMutationFixture(),
): XXXLRuntimeStateMutationFixtureValidationResult {
  const errors: XXXLRuntimeStateMutationFixtureErrorCode[] = [];

  if (
    fixture.status !==
    XXXL_RUNTIME_STATE_MUTATION_FIXTURE_STATUS.MutationFixtureOnly
  ) {
    errors.push(XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.WrongStatus);
  }

  if (
    fixture.processedEventMutation.function !== "mark_processed_event_consumed" ||
    !fixture.processedEventMutation.writesConsumedFlag ||
    !fixture.processedEventMutation.writesConsumedAmount ||
    !fixture.processedEventMutation.writesConsumedSlot
  ) {
    errors.push(
      XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingProcessedEventMutation,
    );
  }

  if (
    !fixture.processedEventMutation.rejectsReplay ||
    !fixture.processedEventMutation.requiresCanonicalEventKeyMatch
  ) {
    errors.push(XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingReplayProtection);
  }

  if (
    fixture.recipientBalanceMutation.function !== "credit_recipient_balance" ||
    !fixture.recipientBalanceMutation.writesBalance ||
    !fixture.recipientBalanceMutation.writesLastCanonicalEventKey
  ) {
    errors.push(
      XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingRecipientBalanceMutation,
    );
  }

  if (!fixture.recipientBalanceMutation.usesCheckedAdd) {
    errors.push(XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingOverflowProtection);
  }

  if (
    fixture.liveExecution.processInstructionMutatesProcessedEvent ||
    fixture.liveExecution.processInstructionMutatesRecipientBalance ||
    fixture.liveExecution.routeActivationEnabled ||
    fixture.liveExecution.mintToInvocationEnabled
  ) {
    errors.push(XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.LiveRouteActivated);
  }

  for (const nonGoal of [
    "NO_LIVE_ROUTE_ACTIVATION",
    "NO_MINT_TO_INVOCATION_FROM_HANDLER",
    "NO_DEPLOYMENT",
  ]) {
    if (!fixture.nonGoals.includes(nonGoal)) {
      errors.push(XXXL_RUNTIME_STATE_MUTATION_FIXTURE_ERROR.MissingNonGoal);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalRuntimeStateMutationFixtureJson(
  fixture: XXXLRuntimeStateMutationFixture = xxxlRuntimeStateMutationFixture(),
): string {
  return JSON.stringify([
    ["version", fixture.version],
    ["status", fixture.status],
    ["rustModule", fixture.rustModule],
    ["processedEventMutation", fixture.processedEventMutation],
    ["recipientBalanceMutation", fixture.recipientBalanceMutation],
    ["liveExecution", fixture.liveExecution],
    ["guarantees", fixture.guarantees],
    ["nonGoals", fixture.nonGoals],
  ]);
}

export function xxxlRuntimeStateMutationFixtureMarkdown(
  fixture: XXXLRuntimeStateMutationFixture = xxxlRuntimeStateMutationFixture(),
): string {
  return [
    "# XXXL Runtime State Mutation Fixture",
    "",
    `Status: ${fixture.status}`,
    "",
    "This fixture adds deterministic state mutation helpers without activating the live route.",
    "",
    "ProcessedEvent mutation:",
    `- function: ${fixture.processedEventMutation.function}`,
    "- writes consumed flag, amount, and slot",
    "- rejects replay and mismatched canonical event keys",
    "",
    "RecipientBalance mutation:",
    `- function: ${fixture.recipientBalanceMutation.function}`,
    "- uses checked_add",
    "- writes balance and lastCanonicalEventKey",
    "",
    "Non-goals:",
    ...fixture.nonGoals.map((item) => `- ${item}`),
    "",
  ].join("\n");
}

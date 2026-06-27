export const XXXL_HANDLER_INTEGRATION_FIXTURE_VERSION = 1;

export const XXXL_HANDLER_INTEGRATION_FIXTURE_STATUS = {
  PreparedNotLive: "HANDLER_INTEGRATION_FIXTURE_PREPARED_NOT_LIVE_ROUTE",
} as const;

export const XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR = {
  WrongStatus: "WRONG_STATUS",
  MissingDecodeBoundary: "MISSING_DECODE_BOUNDARY",
  MissingAccountViewBoundary: "MISSING_ACCOUNT_VIEW_BOUNDARY",
  MissingValidationBoundary: "MISSING_VALIDATION_BOUNDARY",
  MissingCpiPreparationBoundary: "MISSING_CPI_PREPARATION_BOUNDARY",
  LiveRouteActivated: "LIVE_ROUTE_ACTIVATED",
  MissingNonGoal: "MISSING_NON_GOAL",
} as const;

export type XXXLHandlerIntegrationFixtureErrorCode =
  (typeof XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR)[keyof typeof XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR];

export type XXXLHandlerIntegrationFlowStep =
  | "DECODE_CONSUME_GATEWAY_MINT_INSTRUCTION"
  | "LOAD_CANONICAL_ACCOUNT_INDEXES"
  | "PARSE_RUNTIME_ACCOUNT_VIEWS"
  | "RUN_OWNER_AND_RENT_CHECKS"
  | "RUN_MINT_AND_RECIPIENT_TOKEN_VALIDATION"
  | "VERIFY_GATEWAY_MINT_AUTHORITY_PDA_AND_BUMP"
  | "PREPARE_MINT_TO_CPI_BOUNDARY";

export type XXXLHandlerIntegrationFixture = {
  readonly version: typeof XXXL_HANDLER_INTEGRATION_FIXTURE_VERSION;
  readonly status: typeof XXXL_HANDLER_INTEGRATION_FIXTURE_STATUS.PreparedNotLive;
  readonly rustModule: "programs/xxxl-svm/src/processor.rs";
  readonly flow: readonly XXXLHandlerIntegrationFlowStep[];
  readonly preparedBoundary: {
    readonly tokenProgram: "SPL_TOKEN_PROGRAM";
    readonly mint: "XXXL_SPL_MINT";
    readonly recipientTokenAccount: "RECIPIENT_TOKEN_ACCOUNT";
    readonly mintAuthorityPda: "GATEWAY_MINT_AUTHORITY_PDA";
    readonly amountType: "u64";
  };
  readonly liveExecution: {
    readonly processInstructionCallsMintToCpi: false;
    readonly routeActivationEnabled: false;
    readonly processedEventMutationEnabled: false;
    readonly recipientBalanceMutationEnabled: false;
  };
  readonly guarantees: readonly string[];
  readonly nonGoals: readonly string[];
};

export type XXXLHandlerIntegrationFixtureValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLHandlerIntegrationFixtureErrorCode[];
};

export function xxxlHandlerIntegrationFixture(): XXXLHandlerIntegrationFixture {
  return {
    version: XXXL_HANDLER_INTEGRATION_FIXTURE_VERSION,
    status: XXXL_HANDLER_INTEGRATION_FIXTURE_STATUS.PreparedNotLive,
    rustModule: "programs/xxxl-svm/src/processor.rs",
    flow: [
      "DECODE_CONSUME_GATEWAY_MINT_INSTRUCTION",
      "LOAD_CANONICAL_ACCOUNT_INDEXES",
      "PARSE_RUNTIME_ACCOUNT_VIEWS",
      "RUN_OWNER_AND_RENT_CHECKS",
      "RUN_MINT_AND_RECIPIENT_TOKEN_VALIDATION",
      "VERIFY_GATEWAY_MINT_AUTHORITY_PDA_AND_BUMP",
      "PREPARE_MINT_TO_CPI_BOUNDARY",
    ],
    preparedBoundary: {
      tokenProgram: "SPL_TOKEN_PROGRAM",
      mint: "XXXL_SPL_MINT",
      recipientTokenAccount: "RECIPIENT_TOKEN_ACCOUNT",
      mintAuthorityPda: "GATEWAY_MINT_AUTHORITY_PDA",
      amountType: "u64",
    },
    liveExecution: {
      processInstructionCallsMintToCpi: false,
      routeActivationEnabled: false,
      processedEventMutationEnabled: false,
      recipientBalanceMutationEnabled: false,
    },
    guarantees: [
      "DECODED_INSTRUCTION_CAN_PREPARE_CPI_BOUNDARY",
      "ACCOUNT_VIEWS_ARE_CONNECTED_TO_HANDLER_PREPARATION",
      "OWNER_RENT_MINT_AND_RECIPIENT_TOKEN_CHECKS_RUN_BEFORE_CPI_PREPARATION",
      "PDA_AND_BUMP_ARE_VERIFIED_BEFORE_CPI_PREPARATION",
      "PROCESS_INSTRUCTION_REMAINS_SCAFFOLD_ONLY",
    ],
    nonGoals: [
      "NO_LIVE_MINT_TO_INVOCATION_FROM_HANDLER",
      "NO_ROUTE_ACTIVATION",
      "NO_PROCESSED_EVENT_MUTATION",
      "NO_RECIPIENT_BALANCE_MUTATION",
      "NO_DEPLOYMENT",
      "NO_AUTHORITY_FREEZE_EXECUTION",
    ],
  };
}

export function validateXXXLHandlerIntegrationFixture(
  fixture: XXXLHandlerIntegrationFixture = xxxlHandlerIntegrationFixture(),
): XXXLHandlerIntegrationFixtureValidationResult {
  const errors: XXXLHandlerIntegrationFixtureErrorCode[] = [];

  if (fixture.status !== XXXL_HANDLER_INTEGRATION_FIXTURE_STATUS.PreparedNotLive) {
    errors.push(XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.WrongStatus);
  }

  if (!fixture.flow.includes("DECODE_CONSUME_GATEWAY_MINT_INSTRUCTION")) {
    errors.push(XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.MissingDecodeBoundary);
  }

  if (!fixture.flow.includes("PARSE_RUNTIME_ACCOUNT_VIEWS")) {
    errors.push(XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.MissingAccountViewBoundary);
  }

  if (
    !fixture.flow.includes("RUN_OWNER_AND_RENT_CHECKS") ||
    !fixture.flow.includes("RUN_MINT_AND_RECIPIENT_TOKEN_VALIDATION") ||
    !fixture.flow.includes("VERIFY_GATEWAY_MINT_AUTHORITY_PDA_AND_BUMP")
  ) {
    errors.push(XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.MissingValidationBoundary);
  }

  if (
    !fixture.flow.includes("PREPARE_MINT_TO_CPI_BOUNDARY") ||
    fixture.preparedBoundary.amountType !== "u64"
  ) {
    errors.push(XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.MissingCpiPreparationBoundary);
  }

  if (
    fixture.liveExecution.processInstructionCallsMintToCpi ||
    fixture.liveExecution.routeActivationEnabled ||
    fixture.liveExecution.processedEventMutationEnabled ||
    fixture.liveExecution.recipientBalanceMutationEnabled
  ) {
    errors.push(XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.LiveRouteActivated);
  }

  for (const nonGoal of [
    "NO_LIVE_MINT_TO_INVOCATION_FROM_HANDLER",
    "NO_ROUTE_ACTIVATION",
    "NO_DEPLOYMENT",
  ]) {
    if (!fixture.nonGoals.includes(nonGoal)) {
      errors.push(XXXL_HANDLER_INTEGRATION_FIXTURE_ERROR.MissingNonGoal);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalHandlerIntegrationFixtureJson(
  fixture: XXXLHandlerIntegrationFixture = xxxlHandlerIntegrationFixture(),
): string {
  return JSON.stringify([
    ["version", fixture.version],
    ["status", fixture.status],
    ["rustModule", fixture.rustModule],
    ["flow", fixture.flow],
    ["preparedBoundary", fixture.preparedBoundary],
    ["liveExecution", fixture.liveExecution],
    ["guarantees", fixture.guarantees],
    ["nonGoals", fixture.nonGoals],
  ]);
}

export function xxxlHandlerIntegrationFixtureMarkdown(
  fixture: XXXLHandlerIntegrationFixture = xxxlHandlerIntegrationFixture(),
): string {
  return [
    "# XXXL Handler Integration Fixture",
    "",
    `Status: ${fixture.status}`,
    "",
    "This fixture connects decode, account views, validation, and CPI boundary preparation without live route activation.",
    "",
    "Flow:",
    ...fixture.flow.map((item) => `- ${item}`),
    "",
    "Live execution flags:",
    `- processInstructionCallsMintToCpi: ${fixture.liveExecution.processInstructionCallsMintToCpi}`,
    `- routeActivationEnabled: ${fixture.liveExecution.routeActivationEnabled}`,
    "",
    "Non-goals:",
    ...fixture.nonGoals.map((item) => `- ${item}`),
    "",
  ].join("\n");
}

import {
  XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE,
  XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE,
} from "./runtime-production-byte-layout.js";

export const XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_VERSION = 1;

export const XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_STATUS = {
  RustDecodeFixtureOnly: "RUST_DECODE_FIXTURE_ONLY_NOT_DEPLOYABLE",
} as const;

export const XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR = {
  WrongStatus: "WRONG_STATUS",
  MissingRustModule: "MISSING_RUST_MODULE",
  WrongInstructionLength: "WRONG_INSTRUCTION_LENGTH",
  WrongAccountLength: "WRONG_ACCOUNT_LENGTH",
  MissingDiscriminatorCheck: "MISSING_DISCRIMINATOR_CHECK",
  MissingVersionCheck: "MISSING_VERSION_CHECK",
  MissingNegativeTest: "MISSING_NEGATIVE_TEST",
  MissingNonGoal: "MISSING_NON_GOAL",
} as const;

export type XXXLRuntimeAccountInstructionDecodeFixtureErrorCode =
  (typeof XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR)[keyof typeof XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR];

export type XXXLRuntimeAccountDecodeFixture = {
  readonly kind: string;
  readonly rustView: string;
  readonly byteLength: number;
  readonly discriminatorHex: string;
  readonly version: 1;
};

export type XXXLRuntimeAccountInstructionDecodeFixture = {
  readonly version: typeof XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_VERSION;
  readonly status: typeof XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_STATUS.RustDecodeFixtureOnly;
  readonly rustModules: {
    readonly instruction: "programs/xxxl-svm/src/instruction.rs";
    readonly state: "programs/xxxl-svm/src/state.rs";
  };
  readonly instruction: {
    readonly handler: "consume_gateway_mint";
    readonly byteLength: number;
    readonly discriminatorHex: "f2f4a868bb89fe52";
    readonly version: 1;
    readonly accountMetaCount: 9;
    readonly parsedFields: readonly string[];
  };
  readonly accounts: readonly XXXLRuntimeAccountDecodeFixture[];
  readonly requiredNegativeRustTests: readonly string[];
  readonly guarantees: readonly string[];
  readonly nonGoals: readonly string[];
};

export type XXXLRuntimeAccountInstructionDecodeFixtureValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLRuntimeAccountInstructionDecodeFixtureErrorCode[];
};

export function xxxlRuntimeAccountInstructionDecodeFixture(): XXXLRuntimeAccountInstructionDecodeFixture {
  return {
    version: XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_VERSION,
    status:
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_STATUS.RustDecodeFixtureOnly,
    rustModules: {
      instruction: "programs/xxxl-svm/src/instruction.rs",
      state: "programs/xxxl-svm/src/state.rs",
    },
    instruction: {
      handler: "consume_gateway_mint",
      byteLength:
        XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE.ConsumeGatewayMint,
      discriminatorHex: "f2f4a868bb89fe52",
      version: 1,
      accountMetaCount: 9,
      parsedFields: [
        "accountMetaCount",
        "routeAccountIndex",
        "guardianSetAccountIndex",
        "mintStateAccountIndex",
        "processedEventAccountIndex",
        "recipientBalanceAccountIndex",
        "routeId",
        "guardianSetId",
        "mintId",
        "canonicalEventKey",
        "recipient",
        "amount",
        "sourceChainWeightBps",
      ],
    },
    accounts: [
      {
        kind: "MINT_STATE_ACCOUNT",
        rustView: "MintStateAccountView",
        byteLength: XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.MintState,
        discriminatorHex: "18f0f49966906660",
        version: 1,
      },
      {
        kind: "GATEWAY_CONFIG_ACCOUNT",
        rustView: "GatewayConfigAccountView",
        byteLength:
          XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GatewayConfig,
        discriminatorHex: "a6120c7ed76902ae",
        version: 1,
      },
      {
        kind: "GUARDIAN_SET_ACCOUNT",
        rustView: "GuardianSetAccountView",
        byteLength:
          XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GuardianSet,
        discriminatorHex: "a6f6ef1aaec613ae",
        version: 1,
      },
      {
        kind: "PROCESSED_EVENT_ACCOUNT",
        rustView: "ProcessedEventAccountView",
        byteLength:
          XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.ProcessedEvent,
        discriminatorHex: "8f545b8140a2d5b5",
        version: 1,
      },
      {
        kind: "RECIPIENT_BALANCE_ACCOUNT",
        rustView: "RecipientBalanceAccountView",
        byteLength:
          XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.RecipientBalance,
        discriminatorHex: "b56386245014f5f4",
        version: 1,
      },
    ],
    requiredNegativeRustTests: [
      "WRONG_INSTRUCTION_LENGTH_REJECTED",
      "WRONG_INSTRUCTION_DISCRIMINATOR_REJECTED",
      "WRONG_INSTRUCTION_VERSION_REJECTED",
      "WRONG_ACCOUNT_DISCRIMINATOR_REJECTED",
      "WRONG_ACCOUNT_VERSION_REJECTED",
      "TRUNCATED_ACCOUNT_DATA_REJECTED",
    ],
    guarantees: [
      "RUST_CONSUME_GATEWAY_MINT_BYTES_ARE_PARSED_BEFORE_CPI",
      "RUST_ACCOUNT_VIEWS_CHECK_LENGTH_DISCRIMINATOR_AND_VERSION",
      "PRODUCTION_LAYOUT_SIZES_MATCH_TYPESCRIPT_MODEL",
      "DECODE_FAILURE_HAS_NO_STATE_CHANGE_BOUNDARY",
    ],
    nonGoals: [
      "NO_SPL_TOKEN_CPI_YET",
      "NO_DEPLOYMENT",
      "NO_ROUTE_ACTIVATION",
      "NO_AUTHORITY_FREEZE_EXECUTION",
    ],
  };
}

export function validateXXXLRuntimeAccountInstructionDecodeFixture(
  fixture: XXXLRuntimeAccountInstructionDecodeFixture =
    xxxlRuntimeAccountInstructionDecodeFixture(),
): XXXLRuntimeAccountInstructionDecodeFixtureValidationResult {
  const errors: XXXLRuntimeAccountInstructionDecodeFixtureErrorCode[] = [];

  if (
    fixture.status !==
    XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_STATUS.RustDecodeFixtureOnly
  ) {
    errors.push(XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.WrongStatus);
  }

  if (
    fixture.rustModules.instruction !==
      "programs/xxxl-svm/src/instruction.rs" ||
    fixture.rustModules.state !== "programs/xxxl-svm/src/state.rs"
  ) {
    errors.push(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.MissingRustModule,
    );
  }

  if (
    fixture.instruction.byteLength !==
    XXXL_PRODUCTION_RUNTIME_INSTRUCTION_LAYOUT_TOTAL_SIZE.ConsumeGatewayMint
  ) {
    errors.push(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.WrongInstructionLength,
    );
  }

  if (
    fixture.instruction.discriminatorHex.length !== 16 ||
    fixture.instruction.accountMetaCount !== 9 ||
    !fixture.instruction.parsedFields.includes("amount") ||
    !fixture.instruction.parsedFields.includes("sourceChainWeightBps")
  ) {
    errors.push(
      XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.MissingDiscriminatorCheck,
    );
  }

  const expectedAccountLengths = new Map([
    [
      "MINT_STATE_ACCOUNT",
      XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.MintState,
    ],
    [
      "GATEWAY_CONFIG_ACCOUNT",
      XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GatewayConfig,
    ],
    [
      "GUARDIAN_SET_ACCOUNT",
      XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.GuardianSet,
    ],
    [
      "PROCESSED_EVENT_ACCOUNT",
      XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.ProcessedEvent,
    ],
    [
      "RECIPIENT_BALANCE_ACCOUNT",
      XXXL_PRODUCTION_RUNTIME_ACCOUNT_LAYOUT_TOTAL_SIZE.RecipientBalance,
    ],
  ]);

  for (const account of fixture.accounts) {
    if (account.byteLength !== expectedAccountLengths.get(account.kind)) {
      errors.push(
        XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.WrongAccountLength,
      );
    }

    if (account.discriminatorHex.length !== 16) {
      errors.push(
        XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.MissingDiscriminatorCheck,
      );
    }

    if (account.version !== 1) {
      errors.push(
        XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.MissingVersionCheck,
      );
    }
  }

  for (const requiredTest of [
    "WRONG_INSTRUCTION_LENGTH_REJECTED",
    "WRONG_INSTRUCTION_DISCRIMINATOR_REJECTED",
    "WRONG_INSTRUCTION_VERSION_REJECTED",
    "WRONG_ACCOUNT_DISCRIMINATOR_REJECTED",
    "WRONG_ACCOUNT_VERSION_REJECTED",
    "TRUNCATED_ACCOUNT_DATA_REJECTED",
  ]) {
    if (!fixture.requiredNegativeRustTests.includes(requiredTest)) {
      errors.push(
        XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.MissingNegativeTest,
      );
    }
  }

  for (const nonGoal of [
    "NO_SPL_TOKEN_CPI_YET",
    "NO_DEPLOYMENT",
    "NO_ROUTE_ACTIVATION",
  ]) {
    if (!fixture.nonGoals.includes(nonGoal)) {
      errors.push(
        XXXL_RUNTIME_ACCOUNT_INSTRUCTION_DECODE_FIXTURE_ERROR.MissingNonGoal,
      );
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalRuntimeAccountInstructionDecodeFixtureJson(
  fixture: XXXLRuntimeAccountInstructionDecodeFixture =
    xxxlRuntimeAccountInstructionDecodeFixture(),
): string {
  return JSON.stringify([
    ["version", fixture.version],
    ["status", fixture.status],
    ["rustModules", fixture.rustModules],
    ["instruction", fixture.instruction],
    ["accounts", fixture.accounts],
    ["requiredNegativeRustTests", fixture.requiredNegativeRustTests],
    ["guarantees", fixture.guarantees],
    ["nonGoals", fixture.nonGoals],
  ]);
}

export function xxxlRuntimeAccountInstructionDecodeFixtureMarkdown(
  fixture: XXXLRuntimeAccountInstructionDecodeFixture =
    xxxlRuntimeAccountInstructionDecodeFixture(),
): string {
  return [
    "# XXXL Runtime Account/Instruction Decode Fixture",
    "",
    `Status: ${fixture.status}`,
    "",
    "This fixture hardens the Rust/SVM decode boundary before SPL Token CPI.",
    "",
    "Instruction:",
    `- handler: ${fixture.instruction.handler}`,
    `- byte length: ${fixture.instruction.byteLength}`,
    `- discriminator: ${fixture.instruction.discriminatorHex}`,
    `- account metas: ${fixture.instruction.accountMetaCount}`,
    "",
    "Accounts:",
    ...fixture.accounts.map(
      (account) =>
        `- ${account.kind}: ${account.rustView}, ${account.byteLength} bytes, discriminator ${account.discriminatorHex}`,
    ),
    "",
    "Guarantees:",
    ...fixture.guarantees.map((item) => `- ${item}`),
    "",
    "Non-goals:",
    ...fixture.nonGoals.map((item) => `- ${item}`),
    "",
  ].join("\n");
}

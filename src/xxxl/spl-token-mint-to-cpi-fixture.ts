export const XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_VERSION = 1;

export const XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_STATUS = {
  FixtureOnlyNotLiveHandler: "SPL_TOKEN_MINT_TO_CPI_FIXTURE_ONLY_NOT_LIVE_HANDLER",
} as const;

export const XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR = {
  WrongStatus: "WRONG_STATUS",
  MissingRustModule: "MISSING_RUST_MODULE",
  MissingMintTo: "MISSING_MINT_TO",
  MissingInvokeSigned: "MISSING_INVOKE_SIGNED",
  MissingPdaSigner: "MISSING_PDA_SIGNER",
  MissingOwnerCheck: "MISSING_OWNER_CHECK",
  MissingRentCheck: "MISSING_RENT_CHECK",
  MissingAtaValidation: "MISSING_ATA_VALIDATION",
  MissingNonGoal: "MISSING_NON_GOAL",
} as const;

export type XXXLSplTokenMintToCpiFixtureErrorCode =
  (typeof XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR)[keyof typeof XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR];

export type XXXLSplTokenMintToCpiFixture = {
  readonly version: typeof XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_VERSION;
  readonly status: typeof XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_STATUS.FixtureOnlyNotLiveHandler;
  readonly rustModules: {
    readonly cpi: "programs/xxxl-svm/src/cpi.rs";
    readonly validation: "programs/xxxl-svm/src/validation.rs";
    readonly pda: "programs/xxxl-svm/src/pda.rs";
    readonly processor: "programs/xxxl-svm/src/processor.rs";
  };
  readonly splTokenProgramId: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
  readonly cpiInstruction: {
    readonly function: "spl_token::instruction::mint_to";
    readonly invoke: "solana_program::program::invoke_signed";
    readonly amountType: "u64";
    readonly zeroAmountRejected: true;
  };
  readonly signer: {
    readonly pdaSeeds: readonly ["xxxl", "gateway-mint-authority", "v1"];
    readonly bumpRequired: true;
    readonly findProgramAddressRequired: true;
  };
  readonly accountChecks: {
    readonly mintOwnedBySplToken: true;
    readonly mintInitialized: true;
    readonly mintAuthorityMustMatchGatewayPda: true;
    readonly recipientTokenAccountOwnedBySplToken: true;
    readonly recipientTokenAccountInitialized: true;
    readonly recipientTokenAccountMintMustMatch: true;
    readonly recipientTokenAccountOwnerMustMatch: true;
    readonly rentExemptionHelperAvailable: true;
  };
  readonly guarantees: readonly string[];
  readonly nonGoals: readonly string[];
};

export type XXXLSplTokenMintToCpiFixtureValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLSplTokenMintToCpiFixtureErrorCode[];
};

export function xxxlSplTokenMintToCpiFixture(): XXXLSplTokenMintToCpiFixture {
  return {
    version: XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_VERSION,
    status:
      XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_STATUS.FixtureOnlyNotLiveHandler,
    rustModules: {
      cpi: "programs/xxxl-svm/src/cpi.rs",
      validation: "programs/xxxl-svm/src/validation.rs",
      pda: "programs/xxxl-svm/src/pda.rs",
      processor: "programs/xxxl-svm/src/processor.rs",
    },
    splTokenProgramId: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    cpiInstruction: {
      function: "spl_token::instruction::mint_to",
      invoke: "solana_program::program::invoke_signed",
      amountType: "u64",
      zeroAmountRejected: true,
    },
    signer: {
      pdaSeeds: ["xxxl", "gateway-mint-authority", "v1"],
      bumpRequired: true,
      findProgramAddressRequired: true,
    },
    accountChecks: {
      mintOwnedBySplToken: true,
      mintInitialized: true,
      mintAuthorityMustMatchGatewayPda: true,
      recipientTokenAccountOwnedBySplToken: true,
      recipientTokenAccountInitialized: true,
      recipientTokenAccountMintMustMatch: true,
      recipientTokenAccountOwnerMustMatch: true,
      rentExemptionHelperAvailable: true,
    },
    guarantees: [
      "REAL_SPL_TOKEN_MINT_TO_INSTRUCTION_IS_BUILT",
      "PDA_SIGNER_SEEDS_INCLUDE_REAL_BUMP",
      "INVOKE_SIGNED_BOUNDARY_EXISTS",
      "MINT_AND_RECIPIENT_TOKEN_ACCOUNT_VALIDATION_EXISTS",
      "PROCESSOR_NOT_WIRED_FOR_LIVE_ROUTE_EXECUTION_YET",
    ],
    nonGoals: [
      "NO_LIVE_ROUTE_ACTIVATION",
      "NO_DEPLOYMENT",
      "NO_AUTHORITY_FREEZE_EXECUTION",
      "NO_PROCESSED_EVENT_MUTATION_YET",
      "NO_RECIPIENT_BALANCE_MUTATION_YET",
    ],
  };
}

export function validateXXXLSplTokenMintToCpiFixture(
  fixture: XXXLSplTokenMintToCpiFixture = xxxlSplTokenMintToCpiFixture(),
): XXXLSplTokenMintToCpiFixtureValidationResult {
  const errors: XXXLSplTokenMintToCpiFixtureErrorCode[] = [];

  if (
    fixture.status !==
    XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_STATUS.FixtureOnlyNotLiveHandler
  ) {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.WrongStatus);
  }

  if (
    fixture.rustModules.cpi !== "programs/xxxl-svm/src/cpi.rs" ||
    fixture.rustModules.validation !== "programs/xxxl-svm/src/validation.rs" ||
    fixture.rustModules.pda !== "programs/xxxl-svm/src/pda.rs"
  ) {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingRustModule);
  }

  if (
    fixture.cpiInstruction.function !== "spl_token::instruction::mint_to" ||
    fixture.cpiInstruction.amountType !== "u64" ||
    !fixture.cpiInstruction.zeroAmountRejected
  ) {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingMintTo);
  }

  if (fixture.cpiInstruction.invoke !== "solana_program::program::invoke_signed") {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingInvokeSigned);
  }

  if (
    fixture.signer.pdaSeeds.join("/") !== "xxxl/gateway-mint-authority/v1" ||
    !fixture.signer.bumpRequired ||
    !fixture.signer.findProgramAddressRequired
  ) {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingPdaSigner);
  }

  if (
    !fixture.accountChecks.mintOwnedBySplToken ||
    !fixture.accountChecks.recipientTokenAccountOwnedBySplToken
  ) {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingOwnerCheck);
  }

  if (!fixture.accountChecks.rentExemptionHelperAvailable) {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingRentCheck);
  }

  if (
    !fixture.accountChecks.recipientTokenAccountInitialized ||
    !fixture.accountChecks.recipientTokenAccountMintMustMatch ||
    !fixture.accountChecks.recipientTokenAccountOwnerMustMatch
  ) {
    errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingAtaValidation);
  }

  for (const nonGoal of ["NO_LIVE_ROUTE_ACTIVATION", "NO_DEPLOYMENT"]) {
    if (!fixture.nonGoals.includes(nonGoal)) {
      errors.push(XXXL_SPL_TOKEN_MINT_TO_CPI_FIXTURE_ERROR.MissingNonGoal);
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalSplTokenMintToCpiFixtureJson(
  fixture: XXXLSplTokenMintToCpiFixture = xxxlSplTokenMintToCpiFixture(),
): string {
  return JSON.stringify([
    ["version", fixture.version],
    ["status", fixture.status],
    ["rustModules", fixture.rustModules],
    ["splTokenProgramId", fixture.splTokenProgramId],
    ["cpiInstruction", fixture.cpiInstruction],
    ["signer", fixture.signer],
    ["accountChecks", fixture.accountChecks],
    ["guarantees", fixture.guarantees],
    ["nonGoals", fixture.nonGoals],
  ]);
}

export function xxxlSplTokenMintToCpiFixtureMarkdown(
  fixture: XXXLSplTokenMintToCpiFixture = xxxlSplTokenMintToCpiFixture(),
): string {
  return [
    "# XXXL SPL Token mint_to CPI Fixture",
    "",
    `Status: ${fixture.status}`,
    "",
    "This fixture introduces the native SPL Token mint_to CPI boundary without enabling live route execution.",
    "",
    "CPI:",
    `- instruction: ${fixture.cpiInstruction.function}`,
    `- invoke: ${fixture.cpiInstruction.invoke}`,
    `- amount type: ${fixture.cpiInstruction.amountType}`,
    "",
    "Signer:",
    `- PDA seeds: ${fixture.signer.pdaSeeds.join(" / ")}`,
    "- bump required: true",
    "",
    "Guarantees:",
    ...fixture.guarantees.map((item) => `- ${item}`),
    "",
    "Non-goals:",
    ...fixture.nonGoals.map((item) => `- ${item}`),
    "",
  ].join("\n");
}

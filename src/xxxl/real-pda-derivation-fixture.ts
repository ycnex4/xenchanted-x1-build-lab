import {
  XXXL_X1_SVM_PORT_SCAFFOLD_STATUS,
  validateXXXLX1SvmPortScaffold,
} from "./x1-svm-port-scaffold.js";
import {
  XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
} from "./x1-svm-program-skeleton.js";

export const XXXL_REAL_PDA_DERIVATION_FIXTURE_VERSION = 1;

export const XXXL_REAL_PDA_DERIVATION_FIXTURE_STATUS = {
  FixtureBoundaryOnly: "REAL_PDA_DERIVATION_FIXTURE_BOUNDARY_ONLY",
} as const;

export type XXXLRealPdaDerivationFixtureStatus =
  (typeof XXXL_REAL_PDA_DERIVATION_FIXTURE_STATUS)[keyof typeof XXXL_REAL_PDA_DERIVATION_FIXTURE_STATUS];

export const XXXL_REAL_PDA_DERIVATION_FIXTURE_PROGRAM_ID =
  "11111111111111111111111111111111";

export const XXXL_REAL_PDA_DERIVATION_RUST_FUNCTION =
  "Pubkey::find_program_address";

export const XXXL_REAL_PDA_DERIVATION_RUST_MODULE =
  "programs/xxxl-svm/src/pda.rs";

export const XXXL_REAL_PDA_DERIVATION_CARGO_TEST =
  "cargo test --manifest-path programs/xxxl-svm/Cargo.toml pda -- --nocapture";

export const XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR = {
  ScaffoldInvalid: "SCAFFOLD_INVALID",
  WrongSeeds: "WRONG_SEEDS",
  PlaceholderAcceptedAsLive: "PLACEHOLDER_ACCEPTED_AS_LIVE",
  ModelOnlyPdaAcceptedAsLive: "MODEL_ONLY_PDA_ACCEPTED_AS_LIVE",
  MissingRealFindProgramAddress: "MISSING_REAL_FIND_PROGRAM_ADDRESS",
  MissingDeployTimeDependency: "MISSING_DEPLOY_TIME_DEPENDENCY",
  MissingRustFixture: "MISSING_RUST_FIXTURE",
  MissingCpiDependency: "MISSING_CPI_DEPENDENCY",
} as const;

export type XXXLRealPdaDerivationFixtureErrorCode =
  (typeof XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR)[keyof typeof XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR];

export type XXXLRealPdaDerivationFixture = {
  readonly version: typeof XXXL_REAL_PDA_DERIVATION_FIXTURE_VERSION;
  readonly status: XXXLRealPdaDerivationFixtureStatus;
  readonly scaffoldStatus: typeof XXXL_X1_SVM_PORT_SCAFFOLD_STATUS.ScaffoldOnly;
  readonly fixtureProgramId: typeof XXXL_REAL_PDA_DERIVATION_FIXTURE_PROGRAM_ID;
  readonly placeholderProgramId: typeof XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER;
  readonly placeholderAcceptedAsLive: false;
  readonly modelOnlyPdaAcceptedAsLive: false;
  readonly realFindProgramAddressRequired: true;
  readonly deployTimeProgramIdRequired: true;
  readonly rustFunction: typeof XXXL_REAL_PDA_DERIVATION_RUST_FUNCTION;
  readonly rustModule: typeof XXXL_REAL_PDA_DERIVATION_RUST_MODULE;
  readonly cargoTest: typeof XXXL_REAL_PDA_DERIVATION_CARGO_TEST;
  readonly seeds: readonly string[];
  readonly seedBytesHex: readonly string[];
  readonly cpiSignerDependency: {
    readonly requiredForInvokeSigned: true;
    readonly usedBySplTokenMintTo: true;
    readonly bumpRequired: true;
  };
  readonly guarantees: readonly string[];
  readonly nonGoals: readonly string[];
};

export type XXXLRealPdaDerivationFixtureValidationResult = {
  readonly ok: boolean;
  readonly errors: readonly XXXLRealPdaDerivationFixtureErrorCode[];
};

export function xxxlRealPdaDerivationFixture(): XXXLRealPdaDerivationFixture {
  return {
    version: XXXL_REAL_PDA_DERIVATION_FIXTURE_VERSION,
    status: XXXL_REAL_PDA_DERIVATION_FIXTURE_STATUS.FixtureBoundaryOnly,
    scaffoldStatus: XXXL_X1_SVM_PORT_SCAFFOLD_STATUS.ScaffoldOnly,
    fixtureProgramId: XXXL_REAL_PDA_DERIVATION_FIXTURE_PROGRAM_ID,
    placeholderProgramId: XXXL_X1_SVM_PROGRAM_ID_PLACEHOLDER,
    placeholderAcceptedAsLive: false,
    modelOnlyPdaAcceptedAsLive: false,
    realFindProgramAddressRequired: true,
    deployTimeProgramIdRequired: true,
    rustFunction: XXXL_REAL_PDA_DERIVATION_RUST_FUNCTION,
    rustModule: XXXL_REAL_PDA_DERIVATION_RUST_MODULE,
    cargoTest: XXXL_REAL_PDA_DERIVATION_CARGO_TEST,
    seeds: [...XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS],
    seedBytesHex: XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS.map((seed) =>
      Buffer.from(seed, "utf8").toString("hex"),
    ),
    cpiSignerDependency: {
      requiredForInvokeSigned: true,
      usedBySplTokenMintTo: true,
      bumpRequired: true,
    },
    guarantees: [
      "REAL_SVM_PUBKEY_FIND_PROGRAM_ADDRESS_IS_USED_IN_RUST_FIXTURE",
      "PDA_SEEDS_MATCH_MODEL_LAYER",
      "PDA_CHANGES_WITH_PROGRAM_ID",
      "PDA_DERIVATION_IS_DETERMINISTIC_FOR_A_GIVEN_PROGRAM_ID",
      "DEPLOY_TIME_PROGRAM_ID_REQUIRED_FOR_LIVE_PDA",
      "MODEL_ONLY_PDA_REJECTED_FOR_LIVE_USE",
    ],
    nonGoals: [
      "NO_DEPLOYMENT",
      "NO_LIVE_TRANSACTION_SUBMISSION",
      "NO_REAL_XXXL_PROGRAM_ID_YET",
      "NO_SPL_TOKEN_CPI_YET",
      "NO_ROUTE_ACTIVATION",
    ],
  };
}

export function validateXXXLRealPdaDerivationFixture(
  fixture: XXXLRealPdaDerivationFixture = xxxlRealPdaDerivationFixture(),
): XXXLRealPdaDerivationFixtureValidationResult {
  const errors: XXXLRealPdaDerivationFixtureErrorCode[] = [];
  const scaffoldValidation = validateXXXLX1SvmPortScaffold();

  if (!scaffoldValidation.ok) {
    errors.push(XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.ScaffoldInvalid);
  }

  if (
    fixture.seeds.length !== XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS.length ||
    !fixture.seeds.every(
      (seed, index) => seed === XXXL_X1_SVM_GATEWAY_MINT_AUTHORITY_PDA_SEEDS[index],
    ) ||
    fixture.seedBytesHex.join("|") !== "7878786c|676174657761792d6d696e742d617574686f72697479|7631"
  ) {
    errors.push(XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.WrongSeeds);
  }

  if (fixture.placeholderAcceptedAsLive) {
    errors.push(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.PlaceholderAcceptedAsLive,
    );
  }

  if (fixture.modelOnlyPdaAcceptedAsLive) {
    errors.push(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.ModelOnlyPdaAcceptedAsLive,
    );
  }

  if (fixture.rustFunction !== XXXL_REAL_PDA_DERIVATION_RUST_FUNCTION) {
    errors.push(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.MissingRealFindProgramAddress,
    );
  }

  if (!fixture.deployTimeProgramIdRequired) {
    errors.push(
      XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.MissingDeployTimeDependency,
    );
  }

  if (
    fixture.rustModule !== XXXL_REAL_PDA_DERIVATION_RUST_MODULE ||
    fixture.cargoTest !== XXXL_REAL_PDA_DERIVATION_CARGO_TEST
  ) {
    errors.push(XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.MissingRustFixture);
  }

  if (
    !fixture.cpiSignerDependency.requiredForInvokeSigned ||
    !fixture.cpiSignerDependency.usedBySplTokenMintTo ||
    !fixture.cpiSignerDependency.bumpRequired
  ) {
    errors.push(XXXL_REAL_PDA_DERIVATION_FIXTURE_ERROR.MissingCpiDependency);
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlCanonicalRealPdaDerivationFixtureJson(
  fixture: XXXLRealPdaDerivationFixture = xxxlRealPdaDerivationFixture(),
): string {
  return JSON.stringify([
    ["version", fixture.version],
    ["status", fixture.status],
    ["scaffoldStatus", fixture.scaffoldStatus],
    ["fixtureProgramId", fixture.fixtureProgramId],
    ["placeholderProgramId", fixture.placeholderProgramId],
    ["placeholderAcceptedAsLive", fixture.placeholderAcceptedAsLive],
    ["modelOnlyPdaAcceptedAsLive", fixture.modelOnlyPdaAcceptedAsLive],
    ["realFindProgramAddressRequired", fixture.realFindProgramAddressRequired],
    ["deployTimeProgramIdRequired", fixture.deployTimeProgramIdRequired],
    ["rustFunction", fixture.rustFunction],
    ["rustModule", fixture.rustModule],
    ["cargoTest", fixture.cargoTest],
    ["seeds", fixture.seeds],
    ["seedBytesHex", fixture.seedBytesHex],
    [
      "cpiSignerDependency",
      [
        [
          "requiredForInvokeSigned",
          fixture.cpiSignerDependency.requiredForInvokeSigned,
        ],
        ["usedBySplTokenMintTo", fixture.cpiSignerDependency.usedBySplTokenMintTo],
        ["bumpRequired", fixture.cpiSignerDependency.bumpRequired],
      ],
    ],
    ["guarantees", fixture.guarantees],
    ["nonGoals", fixture.nonGoals],
  ]);
}

export function xxxlRealPdaDerivationFixtureMarkdown(
  fixture: XXXLRealPdaDerivationFixture = xxxlRealPdaDerivationFixture(),
): string {
  return [
    "# XXXL Real PDA Derivation Fixture",
    "",
    `Status: ${fixture.status}`,
    `Rust function: ${fixture.rustFunction}`,
    `Rust module: ${fixture.rustModule}`,
    `Cargo test: ${fixture.cargoTest}`,
    "",
    "## Seeds",
    ...fixture.seeds.map((seed, index) => `- ${index}: ${seed}`),
    "",
    "## Guarantees",
    ...fixture.guarantees.map((guarantee) => `- ${guarantee}`),
    "",
    "## Non-goals",
    ...fixture.nonGoals.map((goal) => `- ${goal}`),
    "",
  ].join("\n");
}

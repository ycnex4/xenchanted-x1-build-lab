import {
  ETHEREUM_MAINNET_CHAIN_ID,
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MINT_TOKEN,
} from "./program-v1.js";

export const XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_POLICY_VERSION = 1;

export const XXXL_BPS_DENOMINATOR = 10_000;
export const XXXL_ETHEREUM_FULL_WEIGHT_BPS = 10_000;

export const AVALANCHE_MAINNET_CHAIN_ID = 43_114;

export const XXXL_AVALANCHE_HARD_MAX_WEIGHT_BPS = 25;
export const XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MIN_BPS = 5;
export const XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MAX_BPS = 10;

export const XXXL_MULTICHAIN_ROUTE_ID = {
  EthereumPrimary: XXXL_GATEWAY_ROUTE_ID,
  AvalancheLowWeight: "AVALANCHE_XNTD_TO_X1_XXXL",
} as const;

export const XXXL_MULTICHAIN_ROUTE_CLASS = {
  EthereumPrimaryFullWeight: "ETHEREUM_PRIMARY_FULL_WEIGHT",
  AvalancheLowWeight: "AVALANCHE_LOW_WEIGHT",
  OtherNonEthereumLowWeight: "OTHER_NON_ETHEREUM_LOW_WEIGHT",
} as const;

export type XXXLMultichainRouteClass =
  (typeof XXXL_MULTICHAIN_ROUTE_CLASS)[keyof typeof XXXL_MULTICHAIN_ROUTE_CLASS];

export const XXXL_MULTICHAIN_ROUTE_STATUS = {
  Candidate: "CANDIDATE",
  Active: "ACTIVE",
  Paused: "PAUSED",
  Retired: "RETIRED",
} as const;

export type XXXLMultichainRouteStatus =
  (typeof XXXL_MULTICHAIN_ROUTE_STATUS)[keyof typeof XXXL_MULTICHAIN_ROUTE_STATUS];

export type XXXLMultichainRouteCaps = {
  readonly perEventMintCap: bigint;
  readonly dailyRouteMintCap: bigint;
  readonly epochRouteMintCap: bigint;
  readonly globalNonEthereumSupplyShareCapBps: number;
};

export type XXXLMultichainLowWeightRouteCandidate = {
  readonly version: number;
  readonly routeId: string;
  readonly sourceChainId: bigint;
  readonly sourceChainName: string;
  readonly sourceToken: string;
  readonly sourceTokenSymbol: string;
  readonly targetMintToken: string;
  readonly routeClass: XXXLMultichainRouteClass;
  readonly sourceChainWeightBps: number;
  readonly status: XXXLMultichainRouteStatus;
  readonly guardianSetId: string;
  readonly finalityRuleId: string;
  readonly caps?: XXXLMultichainRouteCaps;
};

export const XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR = {
  UnsupportedVersion: "UNSUPPORTED_VERSION",
  DuplicateRouteId: "DUPLICATE_ROUTE_ID",
  EmptyRouteId: "EMPTY_ROUTE_ID",
  MissingPrimaryEthereumRoute: "MISSING_PRIMARY_ETHEREUM_ROUTE",
  WrongPrimaryEthereumRouteId: "WRONG_PRIMARY_ETHEREUM_ROUTE_ID",
  WrongEthereumSourceChain: "WRONG_ETHEREUM_SOURCE_CHAIN",
  WrongAvalancheSourceChain: "WRONG_AVALANCHE_SOURCE_CHAIN",
  WrongTargetMintToken: "WRONG_TARGET_MINT_TOKEN",
  InvalidWeightBps: "INVALID_WEIGHT_BPS",
  EthereumRouteMustBeFullWeight: "ETHEREUM_ROUTE_MUST_BE_FULL_WEIGHT",
  NonEthereumFullWeightNotAllowed: "NON_ETHEREUM_FULL_WEIGHT_NOT_ALLOWED",
  AvalancheWeightExceedsHardMax: "AVALANCHE_WEIGHT_EXCEEDS_HARD_MAX",
  OtherNonEthereumExceedsAvalancheWeight:
    "OTHER_NON_ETHEREUM_EXCEEDS_AVALANCHE_WEIGHT",
  MissingNonEthereumCaps: "MISSING_NON_ETHEREUM_CAPS",
  InvalidNonEthereumCaps: "INVALID_NON_ETHEREUM_CAPS",
  EmptyGuardianSetId: "EMPTY_GUARDIAN_SET_ID",
  EmptyFinalityRuleId: "EMPTY_FINALITY_RULE_ID",
} as const;

export type XXXLMultichainLowWeightRouteErrorCode =
  (typeof XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR)[keyof typeof XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR];

export type XXXLMultichainLowWeightRouteValidationResult = {
  readonly ok: boolean;
  readonly errors: XXXLMultichainLowWeightRouteErrorCode[];
};

export type XXXLMultichainRouteWeightRange = {
  readonly hardMaxBps: number;
  readonly conservativeInitialMinBps: number;
  readonly conservativeInitialMaxBps: number;
};

export function xxxlAvalancheLowWeightRange(): XXXLMultichainRouteWeightRange {
  return {
    hardMaxBps: XXXL_AVALANCHE_HARD_MAX_WEIGHT_BPS,
    conservativeInitialMinBps: XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MIN_BPS,
    conservativeInitialMaxBps: XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MAX_BPS,
  };
}

export function xxxlWeightedMintAmount(
  burnedSourceAmount: bigint,
  sourceChainWeightBps: number,
): bigint {
  if (
    burnedSourceAmount < 0n ||
    sourceChainWeightBps <= 0 ||
    sourceChainWeightBps > XXXL_BPS_DENOMINATOR
  ) {
    throw new Error("Invalid weighted mint input");
  }

  return (
    burnedSourceAmount * BigInt(sourceChainWeightBps)
  ) / BigInt(XXXL_BPS_DENOMINATOR);
}

function hasInvalidCaps(caps: XXXLMultichainRouteCaps): boolean {
  return (
    caps.perEventMintCap <= 0n ||
    caps.dailyRouteMintCap <= 0n ||
    caps.epochRouteMintCap <= 0n ||
    caps.perEventMintCap > caps.dailyRouteMintCap ||
    caps.dailyRouteMintCap > caps.epochRouteMintCap ||
    caps.globalNonEthereumSupplyShareCapBps <= 0 ||
    caps.globalNonEthereumSupplyShareCapBps > XXXL_BPS_DENOMINATOR
  );
}

function avalancheConfiguredWeightOrHardMax(
  routes: readonly XXXLMultichainLowWeightRouteCandidate[],
): number {
  const avalanche = routes.find(
    (route) =>
      route.routeClass === XXXL_MULTICHAIN_ROUTE_CLASS.AvalancheLowWeight,
  );

  return avalanche?.sourceChainWeightBps ?? XXXL_AVALANCHE_HARD_MAX_WEIGHT_BPS;
}

export function validateXXXLMultichainLowWeightRoutePolicy(
  routes: readonly XXXLMultichainLowWeightRouteCandidate[],
): XXXLMultichainLowWeightRouteValidationResult {
  const errors: XXXLMultichainLowWeightRouteErrorCode[] = [];
  const routeIds = new Set<string>();

  for (const route of routes) {
    if (routeIds.has(route.routeId)) {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.DuplicateRouteId);
    }
    routeIds.add(route.routeId);
  }

  const primaryEthereumRoute = routes.find(
    (route) =>
      route.routeClass ===
        XXXL_MULTICHAIN_ROUTE_CLASS.EthereumPrimaryFullWeight &&
      route.sourceChainId === BigInt(ETHEREUM_MAINNET_CHAIN_ID),
  );

  if (!primaryEthereumRoute) {
    errors.push(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.MissingPrimaryEthereumRoute,
    );
  }

  const otherNonEthereumLimit = avalancheConfiguredWeightOrHardMax(routes);

  for (const route of routes) {
    if (route.version !== XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_POLICY_VERSION) {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.UnsupportedVersion);
    }

    if (route.routeId.trim() === "") {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.EmptyRouteId);
    }

    if (route.guardianSetId.trim() === "") {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.EmptyGuardianSetId);
    }

    if (route.finalityRuleId.trim() === "") {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.EmptyFinalityRuleId);
    }

    if (route.targetMintToken !== XXXL_MINT_TOKEN) {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.WrongTargetMintToken);
    }

    if (
      route.sourceChainWeightBps <= 0 ||
      route.sourceChainWeightBps > XXXL_BPS_DENOMINATOR
    ) {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.InvalidWeightBps);
    }

    if (
      route.routeClass ===
      XXXL_MULTICHAIN_ROUTE_CLASS.EthereumPrimaryFullWeight
    ) {
      if (route.routeId !== XXXL_MULTICHAIN_ROUTE_ID.EthereumPrimary) {
        errors.push(
          XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.WrongPrimaryEthereumRouteId,
        );
      }

      if (route.sourceChainId !== BigInt(ETHEREUM_MAINNET_CHAIN_ID)) {
        errors.push(
          XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.WrongEthereumSourceChain,
        );
      }

      if (route.sourceChainWeightBps !== XXXL_ETHEREUM_FULL_WEIGHT_BPS) {
        errors.push(
          XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.EthereumRouteMustBeFullWeight,
        );
      }

      continue;
    }

    if (route.sourceChainWeightBps >= XXXL_ETHEREUM_FULL_WEIGHT_BPS) {
      errors.push(
        XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.NonEthereumFullWeightNotAllowed,
      );
    }

    if (!route.caps) {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.MissingNonEthereumCaps);
    } else if (hasInvalidCaps(route.caps)) {
      errors.push(XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.InvalidNonEthereumCaps);
    }

    if (route.routeClass === XXXL_MULTICHAIN_ROUTE_CLASS.AvalancheLowWeight) {
      if (route.sourceChainId !== BigInt(AVALANCHE_MAINNET_CHAIN_ID)) {
        errors.push(
          XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.WrongAvalancheSourceChain,
        );
      }

      if (route.sourceChainWeightBps > XXXL_AVALANCHE_HARD_MAX_WEIGHT_BPS) {
        errors.push(
          XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.AvalancheWeightExceedsHardMax,
        );
      }
    }

    if (
      route.routeClass ===
        XXXL_MULTICHAIN_ROUTE_CLASS.OtherNonEthereumLowWeight &&
      route.sourceChainWeightBps > otherNonEthereumLimit
    ) {
      errors.push(
        XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR
          .OtherNonEthereumExceedsAvalancheWeight,
      );
    }
  }

  return {
    ok: errors.length === 0,
    errors,
  };
}

export function xxxlEthereumPrimaryRouteCandidate(): XXXLMultichainLowWeightRouteCandidate {
  return {
    version: XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_POLICY_VERSION,
    routeId: XXXL_MULTICHAIN_ROUTE_ID.EthereumPrimary,
    sourceChainId: BigInt(ETHEREUM_MAINNET_CHAIN_ID),
    sourceChainName: "Ethereum",
    sourceToken: "XNTD_ETHEREUM_MAINNET",
    sourceTokenSymbol: "XNTD",
    targetMintToken: XXXL_MINT_TOKEN,
    routeClass: XXXL_MULTICHAIN_ROUTE_CLASS.EthereumPrimaryFullWeight,
    sourceChainWeightBps: XXXL_ETHEREUM_FULL_WEIGHT_BPS,
    status: XXXL_MULTICHAIN_ROUTE_STATUS.Active,
    guardianSetId: "guardian-set-1",
    finalityRuleId: "ethereum-finalized",
  };
}

export function xxxlAvalancheLowWeightRouteCandidate(
  weightBps = XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MAX_BPS,
): XXXLMultichainLowWeightRouteCandidate {
  return {
    version: XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_POLICY_VERSION,
    routeId: XXXL_MULTICHAIN_ROUTE_ID.AvalancheLowWeight,
    sourceChainId: BigInt(AVALANCHE_MAINNET_CHAIN_ID),
    sourceChainName: "Avalanche",
    sourceToken: "XNTD_AVALANCHE_MAINNET",
    sourceTokenSymbol: "XNTD",
    targetMintToken: XXXL_MINT_TOKEN,
    routeClass: XXXL_MULTICHAIN_ROUTE_CLASS.AvalancheLowWeight,
    sourceChainWeightBps: weightBps,
    status: XXXL_MULTICHAIN_ROUTE_STATUS.Candidate,
    guardianSetId: "guardian-set-avalanche-1",
    finalityRuleId: "avalanche-finalized",
    caps: {
      perEventMintCap: 1_000_000_000_000n,
      dailyRouteMintCap: 5_000_000_000_000n,
      epochRouteMintCap: 25_000_000_000_000n,
      globalNonEthereumSupplyShareCapBps: 100,
    },
  };
}

export function xxxlOtherLowWeightRouteCandidate(
  routeId: string,
  sourceChainId: bigint,
  weightBps: number,
): XXXLMultichainLowWeightRouteCandidate {
  return {
    version: XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_POLICY_VERSION,
    routeId,
    sourceChainId,
    sourceChainName: "Other non-Ethereum XC route",
    sourceToken: "XNTD_OTHER_NON_ETHEREUM",
    sourceTokenSymbol: "XNTD",
    targetMintToken: XXXL_MINT_TOKEN,
    routeClass: XXXL_MULTICHAIN_ROUTE_CLASS.OtherNonEthereumLowWeight,
    sourceChainWeightBps: weightBps,
    status: XXXL_MULTICHAIN_ROUTE_STATUS.Candidate,
    guardianSetId: "guardian-set-other-1",
    finalityRuleId: "other-finalized",
    caps: {
      perEventMintCap: 100_000_000_000n,
      dailyRouteMintCap: 500_000_000_000n,
      epochRouteMintCap: 2_500_000_000_000n,
      globalNonEthereumSupplyShareCapBps: 25,
    },
  };
}

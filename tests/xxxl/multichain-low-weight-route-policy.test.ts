import { describe, expect, it } from "vitest";

import {
  XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MAX_BPS,
  XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MIN_BPS,
  XXXL_AVALANCHE_HARD_MAX_WEIGHT_BPS,
  XXXL_BPS_DENOMINATOR,
  XXXL_ETHEREUM_FULL_WEIGHT_BPS,
  XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR,
  XXXL_MULTICHAIN_ROUTE_CLASS,
  validateXXXLMultichainLowWeightRoutePolicy,
  xxxlAvalancheLowWeightRange,
  xxxlAvalancheLowWeightRouteCandidate,
  xxxlEthereumPrimaryRouteCandidate,
  xxxlOtherLowWeightRouteCandidate,
  xxxlWeightedMintAmount,
} from "../../src/index.js";

describe("XXXL multichain low-weight route policy", () => {
  it("accepts an Ethereum-only initial route policy", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlEthereumPrimaryRouteCandidate(),
    ]);

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("accepts Ethereum plus conservative Avalanche low-weight route", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlEthereumPrimaryRouteCandidate(),
      xxxlAvalancheLowWeightRouteCandidate(10),
    ]);

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("requires Ethereum to remain the primary full-weight route", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      {
        ...xxxlEthereumPrimaryRouteCandidate(),
        sourceChainWeightBps: 9_999,
      },
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.EthereumRouteMustBeFullWeight,
    );
  });

  it("rejects route sets without a primary Ethereum route", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlAvalancheLowWeightRouteCandidate(10),
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.MissingPrimaryEthereumRoute,
    );
  });

  it("rejects full-weight non-Ethereum routes", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlEthereumPrimaryRouteCandidate(),
      {
        ...xxxlAvalancheLowWeightRouteCandidate(XXXL_ETHEREUM_FULL_WEIGHT_BPS),
      },
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.NonEthereumFullWeightNotAllowed,
    );
  });

  it("caps Avalanche at 25 bps hard maximum", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlEthereumPrimaryRouteCandidate(),
      xxxlAvalancheLowWeightRouteCandidate(XXXL_AVALANCHE_HARD_MAX_WEIGHT_BPS + 1),
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.AvalancheWeightExceedsHardMax,
    );
  });

  it("exposes conservative Avalanche initial range below the hard max", () => {
    const range = xxxlAvalancheLowWeightRange();

    expect(range.hardMaxBps).toBe(XXXL_AVALANCHE_HARD_MAX_WEIGHT_BPS);
    expect(range.conservativeInitialMinBps).toBe(
      XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MIN_BPS,
    );
    expect(range.conservativeInitialMaxBps).toBe(
      XXXL_AVALANCHE_CONSERVATIVE_INITIAL_MAX_BPS,
    );
    expect(range.conservativeInitialMaxBps).toBeLessThan(range.hardMaxBps);
  });

  it("requires other non-Ethereum routes to stay at or below configured Avalanche weight", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlEthereumPrimaryRouteCandidate(),
      xxxlAvalancheLowWeightRouteCandidate(10),
      xxxlOtherLowWeightRouteCandidate("BSC_XNTD_TO_X1_XXXL", 56n, 11),
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR
        .OtherNonEthereumExceedsAvalancheWeight,
    );
  });

  it("requires caps for every non-Ethereum route", () => {
    const { caps: _caps, ...routeWithoutCaps } =
      xxxlAvalancheLowWeightRouteCandidate(10);

    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlEthereumPrimaryRouteCandidate(),
      routeWithoutCaps,
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.MissingNonEthereumCaps,
    );
  });

  it("rejects invalid non-Ethereum caps", () => {
    const result = validateXXXLMultichainLowWeightRoutePolicy([
      xxxlEthereumPrimaryRouteCandidate(),
      {
        ...xxxlAvalancheLowWeightRouteCandidate(10),
        caps: {
          perEventMintCap: 10n,
          dailyRouteMintCap: 5n,
          epochRouteMintCap: 100n,
          globalNonEthereumSupplyShareCapBps: 0,
        },
      },
    ]);

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_MULTICHAIN_LOW_WEIGHT_ROUTE_ERROR.InvalidNonEthereumCaps,
    );
  });

  it("computes weighted XXXL mint amount using sourceChainWeightBps", () => {
    const oneThousandTokens = 1_000_000_000_000n;

    expect(
      xxxlWeightedMintAmount(oneThousandTokens, XXXL_BPS_DENOMINATOR),
    ).toBe(1_000_000_000_000n);

    expect(xxxlWeightedMintAmount(oneThousandTokens, 25)).toBe(
      2_500_000_000n,
    );

    expect(xxxlWeightedMintAmount(oneThousandTokens, 10)).toBe(
      1_000_000_000n,
    );

    expect(xxxlWeightedMintAmount(oneThousandTokens, 5)).toBe(
      500_000_000n,
    );
  });

  it("keeps non-Ethereum routes as low-weight historical access routes", () => {
    const avalanche = xxxlAvalancheLowWeightRouteCandidate(10);
    const other = xxxlOtherLowWeightRouteCandidate(
      "POLYGON_XNTD_TO_X1_XXXL",
      137n,
      5,
    );

    expect(avalanche.routeClass).toBe(
      XXXL_MULTICHAIN_ROUTE_CLASS.AvalancheLowWeight,
    );
    expect(other.routeClass).toBe(
      XXXL_MULTICHAIN_ROUTE_CLASS.OtherNonEthereumLowWeight,
    );
    expect(avalanche.sourceChainWeightBps).toBeLessThan(100);
    expect(other.sourceChainWeightBps).toBeLessThanOrEqual(
      avalanche.sourceChainWeightBps,
    );
  });
});

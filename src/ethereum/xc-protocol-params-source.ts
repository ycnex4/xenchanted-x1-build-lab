export interface XcProtocolParamsReadProvider {
  readContract(input: {
    readonly address: string;
    readonly abi: unknown;
    readonly functionName: string;
    readonly args?: readonly unknown[];
    readonly blockNumber?: bigint;
  }): Promise<unknown>;
}

export interface XcProtocolParams {
  readonly genesisTs: bigint;
  readonly halvingInterval: bigint;
  readonly xenBurnHalvingInterval: bigint;
  readonly currentEpoch: bigint;
  readonly nextHalvingTs: bigint;
  readonly initialNominal: bigint;
  readonly currentBaseNominal: bigint;
  readonly initialXenBurn: bigint;
  readonly currentXenBurnAmount: bigint;
  readonly enchantMultiplier: bigint;
  readonly maxLevel: number;
  readonly baseAprBpsNow: number;
  readonly bpsDenom: bigint;
  readonly earlyPenaltyBps: bigint;
  readonly maxWalletNfts: bigint;
}

export interface XcProtocolParamsSource {
  readProtocolParams(): Promise<XcProtocolParams>;
}

export interface XcProtocolParamsSourceConfig {
  readonly provider: XcProtocolParamsReadProvider;
  readonly lensAddress: string;
}

const GET_PROTOCOL_PARAMS_ABI = [
  {
    inputs: [],
    name: "getProtocolParams",
    outputs: [
      {
        components: [
          { internalType: "uint64", name: "genesisTs", type: "uint64" },
          { internalType: "uint256", name: "halvingInterval", type: "uint256" },
          { internalType: "uint256", name: "xenBurnHalvingInterval", type: "uint256" },
          { internalType: "uint256", name: "currentEpoch", type: "uint256" },
          { internalType: "uint256", name: "nextHalvingTs", type: "uint256" },
          { internalType: "uint256", name: "initialNominal", type: "uint256" },
          { internalType: "uint256", name: "currentBaseNominal", type: "uint256" },
          { internalType: "uint256", name: "initialXenBurn", type: "uint256" },
          { internalType: "uint256", name: "currentXenBurnAmount", type: "uint256" },
          { internalType: "uint256", name: "enchantMultiplier", type: "uint256" },
          { internalType: "uint8", name: "maxLevel", type: "uint8" },
          { internalType: "uint16", name: "baseAprBpsNow", type: "uint16" },
          { internalType: "uint256", name: "bpsDenom", type: "uint256" },
          { internalType: "uint256", name: "earlyPenaltyBps", type: "uint256" },
          { internalType: "uint256", name: "maxWalletNfts", type: "uint256" }
        ],
        name: "p",
        type: "tuple"
      }
    ],
    stateMutability: "view",
    type: "function"
  }
] as const;

const FIELD_NAMES = [
  "genesisTs",
  "halvingInterval",
  "xenBurnHalvingInterval",
  "currentEpoch",
  "nextHalvingTs",
  "initialNominal",
  "currentBaseNominal",
  "initialXenBurn",
  "currentXenBurnAmount",
  "enchantMultiplier",
  "maxLevel",
  "baseAprBpsNow",
  "bpsDenom",
  "earlyPenaltyBps",
  "maxWalletNfts"
] as const;

type FieldName = (typeof FIELD_NAMES)[number];

const ADDRESS_PATTERN = /^0x[a-fA-F0-9]{40}$/u;

export function createXcProtocolParamsSourceFromEthereumReadProvider(
  config: XcProtocolParamsSourceConfig
): XcProtocolParamsSource {
  if (!ADDRESS_PATTERN.test(config.lensAddress)) {
    throw new Error("Invalid XC protocol params source config: lensAddress");
  }

  return {
    async readProtocolParams(): Promise<XcProtocolParams> {
      let result: unknown;

      try {
        result = await config.provider.readContract({
          address: config.lensAddress,
          abi: GET_PROTOCOL_PARAMS_ABI,
          functionName: "getProtocolParams",
          args: []
        });
      } catch {
        throw new Error("Failed to read XC protocol params");
      }

      return normalizeXcProtocolParams(result);
    }
  };
}

export function normalizeXcProtocolParams(result: unknown): XcProtocolParams {
  return {
    genesisTs: readBigIntField(result, "genesisTs", 0),
    halvingInterval: readBigIntField(result, "halvingInterval", 1),
    xenBurnHalvingInterval: readBigIntField(result, "xenBurnHalvingInterval", 2),
    currentEpoch: readBigIntField(result, "currentEpoch", 3),
    nextHalvingTs: readBigIntField(result, "nextHalvingTs", 4),
    initialNominal: readBigIntField(result, "initialNominal", 5),
    currentBaseNominal: readBigIntField(result, "currentBaseNominal", 6),
    initialXenBurn: readBigIntField(result, "initialXenBurn", 7),
    currentXenBurnAmount: readBigIntField(result, "currentXenBurnAmount", 8),
    enchantMultiplier: readBigIntField(result, "enchantMultiplier", 9),
    maxLevel: readSafeNumberField(result, "maxLevel", 10),
    baseAprBpsNow: readSafeNumberField(result, "baseAprBpsNow", 11),
    bpsDenom: readBigIntField(result, "bpsDenom", 12),
    earlyPenaltyBps: readBigIntField(result, "earlyPenaltyBps", 13),
    maxWalletNfts: readBigIntField(result, "maxWalletNfts", 14)
  };
}

function readBigIntField(result: unknown, name: FieldName, index: number): bigint {
  const value = readTupleField(result, name, index);

  if (typeof value === "bigint") {
    return value;
  }

  if (typeof value === "number" && Number.isSafeInteger(value) && value >= 0) {
    return BigInt(value);
  }

  if (typeof value === "string" && /^[0-9]+$/u.test(value)) {
    return BigInt(value);
  }

  throw new Error(`Invalid XC protocol params result: invalid ${name}`);
}

function readSafeNumberField(result: unknown, name: FieldName, index: number): number {
  const value = readTupleField(result, name, index);

  if (typeof value === "number" && Number.isSafeInteger(value) && value >= 0) {
    return value;
  }

  if (typeof value === "bigint" && value <= BigInt(Number.MAX_SAFE_INTEGER)) {
    return Number(value);
  }

  if (typeof value === "string" && /^[0-9]+$/u.test(value)) {
    const parsed = Number(value);

    if (Number.isSafeInteger(parsed) && parsed >= 0) {
      return parsed;
    }
  }

  throw new Error(`Invalid XC protocol params result: invalid ${name}`);
}

function readTupleField(result: unknown, name: FieldName, index: number): unknown {
  if (typeof result !== "object" || result === null) {
    throw new Error("Invalid XC protocol params result: malformed tuple");
  }

  if (name in result) {
    const value = (result as Record<string, unknown>)[name];

    if (value !== undefined) {
      return value;
    }
  }

  if (Array.isArray(result) && index in result) {
    const value = result[index];

    if (value !== undefined) {
      return value;
    }
  }

  throw new Error(`Invalid XC protocol params result: missing ${name}`);
}

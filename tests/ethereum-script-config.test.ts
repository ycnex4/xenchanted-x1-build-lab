import { describe, expect, it } from "vitest";
import {
  parseEthereumScriptConfig,
  summarizeEthereumScriptConfig,
  type EthereumScriptConfigEnv
} from "../src/index.js";

const RPC_URL = "https://provider.example/SECRET_API_KEY";
const LENS_ADDRESS = "0x1111111111111111111111111111111111111111";

function validEnv(overrides: EthereumScriptConfigEnv = {}): EthereumScriptConfigEnv {
  return {
    XC_ETHEREUM_RPC_URL: RPC_URL,
    XC_ETHEREUM_CHAIN_ID: "eip155-1",
    XC_ETHEREUM_LENS_ADDRESS: LENS_ADDRESS,
    XC_ETHEREUM_FINALITY: "finalized",
    XC_ETHEREUM_LOCK_EPOCHS: "0,1,2",
    XC_ETHEREUM_REAL_RPC_CONFIRM: "I_UNDERSTAND_THIS_USES_REAL_RPC",
    ...overrides
  };
}

function expectSanitizedConfigError(env: EthereumScriptConfigEnv): void {
  try {
    parseEthereumScriptConfig(env);
    throw new Error("expected parseEthereumScriptConfig to throw");
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);

    expect(message).not.toContain(RPC_URL);
    expect(message).not.toContain("SECRET_API_KEY");
    expect(message).not.toContain("provider.example");
    expect(message).not.toContain("https://");
  }
}

describe("Ethereum script config parsing", () => {
  it("parses required env into config", () => {
    const config = parseEthereumScriptConfig(validEnv());

    expect(config).toEqual({
      rpcUrl: RPC_URL,
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "finalized" },
      lockEpochs: [0, 1, 2],
      epochMinimumFunctionName: "epochMinimum",
      realRpcConfirmed: true
    });
  });

  it("normalizes Lens address to lowercase", () => {
    const config = parseEthereumScriptConfig(
      validEnv({
        XC_ETHEREUM_LENS_ADDRESS: "0xABCDEFabcdefABCDEFabcdefABCDEFabcdefABCD"
      })
    );

    expect(config.lensAddress).toBe("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd");
  });

  it("parses safe finality", () => {
    const config = parseEthereumScriptConfig(
      validEnv({ XC_ETHEREUM_FINALITY: "safe" })
    );

    expect(config.finalityPolicy).toEqual({ kind: "safe" });
  });

  it("parses confirmed finality with confirmations", () => {
    const config = parseEthereumScriptConfig(
      validEnv({
        XC_ETHEREUM_FINALITY: "confirmed",
        XC_ETHEREUM_CONFIRMATIONS: "12"
      })
    );

    expect(config.finalityPolicy).toEqual({
      kind: "confirmed",
      confirmations: 12
    });
  });

  it("parses optional function name and ABI path", () => {
    const config = parseEthereumScriptConfig(
      validEnv({
        XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION: "customEpochMinimum",
        XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH: "./abi/xc-lens.json"
      })
    );

    expect(config.epochMinimumFunctionName).toBe("customEpochMinimum");
    expect(config.epochMinimumAbiPath).toBe("./abi/xc-lens.json");
  });

  it("creates safe summary without RPC URL", () => {
    const config = parseEthereumScriptConfig(
      validEnv({
        XC_ETHEREUM_EPOCH_MINIMUM_ABI_PATH: "./abi/xc-lens.json"
      })
    );

    const summary = summarizeEthereumScriptConfig(config);

    expect(summary).toEqual({
      chainId: "eip155-1",
      lensAddress: LENS_ADDRESS,
      finalityPolicy: { kind: "finalized" },
      lockEpochCount: 3,
      epochMinimumFunctionName: "epochMinimum",
      hasEpochMinimumAbiPath: true,
      realRpcConfirmed: true
    });

    expect(JSON.stringify(summary)).not.toContain(RPC_URL);
    expect(JSON.stringify(summary)).not.toContain("SECRET_API_KEY");
    expect(JSON.stringify(summary)).not.toContain("https://");
  });

  it("rejects missing RPC URL with sanitized error", () => {
    expect(() =>
      parseEthereumScriptConfig(validEnv({ XC_ETHEREUM_RPC_URL: "" }))
    ).toThrow("Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL");

    expectSanitizedConfigError(validEnv({ XC_ETHEREUM_RPC_URL: "" }));
  });

  it("rejects invalid chain ID", () => {
    expect(() =>
      parseEthereumScriptConfig(validEnv({ XC_ETHEREUM_CHAIN_ID: "1" }))
    ).toThrow("Invalid Ethereum script config: XC_ETHEREUM_CHAIN_ID");
  });

  it("rejects invalid Lens address", () => {
    expect(() =>
      parseEthereumScriptConfig(validEnv({ XC_ETHEREUM_LENS_ADDRESS: "0x123" }))
    ).toThrow("Invalid Ethereum script config: XC_ETHEREUM_LENS_ADDRESS");
  });

  it("rejects invalid finality", () => {
    expect(() =>
      parseEthereumScriptConfig(validEnv({ XC_ETHEREUM_FINALITY: "latest" }))
    ).toThrow("Invalid Ethereum script config: XC_ETHEREUM_FINALITY");
  });

  it("requires confirmations for confirmed finality", () => {
    expect(() =>
      parseEthereumScriptConfig(
        validEnv({
          XC_ETHEREUM_FINALITY: "confirmed",
          XC_ETHEREUM_CONFIRMATIONS: undefined
        })
      )
    ).toThrow("Missing required Ethereum script config: XC_ETHEREUM_CONFIRMATIONS");
  });

  it("rejects non-positive confirmations", () => {
    expect(() =>
      parseEthereumScriptConfig(
        validEnv({
          XC_ETHEREUM_FINALITY: "confirmed",
          XC_ETHEREUM_CONFIRMATIONS: "0"
        })
      )
    ).toThrow("Invalid Ethereum script config: XC_ETHEREUM_CONFIRMATIONS");
  });

  it("rejects empty lock epoch list", () => {
    expect(() =>
      parseEthereumScriptConfig(validEnv({ XC_ETHEREUM_LOCK_EPOCHS: " , " }))
    ).toThrow("Invalid Ethereum script config: XC_ETHEREUM_LOCK_EPOCHS");
  });

  it("rejects invalid lock epoch item", () => {
    expect(() =>
      parseEthereumScriptConfig(validEnv({ XC_ETHEREUM_LOCK_EPOCHS: "0,abc" }))
    ).toThrow("Invalid Ethereum script config: XC_ETHEREUM_LOCK_EPOCHS");
  });

  it("requires explicit real RPC confirmation", () => {
    expect(() =>
      parseEthereumScriptConfig(
        validEnv({ XC_ETHEREUM_REAL_RPC_CONFIRM: "YES" })
      )
    ).toThrow(
      "Missing required Ethereum script confirmation: XC_ETHEREUM_REAL_RPC_CONFIRM"
    );
  });

  it("rejects invalid function name", () => {
    expect(() =>
      parseEthereumScriptConfig(
        validEnv({ XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION: "123bad" })
      )
    ).toThrow(
      "Invalid Ethereum script config: XC_ETHEREUM_EPOCH_MINIMUM_FUNCTION"
    );
  });

  it("does not include RPC URL or API key in validation errors", () => {
    expectSanitizedConfigError(
      validEnv({ XC_ETHEREUM_CHAIN_ID: "invalid-chain" })
    );

    expectSanitizedConfigError(
      validEnv({ XC_ETHEREUM_FINALITY: "confirmed" })
    );

    expectSanitizedConfigError(
      validEnv({ XC_ETHEREUM_LOCK_EPOCHS: "0,not-a-number" })
    );
  });
});

import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { runCliCommand } from "../src/index.js";

async function writeFixtureFile(value: unknown): Promise<string> {
  const dir = await mkdtemp(join(tmpdir(), "x1-build-gateway-fixture-"));
  const filePath = join(dir, "profile-scan.json");

  await writeFile(filePath, JSON.stringify(value, null, 2), "utf8");

  return filePath;
}

describe("CLI gateway fixture preview command", () => {
  it("lists the fixture gateway preview command in help", async () => {
    const result = await runCliCommand(["help"]);

    expect(result.exitCode).toBe(0);
    expect(result.stdout).toContain("gateway:preview:fixture --file <path>");
  });

  it("returns an eligible JSON-safe preview DTO from a fixture file", async () => {
    const filePath = await writeFixtureFile({
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      scannedAt: "1200",
      validatedAt: "1300",
      coreRedeemCandidates: [
        {
          transactionHash: "tx-core-1",
          eventIndex: 0,
          amountBld: "121",
          coreTokenId: "1",
        },
      ],
      xenBurnCandidates: [
        {
          transactionHash: "tx-xen-1",
          eventIndex: 0,
          amountXbp: "1000",
          xenAmountBurned: "100000000",
        },
      ],
      xntdLockCandidate: {
        transactionHash: "tx-lock-1",
        eventIndex: 0,
        amountXntd: "100000000",
        observedRequiredXntdLock: "100000000",
        lockEpoch: 0,
      },
    });

    const result = await runCliCommand([
      "gateway:preview:fixture",
      "--file",
      filePath,
    ]);

    expect(result.exitCode).toBe(0);
    expect(result.stderr).toBe("");

    const parsed = JSON.parse(result.stdout) as {
      action: string;
      canCreateOrActivateBuild: boolean;
      preview: {
        totalPreviewHistoryBld: string;
        totalPreviewHistoryXbp: string;
        previewLockedXntd: string;
        previewRequiredXntdLock: string;
        missingRequirements: string[];
      };
    };

    expect(parsed.action).toBe("CREATE_BUILD");
    expect(parsed.canCreateOrActivateBuild).toBe(true);
    expect(parsed.preview.totalPreviewHistoryBld).toBe("121");
    expect(parsed.preview.totalPreviewHistoryXbp).toBe("1000");
    expect(parsed.preview.previewLockedXntd).toBe("100000000");
    expect(parsed.preview.previewRequiredXntdLock).toBe("100000000");
    expect(parsed.preview.missingRequirements).toEqual([]);
  });

  it("returns an unavailable preview DTO from a verified-zero fixture file", async () => {
    const filePath = await writeFixtureFile({
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      scannedAt: "1200",
      validatedAt: "1300",
    });

    const result = await runCliCommand([
      "gateway:preview:fixture",
      "--file",
      filePath,
    ]);

    expect(result.exitCode).toBe(0);
    expect(result.stderr).toBe("");

    const parsed = JSON.parse(result.stdout) as {
      action: string;
      canCreateOrActivateBuild: boolean;
      preview: {
        totalPreviewHistoryBld: string;
        previewLockedXntd: string;
        missingRequirements: string[];
      };
    };

    expect(parsed.action).toBe("UNAVAILABLE");
    expect(parsed.canCreateOrActivateBuild).toBe(false);
    expect(parsed.preview.totalPreviewHistoryBld).toBe("0");
    expect(parsed.preview.previewLockedXntd).toBe("0");
    expect(parsed.preview.missingRequirements).toEqual([
      "MINIMUM_CORE_REDEEM_HISTORY",
      "MINIMUM_XNTD_LOCK",
    ]);
  });

  it("returns structured failure for missing fixture file flag", async () => {
    const result = await runCliCommand(["gateway:preview:fixture"]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr).toBe("Missing required flag: --file\n");
  });

  it("returns structured failure for invalid fixture bigint fields", async () => {
    const filePath = await writeFixtureFile({
      buildId: "build-1",
      owner: "x1-owner",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      coreRedeemCandidates: [
        {
          amountBld: "not-a-number",
        },
      ],
    });

    const result = await runCliCommand([
      "gateway:preview:fixture",
      "--file",
      filePath,
    ]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr).toBe(
      "Fixture field coreRedeemCandidates[0].amountBld must be a decimal string\n",
    );
  });
});

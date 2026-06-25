import { describe, expect, it } from "vitest";
import { runCliCommand } from "../src/index.js";

describe("CLI gateway preview command", () => {
  it("lists the static gateway preview command in help", async () => {
    const result = await runCliCommand(["help"]);

    expect(result.exitCode).toBe(0);
    expect(result.stdout).toContain("gateway:preview:static --build-id <id>");
  });

  it("returns an eligible JSON-safe preview DTO from static CLI inputs", async () => {
    const result = await runCliCommand([
      "gateway:preview:static",
      "--build-id",
      "build-1",
      "--owner",
      "x1-owner",
      "--ethereum",
      "0x0000000000000000000000000000000000000001",
      "--core-bld",
      "121",
      "--xbp",
      "1000",
      "--xntd-lock",
      "100000000",
      "--required-xntd-lock",
      "100000000",
      "--lock-epoch",
      "0",
    ]);

    expect(result.exitCode).toBe(0);
    expect(result.stderr).toBe("");

    const parsed = JSON.parse(result.stdout) as {
      action: string;
      canCreateOrUpdateBuild: boolean;
      preview: {
        totalPreviewHistoryBld: string;
        totalPreviewHistoryXbp: string;
        previewLockedXntd: string;
        previewRequiredXntdLock: string;
        missingRequirements: string[];
      };
    };

    expect(parsed.action).toBe("CREATE_BUILD");
    expect(parsed.canCreateOrUpdateBuild).toBe(true);
    expect(parsed.preview.totalPreviewHistoryBld).toBe("121");
    expect(parsed.preview.totalPreviewHistoryXbp).toBe("1000");
    expect(parsed.preview.previewLockedXntd).toBe("100000000");
    expect(parsed.preview.previewRequiredXntdLock).toBe("100000000");
    expect(parsed.preview.missingRequirements).toEqual([]);
  });

  it("returns an unavailable preview DTO for verified-zero static CLI inputs", async () => {
    const result = await runCliCommand([
      "gateway:preview:static",
      "--build-id",
      "build-1",
      "--owner",
      "x1-owner",
      "--ethereum",
      "0x0000000000000000000000000000000000000001",
    ]);

    expect(result.exitCode).toBe(0);
    expect(result.stderr).toBe("");

    const parsed = JSON.parse(result.stdout) as {
      action: string;
      canCreateOrUpdateBuild: boolean;
      preview: {
        totalPreviewHistoryBld: string;
        previewLockedXntd: string;
        missingRequirements: string[];
      };
    };

    expect(parsed.action).toBe("UNAVAILABLE");
    expect(parsed.canCreateOrUpdateBuild).toBe(false);
    expect(parsed.preview.totalPreviewHistoryBld).toBe("0");
    expect(parsed.preview.previewLockedXntd).toBe("0");
    expect(parsed.preview.missingRequirements).toEqual([
      "MINIMUM_CORE_REDEEM_HISTORY",
      "MINIMUM_XNTD_LOCK",
    ]);
  });

  it("returns structured failure for missing required gateway preview flags", async () => {
    const result = await runCliCommand(["gateway:preview:static"]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr).toBe("Missing required flag: --build-id\n");
  });

  it("returns structured failure for invalid numeric gateway preview flags", async () => {
    const result = await runCliCommand([
      "gateway:preview:static",
      "--build-id",
      "build-1",
      "--owner",
      "x1-owner",
      "--ethereum",
      "0x0000000000000000000000000000000000000001",
      "--core-bld",
      "not-a-number",
    ]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr).toBe(
      "Flag --core-bld must be a non-negative integer\n",
    );
  });
});

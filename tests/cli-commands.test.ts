import { mkdtemp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import {
  appCreateBuild,
  createBuildApplicationState,
  parseCliArgs,
  runCliCommand,
  saveSnapshotFile
} from "../src/index.js";

describe("CLI command layer", () => {
  it("parses commands, positional args, and flags", () => {
    const parsed = parseCliArgs([
      "snapshot:show",
      "extra",
      "--file",
      "snapshot.json",
      "--json"
    ]);

    expect(parsed.command).toBe("snapshot:show");
    expect(parsed.positional).toEqual(["extra"]);
    expect(parsed.flags.get("file")).toBe("snapshot.json");
    expect(parsed.flags.get("json")).toBe(true);
  });

  it("returns help output", async () => {
    const result = await runCliCommand(["help"]);

    expect(result.exitCode).toBe(0);
    expect(result.stderr).toBe("");
    expect(result.stdout).toContain("xenchanted-x1-build-lab CLI");
    expect(result.stdout).toContain("snapshot:show --file <path>");
    expect(result.stdout).toContain("snapshot:verify --file <path>");
  });

  it("returns version output", async () => {
    const result = await runCliCommand(["version"]);

    expect(result.exitCode).toBe(0);
    expect(result.stdout).toBe("0.1.0\n");
    expect(result.stderr).toBe("");
  });

  it("shows snapshot summary", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-cli-"));
    const filePath = join(dir, "snapshot.json");

    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      createdAt: 100n
    });

    expect(created.ok).toBe(true);

    await saveSnapshotFile(filePath, app, 2000n);

    const result = await runCliCommand(["snapshot:show", "--file", filePath]);

    expect(result.exitCode).toBe(0);
    expect(result.stderr).toBe("");

    const parsed = JSON.parse(result.stdout) as {
      createdAt: string;
      buildCount: number;
      registrarAuthority: string;
      processedMessageCount: number;
      usedRedeemEventCount: number;
      usedXenBurnEventCount: number;
    };

    expect(parsed).toEqual({
      createdAt: "2000",
      buildCount: 1,
      registrarAuthority: "registrar-1",
      processedMessageCount: 0,
      usedRedeemEventCount: 0,
      usedXenBurnEventCount: 0
    });
  });

  it("returns structured failure for missing snapshot file flag", async () => {
    const result = await runCliCommand(["snapshot:show"]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr).toBe("Missing required flag: --file\n");
  });

  it("returns structured failure for unknown command", async () => {
    const result = await runCliCommand(["unknown"]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr).toBe("Unknown command: unknown\n");
  });

  it("verifies snapshot files", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-cli-verify-"));
    const filePath = join(dir, "snapshot.json");

    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      createdAt: 100n
    });

    expect(created.ok).toBe(true);

    await saveSnapshotFile(filePath, app, 3000n);

    const result = await runCliCommand(["snapshot:verify", "--file", filePath]);

    expect(result.exitCode).toBe(0);
    expect(result.stderr).toBe("");

    const parsed = JSON.parse(result.stdout) as {
      valid: boolean;
      createdAt: string;
      buildCount: number;
      registrarAuthority: string;
      processedMessageCount: number;
      usedRedeemEventCount: number;
      usedXenBurnEventCount: number;
    };

    expect(parsed).toEqual({
      valid: true,
      createdAt: "3000",
      buildCount: 1,
      registrarAuthority: "registrar-1",
      processedMessageCount: 0,
      usedRedeemEventCount: 0,
      usedXenBurnEventCount: 0
    });
  });

  it("returns structured failure for missing snapshot verify file flag", async () => {
    const result = await runCliCommand(["snapshot:verify"]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr).toBe("Missing required flag: --file\n");
  });

  it("returns structured failure for invalid snapshot verification", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-cli-invalid-"));
    const filePath = join(dir, "snapshot.json");

    await writeFile(filePath, "{not-json", "utf8");

    const result = await runCliCommand(["snapshot:verify", "--file", filePath]);

    expect(result.exitCode).toBe(1);
    expect(result.stdout).toBe("");
    expect(result.stderr.length).toBeGreaterThan(0);
  });

});

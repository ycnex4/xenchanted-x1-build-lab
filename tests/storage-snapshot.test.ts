import { mkdtemp, readFile, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import {
  appApplyRegistrarCoreRedeem,
  appApplyRegistrarX1FeeCheckpoint,
  acceptXntdCommitmentEvent,
  appApplyRegistrarXenBurn,
  appCreateBuild,
  createBuildApplicationState,
  decodeSnapshotJson,
  encodeSnapshotJson,
  loadSnapshotFile,
  loadSnapshotFileWithRecovery,
  saveSnapshotFile,
  saveSnapshotFileWithBackup,
  serializeBuildApplicationSnapshot,
  verifySnapshotFile,
  verifySnapshotJson
} from "../src/index.js";

describe("storage snapshot", () => {
  it("round-trips application state through snapshot serialization", () => {
    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 100n
    });

    expect(created.ok).toBe(true);

    if (!created.ok) {
      throw new Error("Build creation failed");
    }

    appApplyRegistrarCoreRedeem({
      app,
      message: {
        messageId: "message-core-redeem-1",
        kind: "CORE_REDEEM",
        submittedBy: "registrar-1",
        createdAt: 110n
      },
      build: created.value,
      redeemKey: "redeem-1",
      amountBld: 121n,
      redeemedAt: 110n
    });

    appApplyRegistrarXenBurn({
      app,
      message: {
        messageId: "message-xen-burn-1",
        kind: "XEN_BURN",
        submittedBy: "registrar-1",
        createdAt: 120n
      },
      build: created.value,
      xenBurnKey: "xen-burn-1",
      amountXbp: 1000n,
      burnedAt: 120n
    });

    appApplyRegistrarX1FeeCheckpoint({
      app,
      message: {
        messageId: "message-fee-1",
        kind: "X1_FEE_CHECKPOINT",
        submittedBy: "registrar-1",
        createdAt: 130n
      },
      build: created.value,
      feeAmount: 777n,
      txCount: 11n,
      countedUntilSlot: 9000n,
      updatedAt: 130n
    });

    acceptXntdCommitmentEvent(
      app.xntdCommitmentEvents,
      "xntd-commitment-1"
    );

    const snapshot = serializeBuildApplicationSnapshot(app, 1000n);
    const restored = decodeSnapshotJson(encodeSnapshotJson(snapshot));

    expect(restored.createdAt).toBe(1000n);
    expect(restored.app.registry.buildsById.get("build-1")).toEqual(
      created.value
    );
    expect(restored.app.registrar.processedMessages.size).toBe(3);
    expect(restored.app.redeemEvents.usedRedeemEvents.has("redeem-1")).toBe(
      true
    );
    expect(restored.app.xenBurnEvents.usedXenBurnEvents.has("xen-burn-1")).toBe(
      true
    );
    expect(
      restored.app.xntdCommitmentEvents.usedXntdCommitmentEvents.has(
        "xntd-commitment-1"
      )
    ).toBe(true);
  });

  it("saves and loads snapshot files", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-snapshot-"));
    const filePath = join(dir, "snapshot.json");

    const app = createBuildApplicationState("registrar-1");

    const created = appCreateBuild(app, {
      owner: "x1-owner",
      buildId: "build-1",
      createdAt: 100n
    });

    expect(created.ok).toBe(true);

    await saveSnapshotFile(filePath, app, 2000n);

    const raw = await readFile(filePath, "utf8");
    expect(raw).toContain('"kind": "BuildApplicationSnapshot"');
    expect(raw.endsWith("\n")).toBe(true);

    const loaded = await loadSnapshotFile(filePath);

    expect(loaded.createdAt).toBe(2000n);
    expect(loaded.app.registry.buildsById.get("build-1")?.owner).toBe(
      "x1-owner"
    );
    expect(loaded.app.registrar.registrarAuthority).toBe("registrar-1");
  });

  it("rejects invalid snapshot kind", () => {
    const app = createBuildApplicationState("registrar-1");
    const snapshot = serializeBuildApplicationSnapshot(app, 1000n);

    expect(() =>
      decodeSnapshotJson(
        JSON.stringify({
          ...snapshot,
          kind: "WrongKind"
        })
      )
    ).toThrow("Invalid serialized kind for BuildApplicationSnapshot");
  });

  it("rejects invalid snapshot schema version", () => {
    const app = createBuildApplicationState("registrar-1");
    const snapshot = serializeBuildApplicationSnapshot(app, 1000n);

    expect(() =>
      decodeSnapshotJson(
        JSON.stringify({
          ...snapshot,
          schemaVersion: 999
        })
      )
    ).toThrow("Unsupported schema version for BuildApplicationSnapshot");
  });

  it("verifies snapshot JSON and files", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-snapshot-verify-"));
    const filePath = join(dir, "snapshot.json");

    const app = createBuildApplicationState("registrar-1");
    const snapshot = serializeBuildApplicationSnapshot(app, 3000n);
    const json = encodeSnapshotJson(snapshot);

    const verifiedJson = verifySnapshotJson(json);

    expect(verifiedJson.createdAt).toBe(3000n);
    expect(verifiedJson.app.registrar.registrarAuthority).toBe("registrar-1");

    await saveSnapshotFile(filePath, app, 4000n);

    const verifiedFile = await verifySnapshotFile(filePath);

    expect(verifiedFile.createdAt).toBe(4000n);
    expect(verifiedFile.app.registrar.registrarAuthority).toBe("registrar-1");
  });

  it("rejects invalid snapshot JSON during verification", () => {
    expect(() => verifySnapshotJson("{not-json")).toThrow();
  });

  it("rejects invalid snapshot schema during verification", () => {
    const app = createBuildApplicationState("registrar-1");
    const snapshot = serializeBuildApplicationSnapshot(app, 1000n);

    expect(() =>
      verifySnapshotJson(
        JSON.stringify({
          ...snapshot,
          schemaVersion: 999
        })
      )
    ).toThrow("Unsupported schema version for BuildApplicationSnapshot");
  });

  it("saves snapshot with backup when canonical snapshot already exists", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-snapshot-backup-"));
    const filePath = join(dir, "snapshot.json");
    const backupPath = `${filePath}.bak`;

    const firstApp = createBuildApplicationState("registrar-1");

    const firstBuild = appCreateBuild(firstApp, {
      owner: "x1-owner-1",
      buildId: "build-1",
      createdAt: 100n
    });

    expect(firstBuild.ok).toBe(true);

    await saveSnapshotFile(filePath, firstApp, 1000n);

    const secondApp = createBuildApplicationState("registrar-1");

    const secondBuild = appCreateBuild(secondApp, {
      owner: "x1-owner-2",
      buildId: "build-2",
      createdAt: 200n
    });

    expect(secondBuild.ok).toBe(true);

    await saveSnapshotFileWithBackup(filePath, secondApp, 2000n);

    const canonical = await loadSnapshotFile(filePath);
    const backup = await loadSnapshotFile(backupPath);

    expect(canonical.createdAt).toBe(2000n);
    expect(canonical.app.registry.buildsById.has("build-2")).toBe(true);

    expect(backup.createdAt).toBe(1000n);
    expect(backup.app.registry.buildsById.has("build-1")).toBe(true);
  });

  it("saves snapshot without backup when canonical snapshot does not exist", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-snapshot-no-backup-"));
    const filePath = join(dir, "snapshot.json");
    const backupPath = `${filePath}.bak`;

    const app = createBuildApplicationState("registrar-1");

    await saveSnapshotFileWithBackup(filePath, app, 1000n);

    const canonical = await loadSnapshotFile(filePath);

    expect(canonical.createdAt).toBe(1000n);
    await expect(readFile(backupPath, "utf8")).rejects.toThrow();
  });

  it("rejects corrupted canonical snapshot before backup-enabled save", async () => {
    const dir = await mkdtemp(join(tmpdir(), "x1-build-snapshot-corrupt-"));
    const filePath = join(dir, "snapshot.json");
    const backupPath = `${filePath}.bak`;

    await writeFile(filePath, "{not-json", "utf8");

    const app = createBuildApplicationState("registrar-1");

    await expect(
      saveSnapshotFileWithBackup(filePath, app, 1000n)
    ).rejects.toThrow();

    const canonical = await readFile(filePath, "utf8");

    expect(canonical).toBe("{not-json");
    await expect(readFile(backupPath, "utf8")).rejects.toThrow();
  });

  it("loads canonical snapshot through recovery helper when canonical is valid", async () => {
    const dir = await mkdtemp(
      join(tmpdir(), "x1-build-snapshot-recovery-canonical-")
    );
    const filePath = join(dir, "snapshot.json");

    const app = createBuildApplicationState("registrar-1");

    await saveSnapshotFile(filePath, app, 1000n);

    const loaded = await loadSnapshotFileWithRecovery(filePath);

    expect(loaded.source).toBe("canonical");
    expect(loaded.filePath).toBe(filePath);
    expect(loaded.createdAt).toBe(1000n);
    expect(loaded.app.registrar.registrarAuthority).toBe("registrar-1");
  });

  it("loads backup snapshot through recovery helper when canonical is corrupted", async () => {
    const dir = await mkdtemp(
      join(tmpdir(), "x1-build-snapshot-recovery-backup-")
    );
    const filePath = join(dir, "snapshot.json");
    const backupPath = `${filePath}.bak`;

    const app = createBuildApplicationState("registrar-1");

    await saveSnapshotFile(backupPath, app, 1000n);
    await writeFile(filePath, "{not-json", "utf8");

    const loaded = await loadSnapshotFileWithRecovery(filePath);

    expect(loaded.source).toBe("backup");
    expect(loaded.filePath).toBe(backupPath);
    expect(loaded.createdAt).toBe(1000n);
    expect(loaded.app.registrar.registrarAuthority).toBe("registrar-1");

    const canonical = await readFile(filePath, "utf8");
    expect(canonical).toBe("{not-json");
  });

  it("supports custom backup path during recovery load", async () => {
    const dir = await mkdtemp(
      join(tmpdir(), "x1-build-snapshot-recovery-custom-")
    );
    const filePath = join(dir, "snapshot.json");
    const backupPath = join(dir, "backups", "snapshot.custom.bak");

    const app = createBuildApplicationState("registrar-1");

    await saveSnapshotFile(backupPath, app, 2000n);
    await writeFile(filePath, "{not-json", "utf8");

    const loaded = await loadSnapshotFileWithRecovery(filePath, {
      backupPath
    });

    expect(loaded.source).toBe("backup");
    expect(loaded.filePath).toBe(backupPath);
    expect(loaded.createdAt).toBe(2000n);
  });

  it("rejects recovery load when canonical and backup are both unavailable or invalid", async () => {
    const dir = await mkdtemp(
      join(tmpdir(), "x1-build-snapshot-recovery-fail-")
    );
    const filePath = join(dir, "snapshot.json");
    const backupPath = `${filePath}.bak`;

    await writeFile(filePath, "{not-json", "utf8");
    await writeFile(backupPath, "{also-not-json", "utf8");

    await expect(loadSnapshotFileWithRecovery(filePath)).rejects.toThrow(
      "Failed to load canonical snapshot or backup"
    );
  });

});

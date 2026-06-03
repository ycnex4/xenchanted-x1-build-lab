import { mkdtemp, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import {
  appApplyRegistrarCoreRedeem,
  appApplyRegistrarX1FeeCheckpoint,
  appApplyRegistrarXenBurn,
  appCreateBuild,
  createBuildApplicationState,
  decodeSnapshotJson,
  encodeSnapshotJson,
  loadSnapshotFile,
  saveSnapshotFile,
  serializeBuildApplicationSnapshot
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
});

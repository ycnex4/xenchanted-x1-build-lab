import { describe, expect, it } from "vitest";
import {
  createBuild,
  createEmptyBuildRegistry,
  createRedeemEventState,
  createRegisteredBuild,
  createRegistrarState,
  createXenBurnEventState,
  deserializeBuildRegistry,
  deserializeBuildState,
  deserializeRedeemEventState,
  deserializeRegistrarState,
  deserializeXenBurnEventState,
  serializeBuildRegistry,
  serializeBuildState,
  serializeRedeemEventState,
  serializeRegistrarState,
  serializeXenBurnEventState,
} from "../src/index.js";

describe("storage serialization", () => {
  it("round-trips BuildState with bigint values encoded as decimal strings", () => {
    const build = createBuild({
      owner: "x1-owner",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 100n,
      buildName: "Sergey Build",
      logoUri: "ipfs://logo",
    });

    build.updatedAt = 200n;
    build.metadataUpdatedAt = 250n;
    build.historyBld = 121n;
    build.originBld = 55n;
    build.historyXbp = 1000n;
    build.lockedXntd = 250n;
    build.requiredXntdLock = 250n;
    build.lockEpoch = 2;
    build.xcCommitmentActive = true;
    build.x1FeeContribution = 777n;
    build.x1TxCount = 11n;
    build.x1FeeCountedUntilSlot = 9000n;
    build.lastFeeUpdateAt = 300n;

    const serialized = serializeBuildState(build);

    expect(serialized.buildName).toBe("Sergey Build");
    expect(serialized.logoUri).toBe("ipfs://logo");
    expect(serialized.metadataUpdatedAt).toBe("250");
    expect(serialized.historyBld).toBe("121");
    expect(serialized.originBld).toBe("55");
    expect(serialized.historyXbp).toBe("1000");
    expect("availableBld" in serialized).toBe(false);
    expect("availableXbp" in serialized).toBe(false);
    expect("earnedXbp" in serialized).toBe(false);

    const restored = deserializeBuildState(serialized);

    expect(restored).toEqual(build);
  });

  it("round-trips BuildRegistry with indexes", () => {
    const registry = createEmptyBuildRegistry();

    const build = createRegisteredBuild(registry, {
      owner: "x1-owner",
      buildId: "build-1",
      ethereumIdentity: "0x0000000000000000000000000000000000000001",
      createdAt: 100n,
    });

    build.historyBld = 11n;

    const serialized = serializeBuildRegistry(registry);
    const restored = deserializeBuildRegistry(serialized);

    expect(restored.buildsById.get("build-1")).toEqual(build);
    expect(restored.canonicalBuildByOwner.get("x1-owner")).toBe("build-1");
    expect(
      restored.canonicalBuildByEthereumIdentity.get(
        "0x0000000000000000000000000000000000000001",
      ),
    ).toBe("build-1");
  });

  it("round-trips registrar and replay states", () => {
    const registrar = createRegistrarState("registrar-1");
    registrar.processedMessages.add("message-2");
    registrar.processedMessages.add("message-1");

    const redeemEvents = createRedeemEventState();
    redeemEvents.usedRedeemEvents.add("redeem-2");
    redeemEvents.usedRedeemEvents.add("redeem-1");

    const xenBurnEvents = createXenBurnEventState();
    xenBurnEvents.usedXenBurnEvents.add("xen-burn-2");
    xenBurnEvents.usedXenBurnEvents.add("xen-burn-1");

    const restoredRegistrar = deserializeRegistrarState(
      serializeRegistrarState(registrar),
    );
    const restoredRedeemEvents = deserializeRedeemEventState(
      serializeRedeemEventState(redeemEvents),
    );
    const restoredXenBurnEvents = deserializeXenBurnEventState(
      serializeXenBurnEventState(xenBurnEvents),
    );

    expect(restoredRegistrar.registrarAuthority).toBe("registrar-1");
    expect([...restoredRegistrar.processedMessages]).toEqual([
      "message-1",
      "message-2",
    ]);

    expect([...restoredRedeemEvents.usedRedeemEvents]).toEqual([
      "redeem-1",
      "redeem-2",
    ]);

    expect([...restoredXenBurnEvents.usedXenBurnEvents]).toEqual([
      "xen-burn-1",
      "xen-burn-2",
    ]);
  });

  it("rejects invalid bigint strings", () => {
    const build = createBuild({
      owner: "x1-owner",
      buildId: "build-1",
      createdAt: 100n,
    });

    const serialized = {
      ...serializeBuildState(build),
      historyBld: "1.5",
    };

    expect(() => deserializeBuildState(serialized)).toThrow(
      "historyBld must be a non-negative decimal bigint string",
    );
  });

  it("rejects duplicate set entries during deserialization", () => {
    const registrar = createRegistrarState("registrar-1");
    const serialized = {
      ...serializeRegistrarState(registrar),
      processedMessages: ["message-1", "message-1"],
    };

    expect(() => deserializeRegistrarState(serialized)).toThrow(
      "processedMessages contains duplicate value: message-1",
    );
  });
});

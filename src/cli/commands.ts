import { createBuildApplicationState } from "../app/build-service.js";
import { appGetGatewayProfilePreviewDtoFromScan } from "../app/gateway-profile-scan-preview.js";
import { loadGatewayProfilePreviewFixtureFile } from "../app/gateway-profile-scan-fixture.js";
import { createStaticGatewayProfileScanner } from "../app/gateway-profile-scan.js";
import {
  loadSnapshotFile,
  loadSnapshotFileWithRecovery,
  verifySnapshotFile,
} from "../storage/snapshot.js";
import {
  createCoreRedeemCandidate,
  createXenBurnCandidate,
  createXntdLockCandidate,
} from "../watchers/watcher-candidates.js";
import { type ParsedCliArgs, getStringFlag, parseCliArgs } from "./parse.js";

export interface CliCommandResult {
  exitCode: number;
  stdout: string;
  stderr: string;
}

export const CLI_VERSION = "0.1.0";

function ok(stdout: string): CliCommandResult {
  return {
    exitCode: 0,
    stdout,
    stderr: "",
  };
}

function fail(message: string): CliCommandResult {
  return {
    exitCode: 1,
    stdout: "",
    stderr: `${message}\n`,
  };
}

function requireStringFlag(parsed: ParsedCliArgs, name: string): string {
  const value = getStringFlag(parsed, name);

  if (value === undefined) {
    throw new Error(`Missing required flag: --${name}`);
  }

  return value;
}

function parseNonNegativeBigIntFlag(
  parsed: ParsedCliArgs,
  name: string,
  fallback: bigint,
): bigint {
  const raw = getStringFlag(parsed, name);

  if (raw === undefined) {
    return fallback;
  }

  if (!/^\d+$/.test(raw)) {
    throw new Error(`Flag --${name} must be a non-negative integer`);
  }

  return BigInt(raw);
}

function parseNonNegativeNumberFlag(
  parsed: ParsedCliArgs,
  name: string,
  fallback: number,
): number {
  const raw = getStringFlag(parsed, name);

  if (raw === undefined) {
    return fallback;
  }

  if (!/^\d+$/.test(raw)) {
    throw new Error(`Flag --${name} must be a non-negative integer`);
  }

  return Number(raw);
}

export function renderCliHelp(): string {
  return (
    [
      "xenchanted-x1-build-lab CLI",
      "",
      "Commands:",
      "  help",
      "  version",
      "  snapshot:show --file <path>",
      "  snapshot:verify --file <path>",
      "  snapshot:recover --file <path> [--backup <path>]",
      "  gateway:preview:static --build-id <id> --owner <x1-owner> --ethereum <address> [--core-bld <amount>] [--xbp <amount>] [--xntd-lock <amount>] [--required-xntd-lock <amount>] [--lock-epoch <number>]",
      "  gateway:preview:fixture --file <path>",
      "",
      "Notes:",
      "  This CLI layer is intentionally minimal.",
      "  It does not mutate protocol state yet.",
    ].join("\n") + "\n"
  );
}

export async function runCliCommand(args: string[]): Promise<CliCommandResult> {
  try {
    const parsed = parseCliArgs(args);

    if (parsed.command === "help") {
      return ok(renderCliHelp());
    }

    if (parsed.command === "version") {
      return ok(`${CLI_VERSION}\n`);
    }

    if (parsed.command === "gateway:preview:fixture") {
      const file = requireStringFlag(parsed, "file");
      const fixture = await loadGatewayProfilePreviewFixtureFile(file);

      const result = appGetGatewayProfilePreviewDtoFromScan({
        app: createBuildApplicationState("cli-fixture-registrar"),
        scanner: fixture.scanner,
        buildId: fixture.buildId,
        owner: fixture.owner,
        ethereumIdentity: fixture.ethereumIdentity,
        validatedAt: fixture.validatedAt,
      });

      return ok(JSON.stringify(result.dto, null, 2) + "\n");
    }

    if (parsed.command === "gateway:preview:static") {
      const buildId = requireStringFlag(parsed, "build-id");
      const owner = requireStringFlag(parsed, "owner");
      const ethereumIdentity = requireStringFlag(parsed, "ethereum");

      const coreBld = parseNonNegativeBigIntFlag(parsed, "core-bld", 0n);
      const xbp = parseNonNegativeBigIntFlag(parsed, "xbp", 0n);
      const xntdLock = parseNonNegativeBigIntFlag(parsed, "xntd-lock", 0n);
      const requiredXntdLock = parseNonNegativeBigIntFlag(
        parsed,
        "required-xntd-lock",
        xntdLock > 0n ? xntdLock : 0n,
      );
      const lockEpoch = parseNonNegativeNumberFlag(parsed, "lock-epoch", 0);
      const validatedAt = parseNonNegativeBigIntFlag(
        parsed,
        "validated-at",
        1000n,
      );
      const scannedAt = parseNonNegativeBigIntFlag(
        parsed,
        "scanned-at",
        validatedAt,
      );

      const coreRedeemCandidates =
        coreBld === 0n
          ? []
          : [
              createCoreRedeemCandidate({
                sourceChainId: "eip155-1",
                sourceAddress: "cli-core",
                eventKind: "CORE_REDEEM",
                transactionHash: "cli-core-redeem",
                eventIndex: 0,
                observedAt: scannedAt,
                finalized: true,
                buildId,
                owner,
                amountBld: coreBld,
                redeemedAt: scannedAt,
                coreTokenId: "cli-core-token",
              }),
            ];

      const xenBurnCandidates =
        xbp === 0n
          ? []
          : [
              createXenBurnCandidate({
                sourceChainId: "eip155-1",
                sourceAddress: "cli-xen",
                eventKind: "XEN_BURN",
                transactionHash: "cli-xen-burn",
                eventIndex: 0,
                observedAt: scannedAt,
                finalized: true,
                buildId,
                owner,
                amountXbp: xbp,
                burnedAt: scannedAt,
                xenAmountBurned: xbp,
              }),
            ];

      const xntdLockCandidate =
        xntdLock === 0n
          ? null
          : createXntdLockCandidate({
              sourceChainId: "eip155-1",
              sourceAddress: "cli-xntd-lock",
              eventKind: "XNTD_LOCK",
              transactionHash: "cli-xntd-lock",
              eventIndex: 0,
              observedAt: scannedAt,
              finalized: true,
              buildId,
              owner,
              amountXntd: xntdLock,
              observedRequiredXntdLock: requiredXntdLock,
              lockEpoch,
              lockedAt: scannedAt,
            });

      const scanner = createStaticGatewayProfileScanner({
        coreRedeemScanCompleted: true,
        xenBurnScanCompleted: true,
        xntdLockScanCompleted: true,
        coreRedeemCandidates,
        xenBurnCandidates,
        xntdLockCandidate,
        scannedAt,
      });

      const result = appGetGatewayProfilePreviewDtoFromScan({
        app: createBuildApplicationState("cli-static-registrar"),
        scanner,
        buildId,
        owner,
        ethereumIdentity,
        validatedAt,
      });

      return ok(JSON.stringify(result.dto, null, 2) + "\n");
    }

    if (parsed.command === "snapshot:recover") {
      const file = getStringFlag(parsed, "file");

      if (file === undefined) {
        return fail("Missing required flag: --file");
      }

      const backup = getStringFlag(parsed, "backup");
      const snapshot = await loadSnapshotFileWithRecovery(file, {
        ...(backup === undefined ? {} : { backupPath: backup }),
      });

      return ok(
        JSON.stringify(
          {
            recovered: true,
            source: snapshot.source,
            filePath: snapshot.filePath,
            createdAt: snapshot.createdAt.toString(10),
            buildCount: snapshot.app.registry.buildsById.size,
            registrarAuthority: snapshot.app.registrar.registrarAuthority,
            processedMessageCount:
              snapshot.app.registrar.processedMessages.size,
            usedRedeemEventCount:
              snapshot.app.redeemEvents.usedRedeemEvents.size,
            usedXenBurnEventCount:
              snapshot.app.xenBurnEvents.usedXenBurnEvents.size,
            usedXntdCommitmentEventCount:
              snapshot.app.xntdCommitmentEvents.usedXntdCommitmentEvents.size,
          },
          null,
          2,
        ) + "\n",
      );
    }

    if (parsed.command === "snapshot:verify") {
      const file = getStringFlag(parsed, "file");

      if (file === undefined) {
        return fail("Missing required flag: --file");
      }

      const snapshot = await verifySnapshotFile(file);

      return ok(
        JSON.stringify(
          {
            valid: true,
            createdAt: snapshot.createdAt.toString(10),
            buildCount: snapshot.app.registry.buildsById.size,
            registrarAuthority: snapshot.app.registrar.registrarAuthority,
            processedMessageCount:
              snapshot.app.registrar.processedMessages.size,
            usedRedeemEventCount:
              snapshot.app.redeemEvents.usedRedeemEvents.size,
            usedXenBurnEventCount:
              snapshot.app.xenBurnEvents.usedXenBurnEvents.size,
            usedXntdCommitmentEventCount:
              snapshot.app.xntdCommitmentEvents.usedXntdCommitmentEvents.size,
          },
          null,
          2,
        ) + "\n",
      );
    }

    if (parsed.command === "snapshot:show") {
      const file = getStringFlag(parsed, "file");

      if (file === undefined) {
        return fail("Missing required flag: --file");
      }

      const snapshot = await loadSnapshotFile(file);

      return ok(
        JSON.stringify(
          {
            createdAt: snapshot.createdAt.toString(10),
            buildCount: snapshot.app.registry.buildsById.size,
            registrarAuthority: snapshot.app.registrar.registrarAuthority,
            processedMessageCount:
              snapshot.app.registrar.processedMessages.size,
            usedRedeemEventCount:
              snapshot.app.redeemEvents.usedRedeemEvents.size,
            usedXenBurnEventCount:
              snapshot.app.xenBurnEvents.usedXenBurnEvents.size,
            usedXntdCommitmentEventCount:
              snapshot.app.xntdCommitmentEvents.usedXntdCommitmentEvents.size,
          },
          null,
          2,
        ) + "\n",
      );
    }

    return fail(`Unknown command: ${parsed.command}`);
  } catch (error) {
    if (error instanceof Error) {
      return fail(error.message);
    }

    return fail("Unknown CLI error");
  }
}

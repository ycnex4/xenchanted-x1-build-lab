import { loadSnapshotFile, verifySnapshotFile } from "../storage/snapshot.js";
import { parseCliArgs, getStringFlag } from "./parse.js";

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
    stderr: ""
  };
}

function fail(message: string): CliCommandResult {
  return {
    exitCode: 1,
    stdout: "",
    stderr: `${message}\n`
  };
}

export function renderCliHelp(): string {
  return [
    "xenchanted-x1-build-lab CLI",
    "",
    "Commands:",
    "  help",
    "  version",
    "  snapshot:show --file <path>",
    "  snapshot:verify --file <path>",
    "",
    "Notes:",
    "  This CLI layer is intentionally minimal.",
    "  It does not mutate protocol state yet."
  ].join("\n") + "\n";
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
              snapshot.app.xenBurnEvents.usedXenBurnEvents.size
          },
          null,
          2
        ) + "\n"
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
              snapshot.app.xenBurnEvents.usedXenBurnEvents.size
          },
          null,
          2
        ) + "\n"
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

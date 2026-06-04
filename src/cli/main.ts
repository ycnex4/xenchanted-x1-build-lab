#!/usr/bin/env node
import { fileURLToPath } from "node:url";
import { runCliCommand } from "./commands.js";

export interface CliMainIO {
  stdout: {
    write(chunk: string): void;
  };
  stderr: {
    write(chunk: string): void;
  };
}

export async function runCliMain(
  args: string[],
  io: CliMainIO = {
    stdout: process.stdout,
    stderr: process.stderr
  }
): Promise<number> {
  const result = await runCliCommand(args);

  if (result.stdout.length > 0) {
    io.stdout.write(result.stdout);
  }

  if (result.stderr.length > 0) {
    io.stderr.write(result.stderr);
  }

  return result.exitCode;
}

async function main(): Promise<void> {
  const exitCode = await runCliMain(process.argv.slice(2));
  process.exitCode = exitCode;
}

const entryPointPath = process.argv[1];

if (
  entryPointPath !== undefined &&
  fileURLToPath(import.meta.url) === entryPointPath
) {
  main().catch((error: unknown) => {
    const message = error instanceof Error ? error.message : "Unknown CLI error";
    process.stderr.write(`${message}\n`);
    process.exitCode = 1;
  });
}

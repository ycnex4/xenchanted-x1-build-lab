import { describe, expect, it } from "vitest";
import { runCliMain } from "../src/cli/main.js";

function createMemoryIo() {
  let stdout = "";
  let stderr = "";

  return {
    io: {
      stdout: {
        write(chunk: string): void {
          stdout += chunk;
        }
      },
      stderr: {
        write(chunk: string): void {
          stderr += chunk;
        }
      }
    },
    getStdout(): string {
      return stdout;
    },
    getStderr(): string {
      return stderr;
    }
  };
}

describe("CLI binary entry point", () => {
  it("writes help output and returns success exit code", async () => {
    const memory = createMemoryIo();

    const exitCode = await runCliMain(["help"], memory.io);

    expect(exitCode).toBe(0);
    expect(memory.getStdout()).toContain("xenchanted-x1-build-lab CLI");
    expect(memory.getStderr()).toBe("");
  });

  it("writes version output and returns success exit code", async () => {
    const memory = createMemoryIo();

    const exitCode = await runCliMain(["version"], memory.io);

    expect(exitCode).toBe(0);
    expect(memory.getStdout()).toBe("0.1.0\n");
    expect(memory.getStderr()).toBe("");
  });

  it("writes command failures to stderr and returns failure exit code", async () => {
    const memory = createMemoryIo();

    const exitCode = await runCliMain(["unknown"], memory.io);

    expect(exitCode).toBe(1);
    expect(memory.getStdout()).toBe("");
    expect(memory.getStderr()).toBe("Unknown command: unknown\n");
  });
});

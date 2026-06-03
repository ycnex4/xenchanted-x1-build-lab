export interface ParsedCliArgs {
  command: string;
  positional: string[];
  flags: Map<string, string | boolean>;
}

export function parseCliArgs(args: string[]): ParsedCliArgs {
  const positional: string[] = [];
  const flags = new Map<string, string | boolean>();

  let command = "";

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];

    if (arg === undefined) {
      continue;
    }

    if (arg.startsWith("--")) {
      const raw = arg.slice(2);

      if (raw.length === 0) {
        throw new Error("Flag name must not be empty");
      }

      const equalsIndex = raw.indexOf("=");

      if (equalsIndex >= 0) {
        const key = raw.slice(0, equalsIndex);
        const value = raw.slice(equalsIndex + 1);

        if (key.length === 0) {
          throw new Error("Flag name must not be empty");
        }

        flags.set(key, value);
        continue;
      }

      const next = args[index + 1];

      if (next !== undefined && !next.startsWith("--")) {
        flags.set(raw, next);
        index += 1;
      } else {
        flags.set(raw, true);
      }

      continue;
    }

    if (command.length === 0) {
      command = arg;
    } else {
      positional.push(arg);
    }
  }

  if (command.length === 0) {
    command = "help";
  }

  return {
    command,
    positional,
    flags
  };
}

export function getStringFlag(
  parsed: ParsedCliArgs,
  name: string
): string | undefined {
  const value = parsed.flags.get(name);

  if (value === undefined) {
    return undefined;
  }

  if (typeof value !== "string") {
    throw new Error(`Flag --${name} requires a value`);
  }

  return value;
}

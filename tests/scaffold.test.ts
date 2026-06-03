import { describe, expect, it } from "vitest";
import { BuildErrorCode } from "../src/index.js";

describe("scaffold", () => {
  it("loads exported types and constants", () => {
    expect(BuildErrorCode.NotImplemented).toBe("NOT_IMPLEMENTED");
  });
});

import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  assertAuthoritativeXcEpochMinimum,
  createStaticXcEpochMinimumSource
} from "../src/index.js";

describe("XC epoch minimum source", () => {
  it("accepts observed required XNTD lock matching authoritative epoch minimum", () => {
    const source = createStaticXcEpochMinimumSource(
      new Map<number, bigint>([
        [0, 100n],
        [1, 50n]
      ])
    );

    expect(() =>
      assertAuthoritativeXcEpochMinimum(source, 0, 100n)
    ).not.toThrow();

    expect(() =>
      assertAuthoritativeXcEpochMinimum(source, 1, 50n)
    ).not.toThrow();
  });

  it("rejects missing authoritative epoch minimum", () => {
    const source = createStaticXcEpochMinimumSource(
      new Map<number, bigint>([[0, 100n]])
    );

    expect(() =>
      assertAuthoritativeXcEpochMinimum(source, 2, 25n)
    ).toThrow(BuildError);

    try {
      assertAuthoritativeXcEpochMinimum(source, 2, 25n);
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.MissingAuthoritativeXcEpochMinimum
      );
    }
  });

  it("rejects observed required XNTD lock mismatch", () => {
    const source = createStaticXcEpochMinimumSource(
      new Map<number, bigint>([[0, 100n]])
    );

    expect(() =>
      assertAuthoritativeXcEpochMinimum(source, 0, 50n)
    ).toThrow(BuildError);

    try {
      assertAuthoritativeXcEpochMinimum(source, 0, 50n);
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.MismatchedAuthoritativeXcEpochMinimum
      );
    }
  });
});

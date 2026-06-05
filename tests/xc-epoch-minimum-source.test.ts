import { describe, expect, it } from "vitest";
import {
  BuildError,
  BuildErrorCode,
  assertAuthoritativeXcEpochMinimum,
  createStaticXcEpochMinimumSource,
  createXcEpochMinimumSourceFromRecords
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

  it("builds XC epoch minimum source from production-shaped records", () => {
    const source = createXcEpochMinimumSourceFromRecords([
      {
        lockEpoch: 0,
        minimumXntd: 100n,
        observedAt: 1000n,
        sourceChainId: "eip155-1",
        sourceBlockNumber: 100n,
        sourceBlockHash: "0xblock0"
      },
      {
        lockEpoch: 1,
        minimumXntd: 50n,
        observedAt: 1100n,
        sourceChainId: "eip155-1",
        sourceBlockNumber: 110n,
        sourceBlockHash: "0xblock1"
      }
    ]);

    expect(source.authoritativeEpochMinimum(0)).toBe(100n);
    expect(source.authoritativeEpochMinimum(1)).toBe(50n);
    expect(source.authoritativeEpochMinimum(2)).toBeNull();
  });

  it("allows duplicate XC epoch minimum records when minimums match", () => {
    const source = createXcEpochMinimumSourceFromRecords([
      {
        lockEpoch: 1,
        minimumXntd: 50n,
        observedAt: 1000n
      },
      {
        lockEpoch: 1,
        minimumXntd: 50n,
        observedAt: 1100n
      }
    ]);

    expect(source.authoritativeEpochMinimum(1)).toBe(50n);
  });

  it("rejects conflicting duplicate XC epoch minimum records", () => {
    expect(() =>
      createXcEpochMinimumSourceFromRecords([
        {
          lockEpoch: 1,
          minimumXntd: 50n,
          observedAt: 1000n
        },
        {
          lockEpoch: 1,
          minimumXntd: 40n,
          observedAt: 1100n
        }
      ])
    ).toThrow(BuildError);

    try {
      createXcEpochMinimumSourceFromRecords([
        {
          lockEpoch: 1,
          minimumXntd: 50n,
          observedAt: 1000n
        },
        {
          lockEpoch: 1,
          minimumXntd: 40n,
          observedAt: 1100n
        }
      ]);
    } catch (error) {
      expect(error).toBeInstanceOf(BuildError);
      expect((error as BuildError).code).toBe(
        BuildErrorCode.InvalidXntdLockAmount
      );
    }
  });

  it("rejects invalid XC epoch minimum records", () => {
    expect(() =>
      createXcEpochMinimumSourceFromRecords([
        {
          lockEpoch: 1,
          minimumXntd: 0n,
          observedAt: 1000n
        }
      ])
    ).toThrow(BuildError);

    expect(() =>
      createXcEpochMinimumSourceFromRecords([
        {
          lockEpoch: -1,
          minimumXntd: 100n,
          observedAt: 1000n
        }
      ])
    ).toThrow(BuildError);
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

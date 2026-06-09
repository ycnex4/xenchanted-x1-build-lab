import { describe, expect, it } from "vitest";

import {
  IntentionalReferenceFailure,
  ReferenceAtomicLedger,
  type ReferenceAccountState,
} from "../../src/prototypes/atomic-rollback-reference.js";

describe("atomic rollback reference model", () => {
  const accountA: ReferenceAccountState = {
    balance: 100n,
    nonce: 1,
    marker: "initial-a",
  };

  const accountB: ReferenceAccountState = {
    balance: 200n,
    nonce: 2,
    marker: "initial-b",
  };

  it("rolls back all account writes when an operation fails", () => {
    const ledger = new ReferenceAtomicLedger([
      ["account-a", accountA],
      ["account-b", accountB],
    ]);

    const before = ledger.snapshot();

    expect(() =>
      ledger.runAtomic((draft) => {
        draft.write("account-a", {
          balance: 111n,
          nonce: 11,
          marker: "mutated-a",
        });

        draft.write("account-b", {
          balance: 222n,
          nonce: 22,
          marker: "mutated-b",
        });

        throw new IntentionalReferenceFailure();
      }),
    ).toThrow(IntentionalReferenceFailure);

    expect(ledger.snapshot()).toEqual(before);
  });

  it("persists all account writes when an operation succeeds", () => {
    const ledger = new ReferenceAtomicLedger([
      ["account-a", accountA],
      ["account-b", accountB],
    ]);

    ledger.runAtomic((draft) => {
      draft.write("account-a", {
        balance: 111n,
        nonce: 11,
        marker: "mutated-a",
      });

      draft.write("account-b", {
        balance: 222n,
        nonce: 22,
        marker: "mutated-b",
      });
    });

    expect(ledger.get("account-a")).toEqual({
      balance: 111n,
      nonce: 11,
      marker: "mutated-a",
    });

    expect(ledger.get("account-b")).toEqual({
      balance: 222n,
      nonce: 22,
      marker: "mutated-b",
    });
  });

  it("keeps EV-01 and EV-02 open because this is not X1 runtime evidence", () => {
    const referenceOnly = {
      provesExpectedSemantics: true,
      provesX1RuntimeBehavior: false,
      ev01Closed: false,
      ev02Closed: false,
    };

    expect(referenceOnly).toEqual({
      provesExpectedSemantics: true,
      provesX1RuntimeBehavior: false,
      ev01Closed: false,
      ev02Closed: false,
    });
  });
});

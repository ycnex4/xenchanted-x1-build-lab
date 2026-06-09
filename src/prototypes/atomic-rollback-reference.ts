export type ReferenceAccountId = string;

export type ReferenceAccountState = {
  readonly balance: bigint;
  readonly nonce: number;
  readonly marker: string;
};

export type ReferenceLedgerState = ReadonlyMap<ReferenceAccountId, ReferenceAccountState>;

export class ReferenceAtomicLedger {
  private readonly accounts = new Map<ReferenceAccountId, ReferenceAccountState>();

  constructor(initialState: Iterable<readonly [ReferenceAccountId, ReferenceAccountState]> = []) {
    for (const [accountId, state] of initialState) {
      this.accounts.set(accountId, { ...state });
    }
  }

  get(accountId: ReferenceAccountId): ReferenceAccountState | undefined {
    const state = this.accounts.get(accountId);
    return state ? { ...state } : undefined;
  }

  snapshot(): ReferenceLedgerState {
    return new Map(
      Array.from(this.accounts.entries()).map(([accountId, state]) => [
        accountId,
        { ...state },
      ]),
    );
  }

  write(accountId: ReferenceAccountId, state: ReferenceAccountState): void {
    this.accounts.set(accountId, { ...state });
  }

  runAtomic(operation: (ledger: ReferenceAtomicLedger) => void): void {
    const before = this.snapshot();

    try {
      operation(this);
    } catch (error) {
      this.restore(before);
      throw error;
    }
  }

  private restore(snapshot: ReferenceLedgerState): void {
    this.accounts.clear();

    for (const [accountId, state] of snapshot.entries()) {
      this.accounts.set(accountId, { ...state });
    }
  }
}

export class IntentionalReferenceFailure extends Error {
  constructor(message = "Intentional reference failure") {
    super(message);
    this.name = "IntentionalReferenceFailure";
  }
}

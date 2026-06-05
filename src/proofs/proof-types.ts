export type ProofValidationStatus = "CANDIDATE" | "VALIDATED" | "REJECTED";

export type ProofKind =
  | "CORE_REDEEM_PROOF"
  | "XEN_BURN_PROOF"
  | "XNTD_LOCK_PROOF"
  | "XNTD_RELOCK_PROOF"
  | "X1_FEE_CHECKPOINT_PROOF"
  | "GENESIS_ORIGIN_ELIGIBILITY_PROOF";

export type CanonicalEventKey = string;

export interface CanonicalEventKeyInput {
  sourceChainId: string;
  sourceAddress: string;
  eventKind: string;
  transactionHash: string;
  eventIndex: number;
}

export interface ProofSourceMetadata {
  sourceChainId: string;
  sourceAddress: string;
  transactionHash: string;
  eventIndex: number;
  blockNumber: bigint | null;
  slot: bigint | null;
  observedAt: bigint | null;
  finalized: boolean;
}

export interface BaseProof {
  kind: ProofKind;
  status: ProofValidationStatus;
  source: ProofSourceMetadata;
  canonicalEventKey: CanonicalEventKey;
  validatedAt: bigint | null;
  rejectionReason: string | null;
}

export interface CoreRedeemProof extends BaseProof {
  kind: "CORE_REDEEM_PROOF";
  payload: {
    buildId: string;
    owner: string;
    redeemKey: CanonicalEventKey;
    amountBld: bigint;
    redeemedAt: bigint;
    coreTokenId: string | null;
  };
}

export interface XenBurnProof extends BaseProof {
  kind: "XEN_BURN_PROOF";
  payload: {
    buildId: string;
    owner: string;
    xenBurnKey: CanonicalEventKey;
    amountXbp: bigint;
    burnedAt: bigint;
    xenAmountBurned: bigint;
  };
}

export interface XntdLockProof extends BaseProof {
  kind: "XNTD_LOCK_PROOF";
  payload: {
    buildId: string;
    owner: string;
    amountXntd: bigint;
    observedRequiredXntdLock: bigint;
    lockEpoch: number;
    lockedAt: bigint;
  };
}

export interface XntdRelockProof extends BaseProof {
  kind: "XNTD_RELOCK_PROOF";
  payload: {
    buildId: string;
    owner: string;
    amountXntd: bigint;
    observedRequiredXntdLock: bigint;
    lockEpoch: number;
    relockedAt: bigint;
  };
}

export interface X1FeeCheckpointProof extends BaseProof {
  kind: "X1_FEE_CHECKPOINT_PROOF";
  payload: {
    buildId: string;
    feeAmount: bigint;
    txCount: bigint;
    countedUntilSlot: bigint;
    updatedAt: bigint;
  };
}

export interface GenesisOriginEligibilityProof extends BaseProof {
  kind: "GENESIS_ORIGIN_ELIGIBILITY_PROOF";
  payload: {
    buildId: string;
    owner: string;
    historyBld: bigint;
    eligibleOriginBld: bigint;
    snapshotId: string | null;
    claimedAt: bigint;
  };
}

export type BuildProof =
  | CoreRedeemProof
  | XenBurnProof
  | XntdLockProof
  | XntdRelockProof
  | X1FeeCheckpointProof
  | GenesisOriginEligibilityProof;

function requireNonEmptyString(value: string, field: string): void {
  if (value.length === 0) {
    throw new Error(`${field} must not be empty`);
  }

  if (value.includes(":")) {
    throw new Error(`${field} must not contain ':'`);
  }
}

export function createCanonicalEventKey(
  input: CanonicalEventKeyInput
): CanonicalEventKey {
  requireNonEmptyString(input.sourceChainId, "sourceChainId");
  requireNonEmptyString(input.sourceAddress, "sourceAddress");
  requireNonEmptyString(input.eventKind, "eventKind");
  requireNonEmptyString(input.transactionHash, "transactionHash");

  if (!Number.isInteger(input.eventIndex) || input.eventIndex < 0) {
    throw new Error("eventIndex must be a non-negative integer");
  }

  return [
    input.sourceChainId,
    input.sourceAddress,
    input.eventKind,
    input.transactionHash,
    String(input.eventIndex)
  ].join(":");
}

export function createProofSourceMetadata(
  input: CanonicalEventKeyInput & {
    blockNumber?: bigint | null;
    slot?: bigint | null;
    observedAt?: bigint | null;
    finalized?: boolean;
  }
): ProofSourceMetadata {
  return {
    sourceChainId: input.sourceChainId,
    sourceAddress: input.sourceAddress,
    transactionHash: input.transactionHash,
    eventIndex: input.eventIndex,
    blockNumber: input.blockNumber ?? null,
    slot: input.slot ?? null,
    observedAt: input.observedAt ?? null,
    finalized: input.finalized ?? false
  };
}

export function isValidatedProof(proof: BuildProof): boolean {
  return proof.status === "VALIDATED";
}

export function assertValidatedProof(proof: BuildProof): void {
  if (!isValidatedProof(proof)) {
    throw new Error(`Proof is not validated: ${proof.kind}`);
  }
}

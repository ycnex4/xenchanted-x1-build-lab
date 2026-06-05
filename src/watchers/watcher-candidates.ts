import {
  type CanonicalEventKey,
  type CanonicalEventKeyInput,
  createCanonicalEventKey,
  createProofSourceMetadata,
  type ProofSourceMetadata
} from "../proofs/proof-types.js";

export type WatcherCandidateKind =
  | "CORE_REDEEM_CANDIDATE"
  | "XEN_BURN_CANDIDATE"
  | "XNTD_LOCK_CANDIDATE"
  | "XNTD_RELOCK_CANDIDATE"
  | "X1_FEE_CHECKPOINT_CANDIDATE";

export interface WatcherCandidateBase {
  kind: WatcherCandidateKind;
  source: ProofSourceMetadata;
  canonicalEventKey: CanonicalEventKey;
  observedAt: bigint;
}

export interface CoreRedeemCandidate extends WatcherCandidateBase {
  kind: "CORE_REDEEM_CANDIDATE";
  payload: {
    buildId: string;
    owner: string;
    amountBld: bigint;
    redeemedAt: bigint;
    coreTokenId: string | null;
  };
}

export interface XenBurnCandidate extends WatcherCandidateBase {
  kind: "XEN_BURN_CANDIDATE";
  payload: {
    buildId: string;
    owner: string;
    amountXbp: bigint;
    burnedAt: bigint;
    xenAmountBurned: bigint;
  };
}

export interface XntdLockCandidate extends WatcherCandidateBase {
  kind: "XNTD_LOCK_CANDIDATE";
  payload: {
    buildId: string;
    owner: string;
    amountXntd: bigint;
    observedRequiredXntdLock: bigint;
    lockEpoch: number;
    lockedAt: bigint;
  };
}

export interface XntdRelockCandidate extends WatcherCandidateBase {
  kind: "XNTD_RELOCK_CANDIDATE";
  payload: {
    buildId: string;
    owner: string;
    amountXntd: bigint;
    observedRequiredXntdLock: bigint;
    lockEpoch: number;
    relockedAt: bigint;
  };
}

export interface X1FeeCheckpointCandidate extends WatcherCandidateBase {
  kind: "X1_FEE_CHECKPOINT_CANDIDATE";
  payload: {
    buildId: string;
    feeAmount: bigint;
    txCount: bigint;
    countedUntilSlot: bigint;
    updatedAt: bigint;
  };
}

export type WatcherCandidate =
  | CoreRedeemCandidate
  | XenBurnCandidate
  | XntdLockCandidate
  | XntdRelockCandidate
  | X1FeeCheckpointCandidate;

export interface CreateWatcherCandidateBaseInput
  extends CanonicalEventKeyInput {
  blockNumber?: bigint | null;
  slot?: bigint | null;
  observedAt: bigint;
  finalized?: boolean;
}

export function createWatcherCandidateBase(
  input: CreateWatcherCandidateBaseInput
): Pick<WatcherCandidateBase, "source" | "canonicalEventKey" | "observedAt"> {
  return {
    source: createProofSourceMetadata({
      ...input,
      observedAt: input.observedAt,
      finalized: input.finalized ?? false
    }),
    canonicalEventKey: createCanonicalEventKey(input),
    observedAt: input.observedAt
  };
}

export function isFinalizedWatcherCandidate(
  candidate: WatcherCandidate
): boolean {
  return candidate.source.finalized;
}

export function assertFinalizedWatcherCandidate(
  candidate: WatcherCandidate
): void {
  if (!isFinalizedWatcherCandidate(candidate)) {
    throw new Error(`Watcher candidate is not finalized: ${candidate.kind}`);
  }
}

export function createCoreRedeemCandidate(
  input: CreateWatcherCandidateBaseInput & CoreRedeemCandidate["payload"]
): CoreRedeemCandidate {
  return {
    kind: "CORE_REDEEM_CANDIDATE",
    ...createWatcherCandidateBase(input),
    payload: {
      buildId: input.buildId,
      owner: input.owner,
      amountBld: input.amountBld,
      redeemedAt: input.redeemedAt,
      coreTokenId: input.coreTokenId
    }
  };
}

export function createXenBurnCandidate(
  input: CreateWatcherCandidateBaseInput & XenBurnCandidate["payload"]
): XenBurnCandidate {
  return {
    kind: "XEN_BURN_CANDIDATE",
    ...createWatcherCandidateBase(input),
    payload: {
      buildId: input.buildId,
      owner: input.owner,
      amountXbp: input.amountXbp,
      burnedAt: input.burnedAt,
      xenAmountBurned: input.xenAmountBurned
    }
  };
}

export function createXntdLockCandidate(
  input: CreateWatcherCandidateBaseInput & XntdLockCandidate["payload"]
): XntdLockCandidate {
  return {
    kind: "XNTD_LOCK_CANDIDATE",
    ...createWatcherCandidateBase(input),
    payload: {
      buildId: input.buildId,
      owner: input.owner,
      amountXntd: input.amountXntd,
      observedRequiredXntdLock: input.observedRequiredXntdLock,
      lockEpoch: input.lockEpoch,
      lockedAt: input.lockedAt
    }
  };
}

export function createXntdRelockCandidate(
  input: CreateWatcherCandidateBaseInput & XntdRelockCandidate["payload"]
): XntdRelockCandidate {
  return {
    kind: "XNTD_RELOCK_CANDIDATE",
    ...createWatcherCandidateBase(input),
    payload: {
      buildId: input.buildId,
      owner: input.owner,
      amountXntd: input.amountXntd,
      observedRequiredXntdLock: input.observedRequiredXntdLock,
      lockEpoch: input.lockEpoch,
      relockedAt: input.relockedAt
    }
  };
}

export function createX1FeeCheckpointCandidate(
  input: CreateWatcherCandidateBaseInput & X1FeeCheckpointCandidate["payload"]
): X1FeeCheckpointCandidate {
  return {
    kind: "X1_FEE_CHECKPOINT_CANDIDATE",
    ...createWatcherCandidateBase(input),
    payload: {
      buildId: input.buildId,
      feeAmount: input.feeAmount,
      txCount: input.txCount,
      countedUntilSlot: input.countedUntilSlot,
      updatedAt: input.updatedAt
    }
  };
}

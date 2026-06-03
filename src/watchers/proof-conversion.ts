import {
  type BuildProof,
  type CoreRedeemProof,
  type X1FeeCheckpointProof,
  type XenBurnProof,
  type XntdLockProof,
  type XntdRelockProof
} from "../proofs/proof-types.js";
import {
  assertFinalizedWatcherCandidate,
  type CoreRedeemCandidate,
  type WatcherCandidate,
  type X1FeeCheckpointCandidate,
  type XenBurnCandidate,
  type XntdLockCandidate,
  type XntdRelockCandidate
} from "./watcher-candidates.js";

export interface WatcherProofConversionInput {
  validatedAt: bigint;
}

export function convertCoreRedeemCandidateToProof(
  candidate: CoreRedeemCandidate,
  input: WatcherProofConversionInput
): CoreRedeemProof {
  assertFinalizedWatcherCandidate(candidate);

  return {
    kind: "CORE_REDEEM_PROOF",
    status: "VALIDATED",
    source: candidate.source,
    canonicalEventKey: candidate.canonicalEventKey,
    validatedAt: input.validatedAt,
    rejectionReason: null,
    payload: {
      buildId: candidate.payload.buildId,
      owner: candidate.payload.owner,
      redeemKey: candidate.canonicalEventKey,
      amountBld: candidate.payload.amountBld,
      redeemedAt: candidate.payload.redeemedAt,
      coreTokenId: candidate.payload.coreTokenId
    }
  };
}

export function convertXenBurnCandidateToProof(
  candidate: XenBurnCandidate,
  input: WatcherProofConversionInput
): XenBurnProof {
  assertFinalizedWatcherCandidate(candidate);

  return {
    kind: "XEN_BURN_PROOF",
    status: "VALIDATED",
    source: candidate.source,
    canonicalEventKey: candidate.canonicalEventKey,
    validatedAt: input.validatedAt,
    rejectionReason: null,
    payload: {
      buildId: candidate.payload.buildId,
      owner: candidate.payload.owner,
      xenBurnKey: candidate.canonicalEventKey,
      amountXbp: candidate.payload.amountXbp,
      burnedAt: candidate.payload.burnedAt,
      xenAmountBurned: candidate.payload.xenAmountBurned
    }
  };
}

export function convertXntdLockCandidateToProof(
  candidate: XntdLockCandidate,
  input: WatcherProofConversionInput
): XntdLockProof {
  assertFinalizedWatcherCandidate(candidate);

  return {
    kind: "XNTD_LOCK_PROOF",
    status: "VALIDATED",
    source: candidate.source,
    canonicalEventKey: candidate.canonicalEventKey,
    validatedAt: input.validatedAt,
    rejectionReason: null,
    payload: {
      buildId: candidate.payload.buildId,
      owner: candidate.payload.owner,
      amountXntd: candidate.payload.amountXntd,
      lockEpoch: candidate.payload.lockEpoch,
      lockedAt: candidate.payload.lockedAt
    }
  };
}

export function convertXntdRelockCandidateToProof(
  candidate: XntdRelockCandidate,
  input: WatcherProofConversionInput
): XntdRelockProof {
  assertFinalizedWatcherCandidate(candidate);

  return {
    kind: "XNTD_RELOCK_PROOF",
    status: "VALIDATED",
    source: candidate.source,
    canonicalEventKey: candidate.canonicalEventKey,
    validatedAt: input.validatedAt,
    rejectionReason: null,
    payload: {
      buildId: candidate.payload.buildId,
      owner: candidate.payload.owner,
      amountXntd: candidate.payload.amountXntd,
      lockEpoch: candidate.payload.lockEpoch,
      relockedAt: candidate.payload.relockedAt
    }
  };
}

export function convertX1FeeCheckpointCandidateToProof(
  candidate: X1FeeCheckpointCandidate,
  input: WatcherProofConversionInput
): X1FeeCheckpointProof {
  assertFinalizedWatcherCandidate(candidate);

  return {
    kind: "X1_FEE_CHECKPOINT_PROOF",
    status: "VALIDATED",
    source: candidate.source,
    canonicalEventKey: candidate.canonicalEventKey,
    validatedAt: input.validatedAt,
    rejectionReason: null,
    payload: {
      buildId: candidate.payload.buildId,
      feeAmount: candidate.payload.feeAmount,
      txCount: candidate.payload.txCount,
      countedUntilSlot: candidate.payload.countedUntilSlot,
      updatedAt: candidate.payload.updatedAt
    }
  };
}

export function convertWatcherCandidateToProof(
  candidate: WatcherCandidate,
  input: WatcherProofConversionInput
): BuildProof {
  switch (candidate.kind) {
    case "CORE_REDEEM_CANDIDATE":
      return convertCoreRedeemCandidateToProof(candidate, input);

    case "XEN_BURN_CANDIDATE":
      return convertXenBurnCandidateToProof(candidate, input);

    case "XNTD_LOCK_CANDIDATE":
      return convertXntdLockCandidateToProof(candidate, input);

    case "XNTD_RELOCK_CANDIDATE":
      return convertXntdRelockCandidateToProof(candidate, input);

    case "X1_FEE_CHECKPOINT_CANDIDATE":
      return convertX1FeeCheckpointCandidateToProof(candidate, input);

    default: {
      const exhaustive: never = candidate;
      return exhaustive;
    }
  }
}

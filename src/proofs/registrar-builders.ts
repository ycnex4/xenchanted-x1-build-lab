import { type RegistrarMessage, type RegistrarMessageKind } from "../model/registrar.js";
import {
  assertValidatedProof,
  type BuildProof,
  type CoreRedeemProof,
  type X1FeeCheckpointProof,
  type XenBurnProof,
  type XntdLockProof,
  type XntdRelockProof
} from "./proof-types.js";

export interface CreateRegistrarPayloadInput {
  submittedBy: string;
  createdAt: bigint;
  messageId?: string;
}

export interface CoreRedeemRegistrarPayload {
  message: RegistrarMessage;
  buildId: string;
  redeemKey: string;
  amountBld: bigint;
  redeemedAt: bigint;
}

export interface XenBurnRegistrarPayload {
  message: RegistrarMessage;
  buildId: string;
  xenBurnKey: string;
  amountXbp: bigint;
  burnedAt: bigint;
}

export interface XntdLockRegistrarPayload {
  message: RegistrarMessage;
  buildId: string;
  xntdCommitmentEventKey: string;
  amountXntd: bigint;
  observedRequiredXntdLock: bigint;
  lockEpoch: number;
  lockedAt: bigint;
}

export interface XntdRelockRegistrarPayload {
  message: RegistrarMessage;
  buildId: string;
  xntdCommitmentEventKey: string;
  amountXntd: bigint;
  observedRequiredXntdLock: bigint;
  lockEpoch: number;
  relockedAt: bigint;
}

export interface X1FeeCheckpointRegistrarPayload {
  message: RegistrarMessage;
  buildId: string;
  feeAmount: bigint;
  txCount: bigint;
  countedUntilSlot: bigint;
  updatedAt: bigint;
}

export type RegistrarPayloadFromProof =
  | CoreRedeemRegistrarPayload
  | XenBurnRegistrarPayload
  | XntdLockRegistrarPayload
  | XntdRelockRegistrarPayload
  | X1FeeCheckpointRegistrarPayload;

function defaultMessageId(proof: BuildProof): string {
  return `proof:${proof.kind}:${proof.canonicalEventKey}`;
}

function createRegistrarMessage(
  proof: BuildProof,
  kind: RegistrarMessageKind,
  input: CreateRegistrarPayloadInput
): RegistrarMessage {
  return {
    messageId: input.messageId ?? defaultMessageId(proof),
    kind,
    submittedBy: input.submittedBy,
    createdAt: input.createdAt
  };
}

export function buildCoreRedeemRegistrarPayload(
  proof: CoreRedeemProof,
  input: CreateRegistrarPayloadInput
): CoreRedeemRegistrarPayload {
  assertValidatedProof(proof);

  return {
    message: createRegistrarMessage(proof, "CORE_REDEEM", input),
    buildId: proof.payload.buildId,
    redeemKey: proof.payload.redeemKey,
    amountBld: proof.payload.amountBld,
    redeemedAt: proof.payload.redeemedAt
  };
}

export function buildXenBurnRegistrarPayload(
  proof: XenBurnProof,
  input: CreateRegistrarPayloadInput
): XenBurnRegistrarPayload {
  assertValidatedProof(proof);

  return {
    message: createRegistrarMessage(proof, "XEN_BURN", input),
    buildId: proof.payload.buildId,
    xenBurnKey: proof.payload.xenBurnKey,
    amountXbp: proof.payload.amountXbp,
    burnedAt: proof.payload.burnedAt
  };
}

export function buildXntdLockRegistrarPayload(
  proof: XntdLockProof,
  input: CreateRegistrarPayloadInput
): XntdLockRegistrarPayload {
  assertValidatedProof(proof);

  return {
    message: createRegistrarMessage(proof, "LOCK_XNTD", input),
    buildId: proof.payload.buildId,
    xntdCommitmentEventKey: proof.canonicalEventKey,
    amountXntd: proof.payload.amountXntd,
    observedRequiredXntdLock: proof.payload.observedRequiredXntdLock,
    lockEpoch: proof.payload.lockEpoch,
    lockedAt: proof.payload.lockedAt
  };
}

export function buildXntdRelockRegistrarPayload(
  proof: XntdRelockProof,
  input: CreateRegistrarPayloadInput
): XntdRelockRegistrarPayload {
  assertValidatedProof(proof);

  return {
    message: createRegistrarMessage(proof, "RELOCK_XNTD", input),
    buildId: proof.payload.buildId,
    xntdCommitmentEventKey: proof.canonicalEventKey,
    amountXntd: proof.payload.amountXntd,
    observedRequiredXntdLock: proof.payload.observedRequiredXntdLock,
    lockEpoch: proof.payload.lockEpoch,
    relockedAt: proof.payload.relockedAt
  };
}

export function buildX1FeeCheckpointRegistrarPayload(
  proof: X1FeeCheckpointProof,
  input: CreateRegistrarPayloadInput
): X1FeeCheckpointRegistrarPayload {
  assertValidatedProof(proof);

  return {
    message: createRegistrarMessage(proof, "X1_FEE_CHECKPOINT", input),
    buildId: proof.payload.buildId,
    feeAmount: proof.payload.feeAmount,
    txCount: proof.payload.txCount,
    countedUntilSlot: proof.payload.countedUntilSlot,
    updatedAt: proof.payload.updatedAt
  };
}

export function buildRegistrarPayloadFromProof(
  proof: BuildProof,
  input: CreateRegistrarPayloadInput
): RegistrarPayloadFromProof {
  switch (proof.kind) {
    case "CORE_REDEEM_PROOF":
      return buildCoreRedeemRegistrarPayload(proof, input);

    case "XEN_BURN_PROOF":
      return buildXenBurnRegistrarPayload(proof, input);

    case "XNTD_LOCK_PROOF":
      return buildXntdLockRegistrarPayload(proof, input);

    case "XNTD_RELOCK_PROOF":
      return buildXntdRelockRegistrarPayload(proof, input);

    case "X1_FEE_CHECKPOINT_PROOF":
      return buildX1FeeCheckpointRegistrarPayload(proof, input);

    case "GENESIS_ORIGIN_ELIGIBILITY_PROOF":
      throw new Error("Genesis Origin proof does not map to a registrar payload");

    default: {
      const exhaustive: never = proof;
      return exhaustive;
    }
  }
}

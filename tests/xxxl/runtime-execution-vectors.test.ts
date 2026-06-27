import { describe, expect, it } from "vitest";

import {
  XXXL_GATEWAY_ROUTE_ID,
  XXXL_MULTICHAIN_ROUTE_ID,
  XXXL_RUNTIME_EXECUTION_VECTOR_ERROR,
  XXXL_RUNTIME_EXECUTION_VECTOR_ID,
  XXXL_RUNTIME_PROGRAM_SKELETON_ERROR,
  XXXL_RUNTIME_PROGRAM_SKELETON_STEP,
  validateXXXLRuntimeExecutionVectors,
  xxxlCanonicalRuntimeExecutionVectorJson,
  xxxlRuntimeExecutionVectors,
  type XXXLRuntimeExecutionVector,
} from "../../src/index.js";

function executionVectorAt(index: number): XXXLRuntimeExecutionVector {
  const vector = xxxlRuntimeExecutionVectors()[index];

  expect(vector).toBeDefined();

  return vector as XXXLRuntimeExecutionVector;
}

describe("XXXL runtime execution vectors", () => {
  it("exports all mandatory runtime execution vectors in deterministic order", () => {
    const vectors = xxxlRuntimeExecutionVectors();

    expect(vectors.map((vector) => vector.vectorId)).toEqual([
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InvalidRoutePolicyRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.MissingRouteRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.Stage1AuthorizationRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.EventKeyMismatchRejected,
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InstructionSerializationRejected,
    ]);
  });

  it("records valid Ethereum gateway mint execution", () => {
    const vector = executionVectorAt(0);

    expect(vector.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidEthereumGatewayMint,
    );
    expect(vector.routeId).toBe(XXXL_GATEWAY_ROUTE_ID);
    expect(vector.actualOk).toBe(true);
    expect(vector.actualExecuted).toBe(true);
    expect(vector.actualCpiSkipped).toBe(false);
    expect(vector.actualSupplyAuditOk).toBe(true);
    expect(vector.totalSupplyBefore).toBe(500n);
    expect(vector.totalSupplyAfter).toBe(1500n);
    expect(vector.recipientBalanceBefore).toBe(200n);
    expect(vector.recipientBalanceAfter).toBe(1200n);
  });

  it("records valid Avalanche low-weight route-aware execution", () => {
    const vector = executionVectorAt(1);

    expect(vector.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.ValidAvalancheLowWeightRoute,
    );
    expect(vector.routeId).toBe(XXXL_MULTICHAIN_ROUTE_ID.AvalancheLowWeight);
    expect(vector.routeId).not.toBe(XXXL_GATEWAY_ROUTE_ID);
    expect(vector.actualOk).toBe(true);
    expect(vector.actualExecuted).toBe(true);
    expect(vector.actualCpiSkipped).toBe(false);
    expect(vector.actualSupplyAuditOk).toBe(true);
  });

  it("records invalid route policy rejection before transition", () => {
    const vector = executionVectorAt(2);

    expect(vector.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InvalidRoutePolicyRejected,
    );
    expect(vector.actualOk).toBe(false);
    expect(vector.actualExecuted).toBe(false);
    expect(vector.actualErrors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyInvalid,
    ]);
    expect(vector.actualSteps).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.LoadAccounts,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateInstructionSerializationBoundary,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateRoutePolicy,
    ]);
  });

  it("records missing route rejection before transition", () => {
    const vector = executionVectorAt(3);

    expect(vector.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.MissingRouteRejected,
    );
    expect(vector.actualOk).toBe(false);
    expect(vector.actualErrors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.RoutePolicyMissingRoute,
    ]);
    expect(vector.actualCpiSkipped).toBe(true);
  });

  it("records Stage 1 authorization rejection without CPI", () => {
    const vector = executionVectorAt(4);

    expect(vector.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.Stage1AuthorizationRejected,
    );
    expect(vector.actualOk).toBe(false);
    expect(vector.actualExecuted).toBe(false);
    expect(vector.actualErrors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected,
    ]);
    expect(vector.actualCpiSkipped).toBe(true);
    expect(vector.actualSupplyAuditOk).toBe(false);
  });

  it("records replay rejection without CPI", () => {
    const vector = executionVectorAt(5);

    expect(vector.vectorId).toBe(XXXL_RUNTIME_EXECUTION_VECTOR_ID.ReplayRejected);
    expect(vector.actualOk).toBe(false);
    expect(vector.actualErrors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected,
    ]);
    expect(vector.actualCpiSkipped).toBe(true);
  });

  it("records event key mismatch rejection", () => {
    const vector = executionVectorAt(6);

    expect(vector.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.EventKeyMismatchRejected,
    );
    expect(vector.actualOk).toBe(false);
    expect(vector.actualErrors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.TransitionRejected,
    ]);
    expect(vector.actualCpiSkipped).toBe(true);
  });

  it("records instruction serialization boundary rejection before transition", () => {
    const vector = executionVectorAt(7);

    expect(vector.vectorId).toBe(
      XXXL_RUNTIME_EXECUTION_VECTOR_ID.InstructionSerializationRejected,
    );
    expect(vector.actualOk).toBe(false);
    expect(vector.actualErrors).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_ERROR.InstructionSerializationInvalid,
    ]);
    expect(vector.actualSteps).toEqual([
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.LoadAccounts,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateInstructionSerializationBoundary,
      XXXL_RUNTIME_PROGRAM_SKELETON_STEP.ValidateRoutePolicy,
    ]);
  });

  it("uses canonical JSON with bigint values serialized as decimal strings", () => {
    const vector = executionVectorAt(0);
    const canonicalJson = xxxlCanonicalRuntimeExecutionVectorJson(vector);

    expect(vector.canonicalJson).toBe(canonicalJson);
    expect(vector.canonicalJson).toContain('["totalSupplyBefore","500"]');
    expect(vector.canonicalJson).toContain('["totalSupplyAfter","1500"]');
    expect(vector.canonicalJson).toContain('["acceptedMintAmount","1000"]');
  });

  it("validates generated execution vector set", () => {
    const result = validateXXXLRuntimeExecutionVectors(
      xxxlRuntimeExecutionVectors(),
    );

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("detects duplicate vectors and wrong canonical JSON", () => {
    const vectors = xxxlRuntimeExecutionVectors();
    const firstVector = executionVectorAt(0);
    const duplicateResult = validateXXXLRuntimeExecutionVectors([
      ...vectors,
      firstVector,
    ]);

    expect(duplicateResult.ok).toBe(false);
    expect(duplicateResult.errors).toContain(
      XXXL_RUNTIME_EXECUTION_VECTOR_ERROR.DuplicateVector,
    );

    const wrongCanonicalResult = validateXXXLRuntimeExecutionVectors([
      {
        ...firstVector,
        canonicalJson: "wrong",
      },
      ...vectors.slice(1),
    ]);

    expect(wrongCanonicalResult.ok).toBe(false);
    expect(wrongCanonicalResult.errors).toContain(
      XXXL_RUNTIME_EXECUTION_VECTOR_ERROR.WrongCanonicalJson,
    );
  });
});

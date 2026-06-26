import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_ACCOUNT_KIND,
  XXXL_RUNTIME_AUTHORITY_SURFACE,
  XXXL_RUNTIME_GATEWAY_MINT_AUTHORITY_PDA_SEEDS,
  XXXL_RUNTIME_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY,
  XXXL_RUNTIME_INSTRUCTION,
  XXXL_RUNTIME_MANDATORY_AUTHORITY_SURFACES,
  XXXL_RUNTIME_MANDATORY_SERIALIZED_ACCOUNT_KINDS,
  XXXL_RUNTIME_MANDATORY_SERIALIZED_INSTRUCTIONS,
  XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY,
  XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR,
  XXXL_RUNTIME_SERIALIZATION_BOUNDARY_VERSION,
  XXXL_RUNTIME_SERIALIZATION_ENCODING,
  validateXXXLRuntimeSerializationBoundary,
  xxxlRuntimeDeterministicSerializationVectorPlan,
  xxxlRuntimeSupplyAuditFunctionShape,
  type XXXLRuntimeSerializationBoundaryCandidate,
} from "../../src/index.js";

function validBoundary(): XXXLRuntimeSerializationBoundaryCandidate {
  return {
    version: XXXL_RUNTIME_SERIALIZATION_BOUNDARY_VERSION,
    accountEncoding: XXXL_RUNTIME_SERIALIZATION_ENCODING.CanonicalBinaryV1,
    instructionEncoding: XXXL_RUNTIME_SERIALIZATION_ENCODING.CanonicalBinaryV1,
    accountKinds: [...XXXL_RUNTIME_MANDATORY_SERIALIZED_ACCOUNT_KINDS],
    instructionKinds: [...XXXL_RUNTIME_MANDATORY_SERIALIZED_INSTRUCTIONS],
    mintAuthorityPda: {
      strategy: XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY.GatewayMintAuthorityPda,
      seeds: [...XXXL_RUNTIME_GATEWAY_MINT_AUTHORITY_PDA_SEEDS],
      signsSplTokenMintToCpi: true,
    },
    authoritySurfaces: [...XXXL_RUNTIME_MANDATORY_AUTHORITY_SURFACES],
    cpiAtomicityNote: true,
    programUpgradeAuthoritySeparatedFromMintAuthority: true,
    authorityFreezeCoversProgramUpgradeAuthority: true,
    authorityFreezeCoversSplTokenMintAuthority: true,
    guardianSignatureVerificationBoundary:
      XXXL_RUNTIME_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY
        .Stage1AuthorizationResultOnly,
    supplyAuditFunctionPlanned: true,
    deterministicVectorsPlanned: true,
  };
}

describe("XXXL runtime serialization boundary", () => {
  it("accepts a valid runtime serialization boundary", () => {
    const result = validateXXXLRuntimeSerializationBoundary(validBoundary());

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("requires all runtime account kinds to be serialized", () => {
    const candidate = validBoundary();
    const result = validateXXXLRuntimeSerializationBoundary({
      ...candidate,
      accountKinds: candidate.accountKinds.filter(
        (kind) => kind !== XXXL_RUNTIME_ACCOUNT_KIND.ProcessedEvent,
      ),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingSerializedAccountKind,
    );
  });

  it("rejects duplicate serialized account kinds", () => {
    const candidate = validBoundary();
    const result = validateXXXLRuntimeSerializationBoundary({
      ...candidate,
      accountKinds: [
        ...candidate.accountKinds,
        XXXL_RUNTIME_ACCOUNT_KIND.MintState,
      ],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.DuplicateSerializedAccountKind,
    );
  });

  it("requires the consume gateway mint instruction to be serialized", () => {
    const result = validateXXXLRuntimeSerializationBoundary({
      ...validBoundary(),
      instructionKinds: [],
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingSerializedInstruction,
    );
  });

  it("requires the gateway mint authority PDA to sign SPL Token mint_to CPI", () => {
    const result = validateXXXLRuntimeSerializationBoundary({
      ...validBoundary(),
      mintAuthorityPda: {
        strategy: XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY.GatewayMintAuthorityPda,
        seeds: [...XXXL_RUNTIME_GATEWAY_MINT_AUTHORITY_PDA_SEEDS],
        signsSplTokenMintToCpi: false,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .MintAuthorityPdaDoesNotSignMintToCpi,
    );
  });

  it("requires the canonical mint authority PDA seeds", () => {
    const result = validateXXXLRuntimeSerializationBoundary({
      ...validBoundary(),
      mintAuthorityPda: {
        strategy: XXXL_RUNTIME_MINT_AUTHORITY_PDA_STRATEGY.GatewayMintAuthorityPda,
        seeds: ["xxxl", "v1"],
        signsSplTokenMintToCpi: true,
      },
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingMintAuthorityPdaSeed,
    );
  });

  it("requires explicit CPI atomicity note", () => {
    const result = validateXXXLRuntimeSerializationBoundary({
      ...validBoundary(),
      cpiAtomicityNote: false,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingCpiAtomicityNote,
    );
  });

  it("requires upgrade authority and SPL Token mint authority to be distinct freeze surfaces", () => {
    const result = validateXXXLRuntimeSerializationBoundary({
      ...validBoundary(),
      authoritySurfaces: [
        XXXL_RUNTIME_AUTHORITY_SURFACE.ProgramUpgradeAuthority,
      ],
      authorityFreezeCoversSplTokenMintAuthority: false,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR.MissingAuthoritySurface,
    );
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .FreezeDoesNotCoverSplTokenMintAuthority,
    );
  });

  it("keeps guardian signature verification outside the XXXL runtime boundary", () => {
    const result = validateXXXLRuntimeSerializationBoundary({
      ...validBoundary(),
      guardianSignatureVerificationBoundary:
        XXXL_RUNTIME_GUARDIAN_SIGNATURE_VERIFICATION_BOUNDARY
          .RuntimeEd25519Verification,
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_SERIALIZATION_BOUNDARY_ERROR
        .WrongGuardianSignatureVerificationBoundary,
    );
  });

  it("defines supply audit shape and deterministic serialization vector plan", () => {
    const audit = xxxlRuntimeSupplyAuditFunctionShape();
    const vectors = xxxlRuntimeDeterministicSerializationVectorPlan();

    expect(audit.readonlyOnly).toBe(true);
    expect(audit.invariant).toContain("mintState.totalSupply");
    expect(audit.invariant).toContain("splTokenMint.supply");
    expect(vectors).toHaveLength(6);
    expect(vectors.map((vector) => vector.target)).toContain(
      XXXL_RUNTIME_INSTRUCTION.ConsumeGatewayMint,
    );
  });
});

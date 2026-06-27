import { describe, expect, it } from "vitest";

import {
  XXXL_RUNTIME_TOOLING_ROADMAP_ERROR,
  XXXL_RUNTIME_TOOLING_ROADMAP_STATUS,
  XXXL_RUNTIME_TOOLING_ROADMAP_VERSION,
  validateXXXLRuntimeToolingRoadmap,
  xxxlCanonicalRuntimeToolingRoadmapJson,
  xxxlRuntimeToolingRoadmap,
  xxxlRuntimeToolingRoadmapMarkdown,
} from "../../src/index.js";

describe("XXXL runtime tooling roadmap", () => {
  it("exports roadmap metadata", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();

    expect(roadmap.version).toBe(XXXL_RUNTIME_TOOLING_ROADMAP_VERSION);
    expect(roadmap.status).toBe(XXXL_RUNTIME_TOOLING_ROADMAP_STATUS.Planned);
  });

  it("plans the immediate current runtime layer checks", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const stage = roadmap.stages.find(
      (item) => item.id === "current-runtime-layer-checks",
    );

    expect(stage?.hardGateTools).toContain("npm run typecheck");
    expect(stage?.hardGateTools).toContain("npm test -- --reporter=dot");
    expect(stage?.hardGateTools).toContain("npm run build");
    expect(stage?.hardGateTools).toContain("cargo test targeted modules");
  });

  it("plans rust quality and security baseline without premature clippy hard gate", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const stage = roadmap.stages.find(
      (item) => item.id === "rust-quality-security-baseline",
    );

    expect(stage?.hardGateTools).toContain("cargo fmt --check");
    expect(stage?.hardGateTools).toContain("cargo test");
    expect(stage?.hardGateTools).toContain("cargo audit");
    expect(stage?.hardGateTools).toContain("cargo deny check");
    expect(stage?.reportOnlyTools).toContain("cargo geiger");
    expect(stage?.hardGateTools).not.toContain(
      "cargo clippy --all-targets --all-features -- -D warnings",
    );
  });

  it("plans clippy as a later hard gate after warning cleanup", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const stage = roadmap.stages.find(
      (item) => item.id === "rust-clippy-warning-cleanup",
    );

    expect(stage?.hardGateTools).toEqual([
      "cargo clippy --all-targets --all-features -- -D warnings",
    ]);
  });

  it("plans manual account audit before guarded live handler wiring", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const stage = roadmap.stages.find(
      (item) => item.id === "manual-account-constraint-audit",
    );

    expect(stage?.timing).toBe("before-guarded-live-handler-wiring");
    expect(stage?.hardGateTools).toContain(
      "manual account-constraint audit checklist",
    );
  });

  it("plans Mollusk after guarded handler and Trident after Mollusk", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const mollusk = roadmap.stages.find(
      (item) => item.id === "mollusk-instruction-state-transition-tests",
    );
    const trident = roadmap.stages.find(
      (item) => item.id === "trident-fuzzing-invariants",
    );

    expect(mollusk?.timing).toBe("after-guarded-live-handler-wiring-model");
    expect(mollusk?.hardGateTools).toContain(
      "Mollusk instruction/state-transition tests",
    );
    expect(trident?.timing).toBe(
      "after-mollusk-transition-suite-and-invariant-catalog",
    );
    expect(trident?.hardGateTools).toContain("Trident fuzzing");
  });

  it("plans final predeploy gate with all major tools", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const stage = roadmap.stages.find(
      (item) => item.id === "predeploy-security-readiness-gate",
    );

    expect(stage?.hardGateTools).toContain("cargo fmt --check");
    expect(stage?.hardGateTools).toContain(
      "cargo clippy --all-targets --all-features -- -D warnings",
    );
    expect(stage?.hardGateTools).toContain("cargo test");
    expect(stage?.hardGateTools).toContain("cargo audit");
    expect(stage?.hardGateTools).toContain("cargo deny check");
    expect(stage?.hardGateTools).toContain("Mollusk transition suite");
    expect(stage?.hardGateTools).toContain("Trident fuzz suite");
    expect(stage?.hardGateTools).toContain(
      "manual account-constraint audit checklist",
    );
    expect(stage?.hardGateTools).toContain("manual authority/freeze checklist");
    expect(stage?.hardGateTools).toContain("manual deployment config checklist");
  });

  it("validates default roadmap", () => {
    const result = validateXXXLRuntimeToolingRoadmap();

    expect(result.ok).toBe(true);
    expect(result.errors).toEqual([]);
  });

  it("rejects missing security baseline", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const result = validateXXXLRuntimeToolingRoadmap({
      ...roadmap,
      stages: roadmap.stages.filter(
        (stage) => stage.id !== "rust-quality-security-baseline",
      ),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.MissingSecurityBaseline,
    );
  });

  it("rejects clippy hard gate too early", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const result = validateXXXLRuntimeToolingRoadmap({
      ...roadmap,
      stages: roadmap.stages.map((stage) =>
        stage.id === "rust-quality-security-baseline"
          ? {
              ...stage,
              hardGateTools: [
                ...stage.hardGateTools,
                "cargo clippy --all-targets --all-features -- -D warnings",
              ],
            }
          : stage,
      ),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.ClippyTooEarly,
    );
  });

  it("rejects heavy fuzzing too early", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const result = validateXXXLRuntimeToolingRoadmap({
      ...roadmap,
      stages: roadmap.stages.map((stage) =>
        stage.id === "current-runtime-layer-checks"
          ? {
              ...stage,
              hardGateTools: [...stage.hardGateTools, "Trident fuzzing"],
            }
          : stage,
      ),
    });

    expect(result.ok).toBe(false);
    expect(result.errors).toContain(
      XXXL_RUNTIME_TOOLING_ROADMAP_ERROR.HeavyFuzzingTooEarly,
    );
  });

  it("exports deterministic canonical roadmap JSON", () => {
    const roadmap = xxxlRuntimeToolingRoadmap();
    const json = xxxlCanonicalRuntimeToolingRoadmapJson(roadmap);

    expect(json).toBe(xxxlCanonicalRuntimeToolingRoadmapJson(roadmap));
    expect(json).toContain('["status","RUNTIME_TOOLING_ROADMAP_PLANNED"]');
    expect(json).toContain('"rust-quality-security-baseline"');
    expect(json).toContain('"predeploy-security-readiness-gate"');
  });

  it("exports markdown report", () => {
    const markdown = xxxlRuntimeToolingRoadmapMarkdown();

    expect(markdown).toContain("# XXXL Runtime Tooling Roadmap");
    expect(markdown).toContain("rust-quality-security-baseline");
    expect(markdown).toContain("Mollusk instruction/state-transition tests");
    expect(markdown).toContain("Trident fuzzing");
  });
});

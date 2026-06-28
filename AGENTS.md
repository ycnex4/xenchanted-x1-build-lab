# Repository Instructions for Codex

This repository uses strict staged development. Work through named branches, stages, and checkpoints. Keep changes minimal and targeted.

## Mandatory Workflow

- Always inspect `docs/checkpoints/current-design-checkpoint.md` before making changes.
- If the task is ambiguous, stop and explain the ambiguity before editing code.
- Prefer documentation-first boundaries before runtime behavior changes.
- Use WSL/bash command examples by default.
- Use `.local-logs/` for long command output, and remove `.local-logs/` before commit.
- Do not push to GitHub unless the user explicitly asks.
- Do not print or request secrets, private keys, mnemonics, RPC keys, API keys, or environment values.
- Always report exact files changed and exact validation commands run.
- Runtime safety is more important than making progress quickly.

## Source of Truth

Read these first when relevant:

- `README.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/xxxl/xxxl-deployment-roadmap-boundary.md`
- `docs/xxxl/xxxl-runtime-safety-review-package-boundary.md`
- `docs/xxxl/xxxl-account-contract-review-boundary.md`
- `docs/context/xc-x1-agent-context.md`
- `docs/context/codex-task-template.md`

## Runtime Safety Rules

Treat the XXXL SVM runtime as scaffold-only, locked, unreleasable, and not deployable by default.

- Never convert a documentation boundary into a runtime unlock.
- Never remove deployment blockers unless a separate reviewed boundary explicitly allows it.
- Never enable live route execution unless a separate reviewed boundary explicitly allows it.
- Never enable SPL CPI execution unless a separate reviewed boundary explicitly allows it.
- Never enable invoke_signed unless a separate reviewed boundary explicitly allows it.
- Never enable SPL Token mint_to unless a separate reviewed boundary explicitly allows it.
- Never change Program ID unless a separate reviewed boundary explicitly allows it.
- Never regenerate production PDA fixtures unless a separate reviewed boundary explicitly allows it.
- Never change deployability predicates unless a separate reviewed boundary explicitly allows it.
- Preserve `ACCOUNT_CONTRACT_UNREVIEWED` unless a future reviewed boundary explicitly clears it.
- Preserve `RUNTIME_SAFETY_LOCK_ACTIVE` unless a future reviewed boundary explicitly clears it.

## Preferred Validation

For docs-only changes:

```bash
git diff --check
git status --short
```

For XXXL SVM safety-related changes:

```bash
cd programs/xxxl-svm && cargo fmt --check
cd programs/xxxl-svm && cargo test account_contract --lib
cd programs/xxxl-svm && cargo test deployment_status --lib
cd programs/xxxl-svm && cargo test safety_invariant --lib
```

For runtime code changes, run broader focused and full tests appropriate to the touched code. Do not invent passing results.

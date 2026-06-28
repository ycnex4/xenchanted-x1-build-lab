# Codex Task Template

Use this template for future Codex tasks in this repository.

## Task Name

`<short task name>`

## Stage / Branch Name

`<stage name>`

Suggested branch format:

```bash
git switch -c codex/<stage-name>
```

## Source-of-Truth Files to Read First

- `README.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/context/xc-x1-agent-context.md`
- `docs/xxxl/xxxl-deployment-roadmap-boundary.md`
- `docs/xxxl/xxxl-runtime-safety-review-package-boundary.md`
- `docs/xxxl/xxxl-account-contract-review-boundary.md`
- `<task-specific files>`

## Allowed Changes

- `<allowed docs>`
- `<allowed tests>`
- `<allowed source files>`
- Keep changes minimal and tied to the named stage.

## Forbidden Changes

- Do not remove blockers unless explicitly allowed by the task and a reviewed boundary.
- Do not enable live route execution.
- Do not enable SPL CPI execution.
- Never enable invoke_signed unless a separate reviewed boundary explicitly allows it.
- Never enable SPL Token mint_to unless a separate reviewed boundary explicitly allows it.
- Do not change Program ID.
- Do not regenerate production PDA fixtures.
- Never change deployability predicates unless a separate reviewed boundary explicitly allows it.
- Do not print or request secrets, private keys, mnemonics, RPC keys, API keys, or environment values.

## Required Validation Commands

For docs-only tasks:

```bash
git diff --check
git status --short
```

For XXXL SVM safety-related tasks:

```bash
cd programs/xxxl-svm && cargo fmt --check
cd programs/xxxl-svm && cargo test account_contract --lib
cd programs/xxxl-svm && cargo test deployment_status --lib
cd programs/xxxl-svm && cargo test safety_invariant --lib
```

Add broader focused or full checks when runtime code changes. Report only commands actually run.

## Expected Report Format

Report:

- exact files changed
- exact validation commands run
- validation results
- whether runtime behavior changed
- whether any blockers were removed
- any remaining risks or follow-up work

## Safety Reminder

Deployment blockers must not be removed unless the task explicitly allows it and a separate reviewed boundary supports it. Documentation boundaries must not become runtime unlocks.

## Example Task

Task: XXXL account contract test gap closure

Stage / branch name:

```bash
codex/xxxl-account-contract-test-gap-closure
```

Source-of-truth files to read first:

- `README.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/context/xc-x1-agent-context.md`
- `docs/xxxl/xxxl-account-contract-review-boundary.md`
- `programs/xxxl-svm/src/account_contract.rs`
- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/cpi.rs`

Rules:

- Do not remove `ACCOUNT_CONTRACT_UNREVIEWED`.
- Do not enable live route.
- Do not enable SPL CPI.
- Never enable invoke_signed unless a separate reviewed boundary explicitly allows it.
- Never enable SPL Token mint_to unless a separate reviewed boundary explicitly allows it.
- Do not change Program ID.
- Never change deployability predicates unless a separate reviewed boundary explicitly allows it.
- Add or refine tests only where needed to close the documented account contract matrix.
- Update `docs/checkpoints/current-design-checkpoint.md` only if the stage produces a real checkpoint.
- Run relevant cargo tests and report results.

Goal:

Close documented account contract test gaps without enabling runtime execution.

# Build MVP Scaffold Notes

## Branch

build-mvp-scaffold

## Purpose

This branch prepares the initial implementation scaffold for the X1 Build MVP.

The branch must not implement real accounting logic.

## Current repository state before branch

The repository was documentation-only before this branch.

Existing top-level structure before scaffold:

- README.md
- docs/

## Scaffold decision

The initial implementation scaffold uses a TypeScript model-first approach.

This is not the final X1 on-chain program framework.

Purpose of this choice:

- test accounting invariants before on-chain implementation
- model BuildState and instruction behavior safely
- keep implementation small and reviewable
- avoid premature commitment to a specific X1 program framework

## Tooling

Current scaffold tooling:

- TypeScript
- Vitest
- Node.js / npm

Current commands:

- npm run typecheck
- npm test

## Created scaffold files

Top-level:

- package.json
- package-lock.json
- tsconfig.json
- .gitignore

Source:

- src/index.ts
- src/model/build-state.ts
- src/errors/build-error.ts
- src/instructions/create-build.ts

Tests:

- tests/scaffold.test.ts

## Current test status

Checked in WSL.

Environment:

- node v24.10.0
- npm 11.6.1

Results:

- npm install: completed
- npm run typecheck: passed
- npm test: passed
- Vitest: 1 test file passed, 1 test passed

npm audit reported vulnerabilities after install:

- 5 vulnerabilities
- 4 moderate
- 1 critical

No npm audit fix was applied in this scaffold step.

Reason:

- npm audit fix --force may introduce breaking dependency changes
- scaffold tests and typecheck pass
- dependency audit can be handled separately

## Scaffold constraints

Do not implement in scaffold branch unless explicitly moving to the next milestone:

- history_bld accounting
- available_bld accounting
- origin_bld accounting
- XBP accounting
- XNTD lock / relock
- registrar message processing
- replay protection logic
- fee checkpoint logic

Allowed in scaffold:

- folders
- placeholder modules
- placeholder tests
- config files
- implementation notes

## Next action

Next milestone after scaffold:

BuildState account / object.

The next branch should implement:

- owner
- build_id
- version
- created_at
- updated_at
- initial zeroed accounting fields

No real accounting transitions should be implemented until BuildState structure is stable.

# Build MVP Scaffold Notes

## Branch

build-mvp-scaffold

## Purpose

This branch prepares the initial implementation scaffold for the X1 Build MVP.

The branch must not implement real accounting logic.

## Current repository state

The repository was documentation-only before this branch.

Existing top-level structure:

- README.md
- docs/

## Scaffold decision pending

The exact program tooling is not chosen in this branch step yet.

Before creating code structure, decide:

- target X1 program framework
- language / SDK
- test framework
- local build command
- local test command

## Scaffold constraints

Do not implement:

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

Choose the implementation framework and create the minimal buildable scaffold.

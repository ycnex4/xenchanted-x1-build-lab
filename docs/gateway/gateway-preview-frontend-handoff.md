# Gateway preview frontend handoff

This document defines the handoff from the Build Lab gateway preview model to the future XC site gateway preview page.

The current stage is preview-only.

It does not connect live RPC scanners, does not submit proofs, does not create a Build, and does not update a Build.

## Source of truth

The future XC site page should be based on the human preview view model, not on raw scanner internals.

Current pipeline:

1. fixture or static scan input
2. gateway profile scan
3. JSON-safe preview DTO
4. human preview view model
5. generated human preview examples

The UI should consume the human preview shape:

- status
- tone
- title
- summary
- primaryActionLabel
- canProceed
- cards
- nextSteps

## Generated examples

Use these generated examples as stable UI fixtures:

- docs/gateway/generated/gateway-human-preview-create-build.json
- docs/gateway/generated/gateway-human-preview-update-build.json
- docs/gateway/generated/gateway-human-preview-unavailable.json

These represent the three initial page states:

- READY_TO_CREATE
- READY_TO_UPDATE
- NEEDS_REQUIREMENTS

## Proposed XC site page

Suggested route:

    /gateway-preview

Suggested page name:

    Gateway Preview

Alternative user-facing Russian name:

    Превью шлюза

## UI sections

The page should show:

- status header
- short summary
- primary action area
- Build status card
- Core redeem history card
- XEN burn power card
- XNTD lock card
- requirements card
- next steps list
- technical preview note

The first version may be static/demo-driven using generated JSON examples.

Live RPC scanning can be connected later.

## Status mapping

READY_TO_CREATE

Meaning:

- Build does not exist yet
- profile satisfies requirements
- user can create Build later when submit flow exists

Primary action label:

    Create Build

READY_TO_UPDATE

Meaning:

- Build already exists
- profile can update Build state later when submit flow exists

Primary action label:

    Update Build

NEEDS_REQUIREMENTS

Meaning:

- requirements are not complete yet
- nextSteps explains what is missing
- this is not a permanent rejection

Primary action label:

    none

## Important boundaries

The frontend preview page must not imply that the gateway is live unless live scanners and submit flow are actually connected.

The initial page should say clearly:

- preview mode
- fixture/static examples
- no live RPC scan yet
- no transaction submission yet
- no Build creation or update yet

## Later live version

A later live version can replace fixture input with real scanner adapters:

- Core redeem scanner
- XEN burn scanner
- XNTD lock scanner
- existing Build lookup
- profile validation
- submit proof bundle

Until then, the page is a transparent preview of the planned user experience.
